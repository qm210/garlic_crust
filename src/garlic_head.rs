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

    // TODO: think about -- could the sequences be "const", "static" or something?
    let sequence0: [SeqEvent; 128] = [
        SeqEvent {time: 0.000, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 0.219, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.219, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 0.438, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.438, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 0.657, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.657, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 0.876, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.876, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 1.095, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.095, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 1.314, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.314, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 1.533, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.533, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 1.752, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.752, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 1.971, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.971, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 2.190, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.190, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 2.409, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.409, message: SeqMsg::NoteOn(39, 113) },
        SeqEvent {time: 2.628, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.628, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 2.847, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.847, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 3.066, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.066, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 3.285, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.285, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 3.504, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.504, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 3.723, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.723, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 3.942, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.942, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 4.161, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.161, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 4.380, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.380, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 4.599, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.599, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 4.818, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.818, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 5.036, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.036, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 5.255, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.255, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 5.474, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.474, message: SeqMsg::NoteOn(49, 113) },
        SeqEvent {time: 5.693, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.693, message: SeqMsg::NoteOn(56, 113) },
        SeqEvent {time: 5.912, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.912, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 6.131, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.131, message: SeqMsg::NoteOn(58, 113) },
        SeqEvent {time: 6.350, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.350, message: SeqMsg::NoteOn(52, 113) },
        SeqEvent {time: 6.569, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.569, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 6.788, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.788, message: SeqMsg::NoteOn(55, 113) },
        SeqEvent {time: 7.007, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.007, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 7.226, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.226, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 7.445, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.445, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 7.664, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.664, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 7.883, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.883, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 8.102, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.102, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 8.321, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.321, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 8.540, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.540, message: SeqMsg::NoteOn(36, 113) },
        SeqEvent {time: 8.759, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.759, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 8.978, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.978, message: SeqMsg::NoteOn(39, 113) },
        SeqEvent {time: 9.197, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.197, message: SeqMsg::NoteOn(47, 113) },
        SeqEvent {time: 9.416, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.416, message: SeqMsg::NoteOn(42, 113) },
        SeqEvent {time: 9.635, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.635, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 9.854, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.854, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 10.073, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.073, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 10.292, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.292, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 10.511, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.511, message: SeqMsg::NoteOn(53, 113) },
        SeqEvent {time: 10.730, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.730, message: SeqMsg::NoteOn(41, 113) },
        SeqEvent {time: 10.949, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.949, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 11.168, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.168, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 11.387, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.387, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 11.606, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.606, message: SeqMsg::NoteOn(44, 113) },
        SeqEvent {time: 11.825, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.825, message: SeqMsg::NoteOn(48, 113) },
        SeqEvent {time: 12.044, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.044, message: SeqMsg::NoteOn(40, 113) },
        SeqEvent {time: 12.263, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.263, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 12.482, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.482, message: SeqMsg::NoteOn(49, 113) },
        SeqEvent {time: 12.701, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.701, message: SeqMsg::NoteOn(56, 113) },
        SeqEvent {time: 12.920, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.920, message: SeqMsg::NoteOn(51, 113) },
        SeqEvent {time: 13.139, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.139, message: SeqMsg::NoteOn(58, 113) },
        SeqEvent {time: 13.358, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.358, message: SeqMsg::NoteOn(52, 113) },
        SeqEvent {time: 13.577, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.577, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 13.796, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.796, message: SeqMsg::NoteOn(55, 113) },
        SeqEvent {time: 14.015, message: SeqMsg::NoteOff },
    ];

let sequence1: [SeqEvent; 128] = [
        SeqEvent {time: 0.000, message: SeqMsg::NoteOn(77, 113) },
        SeqEvent {time: 0.219, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.219, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 0.438, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.438, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 0.657, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.657, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 0.876, message: SeqMsg::NoteOff },
        SeqEvent {time: 0.876, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 1.095, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.095, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 1.314, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.314, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 1.533, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.533, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 1.752, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.752, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 1.971, message: SeqMsg::NoteOff },
        SeqEvent {time: 1.971, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 2.190, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.190, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 2.409, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.409, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 2.628, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.628, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 2.847, message: SeqMsg::NoteOff },
        SeqEvent {time: 2.847, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 3.066, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.066, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 3.285, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.285, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 3.504, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.504, message: SeqMsg::NoteOn(77, 113) },
        SeqEvent {time: 3.723, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.723, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 3.942, message: SeqMsg::NoteOff },
        SeqEvent {time: 3.942, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 4.161, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.161, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 4.380, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.380, message: SeqMsg::NoteOn(71, 113) },
        SeqEvent {time: 4.599, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.599, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 4.818, message: SeqMsg::NoteOff },
        SeqEvent {time: 4.818, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 5.036, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.036, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 5.255, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.255, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 5.474, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.474, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 5.693, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.693, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 5.912, message: SeqMsg::NoteOff },
        SeqEvent {time: 5.912, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 6.131, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.131, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 6.350, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.350, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 6.569, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.569, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 6.788, message: SeqMsg::NoteOff },
        SeqEvent {time: 6.788, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 7.007, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.007, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 7.226, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.226, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 7.445, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.445, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 7.664, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.664, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 7.883, message: SeqMsg::NoteOff },
        SeqEvent {time: 7.883, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 8.102, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.102, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 8.321, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.321, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 8.540, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.540, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 8.759, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.759, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 8.978, message: SeqMsg::NoteOff },
        SeqEvent {time: 8.978, message: SeqMsg::NoteOn(75, 113) },
        SeqEvent {time: 9.197, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.197, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 9.416, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.416, message: SeqMsg::NoteOn(75, 113) },
        SeqEvent {time: 9.635, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.635, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 9.854, message: SeqMsg::NoteOff },
        SeqEvent {time: 9.854, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 10.073, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.073, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 10.292, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.292, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 10.511, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.511, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 10.730, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.730, message: SeqMsg::NoteOn(65, 113) },
        SeqEvent {time: 10.949, message: SeqMsg::NoteOff },
        SeqEvent {time: 10.949, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 11.168, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.168, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 11.387, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.387, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 11.606, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.606, message: SeqMsg::NoteOn(60, 113) },
        SeqEvent {time: 11.825, message: SeqMsg::NoteOff },
        SeqEvent {time: 11.825, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 12.044, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.044, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 12.263, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.263, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 12.482, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.482, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 12.701, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.701, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 12.920, message: SeqMsg::NoteOff },
        SeqEvent {time: 12.920, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 13.139, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.139, message: SeqMsg::NoteOn(68, 113) },
        SeqEvent {time: 13.358, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.358, message: SeqMsg::NoteOn(70, 113) },
        SeqEvent {time: 13.577, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.577, message: SeqMsg::NoteOn(67, 113) },
        SeqEvent {time: 13.796, message: SeqMsg::NoteOff },
        SeqEvent {time: 13.796, message: SeqMsg::NoteOn(64, 113) },
        SeqEvent {time: 14.015, message: SeqMsg::NoteOff },
    ];

let sequence2: [SeqEvent; 2] = [
        SeqEvent {time: 8.102, message: SeqMsg::NoteOn(72, 113) },
        SeqEvent {time: 8.321, message: SeqMsg::NoteOff },
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