use super::math::sin;
use libm::{fmodf as fmod};
use crate::garlic_head::{BLOCK_SIZE, BlockArray, EMPTY_BLOCKARRAY};

pub type TimeFloat = f32;
pub type AmpFloat = f32;

pub const TAU: f32 = 3.14159265358979323846264338327950288 * 2.0;
pub const SAMPLERATE: f32 = 44100.;

// idea: any input can be either an array (first prio, if not None), a function (second prio, if not None), or a constant (fallback, has to be given)
#[derive(Copy, Clone)]
pub struct Edge {
    array: Option<BlockArray>,
    function: Option<fn(playhead: TimeFloat) -> AmpFloat>, // hm. is it good to have fn(globaltime, playhead) instead of just fn(playhead) ?
    constant: AmpFloat,
}
pub type PlayFunc = fn(TimeFloat) -> AmpFloat;

impl Edge {
    pub fn Constant(value: f32) -> Edge {
        Edge {
            array: None,
            function: None,
            constant: value,
        }
    }

    pub fn Function(function: PlayFunc) -> Edge { // HAVE NO IDEA ABOUT THIS YET..!!
        Edge {
            array: None,
            function: Some(function),
            constant: 0.,
        }
    }

    pub fn Array(block: BlockArray) -> Edge {
        Edge {
            array: Some(block),
            function: None,
            constant: 0.
        }
    }

    pub fn is_const(&self) -> bool {
        self.array.is_none() && self.function.is_none()
    }

    pub fn evaluate(&self, pos: usize) -> AmpFloat {
        if let Some(array) = self.array {
            return array[pos];
        }
        if let Some(func) = self.function {
            // unwrap and then calculate it at pos / SAMPLERATE, but for now, just return some garbage
            return -1.337;
        }
        return self.constant;
    }

    pub fn scale(&mut self, factor: f32) -> Edge {
        if let Some(mut array) = self.array {
            for pos in 0 .. BLOCK_SIZE {
                array[pos] = factor * array[pos];
            }
            self.array = Some(array);
        }
        if let Some(func) = self.function {
            // ..??
        }
        self.constant *= factor;

        *self
    }

    pub fn times(&mut self, other: &Edge) -> Edge {
        if other.is_const() {
            return self.scale(other.constant);
        }
        let mut array = EMPTY_BLOCKARRAY.clone();
        for pos in 0 .. BLOCK_SIZE {
            array[pos] = other.evaluate(pos) * self.evaluate(pos);
        }

        *self
    }
}


#[derive(Debug)]
pub enum BaseWave {
    Sine,
    Saw,
    Square,
    Zero,
}

#[derive(Debug)]
pub enum BaseEnv {
    ExpDecay,
}

pub struct Oscillator {
    pub shape: BaseWave,
    pub volume: Edge,
    pub frequency: TimeFloat,
    pub phase: TimeFloat,
    // makes sense to define some BaseOperator which holds seq_cursor and output?
    pub seq_cursor: usize,
    pub output: BlockArray,
}


pub trait Operator {
    //fn process(&mut self, sequence: &[SeqEvent], block_offset: usize) -> Edge;
    fn handle_event(&mut self, event: &SeqEvent);
    fn evaluate(&mut self, sample: usize, total_time: TimeFloat) -> AmpFloat;
    fn advance(&mut self);
    fn get_cursor(&mut self) -> usize;
    fn inc_cursor(&mut self);
}

pub fn next_event_option(sequence: &[SeqEvent], cursor: usize) -> Option<&SeqEvent> {
    match cursor == sequence.len() {
        true => None,
        false => Some(&sequence[cursor])
    }
}

pub fn process_operator<O: Operator>(op: &mut O, input: BlockArray, sequence: &[SeqEvent], block_offset: usize) -> BlockArray {
    let mut output = input.clone();

    let mut next_event = next_event_option(&sequence, op.get_cursor());

    for sample in 0 .. BLOCK_SIZE {
        let time: TimeFloat = (sample + block_offset) as TimeFloat / SAMPLERATE;

        while let Some(event) = next_event {
            if event.time > time {
                break;
            }
            op.handle_event(&event);
            op.inc_cursor();
            next_event = next_event_option(&sequence, op.get_cursor());
        }

        output[sample] = op.evaluate(sample, time);

        op.advance();
    }

    output
}

impl Operator for Oscillator {
    fn handle_event(&mut self, event: &SeqEvent) {
        match &event.message {
            SeqMsg::NoteOn => {
                self.phase = 0.;
                self.frequency = note_frequency(event.parameter);
            },
            // could react to Volume or whatevs here.
            _ => ()
        }
    }

    fn evaluate(&mut self, sample: usize, total_time: TimeFloat) -> AmpFloat {
        self.evaluate_at(self.phase) * self.volume.evaluate(sample)
    }

    fn advance(&mut self) {
        self.phase += self.frequency / SAMPLERATE;
        if self.phase >= 1. {
            self.phase -= 1.;
        }
    }

    fn get_cursor(&mut self) -> usize {
        self.seq_cursor
    }

    fn inc_cursor(&mut self) {
        self.seq_cursor += 1;
    }
}

impl Oscillator {
    fn evaluate_at(&self, phase: TimeFloat) -> AmpFloat {
        let basewave_value: AmpFloat = match self.shape {
            BaseWave::Sine => 0.5 * (sin(TAU * phase) + sin(TAU * phase * 1.01)),
            BaseWave::Square => (37. * sin(TAU * phase)).clamp(-1., 1.),
            BaseWave::Saw => 2. * fmod(phase, 1.) - 1.,
            _ => 0.,
        };

        basewave_value.clamp(-1., 1.)
    }

    fn none() -> Oscillator {
        Oscillator {
            shape: BaseWave::Zero,
            volume: Edge::Constant(0.),
            frequency: 0.,
            phase: 0.,
            seq_cursor: 0,
            output: EMPTY_BLOCKARRAY,
        }
    }

    pub fn process(&mut self, sequence: &[SeqEvent], block_offset: usize) -> Edge {
        self.output = EMPTY_BLOCKARRAY.clone();

        self.output = process_operator(self, self.output, &sequence, block_offset);

        Edge::Array(self.output)
    }
}

pub struct Envelope {
    pub shape: BaseEnv,
    pub attack: Edge,
    pub decay: Edge,
    pub sustain: Edge,
    pub playhead: TimeFloat,
    pub seq_cursor: usize,
}

impl Operator for Envelope {
    fn handle_event(&mut self, event: &SeqEvent) {
        match &event.message {
            SeqMsg::NoteOn => {
                self.playhead = 0.;
            },
            _ => ()
        }
    }

    fn evaluate(&mut self, sample: usize, total_time: TimeFloat) -> AmpFloat {
        let attack = self.attack.evaluate(sample);
        let decay = self.decay.evaluate(sample);

        match self.shape {
            BaseEnv::ExpDecay => {
                libm::exp2f(-self.playhead/decay) // * crate::math::smoothstep(0., attack, &self.playhead)
            }
        }
    }

    fn advance(&mut self) {
        self.playhead += 1. / SAMPLERATE;
    }

    fn get_cursor(&mut self) -> usize {
        self.seq_cursor
    }

    fn inc_cursor(&mut self) {
        self.seq_cursor += 1;
    }
}

#[derive(Clone, Debug)]
pub struct SeqEvent {
    pub time: TimeFloat,
    pub message: SeqMsg,
    pub parameter: f32,
}

#[derive(Clone, Debug)]
pub enum SeqMsg {
    NoteOn,
    NoteOff,
    Frequency,
    Volume,
}

// would this work?
pub fn message_to_str(msg: &SeqMsg) -> &str {
    match msg {
        SeqMsg::NoteOn => "noteon\0",
        SeqMsg::NoteOff => "noteoff\0",
        SeqMsg::Frequency => "setfreq\0",
        SeqMsg::Volume => "setvol\0",
    }
}

pub fn note_frequency(note_number: f32) -> f32 {
    440. * libm::powf(2., (note_number - 69.)/12.)
}

// LIST OF INVESTIGATIONS, watch for Size / Performance:
// ... probably after first track exists, to see REAL difference
//
// loop vs for loop -- no difference at all (sizewise)
// unsafe get_unchecked_mut vs. get_mut & unwrap
// math::sin vs other sin?
