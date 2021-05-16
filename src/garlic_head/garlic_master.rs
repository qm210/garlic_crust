use crate::garlic_crust::*;
use super::*;

pub struct GarlicMaster {
    reverb_state: ReverbState,
    waveshape_state: WaveshapeState,
}

pub struct ReverbState {

}

pub struct WaveshapeState {
    amount: f32,
}

impl GarlicMaster {
    pub fn new() -> GarlicMaster {
        GarlicMaster {
            reverb_state: ReverbState {

            },
            waveshape_state: WaveshapeState {
                amount: 0.,
            }
        }
    }

    pub fn process(&mut self, data: &mut TrackArray) {
        // simple waveshaper, for se lolz

        for sample in 0 .. SAMPLES {
            data[sample] = (data[sample] + self.waveshape_state.amount * waveshape1(data[sample])) / (1. + self.waveshape_state.amount);
            self.waveshape_state.amount += 0.7e-5;

            data[sample] = crate::math::satanurate(data[sample]);
        }
    }
}

fn waveshape1(x: AmpFloat) -> AmpFloat {
    x + 0.2 * crate::math::sin(9.*x) - 0.15 * crate::math::sin(x)
}