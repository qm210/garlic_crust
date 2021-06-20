extern crate std;

use super::garlic_head::StereoTrack;
use super::SAMPLERATE_INT;

// we need this for the missing memcmp linker error
#[link(name="vcruntime")]
extern {}

pub unsafe fn write_wave_file(interlaced_stereo_data: &StereoTrack) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLERATE_INT,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float
    };
    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();
    for sample in 0 .. interlaced_stereo_data.len() {
        writer.write_sample(interlaced_stereo_data[sample]).unwrap();
    }
    writer.finalize().unwrap();

    std::println!("[DEBUG] output.wav written!");
}
