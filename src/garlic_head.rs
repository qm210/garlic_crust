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

    super::log!("lel %f\n\0", 2.);

    /*
    unsafe {
        printf_compat::format("%d", [0.2], printf_compat::output(&mut s))
    }
    */

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

        if libm::fmodf(time, 1.) > 0.5 {
            synth.frequency = 110.;
        } else {
            synth.frequency = 55.;
        }
    }
}

