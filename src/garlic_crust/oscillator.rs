use crate::math::sin;
use libm::{fmodf as fmod};
use crate::garlic_head::{BlockArray, EMPTY_BLOCKARRAY};
use super::*;

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
    pub frequency: TimeFloat,
    pub phase: TimeFloat,
    // makes sense to define some BaseOperator which holds seq_cursor and output?
    pub seq_cursor: usize,
    pub output: BlockArray,
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
            volume: Edge::constant(0.),
            frequency: 0.,
            phase: 0.,
            seq_cursor: 0,
            output: EMPTY_BLOCKARRAY,
        }
    }
}
