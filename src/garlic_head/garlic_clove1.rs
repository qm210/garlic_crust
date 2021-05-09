use crate::garlic_crust::*;
use super::*;

// A garlic clove is basically a garlic crust "preset", i.e. its internal wiring

// the member fields
pub struct GarlicClove1State {
    osc1: oscillator::Oscillator,
    env1: envelope::Envelope,
    lp1: filter::Filter,
    cutoff_env1: envelope::Envelope,
}

pub fn create_state() -> GarlicClove1State {
    GarlicClove1State {
        osc1: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(0.5),
            frequency: Edge::zero(),
            phasemod: Edge::zero(),
            detune: Edge::zero(),
            phase: 0.,
            seq_cursor: 0,
            output: Edge::zero(),
        },
        env1: envelope::Envelope {
            attack: Edge::constant(1.0e-4),
            decay: Edge::constant(0.3),
            sustain: Edge::constant(0.),
            shape: envelope::BaseEnv::ExpDecay,
            min: Edge::zero(),
            max: Edge::one(),
            note_vel: 0., // also as Edge? actually this is a note parameter
            seq_cursor: 0,
            playhead: 0., // would not be required if this is a function operator
        },
        lp1: filter::Filter {
            shape: filter::FilterType::LowPass,
            cutoff: Edge::constant(100.),
            state: filter::FilterState::new(),
            input: Edge::zero(),
            output: Edge::zero(),
        },
        cutoff_env1: envelope::Envelope {
            attack: Edge::zero(),
            decay: Edge::constant(0.25),
            sustain: Edge::zero(),
            shape: envelope::BaseEnv::ExpDecay,
            min: Edge::constant(200.),
            max: Edge::function(|t| 30. * (1. + 20. * t)), // Edge::constant(8000.)
            note_vel: 1.,
            seq_cursor: 0,
            playhead: 0., // would not be required if this is a function operator
        },
    }
}

fn cutoff_env1_max(time: TimeFloat) -> f32 {
    800. * (1. + time)
}

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut GarlicClove1State) -> Edge {
    // cloves are monophonic, there is only one time since the last noteon

    // unclear: management of seq_cursor, output could also be in the GarlicClove1State. think about.

    // THESE CHAINS WILL BE GIVEN BY knober

    let env1_output = process_operator(&mut state.env1, &sequence, block_offset);
    state.osc1.volume = env1_output;
    state.lp1.cutoff = process_operator(&mut state.cutoff_env1, &sequence, block_offset);

    state.osc1.detune = env1_output.clone().scale(0.3);

    state.osc1.output = process_operator(&mut state.osc1, &sequence, block_offset);
    state.lp1.input = state.osc1.output;
    state.lp1.output = process_operator_noseq(&mut state.lp1, block_offset);

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
