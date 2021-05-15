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
    lp1: filter::Filter,

    math_lfofiltertransform: Edge,

    osc_osc1_output: Edge,
    env_osc1_output: Edge,
    osc_lfo1_output: Edge,
    osc_osc2_output: Edge,
    env_osc2_output: Edge,
    lp1_output: Edge,
}

pub fn create_state() -> GarlicClove1State {
    GarlicClove1State {
        osc_osc1: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(1.),
            frequency: Edge::zero(),
            phasemod: Edge::function(|t| 0.02 * libm::sinf(4.*t)),
            detune: Edge::function(|t| 0.1 * t),
            phase: 0.,
            seq_cursor: 0,
        },
        env_osc1: envelope::Envelope {
            attack: Edge::zero(),
            decay: Edge::constant(0.3),
            sustain: Edge::constant(1.),
            shape: envelope::BaseEnv::ExpDecay,
            min: Edge::zero(),
            max: Edge::constant(1.),
            note_vel: 0., // also as Edge? actually this is a note parameter
            seq_cursor: 0,
            playhead: 0., // would not be required if this is a function operator
        },
        osc_osc2: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(1.),
            frequency: Edge::zero(),
            phasemod: Edge::zero(),
            detune: Edge::zero(),
            phase: 0.,
            seq_cursor: 0,
        },
        env_osc2: envelope::Envelope {
            attack: Edge::zero(),
            decay: Edge::constant(0.3),
            sustain: Edge::constant(1.),
            shape: envelope::BaseEnv::ExpDecay,
            min: Edge::zero(),
            max: Edge::constant(1.),
            note_vel: 0.,
            seq_cursor: 0,
            playhead: 0.,
        },
        osc_lfo1: oscillator::Oscillator {
            shape: oscillator::BaseWave::Triangle,
            volume: Edge::constant(1.),
            frequency: Edge::constant(12.),
            phasemod: Edge::zero(),
            detune: Edge::zero(),
            phase: 0.,
            seq_cursor: 0,
        },
        lp1: filter::Filter {
            shape: filter::FilterType::LowPass,
            cutoff: Edge::constant(10000.),
            state: filter::FilterState::new(),
            input: Edge::zero(),
        },

        math_lfofiltertransform: Edge::zero(),

        osc_osc1_output: Edge::zero(),
        env_osc1_output: Edge::zero(),
        osc_lfo1_output: Edge::zero(),
        osc_osc2_output: Edge::zero(),
        env_osc2_output: Edge::zero(),
        lp1_output: Edge::zero(),
    }
}

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut GarlicClove1State) -> Edge {
    // cloves are monophonic, there is only one time since the last noteon

    // unclear: management of seq_cursor, output could also be in the GarlicClove1State. think about.
    // sequence would then have to be split into the blocks itself, but this could be done by garlic_extract

    // THESE CHAINS WILL BE GIVEN BY knober

    // first branch
    process_operator_seq(&mut state.env_osc1, &sequence, block_offset, &mut state.env_osc1_output);
    state.osc_osc1.volume = state.env_osc1_output;
    process_operator_seq(&mut state.osc_osc1, &sequence, block_offset, &mut state.osc_osc1_output);

    // second branch
    process_operator_seq(&mut state.env_osc2, &sequence, block_offset, &mut state.env_osc2_output);
    state.osc_osc2.volume = state.env_osc2_output;
    process_operator_seq(&mut state.osc_osc2, &sequence, block_offset, &mut state.osc_osc2_output);

    // third branch
    process_operator(&mut state.osc_lfo1, block_offset, &mut state.osc_lfo1_output);
    state.math_lfofiltertransform = state.osc_lfo1_output.mad(&Edge::constant(0.1), &Edge::constant(0.5)); // this is the simple (m*x + b) math block

    // filter junction
    //state.lp1.input = math_mixer(&osc_osc1_output, &osc_osc2_output, &Edge::constant(0.5)); // more advanced blocks will have to be converted to Rust code, but I can help with that
    state.lp1.input = math_mixer(&state.osc_osc1_output, &Edge::constant(1.), &state.osc_osc2_output); // more advanced blocks will have to be converted to Rust code, but I can help with that
    //state.lp1.cutoff = state.lp1.cutoff.times(&state.math_lfofiltertransform);
    process_operator(&mut state.lp1, block_offset, &mut state.lp1_output);

    state.lp1_output
}

// individual math operators (more complex than Edge::mad()) might be created directly in the clove
// note: this is actually also a .mad block
fn math_mixer(input1: &Edge, input2: &Edge, cv: &Edge) -> Edge {
    let mut output = EMPTY_BLOCKARRAY;
    for sample in 0 .. BLOCK_SIZE {
        output[sample] = cv.evaluate(sample) * (input1.evaluate(sample) + input2.evaluate(sample));
    }
    Edge::array(output)
}

// with this commit: 71.3 seconds for 16 second track (outputs not stored in Op, block_size 1024)
// same with block_size 256: 10 seconds?? wtf?
// block_size 512: 55 seconds;

// THINGS TO TEST:
// put "env_osc1_output" again as a field of "env_osc1.output", if that helps the compiler?
// Split Sequence into Chunks, one for each 512-sample-block
// Put Sequence into Byte Array
// use get_unchecked()
// multithreading?? -- each Clove can be processed simultaneously
// should every Edge always hold its array??