use crate::garlic_crust::*;
use super::*;

pub struct GarlicMaster {
    //reverb: GarlicBreath,
    waveshape_state: WaveshapeState,
    data: BlockArray,
}

pub struct WaveshapeState {
    amount: f32,
}

impl GarlicMaster {
    pub fn new() -> GarlicMaster {
        GarlicMaster {
            //reverb: GarlicBreath::new(),
            waveshape_state: WaveshapeState {
                amount: 0.,
            },
            data: EMPTY_BLOCKARRAY,
        }
    }

    pub fn put_at(&mut self, pos: usize, value: AmpFloat) {
        self.data[pos] = value;
    }

    pub fn add_at(&mut self, pos: usize, value: AmpFloat) {
        self.data[pos] += value;
    }

    pub fn process(&mut self, outer_data: &mut TrackArray, block_offset: usize) {
        // simple waveshaper, for se lolz

        for sample in 0 .. BLOCK_SIZE {
            self.data[sample] = (self.data[sample] + self.waveshape_state.amount * waveshape1(self.data[sample])) / (1. + self.waveshape_state.amount);
            self.waveshape_state.amount += 0.7e-5;

            outer_data[block_offset + sample] = crate::math::satanurate(self.data[sample]);
        }
    }
}

fn waveshape1(x: AmpFloat) -> AmpFloat {
    x + 0.2 * crate::math::sin(9.*x) - 0.15 * crate::math::sin(x)
}


pub struct GarlicBreath {

}
