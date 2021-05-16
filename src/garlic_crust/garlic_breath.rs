/*
use super::*;



pub struct GarlicBreath {
    combs: [Comb; 8],
    ap: [AllPass; 4],
    wet_gain: f32,
    wet: f32,
    width: f32,
    dry: f32,
    input_gain: f32,
    dampening: f32,
    roomsize: f32,
    frozen: bool,
}

impl GarlicBreath {

    pub fn new() -> Self {
        Self {

        }
    }

}

pub struct Comb {
    delay_line: DelayLine,
    feedback: f64,
    filter_state: f64,
    dampening: f64,
    dampening_inverse: f64,
}

impl Comb {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
            feedback: 0.5,
            filter_state: 0.0,
            dampening: 0.5,
            dampening_inverse: 0.5,
        }
    }

    pub fn set_dampening(&mut self, value: f64) {
        self.dampening = value;
        self.dampening_inverse = 1.0 - value;
    }

    pub fn set_feedback(&mut self, value: f64) {
        self.feedback = value;
    }

    pub fn tick(&mut self, input: f64) -> f64 {
        let output = self.delay_line.read();

        self.filter_state =
            output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line
            .write_and_advance(input + self.filter_state * self.feedback);

        output
    }
}


pub struct DelayLine {
    buffer: BlockArray,
    index: usize,
}

impl DelayLine {
    pub fn new() -> Self {
        Self {
            buffer: EMPTY_BLOCKARRAY,
            index: 0,
        }
    }

    pub fn read(&self) -> f64 {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, value: f64) {
        self.buffer[self.index] = value;

        if self.index == self.buffer.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}
*/