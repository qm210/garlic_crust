use crate::garlic_head::{BLOCK_SIZE, BlockArray, EMPTY_BLOCKARRAY};

pub mod oscillator;
pub mod envelope;
pub mod filter;
pub mod edge;
pub mod garlic_breath;

pub use edge::Edge;

pub type StereoFunc = fn(TimeFloat) -> Sample;
pub type MonoFunc = fn(TimeFloat) -> MonoSample;

pub type TimeFloat = f32;
pub type MonoSample = f32;

const TRIGGER: SeqMsg = SeqMsg::Init;
pub type TriggerFunc = dyn Fn(usize) -> bool;

pub type Sample = [MonoSample; 2];
pub const L: usize = 0;
pub const R: usize = 1;
pub const ZERO_SAMPLE: Sample = [0., 0.];

pub const SAMPLERATE: f32 = 44100.;
pub const INV_SAMPLERATE: f32 = 1./44100.;

#[inline]
pub fn as_time(sample: usize) -> TimeFloat {
    (sample as TimeFloat) * INV_SAMPLERATE
}

pub trait Operator {
    fn handle_message(&mut self, message: &SeqMsg);
    fn evaluate(&mut self, sample: usize) -> Sample;
    fn advance(&mut self, sample: usize);
    fn get_cursor(&mut self) -> usize;
    fn inc_cursor(&mut self);
}

pub fn next_event_option(sequence: &[SeqEvent], cursor: usize) -> Option<SeqEvent> {
    match cursor == sequence.len() {
        true => None,
        false => Some(sequence[cursor])
    }
}

pub fn process_operator_seq<O: Operator>(op: &mut O, sequence: &[SeqEvent], block_offset: usize, output: &mut Edge) {
    let mut next_event = next_event_option(&sequence, op.get_cursor());

    for sample in 0 .. BLOCK_SIZE {
        while let Some(event) = next_event {
            if event.pos > sample + block_offset {
                break;
            }
            op.handle_message(&event.message);
            op.inc_cursor();
            next_event = next_event_option(&sequence, op.get_cursor());
        }

        output.put_at(sample, op.evaluate(sample));
        op.advance(sample);
    }
}

pub fn process_operator<O: Operator>(op: &mut O, output: &mut Edge) {
    for sample in 0 .. BLOCK_SIZE {
        output.put_at(sample, op.evaluate(sample));
        op.advance(sample);
    }
}

pub fn process_operator_dyn<O: Operator>(op: &mut O, trigger: &TriggerFunc, block_offset: usize, output: &mut Edge) {

    for sample in 0 .. BLOCK_SIZE {
        let mut value = op.evaluate(sample);

        if trigger(block_offset + sample) {
            op.handle_message(&TRIGGER);
            value[L] *= 0.5;
            value[R] *= 0.5;
        }

        output.put_at(sample, value);
        op.advance(sample);
    }
}

pub fn generate_from_func(func: StereoFunc, block_offset: usize, output: &mut Edge) {
    for sample in 0 .. BLOCK_SIZE {
        output.put_at(sample, func((sample + block_offset) as TimeFloat * INV_SAMPLERATE));
    }
}

pub fn generate_from_mono_func(func: MonoFunc, block_offset: usize, output: &mut Edge) {
    for sample in 0 .. BLOCK_SIZE {
        let result = func((sample + block_offset) as TimeFloat * INV_SAMPLERATE);
        for ch in 0 .. 2 {
            output.put_at_mono(sample, ch, result);
        }
    }
}

pub type SeqParameter = usize; // check whether we have enough withi half::f16

// design decision for now: garlic_extract will take BPM information and give you a sequence over _time_
#[derive(Clone, Copy, Debug)]
pub struct SeqEvent {
    pub pos: usize, // given as the position of the sample, we have max 970.000s with u32 (should be enough). didn't make any difference to define as u32 here.
    pub message: SeqMsg,
}

// can I do this polymorphically in no_std Rust?
#[derive(Clone, Copy, Debug)]
pub enum SeqMsg {
    NoteOn(SeqParameter, SeqParameter),
    NoteOff,
    SetVel,
    SetSlide,
    SetPan,
    Init,
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


pub type SampleTuple = (f32, f32);
pub const ZERO_TUPLE: SampleTuple = (0., 0.);

pub fn sample_to_tuple (sample: &Sample) -> SampleTuple {
    (sample[L], sample[R])
}
