use super::garlic_crust::{GarlicCrust, TimeFloat, AmpFloat, SAMPLERATE};

pub const SECONDS: TimeFloat = 3.;
pub const SAMPLES: usize = (SAMPLERATE * SECONDS) as usize;

pub fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    let mut synth = GarlicCrust::create();

    for sample in 0..SAMPLES {
        let time: TimeFloat = sample as TimeFloat / SAMPLERATE;
        let amp: AmpFloat = synth.next().unwrap();

        data[sample] = amp;

        if time % 1. > 0.5 {
            synth.frequency = 440.;
        } else {
            synth.frequency = 220.;
        }
        // if sample < 600 {
        //     unsafe { super::log!("Time", time, 10. + amp) };
        // }
    }


}
