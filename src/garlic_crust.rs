use crate::garlic_head::{BLOCK_SIZE, BlockArray, EMPTY_BLOCKARRAY};

pub mod oscillator;
pub mod envelope;
pub mod filter;

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
    pub fn constant(value: f32) -> Edge {
        Edge {
            array: None,
            function: None,
            constant: value,
        }
    }

    pub fn function(function: PlayFunc) -> Edge { // HAVE NO IDEA ABOUT THIS YET..!!
        Edge {
            array: None,
            function: Some(function),
            constant: 0.,
        }
    }

    pub fn array(block: BlockArray) -> Edge {
        Edge {
            array: Some(block),
            function: None,
            constant: 0.
        }
    }

    pub fn zero() -> Edge {
        Edge::constant(0.)
    }

    pub fn is_const(&self) -> bool {
        self.array.is_none() && self.function.is_none()
    }

    pub fn evaluate(&self, pos: usize) -> AmpFloat {
        if let Some(array) = self.array {
            return array[pos];
        }
        if let Some(func) = self.function {
            // no idea whether this somehow works or rather is garbage
            return func(pos as TimeFloat / SAMPLERATE);
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

pub fn process_operator<O: Operator>(op: &mut O, sequence: &[SeqEvent], block_offset: usize) -> BlockArray {
    let mut output = EMPTY_BLOCKARRAY; // .clone();

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

pub fn process_operator_noseq<O: Operator>(op: &mut O, block_offset: usize) -> BlockArray {
    let mut output = EMPTY_BLOCKARRAY; // .clone();

    for sample in 0 .. BLOCK_SIZE {
        let time: TimeFloat = (sample + block_offset) as TimeFloat / SAMPLERATE;

        output[sample] = op.evaluate(sample, time);

        op.advance();
    }

    output
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
