use crate::garlic_crust::*;
use super::*;
use garlic_breath::GarlicBreath;

pub struct GarlicMaster {
    reverb: GarlicBreath,
    waveshape_state: WaveshapeState,
    data: MasterBlockArray,
}

pub struct WaveshapeState {
    amount: f32,
}

impl GarlicMaster {
    pub fn new() -> GarlicMaster {
        GarlicMaster {
            reverb: GarlicBreath::new(0.5, 0.95, 0.95, 0.9, false), // (wet, width, dampen, size, frozen)
            waveshape_state: WaveshapeState {
                amount: 0.,
            },
            data: [[0., 0.]; MASTER_BLOCK_SIZE],
        }
    }

    pub fn put_at(&mut self, pos: usize, value: Sample) {
        self.data[pos] = value;
    }

    pub fn add_at(&mut self, pos: usize, value: Sample) {
        self.data[pos][L] += value[L];
        self.data[pos][R] += value[R];
    }

    pub fn write(&self, data: &mut StereoTrack, master_block_offset: usize) {
        for sample in 0 .. MASTER_BLOCK_SIZE {
            data[master_block_offset + 2 * sample] = self.data[sample][0];
            data[master_block_offset + 2 * sample + 1] = self.data[sample][1];
        }
    }

    pub fn apply_reverb(&mut self, sample: usize, amount: f32) {
        let dry = self.data[sample];
        let wet = self.reverb.tick(dry);
        for ch in 0 .. 2 {
            self.data[sample][ch] = dry[ch] + amount * wet[ch];
        }
    }

    pub fn saturate(&mut self, sample: usize) {
        for channel in 0 .. 2 {
            self.data[sample][channel] = crate::math::satanurate(self.data[sample][channel]);
        }
    }

    pub fn waveshape_lel(&mut self, sample: usize) {
        for channel in 0 .. 2 {
            let mut value = self.data[sample][channel];
            value = (value + self.waveshape_state.amount * waveshape1(value)) / (1. + self.waveshape_state.amount);
            self.data[sample][channel] = value.clamp(-1., 1.);
        }
    }
}


fn waveshape1(x: MonoSample) -> MonoSample {
    x + 0.2 * crate::math::sin(9.*x) - 0.15 * crate::math::sin(x)
}
