use super::garlic_crust::{GarlicCrust, TimeFloat, AmpFloat, SAMPLERATE, Oscillator, BaseWave};

pub const SECONDS: TimeFloat = 3.;
pub const SAMPLES: usize = (SAMPLERATE * SECONDS) as usize;

// in case I need alloc --> https://docs.rust-embedded.org/book/collections/

use heapless::Vec;

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    /*
    let (track_header, track_iter) = midly::parse(include_bytes!("../baroque0.mid")).unwrap();

    for (i, event_iter) in track_iter.enumerate() {
        super::log!("track {} has {} events", i as f32, event_iter.unwrap().size_hint());
        for (e, event) in event_iter.unwrap().enumerate() {
            super::log!("event {} {}", e as f32, event);
        }
    }
    */
    let mut synth = GarlicCrust::create(
        Oscillator {
            shape: BaseWave::Sine,
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

