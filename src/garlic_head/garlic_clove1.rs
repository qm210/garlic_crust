use crate::garlic_crust::*;
use super::*;

// A garlic clove is basically a garlic crust "preset", i.e. its internal wiring

// the member fields
pub struct GarlicClove1State {
    osc1: oscillator::Oscillator,
    env1: envelope::Envelope,
    lp1: filter::Filter,
}

pub fn create_state() -> GarlicClove1State {
    GarlicClove1State {
        osc1: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(0.5),
            frequency: 0.,
            phase: 0.,
            seq_cursor: 0,
            output: EMPTY_BLOCKARRAY,
        },
        env1: envelope::Envelope {
            attack: Edge::constant(1.0e-4),
            decay: Edge::constant(0.06),
            sustain: Edge::constant(0.),
            shape: envelope::BaseEnv::ExpDecay,
            seq_cursor: 0,
            playhead: 0., // would not be required if this is a function operator
        },
        lp1: filter::Filter {
            shape: filter::FilterType::LowPass,
            cutoff: Edge::constant(200.),
            input: EMPTY_BLOCKARRAY.clone(),
            output: Edge::zero(),
        }
    }
}

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut GarlicClove1State) -> Edge {
    // cloves are monophonic, there is only one time since the last noteon

    // THESE CHAINS WILL BE GIVEN BY knober

    let env1_output = Edge::array(process_operator(&mut state.env1, &sequence, block_offset));
    state.osc1.volume = env1_output;

    state.osc1.output = process_operator(&mut state.osc1, &sequence, block_offset);
    state.lp1.input = state.osc1.output.clone();

    state.lp1.output = Edge::array(process_operator_noseq(&mut state.lp1, block_offset));

    state.lp1.output
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
