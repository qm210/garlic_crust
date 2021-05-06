use crate::garlic_crust::*;
use super::*;

// A garlic clove is basically a garlic crust "preset", i.e. its internal wiring

// the member fields
pub struct GarlicClove1State {
    osc1: Oscillator,
    env1: Envelope,
}

pub fn create_state() -> GarlicClove1State {
    GarlicClove1State {
        osc1: Oscillator {
            shape: BaseWave::Square,
            volume: Edge::Constant(0.5),
            frequency: 0.,
            phase: 0.,
            seq_cursor: 0,
            output: EMPTY_BLOCKARRAY,
        },
        env1: Envelope {
            attack: Edge::Constant(0.8),
            decay: Edge::Constant(0.3),
            sustain: Edge::Constant(0.),
            shape: BaseEnv::ExpDecay,
            seq_cursor: 0,
            playhead: 0., // would not be required if this is a function operator
        }
    }
}

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut GarlicClove1State) -> Edge {
    // cloves are monophonic, there is only one time since the last noteon

    // THESE CHAINS WILL BE GIVEN BY knober

    let env1_output = env1_build(&mut state.env1, sequence, block_offset);
    state.osc1.volume = env1_output;

    state.osc1.process(sequence, block_offset)
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

// check whether macro helps
// next idea. operators have ONE output array, not a new one each time (e.g. for filter, to have the last one)
pub fn env1_build(operator: &mut Envelope, sequence: &[SeqEvent], block_offset: usize) -> Edge {
    let mut output: BlockArray = [0.; BLOCK_SIZE];

    output = process_operator(operator, output, sequence, block_offset);

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
