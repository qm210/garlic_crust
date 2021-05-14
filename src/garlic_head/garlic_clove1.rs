use crate::garlic_crust::*;
use super::*;

// A garlic clove is basically a garlic crust "preset", i.e. its internal wiring

// the member fields
pub struct GarlicClove1State {
    osc_osc1: oscillator::Oscillator,
    env_osc1: envelope::Envelope,
    osc_osc2: oscillator::Oscillator,
    env_osc2: envelope::Envelope,
    osc_lfo1: oscillator::Oscillator,
    math_lfofiltertransform: Edge,
    lp1: filter::Filter,
}

pub fn create_state() -> GarlicClove1State {
    GarlicClove1State {
        osc_osc1: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(0.5),
            frequency: Edge::zero(),
            phasemod: Edge::function(|t| 0.02 * libm::sinf(4.*t)),
            detune: Edge::function(|t| 0.1 * t),
            phase: 0.,
            seq_cursor: 0,
        },
        env_osc1: envelope::Envelope {
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
        osc_osc2: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(0.5),
            frequency: Edge::zero(),
            phasemod: Edge::zero(),
            detune: Edge::zero(),
            phase: 0.,
            seq_cursor: 0,
        },
        env_osc2: envelope::Envelope {
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
        osc_lfo1: oscillator::Oscillator {
            shape: oscillator::BaseWave::Triangle,
            volume: Edge::one(),
            frequency: Edge::constant(12.),
            phasemod: Edge::zero(),
            detune: Edge::zero(),
            phase: 0.,
            seq_cursor: 0,
        },
        math_lfofiltertransform: Edge::zero(),
        lp1: filter::Filter {
            shape: filter::FilterType::LowPass,
            cutoff: Edge::constant(100.),
            state: filter::FilterState::new(),
            input: Edge::zero(),
        },
    }
}

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut GarlicClove1State) -> Edge {
    // cloves are monophonic, there is only one time since the last noteon

    // unclear: management of seq_cursor, output could also be in the GarlicClove1State. think about.

    // THESE CHAINS WILL BE GIVEN BY knober

    // first branch
    let env_osc1_output = process_operator_seq(&mut state.env_osc1, &sequence, block_offset);
    state.osc_osc1.volume = env_osc1_output;
    let osc_osc1_output = process_operator_seq(&mut state.osc_osc1, &sequence, block_offset);

    // second branch
    let env_osc2_output = process_operator_seq(&mut state.env_osc2, &sequence, block_offset);
    state.osc_osc2.volume = env_osc2_output;
    let osc_osc2_output = process_operator_seq(&mut state.osc_osc2, &sequence, block_offset);

    // third branch
    let osc_lfo1_output = process_operator(&mut state.osc_lfo1, block_offset);
    state.math_lfofiltertransform = osc_lfo1_output.mad(&Edge::constant(0.1), &Edge::constant(0.5));

    state.lp1.input = math_mixer(&osc_osc1_output, &osc_osc2_output, &Edge::constant(0.5));
    state.lp1.cutoff = state.math_lfofiltertransform;
    let lp1_output = process_operator(&mut state.lp1, block_offset);

    lp1_output
}

// individual math operators (more complex than Edge::mad()) might be created directly in the clove
fn math_mixer(input1: &Edge, input2: &Edge, cv: &Edge) -> Edge {
    let mut output = EMPTY_BLOCKARRAY;
    for sample in 0 .. BLOCK_SIZE {
        output[sample] = cv.evaluate(sample) * (input1.evaluate(sample) + input2.evaluate(sample));
    }
    Edge::array(output)
}

// with this commit: 71.3 seconds for 16 second track (outputs not stored in Op)