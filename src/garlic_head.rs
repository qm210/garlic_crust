use super::garlic_crust::*;

pub const SECONDS: TimeFloat = 4.;
pub const BLOCK_SIZE: usize = 4096;
pub const BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS) as usize / BLOCK_SIZE) + 1;
pub const SAMPLES: usize = BLOCK_NUMBER * BLOCK_SIZE;

pub type TrackArray = [AmpFloat; SAMPLES];
pub type BlockArray = [AmpFloat; BLOCK_SIZE];

mod garlic_clove1;

// TODO: track could be a byte array. if that saves us something?

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    // TODO: think about -- could the sequences be "const"?
    let sequence1: [TrackEvent; 4] = [
        TrackEvent {time: 0., message: TrackEventMessage::NoteOn, parameter: 36.},
        TrackEvent {time: 1., message: TrackEventMessage::NoteOff, parameter: 0.},
        TrackEvent {time: 1.5, message: TrackEventMessage::NoteOn, parameter: 34.},
        TrackEvent {time: 2.5, message: TrackEventMessage::EndOfTrack, parameter: 0.}
    ];

    let mut block_cursor = 0;
    while block_cursor < BLOCK_SIZE {
        // our tooling has to know: which track is used by which clove?
        let track1 = garlic_clove1::process(&sequence1, block_cursor);

        for sample in 0 .. BLOCK_SIZE {
            data[block_cursor + sample] = track1[sample];
        }

        block_cursor += BLOCK_SIZE;
    }

    //POST PROCESSSING (e.g. channel combining) COULD HAPPEN HERE

    let mut clipping_count = 0;
    for sample in 0 .. SAMPLES {
        if data[sample] > 1. || data[sample] < -1. {
            clipping_count += 1;
            data[sample] = data[sample].clamp(-1., 1.);
        }
    }

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
                synth.eot = true; // is redundant if there always is a TrackEventMessage::EndOfTrack at the end of each track. but people need to feel safe and secure
            } else {
                next_event = &track_event_array[event_counter];
            }
        }

        let amp: AmpFloat = synth.next_frame(); //synth.next().unwrap();

        data[sample] = amp;
    }


*/