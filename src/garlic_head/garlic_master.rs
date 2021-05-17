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
            reverb: GarlicBreath::new(0.05, 0.5, 0.8, 0.9, false),
            waveshape_state: WaveshapeState {
                amount: 0.,
            },
            data: [0.; MASTER_BLOCK_SIZE],
        }
    }

    pub fn put_at(&mut self, pos: usize, value: AmpFloat) {
        self.data[pos] = value;
    }

    pub fn add_at(&mut self, pos: usize, value: AmpFloat) {
        self.data[pos] += value;
    }

    pub fn write(&self, data: &mut TrackArray, master_block_offset: usize) {
        for sample in 0 .. MASTER_BLOCK_SIZE {
            data[master_block_offset + sample] = self.data[sample];
        }
    }

    pub fn process(&mut self, sample: usize) {
        let mut value = self.data[sample];

        // simple waveshaper, for se lolz
        value = (value + self.waveshape_state.amount * waveshape1(value)) / (1. + self.waveshape_state.amount);
        self.waveshape_state.amount += 0.7e-5;

        value = crate::math::satanurate(0.4 * value);

        value = self.reverb.tick((value, value)).0 + 0.3 * value;

        self.data[sample] = value;
    }

}

fn waveshape1(x: AmpFloat) -> AmpFloat {
    x + 0.2 * crate::math::sin(9.*x) - 0.15 * crate::math::sin(x)
}
