use crate::math::{sin, TAU};
use super::*;

#[derive(Debug, Copy, Clone)]
pub enum BaseWave {
    Sine,
    Saw,
    Square,
    Triangle,
    Zero,
}

pub struct Oscillator {
    pub shape: BaseWave,
    pub volume: Edge,
    pub frequency: Edge,
    pub freq_factor: Edge,
    pub phasemod: Edge,
    pub detune: Edge,
    pub phase: [TimeFloat; 2],
    pub seq_cursor: usize,
}

impl Operator for Oscillator {
    fn handle_message(&mut self, message: &SeqMsg) {
        match &message {
            SeqMsg::NoteOn(note_key, _) => {
                self.phase = [0., 0.];
                self.frequency = self.freq_factor.clone_scaled(note_frequency(*note_key));
            },
            // could react to Volume or whatevs here.
            _ => ()
        }
    }

    fn evaluate(&mut self, sample: usize) -> Sample {
        let phaseL = self.phase[L] + self.phasemod.evaluate_mono(sample, L);
        let phaseR = self.phase[R] + self.phasemod.evaluate_mono(sample, R);

        let resultL = self.evaluate_at(phaseL) * self.volume.evaluate_mono(sample, L);
        let resultR = self.evaluate_at(phaseR) * self.volume.evaluate_mono(sample, R);

        [resultL, resultR]
    }

    fn advance(&mut self, sample: usize) {
        let freq = self.frequency.evaluate(sample);
        for ch in 0 .. 2 {
            self.phase[ch] += freq[ch] / SAMPLERATE;
            if self.phase[ch] >= 1. {
                self.phase[ch] -= 1.;
            }
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
    fn evaluate_at(&self, phase: TimeFloat) -> MonoSample {
        let basewave_value: MonoSample = match self.shape {
            BaseWave::Sine => sin(TAU * phase),
            BaseWave::Square => (37. * sin(TAU * phase)).clamp(-1., 1.),
            BaseWave::Saw => 2. * libm::fmodf(phase, 1.) - 1.,
            BaseWave::Triangle => 4. * libm::fabsf(libm::fmodf(phase, 1.) - 0.5) - 1.0,
            _ => 0.,
        };

        basewave_value.clamp(-1., 1.)
    }
}

impl Default for Oscillator {
    fn default() -> Oscillator {
        Oscillator {
            shape: BaseWave::Sine,
            volume: Edge::constant(1.),
            frequency: Edge::zero(),
            freq_factor: Edge::constant(1.),
            phasemod: Edge::zero(),
            detune: Edge::zero(),
            phase: [0., 0.],
            seq_cursor: 0
        }
    }
}