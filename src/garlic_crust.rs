use super::math_util::sin;

pub type TimeFloat = f32;
pub type AmpFloat = f32;

pub const TAU: f32 = 3.14159265358979323846264338327950288_f32 * 2.0;
pub const SAMPLERATE: f32 = 44100.;

#[derive(Debug)]
enum BaseWave {
    Sine,
    Saw,
    Square,
}

#[derive(Debug)]
pub struct Oscillator {
    shape: BaseWave,
    volume: AmpFloat,
}

impl Oscillator {
    #[inline]
    fn evaluate_at(&self, phase: TimeFloat) -> AmpFloat {
        let basewave_value: AmpFloat = match self.shape {
            BaseWave::Sine => sin(TAU * phase),
//            BaseWave::Saw => (TAU * phase)
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
    pub frequency: f32,
    //pub note_pitch: u32,
    pub note_duration: TimeFloat,

    phase: TimeFloat,
}

impl GarlicCrust {
    #[inline] // todo: think about #[inline]
    pub fn create() -> Self {
        GarlicCrust {
            osc: Oscillator::default(),
            volume: 1.,
            frequency: 220., // could also use 220_f32, might switch to f64 sometime
            //note_pitch: 48,
            note_duration: 1.,

            phase: 0., // better way to init?
        }
    }

}

impl Iterator for GarlicCrust {
    type Item = AmpFloat;
    fn next(&mut self) -> Option<Self::Item> {

        let amp_value: AmpFloat = self.volume * self.osc.evaluate_at(self.phase);
        self.phase += self.frequency / SAMPLERATE;

        Some(amp_value)
    }
}

// LIST OF INVESTIGATIONS, watch for Size / Performance:
// ... probably after first track exists, to see REAL difference
//
// loop vs for loop
// unsafe get_unchecked_mut vs. get_mut & unwrap
// math_util::sin vs other sin?
// directly read value without Iterator / Option / Some