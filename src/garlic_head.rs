use super::garlic_crust::*;

pub const SECONDS: TimeFloat = 4.;
pub const SAMPLES: usize = (SAMPLERATE * SECONDS) as usize;

pub unsafe fn render_track(data: &mut [AmpFloat; SAMPLES]) {

    // TODO: track will be a byte array.
    let track_event_array: [TrackEvent; 4] = [
        TrackEvent {time: 0., message: TrackEventMessage::NoteOn, parameter: 36.},
        TrackEvent {time: 1., message: TrackEventMessage::NoteOff, parameter: 0.},
        TrackEvent {time: 1.5, message: TrackEventMessage::NoteOn, parameter: 34.},
        TrackEvent {time: 2.5, message: TrackEventMessage::EndOfTrack, parameter: 0.}
    ];

    let track_config = InstrumentConfig {
        shape: BaseWave::Saw,
        volume: 0.2,
    };

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
}
