use crate::garlic_crust::ZERO_SAMPLE;
use super::garlic_crust::*;

// need to be programmatically appended
mod garlic_clove1;
mod garlic_clove2;
mod garlic_clove3;
mod garlic_clove4;
mod garlic_master;
mod garlic_smash;

mod garlic_dynamo;

// TODO: track could be a byte array. if that saves us something?

// PUT GARLIC_EXTRACT HERE >>>>>>>>
pub const SECONDS: TimeFloat = 13.064;

const SEQUENCE_0: [SeqEvent; 88] = [
    SeqEvent {pos: 0, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 8139, message: SeqMsg::NoteOff },
    SeqEvent {pos: 8879, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 17018, message: SeqMsg::NoteOff },
    SeqEvent {pos: 17758, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 25897, message: SeqMsg::NoteOff },
    SeqEvent {pos: 26637, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 34776, message: SeqMsg::NoteOff },
    SeqEvent {pos: 53275, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 61414, message: SeqMsg::NoteOff },
    SeqEvent {pos: 62154, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 70293, message: SeqMsg::NoteOff },
    SeqEvent {pos: 71033, message: SeqMsg::NoteOn(64, 100) },
    SeqEvent {pos: 79172, message: SeqMsg::NoteOff },
    SeqEvent {pos: 79912, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 88052, message: SeqMsg::NoteOff },
    SeqEvent {pos: 97671, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 105810, message: SeqMsg::NoteOff },
    SeqEvent {pos: 106550, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 114689, message: SeqMsg::NoteOff },
    SeqEvent {pos: 115429, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 123568, message: SeqMsg::NoteOff },
    SeqEvent {pos: 124308, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 132448, message: SeqMsg::NoteOff },
    SeqEvent {pos: 142067, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 150206, message: SeqMsg::NoteOff },
    SeqEvent {pos: 150946, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 159085, message: SeqMsg::NoteOff },
    SeqEvent {pos: 159825, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 167964, message: SeqMsg::NoteOff },
    SeqEvent {pos: 168704, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 176844, message: SeqMsg::NoteOff },
    SeqEvent {pos: 195342, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 203481, message: SeqMsg::NoteOff },
    SeqEvent {pos: 204221, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 212360, message: SeqMsg::NoteOff },
    SeqEvent {pos: 213100, message: SeqMsg::NoteOn(64, 100) },
    SeqEvent {pos: 221240, message: SeqMsg::NoteOff },
    SeqEvent {pos: 221980, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 230119, message: SeqMsg::NoteOff },
    SeqEvent {pos: 239738, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 256757, message: SeqMsg::NoteOff },
    SeqEvent {pos: 257496, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 283394, message: SeqMsg::NoteOff },
    SeqEvent {pos: 284134, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 292273, message: SeqMsg::NoteOff },
    SeqEvent {pos: 293013, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 301153, message: SeqMsg::NoteOff },
    SeqEvent {pos: 301892, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 310032, message: SeqMsg::NoteOff },
    SeqEvent {pos: 310772, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 318911, message: SeqMsg::NoteOff },
    SeqEvent {pos: 337409, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 345549, message: SeqMsg::NoteOff },
    SeqEvent {pos: 346288, message: SeqMsg::NoteOn(58, 100) },
    SeqEvent {pos: 354428, message: SeqMsg::NoteOff },
    SeqEvent {pos: 355168, message: SeqMsg::NoteOn(64, 100) },
    SeqEvent {pos: 363307, message: SeqMsg::NoteOff },
    SeqEvent {pos: 364047, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 372186, message: SeqMsg::NoteOff },
    SeqEvent {pos: 381805, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 389945, message: SeqMsg::NoteOff },
    SeqEvent {pos: 390684, message: SeqMsg::NoteOn(59, 100) },
    SeqEvent {pos: 398824, message: SeqMsg::NoteOff },
    SeqEvent {pos: 399564, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 407703, message: SeqMsg::NoteOff },
    SeqEvent {pos: 408443, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 416582, message: SeqMsg::NoteOff },
    SeqEvent {pos: 417322, message: SeqMsg::NoteOn(59, 100) },
    SeqEvent {pos: 425461, message: SeqMsg::NoteOff },
    SeqEvent {pos: 426201, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 434341, message: SeqMsg::NoteOff },
    SeqEvent {pos: 435081, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 443220, message: SeqMsg::NoteOff },
    SeqEvent {pos: 443960, message: SeqMsg::NoteOn(56, 100) },
    SeqEvent {pos: 452099, message: SeqMsg::NoteOff },
    SeqEvent {pos: 452839, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 460978, message: SeqMsg::NoteOff },
    SeqEvent {pos: 472077, message: SeqMsg::NoteOn(49, 100) },
    SeqEvent {pos: 487616, message: SeqMsg::NoteOff },
    SeqEvent {pos: 488356, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 496495, message: SeqMsg::NoteOff },
    SeqEvent {pos: 497235, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 505374, message: SeqMsg::NoteOff },
    SeqEvent {pos: 506114, message: SeqMsg::NoteOn(52, 100) },
    SeqEvent {pos: 514253, message: SeqMsg::NoteOff },
    SeqEvent {pos: 523873, message: SeqMsg::NoteOn(54, 100) },
    SeqEvent {pos: 532012, message: SeqMsg::NoteOff },
];

const DYNAMO_BREAKPOINTS: usize = 1;
pub type DynamoArray = [TimeFloat; DYNAMO_BREAKPOINTS];

pub const DYNAMO: garlic_dynamo::Dynamo = garlic_dynamo::Dynamo::create(150.); // original for this midi was 160.13

// <<<<<<<< PUT GARLIC_EXTRACT HERE

pub const BLOCK_SIZE: usize = 100; // larger blocks might result in STATUS_STACK_OVERFLOW
const MASTER_BLOCK_FACTOR: usize = 17; // my stolen freeverb needs BLOCK_SIZE * MASTER_BLOCK_FACTOR >= 1640
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

    //let mut smash_state0 = garlic_smash::create_state(); // this gonne be my kick
    //let mut clove2_state0 = garlic_clove2::create_state();
    //let mut clove3_state0 = garlic_clove3::create_state();
    //let mut clove3_state1 = garlic_clove3::create_state();
    let mut clove4_state0 = garlic_clove4::create_state();

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

            /*
            garlic_smash::process(block_offset, &mut smash_state0);
            garlic_clove2::process(&SEQUENCE_BASS, block_offset, &mut clove2_state0);
            garlic_clove3::process(&SEQUENCE_0, block_offset, &mut clove3_state0);
            garlic_clove3::process(&SEQUENCE_1, block_offset, &mut clove3_state1);
            */
            garlic_clove4::process(&SEQUENCE_0, block_offset, &mut clove4_state0);

            for sample in 0 .. BLOCK_SIZE {
                let master_sample = sample + master_piece * BLOCK_SIZE;

                // could merge the "put ZERO_SAMPLE" and first "add" to one "put", but we gönn ourselves for more symmetry.
                garlic_master.put_at(master_sample, ZERO_SAMPLE);

                /*
                garlic_master.add_at(master_sample, smash_state0.output[sample]);
                garlic_master.add_at(master_sample, clove2_state0.output[sample]);
                garlic_master.add_at(master_sample, clove3_state0.output[sample]);
                garlic_master.add_at(master_sample, clove3_state1.output[sample]);
                */
                garlic_master.add_at(master_sample, clove4_state0.output[sample]);

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
