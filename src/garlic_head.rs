use crate::garlic_crust::ZERO_SAMPLE;
use super::garlic_crust::*;
use crate::sequence::*;

// need to be programmatically appended
mod garlic_clove1;
mod garlic_clove2;
mod garlic_clove3;
mod garlic_clove4;
mod garlic_clove5;
mod garlic_clove6;
mod garlic_master;
mod garlic_smash_kick;
mod garlic_smash_hihat;
mod garlic_smash_snare;
mod garlic_smash_clang;

mod garlic_dynamo;

// TODO: track could be a byte array. if that saves us something?

const DYNAMO_BREAKPOINTS: usize = 1;
pub type DynamoArray = [TimeFloat; DYNAMO_BREAKPOINTS];

pub const DYNAMO: garlic_dynamo::Dynamo = garlic_dynamo::Dynamo::create(149.);

// <<<<<<<< PUT GARLIC_EXTRACT HERE

pub const BLOCK_SIZE: usize = 50; // larger blocks might result in STATUS_STACK_OVERFLOW
const MASTER_BLOCK_FACTOR: usize = 34; // my stolen freeverb needs BLOCK_SIZE * MASTER_BLOCK_FACTOR >= 1640
pub const MASTER_BLOCK_SIZE: usize = BLOCK_SIZE * MASTER_BLOCK_FACTOR;
const MASTER_BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS) as usize / MASTER_BLOCK_SIZE) + 1;
pub const SAMPLES: usize = MASTER_BLOCK_NUMBER * MASTER_BLOCK_SIZE;
pub const SAMPLES_TWICE: usize = SAMPLES * 2;

pub type BlockArray = [Sample; BLOCK_SIZE];
pub type MasterBlockArray = [Sample; MASTER_BLOCK_SIZE];
pub type MasterBlockMono = [MonoSample; MASTER_BLOCK_SIZE];
pub type StereoTrack = [MonoSample; SAMPLES_TWICE];

pub const EMPTY_BLOCKARRAY: BlockArray = [ZERO_SAMPLE; BLOCK_SIZE];

pub unsafe fn render_track(data: &mut StereoTrack) {
    let mut garlic_master = garlic_master::GarlicMaster::new(); // here would configuration go

    // we need global initialization, one per clove and each their sequence
    let mut clove_lead_a = garlic_clove3::create_state();
    let mut clove_bass = garlic_clove4::create_state();
    let mut clove_lead_b = garlic_clove5::create_state();
    let mut clove_strings_0 = garlic_clove6::create_state();
    let mut clove_strings_1 = garlic_clove6::create_state();
    let mut clove_strings_2 = garlic_clove6::create_state();

    let mut smash_kick = garlic_smash_kick::create_state(); // this gonne be my kick
    let mut smash_hihat = garlic_smash_hihat::create_state();
    let mut smash_clang = garlic_smash_clang::create_state();
    let mut smash_snare = garlic_smash_snare::create_state();

    let mut master_block_offset = 0;
    let mut block_offset = 0;

    while master_block_offset < SAMPLES_TWICE {

        for master_piece in 0 .. MASTER_BLOCK_FACTOR {

            garlic_clove4::process(&SEQUENCE_BASS, block_offset, &mut clove_bass);
            garlic_clove3::process(&SEQUENCE_LEAD, block_offset, &mut clove_lead_a);
            garlic_clove5::process(&SEQUENCE_LEAD, block_offset, &mut clove_lead_b);
            garlic_clove6::process(&CHORDS_0, block_offset, &mut clove_strings_0);
            garlic_clove6::process(&CHORDS_1, block_offset, &mut clove_strings_1);
            garlic_clove6::process(&CHORDS_2, block_offset, &mut clove_strings_2);

            garlic_smash_kick::process(block_offset, &mut smash_kick);
            garlic_smash_hihat::process(block_offset, &mut smash_hihat);
            garlic_smash_snare::process(block_offset, &mut smash_snare);
            garlic_smash_clang::process(block_offset, &mut smash_clang);

            for sample in 0 .. BLOCK_SIZE {
                let master_sample = sample + master_piece * BLOCK_SIZE;

                // could merge the "put ZERO_SAMPLE" and first "add" to one "put", but we gönn ourselves for more symmetry.
                garlic_master.put_at(master_sample, ZERO_SAMPLE);

                garlic_master.add_at(master_sample, clove_strings_0.output[sample]);
                garlic_master.add_at(master_sample, clove_strings_1.output[sample]);
                garlic_master.add_at(master_sample, clove_strings_2.output[sample]);

                //garlic_master.saturate(master_sample);

                garlic_master.apply_heavy_reverb(master_sample, 1.0);

                // no idea why it would be there, but remove that first mystery note
                if master_block_offset + master_sample >= SEQUENCE_LEAD[0].pos {
                    garlic_master.add_at(master_sample, clove_lead_a.output[sample]);
                    garlic_master.add_at(master_sample, clove_lead_b.output[sample]);
                }

                garlic_master.add_at(master_sample, smash_clang.output[sample]);

                // remove mystery drums in second zero. have to fix process_operator_dyn / trigger logic, but deadline is in 2h
                if master_block_offset + master_sample >= 10000 {
                    garlic_master.add_at(master_sample, smash_kick.output[sample]);
                    garlic_master.add_at(master_sample, smash_hihat.output[sample]);
                    garlic_master.add_at(master_sample, smash_snare.output[sample]);
                }

                garlic_master.apply_soft_reverb(master_sample, 1.0);

                garlic_master.add_at(master_sample, clove_bass.output[sample]);

                garlic_master.saturate(master_sample, 0.34);
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

    for sample in 0 .. SAMPLES_TWICE {
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

    /*
    let reduce_by = if max_sample > -min_sample { max_sample } else { -min_sample };
    if reduce_by > 0. {
        for sample in 0 .. SAMPLES_TWICE {
            data[sample] = data[sample] / reduce_by;
        }
    }
    */
}
