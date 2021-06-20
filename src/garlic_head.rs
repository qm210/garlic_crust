use crate::garlic_crust::ZERO_SAMPLE;
use super::garlic_crust::*;

// need to be programmatically appended
mod garlic_clove1;
mod garlic_master;
mod garlic_smash;

mod garlic_dynamo;

// TODO: track could be a byte array. if that saves us something?

// PUT GARLIC_EXTRACT HERE >>>>>>>>

pub const SECONDS: TimeFloat = 10.;

const DYNAMO_BREAKPOINTS: usize = 1;
pub type DynamoArray = [TimeFloat; DYNAMO_BREAKPOINTS];

pub const DYNAMO: garlic_dynamo::Dynamo = garlic_dynamo::Dynamo::create(140.);

// <<<<<<<< PUT GARLIC_EXTRACT HERE

pub const BLOCK_SIZE: usize = 330; // larger blocks might result in STATUS_STACK_OVERFLOW
const MASTER_BLOCK_FACTOR: usize = 5; // my stolen freeverb needs BLOCK_SIZE * MASTER_BLOCK_FACTOR >= 1640
pub const MASTER_BLOCK_SIZE: usize = BLOCK_SIZE * MASTER_BLOCK_FACTOR;
const MASTER_BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS * 2.) as usize / MASTER_BLOCK_SIZE) + 1;
pub const SAMPLES: usize = MASTER_BLOCK_NUMBER * MASTER_BLOCK_SIZE;
pub const SAMPLES_TWICE: usize = SAMPLES * 2;

pub type BlockArray = [Sample; BLOCK_SIZE];
pub type MasterBlockArray = [Sample; MASTER_BLOCK_SIZE];
pub type MasterBlockMono = [MonoSample; MASTER_BLOCK_SIZE];
pub type StereoTrack = [MonoSample; SAMPLES_TWICE];

pub const EMPTY_BLOCKARRAY: BlockArray = [ZERO_SAMPLE; BLOCK_SIZE];

pub unsafe fn render_track(data: &mut StereoTrack) {
    let mut garlic_master = garlic_master::GarlicMaster::new(); // here would configuration go

    let mut smash_state0 = garlic_smash::create_state(); // this gonne be my kick

    // we need global initialization, one per clove and each their sequence
    // let clove1_config1 = garlic_clove1::create_config1("default");
    // let clove1_config2 = garlic_clove1::create_config2("default");
    // let mut clove1_state0 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    // let mut clove1_state1 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    // let mut clove1_state2 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    // let mut clove1_state3 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);

    let mut master_block_offset = 0;
    let mut block_offset = 0;

    while master_block_offset < SAMPLES_TWICE {

        for master_piece in 0 .. MASTER_BLOCK_FACTOR {
            // garlic_clove1::process(&SEQUENCE_0, block_offset, &mut clove1_state0);
            // garlic_clove1::process(&SEQUENCE_1, block_offset, &mut clove1_state1);
            // garlic_clove1::process(&SEQUENCE_2, block_offset, &mut clove1_state2);
            // garlic_clove1::process(&SEQUENCE_3, block_offset, &mut clove1_state3);

            garlic_smash::process(block_offset, &mut smash_state0);

            for sample in 0 .. BLOCK_SIZE {
                let master_sample = sample + master_piece * BLOCK_SIZE;

                // could merge the "put ZERO_SAMPLE" and first "add" to one "put", but we gönn ourselves for more symmetry.
                garlic_master.put_at(master_sample, ZERO_SAMPLE);

                garlic_master.add_at(master_sample, smash_state0.output[sample]);
                // garlic_master.add_at(master_sample, clove1_state0.output[sample]);
                // garlic_master.add_at(master_sample, clove1_state1.output[sample]);
                // garlic_master.add_at(master_sample, clove1_state2.output[sample]);
                // garlic_master.add_at(master_sample, clove1_state3.output[sample]);

                garlic_master.process(master_sample);
            }
            block_offset += BLOCK_SIZE;
        }

        garlic_master.write(data, master_block_offset);
        // super::printf("Block finished: %d %d .. %d\n\0".as_ptr(), master_block_offset, block_offset, SAMPLES);

        master_block_offset += 2 * MASTER_BLOCK_SIZE; // 2 * due to Stereo
    }

    let mut clipping_count = 0;
    let mut max_sample = 0.;
    let mut min_sample = 0.;

    for sample in 0 .. 2 * SAMPLES {
        if data[sample] > 1. || data[sample] < -1.
        || data[sample] > 1. || data[sample] < -1. {
            clipping_count += 1;
        }
        if data[sample] > max_sample {
            max_sample = data[sample];
        }
        if data[sample] < min_sample {
            min_sample = data[sample];
        }
    }

    super::printf("Real duration: %.3fs\n\0".as_ptr(), SAMPLES as f64 * INV_SAMPLERATE as f64);
    super::printf("Range: %.3f .. %.3f\n\0".as_ptr(), min_sample as f64, max_sample as f64);
    super::printf("Clipping counter: %d\n\0".as_ptr(), clipping_count);
}
