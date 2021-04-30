use super::math_util::sin;
use super::math_util::approx4;
use libm::{fmodf as fmod};

pub type TimeFloat = f32;
pub type AmpFloat = f32;

pub const TAU: f32 = 3.14159265358979323846264338327950288 * 2.0;
pub const SAMPLERATE: f32 = 44100.;

#[derive(Debug)]
pub enum BaseWave {
    Sine,
    Saw,
    Square,
    Zero,
}

#[derive(Debug)]
pub struct Oscillator {
    pub shape: BaseWave,
    pub volume: AmpFloat,
}

impl Oscillator {
    fn evaluate_at(&self, phase: TimeFloat) -> AmpFloat {
        let basewave_value: AmpFloat = match self.shape {
            BaseWave::Sine => 0.5 * (sin(TAU * phase) + sin(TAU * phase * 1.01)),
            BaseWave::Square => (20. * sin(TAU * phase)).clamp(-1., 1.),
            BaseWave::Saw => 2. * (fmod(phase, 1.) + fmod(1.1*phase, 1.)) - 1.,
            _ => 0.,
        };

        (basewave_value * self.volume).clamp(-1., 1.)
    }

    fn none() -> Oscillator {
        Oscillator {
            shape: BaseWave::Zero,
            volume: 0.,
        }
    }
}

impl Default for Oscillator {
    fn default() -> Self {
        Oscillator {
            shape: BaseWave::Sine,
            volume: 1 as AmpFloat,
        }
    }
}

#[derive(Debug)]
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

    pub fn create_from(config: InstrumentConfig) -> Self {
        GarlicCrust {
            oscA: Oscillator {
                shape: config.shape,
                volume: 1.
            },
            volume: config.volume,
            ..Default::default()
        }
    }

}

// qm: no actual advantage by this implementation... right?
/*
impl Iterator for GarlicCrust {
    type Item = AmpFloat;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_frame())
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

    pub fn handle_event(&mut self, event: &TrackEvent) {
        unsafe { crate::log!("Type %s", message_to_str(&event.message).as_ptr());}

        match event.message {
            TrackEventMessage::NoteOn => {
                self.frequency = note_frequency(event.parameter);
                self.mute = false;
                self.cursor = 0.;
            },
            TrackEventMessage::NoteOff => {
                self.mute = true;
            },
            TrackEventMessage::Frequency => {
                self.frequency = event.parameter;
            },
            TrackEventMessage::Volume => {
                self.volume = event.parameter;
            },
            TrackEventMessage::EndOfTrack => {
                self.eot = true;
            }
        }
    }
}

pub struct InstrumentConfig {
    pub shape: BaseWave,
    pub volume: AmpFloat,
}

pub struct InstrumentTrack<N: heapless::ArrayLength<TrackEvent>> {
    pub config: InstrumentConfig,
    pub events: heapless::Vec<TrackEvent, N>,
}

#[derive(Clone, Debug)]
pub struct TrackEvent {
    pub time: TimeFloat,
    pub message: TrackEventMessage,
    pub parameter: f32,
}

#[derive(Clone, Debug)]
pub enum TrackEventMessage {
    NoteOn,
    NoteOff,
    Frequency,
    Volume,
    EndOfTrack,
}

// would this work?
pub fn message_to_str(msg: &TrackEventMessage) -> &str {
    match msg {
        TrackEventMessage::NoteOn => "noteon\0",
        TrackEventMessage::NoteOff => "noteoff\0",
        TrackEventMessage::Frequency => "setfreq\0",
        TrackEventMessage::Volume => "setvol\0",
        TrackEventMessage::EndOfTrack => "EOT\0",
    }
}

pub fn note_frequency(note_number: f32) -> f32 {
    440. * libm::powf(2., (note_number - 69.)/12.)
}


// LIST OF INVESTIGATIONS, watch for Size / Performance:
// ... probably after first track exists, to see REAL difference
//
// loop vs for loop
// unsafe get_unchecked_mut vs. get_mut & unwrap
// math_util::sin vs other sin?
// directly read value without Iterator / Option / Some