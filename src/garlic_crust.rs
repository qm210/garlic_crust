use super::math::sin;
use libm::{fmodf as fmod};
use crate::garlic_head::{BLOCK_SIZE, BlockArray};

pub type TimeFloat = f32;
pub type AmpFloat = f32;

pub const TAU: f32 = 3.14159265358979323846264338327950288 * 2.0;
pub const SAMPLERATE: f32 = 44100.;

// idea: any input can be either an array (first prio, if not None), a function (second prio, if not None), or a constant (fallback, has to be given)
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

    pub fn Function(function: PlayFunc) -> Edge {
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

    pub fn evaluate(&self, pos: usize) -> AmpFloat {
        if self.array.is_some() {
            return self.array.unwrap()[pos];
        }
        if self.function.is_some() {
            // unwrap and then calculate it at pos / SAMPLERATE, but for now, just return some garbage
            return -1.337;
        }
        return self.constant;
    }
}

#[derive(Debug)]
pub enum BaseWave {
    Sine,
    Saw,
    Square,
    Zero,
}

pub struct Oscillator {
    pub shape: BaseWave,
    pub volume: Edge,
//    pub envelope_function: Option<fn(TimeFloat) -> AmpFloat>,
    pub frequency: TimeFloat,
    pub phase: TimeFloat,
    pub seq_cursor: usize,
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
        }
    }

    pub fn process(&mut self, sequence: &[SeqEvent], block_offset: usize) -> BlockArray {
        let mut output = [0.; BLOCK_SIZE];

        let mut next_event = if self.seq_cursor == sequence.len() { None } else {
            Some(&sequence[self.seq_cursor])
        };

        for sample in 0 .. BLOCK_SIZE {
            let time: TimeFloat = (sample + block_offset) as TimeFloat / SAMPLERATE;

            if next_event.is_some() {
                let event = next_event.unwrap();
                while self.seq_cursor < sequence.len() && event.time <= time {
                    match &event.message {
                        SeqMsg::NoteOn => {
                            self.phase = 0.;
                            self.frequency = note_frequency(event.parameter);
                        },
                        // could react to Volume or whatevs here.
                        _ => ()
                    }
                    self.seq_cursor += 1;

                    if self.seq_cursor == sequence.len() {
                        next_event = None;
                        break;
                    } else {
                        next_event = Some(&sequence[self.seq_cursor]);
                    }
                }
            }

            output[sample] = self.evaluate_at(self.phase) * self.volume.evaluate(sample);

            self.phase += self.frequency / SAMPLERATE;
            if self.phase >= 1. {
                self.phase -= 1.;
            }
        }

        output
    }
}

/*
impl Default for Oscillator {
    fn default() -> Self {
        Oscillator {
            shape: BaseWave::Sine,
            volume: 1.,
            phase: 0.,
            seq_cursor: 0,
        }
    }
}
*/

pub struct Envelope {
    pub attack: Edge,
    pub decay: Edge,
    pub sustain: Edge,
    pub playhead: TimeFloat,
    pub seq_cursor: usize,
}

pub struct GarlicCrust {
    pub oscA: Oscillator,
    pub oscB: Oscillator,
    pub volume: AmpFloat,
    pub frequency: TimeFloat,
    phase: TimeFloat,
    cursor: TimeFloat,

    pub eot: bool,
    mute: bool,
}

/*
impl Default for GarlicCrust {
    fn default() -> GarlicCrust {
        GarlicCrust {
            oscA: Oscillator::default(),
            oscB: Oscillator::none(),
            volume: 1.,
            frequency: 220.,
            phase: 0.,
            cursor: 0.,
            eot: false,
            mute: false,
        }
    }
}

impl GarlicCrust {
    pub fn create_default() -> Self {
        GarlicCrust {
            oscA: Oscillator::default(),
            volume: 1.,
            ..Default::default()
        }
    }

    pub fn create_from(param: OscillatorParam) -> Self {
        GarlicCrust {
            oscA: Oscillator {
                shape: param.shape,
                volume: 1.
                envelop
            },
            volume: param.volume,
            ..Default::default()
        }
    }

}
*/
impl GarlicCrust {
    pub fn next_frame(&mut self) -> AmpFloat {
        if self.eot {
            return 0.;
        }
        let amp_value: AmpFloat = if self.mute {0.} else { self.volume * self.oscA.evaluate_at(self.phase)};
        self.phase += self.frequency / SAMPLERATE;
        if self.phase > 1. {
            self.phase -= 1.;
        }
        self.cursor += 1. / SAMPLERATE;

        amp_value
    }

    pub fn handle_event(&mut self, event: &SeqEvent) {
        unsafe { crate::log!("Type %s", message_to_str(&event.message).as_ptr());}

        match event.message {
            SeqMsg::NoteOn => {
                self.frequency = note_frequency(event.parameter);
                self.mute = false;
                self.cursor = 0.;
            },
            SeqMsg::NoteOff => {
                self.mute = true;
            },
            SeqMsg::Frequency => {
                self.frequency = event.parameter;
            },
            SeqMsg::Volume => {
                self.volume = event.parameter;
            },
        }
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
