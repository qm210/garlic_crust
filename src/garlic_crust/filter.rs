use super::*;
use crate::math::TAU;

#[derive(Debug)]
pub enum FilterType {
    LowPass,
    HiPass,
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

    fn evaluate(&mut self, sample: usize) -> Sample {
        let cutoff = self.cutoff.evaluate(sample);

        match self.shape {
            FilterType::LowPass => {
                self.state.set_lowpass(cutoff);
            },
            FilterType::HiPass => {
                self.state.set_hipass(cutoff);
            }
        }
        let input = self.input.evaluate(sample);
        for ch in 0 .. 2 {
            self.state.z1[ch] = input[ch] * self.state.a0[ch] + self.state.z1[ch] * self.state.b1[ch];
        }

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
    pub a0: Sample,
    pub b1: Sample,
    pub z1: Sample,
}

impl FilterState {
    pub fn new() -> FilterState {
        FilterState {
            a0: [0., 0.],
            b1: [0., 0.],
            z1: [0., 0.],
        }
    }

    pub fn set_coeff(&mut self, cutoff: Sample, b1_precoeff: f32) {
        for ch in 0 .. 2 {
            self.b1[ch] = b1_precoeff * libm::exp2f(-TAU * cutoff[ch] / SAMPLERATE);
            self.a0[ch] = 1. - b1_precoeff * self.b1[ch];
        }
    }

    pub fn set_lowpass(&mut self, cutoff: Sample) {
        self.set_coeff(cutoff, 1.);
    }

    pub fn set_hipass(&mut self, cutoff: Sample) {
        self.set_coeff(cutoff, -1.);
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter {
            shape: FilterType::LowPass,
            cutoff: Edge::zero(),
            state: filter::FilterState::new(),
            input: Edge::zero(),
        }
    }
}