use crate::garlic_crust::*;
use super::*;

// A garlic clove is basically a garlic crust "preset", i.e. its internal wiring

// the member fields
pub struct GarlicClove1State {
    oscA: Oscillator,
    envA: Envelope,
}

pub fn create_state() -> GarlicClove1State {
    GarlicClove1State {
        oscA: Oscillator {
            shape: BaseWave::Square,
            volume: Edge::Constant(0.5),
            frequency: 0.,
            phase: 0.,
            seq_cursor: 0,
        },
        envA: Envelope {
            attack: Edge::Constant(0.1),
            decay: Edge::Constant(0.5),
            sustain: Edge::Constant(0.),
            seq_cursor: 0,
            playhead: 0., // would not be required if this is a function operator
        }
    }
}

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut GarlicClove1State) -> BlockArray {
    // cloves are monophonic, there is only one time since the last noteon

    // THESE CHAINS WILL BE GIVEN BY knober

    let envA_output = envA_build(&mut state.envA, sequence, block_offset);
    state.oscA.volume = envA_output;
    let oscA_output = state.oscA.process(sequence, block_offset);

    oscA_output
}

/*
    Here will all the functions placed...
    Every function has a structure

    fn operator(input1: &TrackArray, ..., sequence: &[TrackEvent]) -> TrackArray {
    as many inputX: &TrackArray arrays which it actually uses, which is part of this operators definition

    e.g.
    dummy
     - inputs: 1
     - call garlic_crust::dummyOperator()
    osci
     - inputs: 0
     - call process_oscA()
     - contribution to state
*/

// TODO: somehow handle sequence as iterator?
// TODO: somehow generalize the structure of these blocks, e.g. by macro_rules! ?

// a function like this could be generated by knober
pub fn envA_build(operator: &mut Envelope, sequence: &[SeqEvent], block_offset: usize) -> Edge {
    let mut output: BlockArray = [0.; BLOCK_SIZE];

    let mut next_event = &sequence[operator.seq_cursor];

    for sample in 0 .. BLOCK_SIZE {

        let time: TimeFloat = (block_offset + sample) as TimeFloat / SAMPLERATE;

        while operator.seq_cursor < sequence.len() && next_event.time <= time {
            match &next_event.message {
                SeqMsg::NoteOn => {
                    operator.playhead = 0.;
                },
                // could react to Volume or whatevs here.
                _ => ()
            }
            operator.seq_cursor += 1;
            if operator.seq_cursor == sequence.len() {
                break;
            } else {
                next_event = &sequence[operator.seq_cursor];
            }
        }

        let attack = operator.attack.evaluate(sample);
        let decay = operator.decay.evaluate(sample);

        // the function will be placed by knober here
        let func = |t: TimeFloat| libm::exp2f(-decay*t) * crate::math::smoothstep(0., attack, &t);

        output[sample] = func(operator.playhead);

        operator.playhead += 1. / SAMPLERATE;
    }

    Edge::Array(output)
}


/* this is old */
pub fn empty_operator(sequence: &[SeqEvent], block_offset: usize) -> BlockArray {
    [0.; BLOCK_SIZE]
}

pub fn dummy_operator(input: &BlockArray, sequence: &[SeqEvent], block_offset: usize) -> BlockArray {
    let mut output = [0.; BLOCK_SIZE];

    for i in 0 .. BLOCK_SIZE {
        output[i] = input[i] + 0.1337;
    }

    output
}
