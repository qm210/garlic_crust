use super::garlic_crust::*;

// TODO: track could be a byte array. if that saves us something?

// PUT GARLIC_EXTRACT HERE >>>>>>>>

pub const SECONDS: TimeFloat = 16.515;

const SEQUENCE_0: [SeqEvent; 65] = [
    SeqEvent {pos: 22050, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 31706, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 41363, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 51020, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 60677, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 70334, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 79991, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 89648, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 99305, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 108962, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 118619, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 128276, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {pos: 137933, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 147590, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 157247, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 166903, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 176560, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 186217, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 195874, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 205531, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 215188, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 224845, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 234502, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 244159, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 253816, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 263473, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {pos: 273130, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {pos: 282787, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 292444, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {pos: 302100, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {pos: 311757, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 321414, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {pos: 331071, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 340728, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 350385, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 360042, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 369699, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 379356, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 389013, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 398670, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 408327, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 417984, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {pos: 427641, message: SeqMsg::NoteOn(47, 113) },
    SeqEvent {pos: 437297, message: SeqMsg::NoteOn(42, 113) },
    SeqEvent {pos: 446954, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 456611, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 466268, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 475925, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 485582, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 495239, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 504896, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 514553, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 524210, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 533867, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 543524, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 553181, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 562838, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 572495, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {pos: 582151, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {pos: 591808, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 601465, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {pos: 611122, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {pos: 620779, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 630436, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {pos: 640093, message: SeqMsg::NoteOff },
];

const SEQUENCE_1: [SeqEvent; 65] = [
    SeqEvent {pos: 22050, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 31706, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 41363, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 51020, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 60677, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 70334, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 79991, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 89648, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 99305, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 108962, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 118619, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 128276, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 137933, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 147590, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 157247, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 166903, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 176560, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 186217, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 195874, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 205531, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 215188, message: SeqMsg::NoteOn(71, 113) },
    SeqEvent {pos: 224845, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 234502, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 244159, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 253816, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 263473, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 273130, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 282787, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 292444, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 302100, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 311757, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 321414, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 331071, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 340728, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 350385, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 360042, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 369699, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 379356, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 389013, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 398670, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 408327, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 417984, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 427641, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 437297, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 446954, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 456611, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 466268, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 475925, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 485582, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 495239, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 504896, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 514553, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 524210, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 533867, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 543524, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 553181, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 562838, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 572495, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 582151, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 591808, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 601465, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 611122, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 620779, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 630436, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 640093, message: SeqMsg::NoteOff },
];

const SEQUENCE_2: [SeqEvent; 2] = [
    SeqEvent {pos: 379356, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 389013, message: SeqMsg::NoteOff },
];

// <<<<<<<< PUT GARLIC_EXTRACT HERE

pub const BLOCK_SIZE: usize = 1024;
pub const BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS) as usize / BLOCK_SIZE) + 1;
pub const SAMPLES: usize = BLOCK_NUMBER * BLOCK_SIZE;

pub type BlockArray = [AmpFloat; BLOCK_SIZE];

pub const EMPTY_BLOCKARRAY: BlockArray = [0.; BLOCK_SIZE];

mod garlic_clove1;

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    // we need global initialization, one per clove and each their sequence
    let clove1_config1 = garlic_clove1::create_config1("default");
    let clove1_config2 = garlic_clove1::create_config2("default");
    let mut clove1_state0 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    let mut clove1_state1 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    let mut clove1_state2 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);

    let mut block_offset = 0;
    while block_offset < SAMPLES {

        // our tooling (knober) has to know: which track is used by which clove?
        let track0 = garlic_clove1::process(&SEQUENCE_0, block_offset, &mut clove1_state0);
        let track1 = garlic_clove1::process(&SEQUENCE_1, block_offset, &mut clove1_state1);
        let track2 = garlic_clove1::process(&SEQUENCE_2, block_offset, &mut clove1_state2);

        for sample in 0 .. BLOCK_SIZE {
            let output0 = track0.evaluate(sample);
            let output1 = track1.evaluate(sample);
            let output2 = track2.evaluate(sample);
            data[block_offset + sample] = crate::math::satanurate(output0 + output1 + output2); // will probably clip
        }

        //super::printf("Block finished: %d .. %d of %d\n\0".as_ptr(), block_offset, block_offset + BLOCK_SIZE, SAMPLES);

        block_offset += BLOCK_SIZE;
    }

    //POST PROCESSSING (e.g. channel mixing / mastering) COULD HAPPEN HERE

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