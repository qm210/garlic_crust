use crate::garlic_crust::*;
use crate::garlic_helper::*;
use super::*;

// A garlic clove is basically a garlic crust "preset", i.e. its internal wiring

// the member fields. I think these might even be called CloveState here?
pub struct Clove3State {
    pub output: BlockArray,
    pub volume: MonoSample,

    osc: oscillator::Oscillator,
    osc_output: Edge,

    env: envelope::Envelope,
    env_output: Edge,

    hp: filter::Filter,
    hp_output: Edge,

}

pub fn create_state() -> Clove3State {
    Clove3State {
        output: EMPTY_BLOCKARRAY,
        volume: 1.,

        osc: oscillator::Oscillator {
            shape: oscillator::BaseWave::Square,
            volume: Edge::constant(0.6),
            freq_factor: Edge::constant(1.00),
            detune: Edge::constant_stereo([0.,0.01]),
            phasemod: Edge::constant_stereo([-0.04,0.1]),
            ..Default::default()
        },
        osc_output: Edge::zero(),

        env: envelope::Envelope {
            shape: envelope::EnvShape::Common {
                base: envelope::BaseEnv::ExpDecay,
                attack: Edge::constant(0.03),
                decay: Edge::constant(1.),
                sustain: Edge::constant(0.5),
            },
            ..Default::default()
        },
        env_output: Edge::zero(),

        hp: filter::Filter {
            shape: filter::FilterType::HiPass,
            cutoff: Edge::constant(1000.),
            ..Default::default()
        },
        hp_output: Edge::zero(),
    }
}

/* process() is the heart of the Garlic Clove and will be generated by knober
 *
 * unclear: management of seq_cursor, output could also be in the GarlicClove1State. think about.
 * sequence would then have to be split into the blocks itself, but this could be done by garlic_extract. meh
 *
 */

#[inline]
pub fn process(sequence: &[SeqEvent], block_offset: usize, state: &mut Clove3State) {

    // first branch
    process_operator_seq(&mut state.env, &sequence, block_offset, &mut state.env_output);
    state.osc.volume = state.env_output;
    process_operator_seq(&mut state.osc, &sequence, block_offset, &mut state.osc_output);

    state.hp.input = state.osc_output;
    process_operator(&mut state.hp, &mut state.hp_output);

    state.hp_output.write_to(&mut state.output, state.volume);
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