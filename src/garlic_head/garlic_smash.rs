use crate::garlic_crust::*;
use super::*;
use crate::garlic_helper::*;

// Garlic Smashsare Drum Synth Cloves
// they are monosynths by design
// and use pattern functions for triggering instead of NoteOn Sequences

// the member fields
pub struct Smash1State {
    pub output: BlockArray,
    pub volume: MonoSample,

    osc: oscillator::Oscillator,
    osc_output: Edge,

    env_vca: envelope::Envelope,
    env_vca_output: Edge,

    env_freq: envelope::Envelope,
    env_freq_output: Edge,

    // waveshapes are just math blocks (i.e. some function), but their parameters have to be set here
    dist: Edge,
    quad_shape: QuadWaveShape,

    lp: filter::Filter,
    lp_output: Edge,
}

pub fn create_state() -> Smash1State {
    Smash1State {
        output: EMPTY_BLOCKARRAY,
        volume: 0.5, // could be parameter in create_state

        osc: oscillator::Oscillator {
            frequency: Edge::constant(46.25), // F#1
            //detune: Edge::constant_stereo([0., 0.02]),
            //phasemod: Edge::constant_stereo([0.3, 0.]),
            shape: oscillator::BaseWave::Sine,
            ..Default::default()
        },
        osc_output: Edge::zero(),

        env_vca: envelope::Envelope {
            shape: envelope::EnvShape::Generic {
                func: kick_amp_env
            },
            ..Default::default()
        },
        env_vca_output: Edge::zero(),

        env_freq: envelope::Envelope {
            shape: envelope::EnvShape::Generic {
                func: kick_freq_env
            },
            ..Default::default()
        },
        env_freq_output: Edge::zero(),

        dist: Edge::constant(2.),
        quad_shape: QuadWaveShape::create(0., 0.1, 0.4, 0., 0.2, 0.15, 0.7),

        lp: filter::Filter {
            shape: filter::FilterType::LowPass,
            cutoff: Edge::constant(340.),
            ..Default::default()
        },
        lp_output: Edge::zero(),
    }
}

fn kick_amp_env(t: TimeFloat) -> MonoSample {
    let a = 0.002;
    let ah = a + 0.12;
    let ahd = ah + 0.00120;
    match t {
        x if x < a => crate::math::slope(t, 0., a, 0., 1.),
        x if x < ah => 1.,
        x if x < ahd => crate::math::powerslope(t, ah, ahd, 1., 0., 2.),
        _ => 0.
    }
}

fn kick_freq_env(t: TimeFloat) -> MonoSample {
    match t {
        x if x < 0.002 => 3000.,
        x if x < 0.01 => crate::math::logslope(x, 0.002, 0.01, 3000., 300.),
        x if x < 0.15 => crate::math::logslope(x, 0.01, 0.15, 300., 46.25),
        _ => 46.25,
    }
}

/* process() is the heart of the Garlic Smash and will be generated by knober
 *
 * unclear: management of seq_cursor, output could also be in the GarlicSmash1State. think about.
 * sequence would then have to be split into the blocks itself, but this could be done by garlic_extract. meh
 *
 */
#[inline]
pub fn process(block_offset: usize, state: &mut Smash1State) {

    process_operator_dyn(&mut state.env_vca, &trigger, block_offset, &mut state.env_vca_output);
    state.osc.volume = state.env_vca_output;

    process_operator_dyn(&mut state.env_freq, &trigger, block_offset, &mut state.env_freq_output);
    state.osc.frequency = state.env_freq_output;

    process_operator(&mut state.osc, &mut state.osc_output);

    state.lp.input = state.osc_output;
    process_operator(&mut state.lp, &mut state.lp_output);

    waveshape_quad(&mut state.lp_output, &state.quad_shape);
    //math_distort(&mut state.osc_output);

    math_overdrive(&mut state.lp_output, &state.dist);

    state.lp_output.write_to(&mut state.output, state.volume);
}

/* trigger() holds, as a mathematical function, the repetition pattern of the kick.
 * it will be produced by dynamo210 some day.
*/
#[inline]
pub fn trigger(total_sample: usize) -> bool {
    let total_beat = DYNAMO.beat(total_sample);
    //if total_beat >= pattern_start_beat && total_beat < pattern_end_beat //inside beat condition
    let pattern_start_beat = 0.;
    let pattern_end_beat = 4.;
    let beat = libm::fmodf(total_beat - pattern_start_beat, pattern_end_beat - pattern_start_beat);
    // two options: something regular (-> fmodf) or one-shots

    let beat_trigger = match beat {
        /*
        x if x > 1.75 && x < 1.85 => {
            INV_SAMPLERATE // anything >= INV_SAMPLERATE means no beat, is there a better constant?
        },
        x if x > 1.85 && x < 2. => {
            libm::fmodf(beat, 0.25 / 3.)
        }
        x if x > 3. => {
            libm::fmodf(beat, 0.25 / 3.) + libm::fmodf(beat + 0.25/3., 0.25 / 3.)
        },
        */
        _ => {
            libm::fmodf(beat, 0.25)
        }
    };

    return beat_trigger < INV_SAMPLERATE; // && beat_trigger >= 0. , is actually wumpe cause always true
}

// inline or not inline?
#[inline]
// individual math operators (more complex than Edge::mad()) might be created directly in the smash
fn math_mixer(input1: &Edge, input2: &Edge, cv: &Edge, output: &mut Edge) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 { // the looping could be hidden by generalizing 2000 + and * 1800 to
            output.put_at_mono(sample, ch,
                cv.evaluate_mono(sample, ch) * (input1.evaluate_mono(sample, ch) + input2.evaluate_mono(sample, ch))
            );
        }
    }
}

// with this commit: 71.3 seconds for 16 second track (outputs not stored in Op, block_size 1024)
// same with block_size 256: 10 seconds?? wtf?
// block_size 512: 55 seconds;

// THINGS TO TEST:
// put "env_osc1_output" again as a field of "env_osc1.output", if that helps the compiler?
// Split Sequence into Chunks, one for each 512-sample-block
// Put Sequence into Byte Array
// use get_unchecked()
// multithreading?? -- each Smash can be processed simultaneously
// should every Edge always hold its array??