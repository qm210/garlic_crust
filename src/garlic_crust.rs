use super::math_util::sin;

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
}

#[derive(Debug)]
pub struct Oscillator {
    pub shape: BaseWave,
    pub volume: AmpFloat,
}

impl Oscillator {
    #[inline]
    fn evaluate_at(&self, phase: TimeFloat) -> AmpFloat {
        let basewave_value: AmpFloat = match self.shape {
            BaseWave::Sine => sin(TAU * phase),
            BaseWave::Square => (20. * sin(TAU * phase)).clamp(-1., 1.),
            BaseWave::Saw => 2. * fmod(phase, 1.) - 1.,
            _ => 0.
        };

        basewave_value * self.volume
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
    pub osc: Oscillator,
    pub volume: AmpFloat,
    pub frequency: TimeFloat,
    //pub note_pitch: u32,
    pub note_duration: TimeFloat,

    phase: TimeFloat,
}

impl GarlicCrust {
    #[inline]
    pub fn create_default() -> Self {
        let default_osc = Oscillator::default();
        Self::create(default_osc)
    }

    #[inline] // todo: think about #[inline]
    pub fn create(osc: Oscillator) -> Self {
        GarlicCrust {
            osc: osc,
            volume: 1.,
            frequency: 220., // could also use 220_f32, might switch to f64 sometime
            //note_pitch: 48,
            note_duration: 1.,

            phase: 0., // better way to init?
        }
    }

}

// qm: no actual advantage by this implementation
impl Iterator for GarlicCrust {
    type Item = AmpFloat;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_frame())
    }
}

impl GarlicCrust {
    pub fn next_frame(&mut self) -> AmpFloat {
        let amp_value: AmpFloat = self.volume * self.osc.evaluate_at(self.phase);
        self.phase += self.frequency / SAMPLERATE;
        if self.phase > 1. {
            self.phase -= 1.;
        }
        amp_value
    }
}

// LIST OF INVESTIGATIONS, watch for Size / Performance:
// ... probably after first track exists, to see REAL difference
//
// loop vs for loop
// unsafe get_unchecked_mut vs. get_mut & unwrap
// math_util::sin vs other sin?
// directly read value without Iterator / Option / Some