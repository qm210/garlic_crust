use super::garlic_crust::{GarlicCrust, TimeFloat, AmpFloat, SAMPLERATE, Oscillator, BaseWave};

pub const SECONDS: TimeFloat = 3.;
pub const SAMPLES: usize = (SAMPLERATE * SECONDS) as usize;

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    let mut synth = GarlicCrust::create(
        Oscillator {
            shape: BaseWave::Square,
            volume: 0.7
        }
    );

    // loop with counter at that position made 0 bytes difference
    for sample in 0..SAMPLES {

        let time: TimeFloat = sample as TimeFloat / SAMPLERATE;
        let amp: AmpFloat = synth.next_frame(); //synth.next().unwrap();

        data[sample] = amp;

        if false && libm::fmodf(time, 1.) > 0.5 {
            synth.frequency = 440.;
        } else {
            synth.frequency = 220.;
        }

    }
}
