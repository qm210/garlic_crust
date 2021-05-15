use super::*;
use crate::math::TAU;

#[derive(Debug)]
pub enum FilterType {
    LowPass,
}

pub struct Filter {
    pub shape: FilterType,
    pub cutoff: Edge,
    pub state: FilterState,
    pub input: Edge,
}

impl Operator for Filter {
    fn handle_message(&mut self, _: &SeqMsg) {
    }

    fn evaluate(&mut self, sample: usize, _: TimeFloat) -> AmpFloat {
        let cutoff = self.cutoff.evaluate(sample);

        self.state.set_lowpass(cutoff);
        self.state.z1 = self.input.evaluate(sample) * self.state.a0 + self.state.z1 * self.state.b1;

        self.state.z1
    }

    fn advance(&mut self, _: usize) {
    }

    fn get_cursor(&mut self) -> usize {
        0
    }

    fn inc_cursor(&mut self) {
    }
}

pub struct FilterState {
    pub a0: f32,
    pub b1: f32,
    pub z1: f32,
}

impl FilterState {
    pub fn new() -> FilterState {
        FilterState {
            a0: 0.,
            b1: 0.,
            z1: 0.,
        }
    }

    pub fn set_lowpass(&mut self, cutoff: f32) {
        self.b1 = libm::exp2f(-TAU * cutoff / SAMPLERATE);
        self.a0 = 1. - self.b1;
    }

    pub fn set_hipass(&mut self, cutoff: f32) {
        self.b1 = -libm::exp2f(-TAU * (0.5 - cutoff / SAMPLERATE));
        self.a0 = 1. + self.b1;
    }
}

