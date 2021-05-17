// we steal https://github.com/irh/freeverb-rs/blob/main/src/freeverb/src/freeverb.rs
// because stealing smells like garlic!

use crate::garlic_head::{MasterBlockArray, MASTER_BLOCK_SIZE};

pub struct GarlicBreath {
    combs: [(Comb, Comb); N_COMBS],
    allpasses: [(AllPass, AllPass); N_ALLPASSES],
    wet_gains: (f32, f32),
    wet: f32,
    width: f32,
    dry: f32,
    input_gain: f32,
    dampening: f32,
    room_size: f32,
    frozen: bool,
}

const N_COMBS: usize = 8;
const N_ALLPASSES: usize = 4;

const FIXED_GAIN: f32 = 0.015;

const SCALE_WET: f32 = 3.0;
const SCALE_DAMPENING: f32 = 0.4;

const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;

const STEREO_SPREAD: usize = 23;

const COMB_TUNING_L1: usize = 1116;
const COMB_TUNING_R1: usize = 1116 + STEREO_SPREAD;
const COMB_TUNING_L2: usize = 1188;
const COMB_TUNING_R2: usize = 1188 + STEREO_SPREAD;
const COMB_TUNING_L3: usize = 1277;
const COMB_TUNING_R3: usize = 1277 + STEREO_SPREAD;
const COMB_TUNING_L4: usize = 1356;
const COMB_TUNING_R4: usize = 1356 + STEREO_SPREAD;
const COMB_TUNING_L5: usize = 1422;
const COMB_TUNING_R5: usize = 1422 + STEREO_SPREAD;
const COMB_TUNING_L6: usize = 1491;
const COMB_TUNING_R6: usize = 1491 + STEREO_SPREAD;
const COMB_TUNING_L7: usize = 1557;
const COMB_TUNING_R7: usize = 1557 + STEREO_SPREAD;
const COMB_TUNING_L8: usize = 1617;
const COMB_TUNING_R8: usize = 1617 + STEREO_SPREAD;

const ALLPASS_TUNING_L1: usize = 556;
const ALLPASS_TUNING_R1: usize = 556 + STEREO_SPREAD;
const ALLPASS_TUNING_L2: usize = 441;
const ALLPASS_TUNING_R2: usize = 441 + STEREO_SPREAD;
const ALLPASS_TUNING_L3: usize = 341;
const ALLPASS_TUNING_R3: usize = 341 + STEREO_SPREAD;
const ALLPASS_TUNING_L4: usize = 225;
const ALLPASS_TUNING_R4: usize = 225 + STEREO_SPREAD;

impl GarlicBreath {

    pub fn new(wet: f32, width: f32, dampen: f32, size: f32, frozen: bool) -> Self {
        let mut freeverb = GarlicBreath {
            combs: [
                (
                    Comb::new(COMB_TUNING_L1),
                    Comb::new(COMB_TUNING_R1),
                ),
                (
                    Comb::new(COMB_TUNING_L2),
                    Comb::new(COMB_TUNING_R2),
                ),
                (
                    Comb::new(COMB_TUNING_L3),
                    Comb::new(COMB_TUNING_R3),
                ),
                (
                    Comb::new(COMB_TUNING_L4),
                    Comb::new(COMB_TUNING_R4),
                ),
                (
                    Comb::new(COMB_TUNING_L5),
                    Comb::new(COMB_TUNING_R5),
                ),
                (
                    Comb::new(COMB_TUNING_L6),
                    Comb::new(COMB_TUNING_R6),
                ),
                (
                    Comb::new(COMB_TUNING_L7),
                    Comb::new(COMB_TUNING_R7),
                ),
                (
                    Comb::new(COMB_TUNING_L8),
                    Comb::new(COMB_TUNING_R8),
                ),
            ],
            allpasses: [
                (
                    AllPass::new(ALLPASS_TUNING_L1),
                    AllPass::new(ALLPASS_TUNING_R1),
                ),
                (
                    AllPass::new(ALLPASS_TUNING_L2),
                    AllPass::new(ALLPASS_TUNING_R2),
                ),
                (
                    AllPass::new(ALLPASS_TUNING_L3),
                    AllPass::new(ALLPASS_TUNING_R3),
                ),
                (
                    AllPass::new(ALLPASS_TUNING_L4),
                    AllPass::new(ALLPASS_TUNING_R4),
                ),
            ],
            wet_gains: (0.0, 0.0),
            wet: 0.0,
            dry: 0.0,
            input_gain: 0.0,
            width: 0.0,
            dampening: 0.0,
            room_size: 0.0,
            frozen: false,
        };

        freeverb.set_wet(wet);
        freeverb.set_width(width);
        freeverb.set_dampening(dampen);
        freeverb.set_room_size(size);
        freeverb.set_frozen(frozen);

        freeverb
    }

    pub fn default() -> Self {
        Self::new(1., 0.5, 0.5, 0.5, false)
    }


    pub fn tick(&mut self, input: (f32, f32)) -> (f32, f32) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;

        let mut out = (0., 0.);

        for i in 0 .. N_COMBS {
            out.0 += self.combs[i].0.tick(input_mixed);
            out.1 += self.combs[i].1.tick(input_mixed);
        }

        for i in 0 .. N_ALLPASSES {
            out.0 = self.allpasses[i].0.tick(out.0);
            out.1 = self.allpasses[i].1.tick(out.1);
        }

        (
            out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry,
            out.1 * self.wet_gains.0 + out.0 * self.wet_gains.1 + input.1 * self.dry,
        )
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: f32) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f32) {
        self.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / 2.0 + 0.5),
            self.wet * ((1.0 - self.width) / 2.0),
        )
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f32) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1.0, 0.0)
        } else {
            (self.room_size, self.dampening)
        };

        for combs in self.combs.iter_mut() {
            combs.0.set_feedback(feedback);
            combs.1.set_feedback(feedback);

            combs.0.set_dampening(dampening);
            combs.1.set_dampening(dampening);
        }
    }

    pub fn set_dry(&mut self, value: f32) {
        self.dry = value;
    }

}

pub struct Comb {
    delay_line: DelayLine,
    feedback: f32,
    filter_state: f32,
    dampening: f32,
    dampening_inverse: f32,
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

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value;
        self.dampening_inverse = 1.0 - value;
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let output = self.delay_line.read();

        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line.write_and_advance(input + self.filter_state * self.feedback);

        output
    }
}


pub struct DelayLine {
    buffer: MasterBlockArray,
    length: usize,
    index: usize,
}

impl DelayLine {
    pub fn new(delay_length: usize) -> Self {
        Self {
            buffer: [0.; MASTER_BLOCK_SIZE], // MASTER_BLOCK_SIZE gives a limit to the total delay line length (e.g. the Comb Filters!)
            length: delay_length,
            index: 0,
        }
    }

    pub fn read(&self) -> f32 {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, value: f32) {
        self.buffer[self.index] = value;

        if self.index == self.length - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}


pub struct AllPass {
    delay_line: DelayLine,
}

impl AllPass {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;

        // in the original version of freeverb this is a member which is never modified
        let feedback = 0.5;

        self.delay_line
            .write_and_advance(input + delayed * feedback);

        output
    }
}