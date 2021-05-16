use super::garlic_crust::*;

// TODO: track could be a byte array. if that saves us something?

// PUT GARLIC_EXTRACT HERE >>>>>>>>

pub const SECONDS: TimeFloat = 16.515;

const SEQUENCE_0: [SeqEvent; 65] = [
    SeqEvent {time: 5000, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 7189, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 9379, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {time: 11569, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 13759, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 15948, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 18138, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 20328, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 22518, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 24708, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 26897, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 29087, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {time: 31277, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 33467, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {time: 35656, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {time: 37846, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {time: 40036, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 42226, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {time: 44416, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {time: 46605, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 48795, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {time: 50985, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 53175, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {time: 55364, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {time: 57554, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 59744, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {time: 61934, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {time: 64124, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {time: 66313, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {time: 68503, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {time: 70693, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 72883, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {time: 75072, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 77262, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 79452, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {time: 81642, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 83832, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 86021, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 88211, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 90401, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {time: 92591, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {time: 94780, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {time: 96970, message: SeqMsg::NoteOn(47, 113) },
    SeqEvent {time: 99160, message: SeqMsg::NoteOn(42, 113) },
    SeqEvent {time: 101350, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 103540, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {time: 105729, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {time: 107919, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {time: 110109, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {time: 112299, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {time: 114489, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {time: 116678, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 118868, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {time: 121058, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {time: 123248, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {time: 125437, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {time: 127627, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 129817, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {time: 132007, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {time: 134197, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {time: 136386, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {time: 138576, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {time: 140766, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 142956, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {time: 145145, message: SeqMsg::NoteOff },
];

const SEQUENCE_1: [SeqEvent; 65] = [
    SeqEvent {time: 5000, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {time: 7189, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 9379, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 11569, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 13759, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 15948, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 18138, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 20328, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 22518, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 24708, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 26897, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 29087, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 31277, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 33467, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {time: 35656, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 37846, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 40036, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {time: 42226, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 44416, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 46605, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 48795, message: SeqMsg::NoteOn(71, 113) },
    SeqEvent {time: 50985, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 53175, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 55364, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 57554, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 59744, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 61934, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 64124, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 66313, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 68503, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 70693, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 72883, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 75072, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 77262, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 79452, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 81642, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 83832, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 86021, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 88211, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {time: 90401, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 92591, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 94780, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {time: 96970, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 99160, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {time: 101350, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 103540, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 105729, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 107919, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 110109, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 112299, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {time: 114489, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 116678, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 118868, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 121058, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {time: 123248, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {time: 125437, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 127627, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 129817, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {time: 132007, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 134197, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 136386, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {time: 138576, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {time: 140766, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {time: 142956, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {time: 145145, message: SeqMsg::NoteOff },
];

const SEQUENCE_2: [SeqEvent; 2] = [
    SeqEvent {time: 86021, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {time: 88211, message: SeqMsg::NoteOff },
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