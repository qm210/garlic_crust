use super::garlic_crust::*;

pub const SECONDS: TimeFloat = 5.;
pub const BLOCK_SIZE: usize = 1024;
pub const BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS) as usize / BLOCK_SIZE) + 1;
pub const SAMPLES: usize = BLOCK_NUMBER * BLOCK_SIZE;

pub type BlockArray = [AmpFloat; BLOCK_SIZE];

pub const EMPTY_BLOCKARRAY: BlockArray = [0.; BLOCK_SIZE];

mod garlic_clove1;

// TODO: track could be a byte array. if that saves us something?

// TODO: Give BPM information via Dynamo210-style Garlic Clock

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    let factor: f32 = 0.6;
    // TODO: think about -- could the sequences be "const"?
    let sequence1: [SeqEvent; 15] = [
        SeqEvent {time: factor * 0., message: SeqMsg::NoteOn, parameter: 36.},
        SeqEvent {time: factor * 0.5, message: SeqMsg::NoteOn, parameter: 38.},
        SeqEvent {time: factor * 1.0, message: SeqMsg::NoteOn, parameter: 43.},
        SeqEvent {time: factor * 1.5, message: SeqMsg::NoteOn, parameter: 60.},
        SeqEvent {time: factor * 2.0, message: SeqMsg::NoteOn, parameter: 36.},
        SeqEvent {time: factor * 2.5, message: SeqMsg::NoteOn, parameter: 50.},
        SeqEvent {time: factor * 3.0, message: SeqMsg::NoteOn, parameter: 55.},
        SeqEvent {time: factor * 3.5, message: SeqMsg::NoteOn, parameter: 60.},
        SeqEvent {time: factor * 4.0, message: SeqMsg::NoteOn, parameter: 36.},
        SeqEvent {time: factor * 4.5, message: SeqMsg::NoteOn, parameter: 38.},
        SeqEvent {time: factor * 5.0, message: SeqMsg::NoteOn, parameter: 43.},
        SeqEvent {time: factor * 5.5, message: SeqMsg::NoteOn, parameter: 60.},
        SeqEvent {time: factor * 6.0, message: SeqMsg::NoteOn, parameter: 36.},
        SeqEvent {time: factor * 6.5, message: SeqMsg::NoteOn, parameter: 50.},
        SeqEvent {time: factor * 7.0, message: SeqMsg::NoteOn, parameter: 32.},
    ];

    // we need global initialization, one per clove and each their sequence
    let mut clove1_state1 = garlic_clove1::create_state();

    let mut block_offset = 0;
    while block_offset < SAMPLES {

        // our tooling (knober) has to know: which track is used by which clove?
        let track1 = garlic_clove1::process(&sequence1, block_offset, &mut clove1_state1);

        for sample in 0 .. BLOCK_SIZE {
            data[block_offset + sample] = track1.evaluate(sample);
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