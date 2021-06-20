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
        let mut data = interlaced_stereo_data[sample];
        /*
        // this is my makeshift function plotter :D
        if sample <= 2 * SAMPLERATE_INT as usize {
            //data = plot_something(sample);
        }
        */
        writer.write_sample(data).unwrap();
    }
    writer.finalize().unwrap();

    std::println!("[DEBUG] output.wav written!");
}

fn plot_something(sample: usize) -> f32 {
    let t = 2. * sample as f32 / super::garlic_crust::SAMPLERATE;
    let quad_shape = crate::math_interpol::QuadWaveShape::create(0., 0.1, 0.4, 0., 0.2, 0.15, 0.7);
    quad_shape.evaluate(t)
}
