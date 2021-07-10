use crate::garlic_crust::*;
use super::*;
use crate::garlic_helper::*;

// Garlic Smashsare Drum Synth Cloves
// they are monosynths by design
// and use pattern functions for triggering instead of NoteOn Sequences

// the member fields
pub struct SmashState {
    pub output: BlockArray,
    pub volume: MonoSample,

    osc: oscillator::Oscillator,
    osc_output: Edge,

    /*
    lfo: oscillator::Oscillator,
    lfo_output: Edge,
    */

    osc_mod: oscillator::Oscillator,
    osc_mod_output: Edge,

    env_vca: envelope::Envelope,
    env_vca_output: Edge,

    // waveshapes are just math blocks (i.e. some function), but their parameters have to be set here
    quad_shape: QuadWaveShape,

    filter: filter::Filter,
    filter_output: Edge,
}

pub fn create_state() -> SmashState {
    SmashState {
        output: EMPTY_BLOCKARRAY,
        volume: 0.07, // could be parameter in create_state

        osc: oscillator::Oscillator {
            frequency: Edge::constant(8000.), // F#1
            phasemod: Edge::constant_stereo([0.3, 0.]),
            shape: oscillator::BaseWave::Sine,
            ..Default::default()
        },
        osc_output: Edge::zero(),

        /* // do this later
        lfo: oscillator::Oscillator {
            frequency: Edge::constant(149. * 0.25),

        },
        lfo_output: Edge::zero(),
        */
        osc_mod: oscillator::Oscillator {
            frequency: Edge::constant(10000.),
            volume: Edge::constant(200.),
            ..Default::default()
        },
        osc_mod_output: Edge::zero(),

        env_vca: envelope::Envelope {
            shape: envelope::EnvShape::Generic {
                func: snare_amp_env
            },
            ..Default::default()
        },
        env_vca_output: Edge::zero(),

        quad_shape: QuadWaveShape::create(0., 0.05, 0.9, 0., 0.2, 0.15, 0.7),

        filter: filter::Filter {
            shape: filter::FilterType::LowPass,
            cutoff: Edge::constant(8456.),
            ..Default::default()
        },
        filter_output: Edge::zero(),
    }
}

const AH: f32 = 0.0;
const AHD: f32 = AH + 0.3;

fn snare_amp_env(t: TimeFloat) -> MonoSample {
    match t {
        x if x < AH => 1.,
        x if x < AHD => crate::math::powerslope(t, AH, AHD, 1., 0., 3.),
        _ => 0.
    }
}

/* process() is the heart of the Garlic Smash and will be generated by knober
 *
 * unclear: management of seq_cursor, output could also be in the GarlicSmashState. think about.
 * sequence would then have to be split into the blocks itself, but this could be done by garlic_extract. meh
 *
 */
#[inline]
pub fn process(block_offset: usize, state: &mut SmashState) {

    process_operator_dyn(&mut state.env_vca, &trigger, block_offset, &mut state.env_vca_output);

    process_operator_dyn(&mut state.osc_mod, &trigger, block_offset, &mut state.osc_mod_output);

    // generate_from_random(make_chaos, block_offset, &mut state.chaos_output); // do this tomorrow or sometime

    state.osc.volume = state.env_vca_output;
    // state.osc.volume.multiply(&state.chaos_output);
    state.osc.detune = state.osc_mod_output;

    process_operator(&mut state.osc, &mut state.osc_output);

    state.filter.input = state.osc_output;
    process_operator(&mut state.filter, &mut state.filter_output);

    waveshape_quad(&mut state.filter_output, &state.quad_shape);

    //math_overdrive(&mut state.filter_output, &state.dist);

    state.filter_output.write_to(&mut state.output, state.volume);
}

/* trigger() holds, as a mathematical function, the repetition pattern of the kick.
 * it will be produced by dynamo210 some day.
*/
#[inline]
pub fn trigger(total_sample: usize) -> bool {
    return false;

    match DYNAMO.beat(total_sample) {
        b => {
            let b_5 = libm::fmodf(b, 0.5);
            libm::fmodf(b, 0.25) < INV_SAMPLERATE && b_5 > 0.25
        }
    }
}

/*
fn make_chaos(t: TimeFloat) -> Sample {
    [
        0., 0.
    ]
}
*/