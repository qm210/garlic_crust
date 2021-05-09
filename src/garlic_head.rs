use super::garlic_crust::*;

pub const SECONDS: TimeFloat = 14.015 + 2.; // add something to be sure to finish
pub const BLOCK_SIZE: usize = 1024;
pub const BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS) as usize / BLOCK_SIZE) + 1;
pub const SAMPLES: usize = BLOCK_NUMBER * BLOCK_SIZE;

pub type BlockArray = [AmpFloat; BLOCK_SIZE];

pub const EMPTY_BLOCKARRAY: BlockArray = [0.; BLOCK_SIZE];

mod garlic_clove1;

// TODO: track could be a byte array. if that saves us something?

// TODO: Give BPM information via Dynamo210-style Garlic Clock

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    // TODO: think about -- could the sequences be "const", "static" or ?
    let sequence0: [SeqEvent; 65] = [
        SeqEvent {time: 0, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 2189, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 4379, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 6569, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 8759, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 10948, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 13138, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 15328, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 17518, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 19708, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 21897, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 24087, message: SeqMsg::NoteOn(39, 113) },
        SeqEvent {time: 26277, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 28467, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 30656, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 32846, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 35036, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 37226, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 39416, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 41605, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 43795, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 45985, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 48175, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 50364, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 52554, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 54744, message: SeqMsg::NoteOn(49, 113) },
        SeqEvent {time: 56934, message: SeqMsg::NoteOn(56, 113) },
        SeqEvent {time: 59124, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 61313, message: SeqMsg::NoteOn(58, 113) },
        SeqEvent {time: 63503, message: SeqMsg::NoteOn(52, 113) },
        SeqEvent {time: 65693, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 67883, message: SeqMsg::NoteOn(55, 113) },
        SeqEvent {time: 70072, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 72262, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 74452, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 76642, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 78832, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 81021, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 83211, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 85401, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 87591, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 89780, message: SeqMsg::NoteOn(39, 113) },
        SeqEvent {time: 91970, message: SeqMsg::NoteOn(47, 113) },
        SeqEvent {time: 94160, message: SeqMsg::NoteOn(42, 113) },
        SeqEvent {time: 96350, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 98540, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 100729, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 102919, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 105109, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 107299, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 109489, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 111678, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 113868, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 116058, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 118248, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 120437, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 122627, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 124817, message: SeqMsg::NoteOn(49, 113) },
        SeqEvent {time: 127007, message: SeqMsg::NoteOn(56, 113) },
        SeqEvent {time: 129197, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 131386, message: SeqMsg::NoteOn(58, 113) },
        SeqEvent {time: 133576, message: SeqMsg::NoteOn(52, 113) },
        SeqEvent {time: 135766, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 137956, message: SeqMsg::NoteOn(55, 113) },
        SeqEvent {time: 140145, message: SeqMsg::NoteOff },
    ];

    let sequence1: [SeqEvent; 65] = [
        SeqEvent {time: 0, message: SeqMsg::NoteOn(77, 113) },
        SeqEvent {time: 2189, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 4379, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 6569, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 8759, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 10948, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 13138, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 15328, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 17518, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 19708, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 21897, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 24087, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 26277, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 28467, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 30656, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 32846, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 35036, message: SeqMsg::NoteOn(77, 113) },
        SeqEvent {time: 37226, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 39416, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 41605, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 43795, message: SeqMsg::NoteOn(71, 113) },
        SeqEvent {time: 45985, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 48175, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 50364, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 52554, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 54744, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 56934, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 59124, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 61313, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 63503, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 65693, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 67883, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 70072, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 72262, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 74452, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 76642, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 78832, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 81021, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 83211, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 85401, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 87591, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 89780, message: SeqMsg::NoteOn(75, 113) },
        SeqEvent {time: 91970, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 94160, message: SeqMsg::NoteOn(75, 113) },
        SeqEvent {time: 96350, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 98540, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 100729, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 102919, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 105109, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 107299, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 109489, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 111678, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 113868, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 116058, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 118248, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 120437, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 122627, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 124817, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 127007, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 129197, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 131386, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 133576, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 135766, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 137956, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 140145, message: SeqMsg::NoteOff },
    ];

    let sequence2: [SeqEvent; 2] = [
        SeqEvent {time: 81021, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 83211, message: SeqMsg::NoteOff },
    ];

    // we need global initialization, one per clove and each their sequence
    let mut clove1_state0 = garlic_clove1::create_state();
    let mut clove1_state1 = garlic_clove1::create_state();
    let mut clove1_state2 = garlic_clove1::create_state();

    let mut block_offset = 0;
    while block_offset < SAMPLES {

        // our tooling (knober) has to know: which track is used by which clove?
        let track0 = garlic_clove1::process(&sequence0, block_offset, &mut clove1_state0);
        let track1 = garlic_clove1::process(&sequence1, block_offset, &mut clove1_state1);
        let track2 = garlic_clove1::process(&sequence2, block_offset, &mut clove1_state2);

        for sample in 0 .. BLOCK_SIZE {
            let output0 = track0.evaluate(sample);
            let output1 = track1.evaluate(sample);
            let output2 = track2.evaluate(sample);
            data[block_offset + sample] = crate::math::satanurate(output0 + output1 + output2); // will probably clip
        }

        //super::printf("Block finished: %d .. %d of %d\n\0".as_ptr(), block_offset, block_offset + BLOCK_SIZE, SAMPLES);

        block_offset += BLOCK_SIZE;
    }

    //POST PROCESSSING (e.g. channel combining) COULD HAPPEN HERE

    let mut clipping_count = 0;
    let mut max_sample = 0.;
    let mut min_sample = 0.;

    for sample in 0 .. SAMPLES {
        if data[sample] > 1. || data[sample] < -1. {
            clipping_count += 1;
            data[sample] = data[sample].clamp(-1., 1.);
        }
        if data[sample] > max_sample {
            max_sample = data[sample];
        }
        if data[sample] < min_sample {
            min_sample = data[sample];
        }
    }

    super::printf("Real duration: %.3fs\n\0".as_ptr(), SAMPLES as f64 / SAMPLERATE as f64);
    super::printf("Range: %.3f .. %.3f\n\0".as_ptr(), min_sample as f64, max_sample as f64);
    super::printf("Clipping counter: %d\n\0".as_ptr(), clipping_count);
}

/*

    let mut synth = GarlicCrust::create_from(track_config);

    let mut event_counter = 0;
    let mut next_event = &track_event_array[event_counter];

    // loop with counter at that position made 0 bytes difference
    for sample in 0..SAMPLES {

        let time: TimeFloat = sample as TimeFloat / SAMPLERATE;

        while !synth.eot && next_event.time <= time {
            synth.handle_event(&next_event);
            event_counter += 1;
            if event_counter == track_event_array.len() {
                synth.eot = true; // is redundant if there always is a SeqMsg::EndOfTrack at the end of each track. but people need to feel safe and secure
            } else {
                next_event = &track_event_array[event_counter];
            }
        }

        let amp: AmpFloat = synth.next_frame(); //synth.next().unwrap();

        data[sample] = amp;
    }


*/