use crate::garlic_crust::ZERO_SAMPLE;
use super::garlic_crust::*;

// need to be programmatically appended
mod garlic_clove1;
mod garlic_clove2;
mod garlic_clove3;
mod garlic_master;
mod garlic_smash;

mod garlic_dynamo;

// TODO: track could be a byte array. if that saves us something?

// PUT GARLIC_EXTRACT HERE >>>>>>>>

pub const SECONDS: TimeFloat = 13.5;

const SEQUENCE_BASS: [SeqEvent; 37] = [
    SeqEvent {pos: 0, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 17640, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 35280, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 44100, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 52920, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 70560, message: SeqMsg::NoteOn(51, 100) },
    SeqEvent {pos: 88200, message: SeqMsg::NoteOn(51, 100) },
    SeqEvent {pos: 97020, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 114660, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 141120, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 158760, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 176400, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 185220, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 202860, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 211680, message: SeqMsg::NoteOn(48, 100) },
    SeqEvent {pos: 229320, message: SeqMsg::NoteOn(48, 100) },
    SeqEvent {pos: 255780, message: SeqMsg::NoteOn(51, 100) },
    SeqEvent {pos: 264600, message: SeqMsg::NoteOn(53, 100) },
    SeqEvent {pos: 282240, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 299880, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 317520, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 326340, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 335160, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 352800, message: SeqMsg::NoteOn(51, 100) },
    SeqEvent {pos: 370440, message: SeqMsg::NoteOn(51, 100) },
    SeqEvent {pos: 379260, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 396900, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 423360, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 441000, message: SeqMsg::NoteOn(55, 100) },
    SeqEvent {pos: 449820, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 467460, message: SeqMsg::NoteOn(57, 100) },
    SeqEvent {pos: 485100, message: SeqMsg::NoteOn(51, 100) },
    SeqEvent {pos: 493920, message: SeqMsg::NoteOn(53, 100) },
    SeqEvent {pos: 511560, message: SeqMsg::NoteOn(53, 100) },
    SeqEvent {pos: 529200, message: SeqMsg::NoteOn(60, 100) },
    SeqEvent {pos: 546840, message: SeqMsg::NoteOn(60, 100) },
    SeqEvent {pos: 564480, message: SeqMsg::NoteOff },
];

const SEQUENCE_0: [SeqEvent; 58] = [
    SeqEvent {pos: 8820, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 17640, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 22050, message: SeqMsg::NoteOff },
    SeqEvent {pos: 35280, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 52920, message: SeqMsg::NoteOff },
    SeqEvent {pos: 61740, message: SeqMsg::NoteOn(72, 100) },
    SeqEvent {pos: 79380, message: SeqMsg::NoteOn(75, 100) },
    SeqEvent {pos: 92610, message: SeqMsg::NoteOff },
    SeqEvent {pos: 105840, message: SeqMsg::NoteOn(75, 100) },
    SeqEvent {pos: 112455, message: SeqMsg::NoteOff },
    SeqEvent {pos: 123480, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 141120, message: SeqMsg::NoteOff },
    SeqEvent {pos: 149940, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 158760, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 163170, message: SeqMsg::NoteOff },
    SeqEvent {pos: 167580, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 171990, message: SeqMsg::NoteOff },
    SeqEvent {pos: 176400, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 185220, message: SeqMsg::NoteOn(69, 100) },
    SeqEvent {pos: 189630, message: SeqMsg::NoteOff },
    SeqEvent {pos: 194040, message: SeqMsg::NoteOn(69, 100) },
    SeqEvent {pos: 202860, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 207270, message: SeqMsg::NoteOff },
    SeqEvent {pos: 211680, message: SeqMsg::NoteOn(72, 100) },
    SeqEvent {pos: 229320, message: SeqMsg::NoteOn(74, 100) },
    SeqEvent {pos: 255780, message: SeqMsg::NoteOn(70, 100) },
    SeqEvent {pos: 282240, message: SeqMsg::NoteOff },
    SeqEvent {pos: 291060, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 299880, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 304290, message: SeqMsg::NoteOff },
    SeqEvent {pos: 317520, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 335160, message: SeqMsg::NoteOff },
    SeqEvent {pos: 343980, message: SeqMsg::NoteOn(72, 100) },
    SeqEvent {pos: 361620, message: SeqMsg::NoteOn(75, 100) },
    SeqEvent {pos: 374850, message: SeqMsg::NoteOff },
    SeqEvent {pos: 388080, message: SeqMsg::NoteOn(70, 100) },
    SeqEvent {pos: 394695, message: SeqMsg::NoteOff },
    SeqEvent {pos: 405720, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 423360, message: SeqMsg::NoteOff },
    SeqEvent {pos: 432180, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 441000, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 445410, message: SeqMsg::NoteOff },
    SeqEvent {pos: 449820, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 454230, message: SeqMsg::NoteOff },
    SeqEvent {pos: 458640, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 467460, message: SeqMsg::NoteOn(69, 100) },
    SeqEvent {pos: 471870, message: SeqMsg::NoteOff },
    SeqEvent {pos: 476280, message: SeqMsg::NoteOn(69, 100) },
    SeqEvent {pos: 485100, message: SeqMsg::NoteOn(67, 100) },
    SeqEvent {pos: 489510, message: SeqMsg::NoteOff },
    SeqEvent {pos: 493920, message: SeqMsg::NoteOn(69, 100) },
    SeqEvent {pos: 504945, message: SeqMsg::NoteOff },
    SeqEvent {pos: 507150, message: SeqMsg::NoteOn(69, 100) },
    SeqEvent {pos: 511560, message: SeqMsg::NoteOff },
    SeqEvent {pos: 520380, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 529200, message: SeqMsg::NoteOff },
    SeqEvent {pos: 538020, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 564480, message: SeqMsg::NoteOff },
];

const SEQUENCE_1: [SeqEvent; 65] = [
    SeqEvent {pos: 8820, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 17640, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 22050, message: SeqMsg::NoteOff },
    SeqEvent {pos: 35280, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 52920, message: SeqMsg::NoteOff },
    SeqEvent {pos: 61740, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 79380, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 92610, message: SeqMsg::NoteOff },
    SeqEvent {pos: 105840, message: SeqMsg::NoteOn(82, 100) },
    SeqEvent {pos: 112455, message: SeqMsg::NoteOff },
    SeqEvent {pos: 123480, message: SeqMsg::NoteOn(82, 100) },
    SeqEvent {pos: 141120, message: SeqMsg::NoteOff },
    SeqEvent {pos: 149940, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 158760, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 163170, message: SeqMsg::NoteOff },
    SeqEvent {pos: 167580, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 171990, message: SeqMsg::NoteOff },
    SeqEvent {pos: 176400, message: SeqMsg::NoteOn(84, 100) },
    SeqEvent {pos: 185220, message: SeqMsg::NoteOn(82, 100) },
    SeqEvent {pos: 189630, message: SeqMsg::NoteOff },
    SeqEvent {pos: 194040, message: SeqMsg::NoteOn(75, 100) },
    SeqEvent {pos: 202860, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 207270, message: SeqMsg::NoteOff },
    SeqEvent {pos: 211680, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 220500, message: SeqMsg::NoteOn(82, 100) },
    SeqEvent {pos: 229320, message: SeqMsg::NoteOn(87, 100) },
    SeqEvent {pos: 238140, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 246960, message: SeqMsg::NoteOn(81, 100) },
    SeqEvent {pos: 255780, message: SeqMsg::NoteOn(84, 100) },
    SeqEvent {pos: 262395, message: SeqMsg::NoteOff },
    SeqEvent {pos: 264600, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 271215, message: SeqMsg::NoteOff },
    SeqEvent {pos: 273420, message: SeqMsg::NoteOn(81, 100) },
    SeqEvent {pos: 282240, message: SeqMsg::NoteOff },
    SeqEvent {pos: 291060, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 299880, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 304290, message: SeqMsg::NoteOff },
    SeqEvent {pos: 317520, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 335160, message: SeqMsg::NoteOff },
    SeqEvent {pos: 343980, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 361620, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 374850, message: SeqMsg::NoteOff },
    SeqEvent {pos: 388080, message: SeqMsg::NoteOn(74, 100) },
    SeqEvent {pos: 394695, message: SeqMsg::NoteOff },
    SeqEvent {pos: 405720, message: SeqMsg::NoteOn(72, 100) },
    SeqEvent {pos: 423360, message: SeqMsg::NoteOff },
    SeqEvent {pos: 432180, message: SeqMsg::NoteOn(76, 100) },
    SeqEvent {pos: 441000, message: SeqMsg::NoteOn(77, 100) },
    SeqEvent {pos: 445410, message: SeqMsg::NoteOff },
    SeqEvent {pos: 449820, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 454230, message: SeqMsg::NoteOff },
    SeqEvent {pos: 458640, message: SeqMsg::NoteOn(84, 100) },
    SeqEvent {pos: 467460, message: SeqMsg::NoteOn(82, 100) },
    SeqEvent {pos: 471870, message: SeqMsg::NoteOff },
    SeqEvent {pos: 476280, message: SeqMsg::NoteOn(75, 100) },
    SeqEvent {pos: 485100, message: SeqMsg::NoteOn(75, 100) },
    SeqEvent {pos: 489510, message: SeqMsg::NoteOff },
    SeqEvent {pos: 493920, message: SeqMsg::NoteOn(72, 100) },
    SeqEvent {pos: 504945, message: SeqMsg::NoteOff },
    SeqEvent {pos: 507150, message: SeqMsg::NoteOn(72, 100) },
    SeqEvent {pos: 511560, message: SeqMsg::NoteOff },
    SeqEvent {pos: 520380, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 529200, message: SeqMsg::NoteOff },
    SeqEvent {pos: 538020, message: SeqMsg::NoteOn(79, 100) },
    SeqEvent {pos: 564480, message: SeqMsg::NoteOff },
];

const DYNAMO_BREAKPOINTS: usize = 1;
pub type DynamoArray = [TimeFloat; DYNAMO_BREAKPOINTS];

pub const DYNAMO: garlic_dynamo::Dynamo = garlic_dynamo::Dynamo::create(150.); // original for this midi was 160.13

// <<<<<<<< PUT GARLIC_EXTRACT HERE

pub const BLOCK_SIZE: usize = 100; // larger blocks might result in STATUS_STACK_OVERFLOW
const MASTER_BLOCK_FACTOR: usize = 17; // my stolen freeverb needs BLOCK_SIZE * MASTER_BLOCK_FACTOR >= 1640
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
    let mut clove2_state0 = garlic_clove2::create_state();
    let mut clove3_state0 = garlic_clove3::create_state();
    let mut clove3_state1 = garlic_clove3::create_state();

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
            garlic_clove2::process(&SEQUENCE_BASS, block_offset, &mut clove2_state0);
            garlic_clove3::process(&SEQUENCE_0, block_offset, &mut clove3_state0);
            garlic_clove3::process(&SEQUENCE_1, block_offset, &mut clove3_state1);

            for sample in 0 .. BLOCK_SIZE {
                let master_sample = sample + master_piece * BLOCK_SIZE;

                // could merge the "put ZERO_SAMPLE" and first "add" to one "put", but we gönn ourselves for more symmetry.
                garlic_master.put_at(master_sample, ZERO_SAMPLE);

                garlic_master.add_at(master_sample, smash_state0.output[sample]);
                garlic_master.add_at(master_sample, clove2_state0.output[sample]);
                garlic_master.add_at(master_sample, clove3_state0.output[sample]);
                garlic_master.add_at(master_sample, clove3_state1.output[sample]);

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
