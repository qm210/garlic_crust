use crate::garlic_head::{BLOCK_SIZE, BlockArray, EMPTY_BLOCKARRAY};

pub mod oscillator;
pub mod envelope;
pub mod filter;
pub mod edge;

pub use edge::Edge;

pub type TimeFloat = f32;
pub type AmpFloat = f32;

pub const SAMPLERATE: f32 = 44100.;

pub trait Operator {
    fn handle_message(&mut self, message: &SeqMsg);
    fn evaluate(&mut self, sample: usize, total_time: TimeFloat) -> AmpFloat;
    fn advance(&mut self, sample: usize);
    fn get_cursor(&mut self) -> usize;
    fn inc_cursor(&mut self);
}

pub fn next_event_option(sequence: &[SeqEvent], cursor: usize) -> Option<SeqNormalizedEvent> {
    match cursor == sequence.len() {
        true => None,
        false => Some(SeqNormalizedEvent::from(&sequence[cursor]))
    }
}

pub fn process_operator_seq<O: Operator>(op: &mut O, sequence: &[SeqEvent], block_offset: usize) -> Edge {
    let mut output = EMPTY_BLOCKARRAY;

    let mut next_event = next_event_option(&sequence, op.get_cursor());

    for sample in 0 .. BLOCK_SIZE {
        let time: TimeFloat = (sample + block_offset) as TimeFloat / SAMPLERATE;

        while let Some(event) = next_event {
            if event.time > time {
                break;
            }
            op.handle_message(&event.message);
            op.inc_cursor();
            next_event = next_event_option(&sequence, op.get_cursor());
        }

        output[sample] = op.evaluate(sample + block_offset, time);

        op.advance(sample);
    }

    Edge::array(output)
}

pub fn process_operator<O: Operator>(op: &mut O, block_offset: usize) -> Edge {
    let mut output = EMPTY_BLOCKARRAY;

    for sample in 0 .. BLOCK_SIZE {
        let time: TimeFloat = (sample + block_offset) as TimeFloat / SAMPLERATE;

        output[sample] = op.evaluate(sample, time);

        op.advance(sample);
    }

    Edge::array(output)
}

pub type SeqParameter = usize; // check whether we have enough withi half::f16

// design decision for now: garlic_extract will take BPM information and give you a sequence over _time_
#[derive(Clone, Copy, Debug)]
pub struct SeqEvent {
    pub time: u32, // in milliseconds, this should be precise enough
    pub message: SeqMsg,
}

#[derive(Clone, Copy, Debug)]
pub struct SeqNormalizedEvent {
    pub time: TimeFloat,
    pub message: SeqMsg,
}

impl SeqNormalizedEvent {
    pub fn from(seq_event: &SeqEvent) -> SeqNormalizedEvent {
        SeqNormalizedEvent {
            time: 0.0001 * seq_event.time as TimeFloat,
            message: seq_event.message,
        }
    }
}

// can I do this polymorphically in no_std Rust?
#[derive(Clone, Copy, Debug)]
pub enum SeqMsg {
    NoteOn(SeqParameter, SeqParameter),
    NoteOff,
    SetVel,
    SetSlide,
    SetPan,
    // ...?
}

pub fn note_frequency(note_number: SeqParameter) -> f32 {
    440. * libm::powf(2., (note_number as f32 - 69.)/12.)
}

// LIST OF INVESTIGATIONS, watch for Size / Performance:
// ... probably after first track exists, to see REAL difference
//
// loop vs for loop -- no difference at all (sizewise)
// unsafe get_unchecked_mut vs. get_mut & unwrap
// math::sin vs other sin?
