use crate::garlic_crust::*;
use crate::garlic_head::*;

// inline or not inline?
#[inline]
// individual math operators (more complex than Edge::mad()) might be created directly in the clove
pub fn math_mixer(input1: &Edge, input2: &Edge, cv: &Edge, output: &mut Edge) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 { // the looping could be hidden by generalizing 2000 + and * 1800 to
            output.put_at_mono(sample, ch, cv.evaluate_mono(sample, ch) * (input1.evaluate_mono(sample, ch) + input2.evaluate_mono(sample, ch)));
        }
    }
}

pub fn math_mixer_const(input1: &Edge, input2: &Edge, factor: MonoSample, output: &mut Edge) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 { // the looping could be hidden by generalizing 2000 + and * 1800 to
            output.put_at_mono(sample, ch, factor * (input1.evaluate_mono(sample, ch) + input2.evaluate_mono(sample, ch)));
        }
    }
}

#[inline]
pub fn waveshape(output: &mut Edge, waveshape: fn(MonoSample) -> MonoSample, amount: f32) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 {
            let _input = output.evaluate_mono(sample, ch);
            let _output = libm::copysignf(waveshape(libm::fabsf(_input)), _input);
            output.put_at_mono(sample, ch, amount * _output + (1.-amount) * _input);
        }
    }
}

#[inline]
pub fn waveshape_quad(output: &mut Edge, quad_shape: &QuadWaveShape) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 {
            let _input = output.evaluate_mono(sample, ch);
            output.put_at_mono(sample, ch, libm::copysignf(quad_shape.evaluate(libm::fabsf(_input)), _input));
        }
    }
}

/*
#[inline]
fn math_distort(output: &mut Edge) {
    waveshape(output, |x| if x >= 0.1 && x < 0.13 { 0.5 } else { x }, 0.3);
}
*/

#[inline]
pub fn math_overdrive(output: &mut Edge, amount: &Edge) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 {
            output.put_at_mono(sample, ch, crate::math::satanurate(amount.evaluate_mono(sample, ch) * output.evaluate_mono(sample, ch)));
        }
    }
}

pub fn math_overdrive_const(output: &mut Edge, gain: MonoSample) {
    for sample in 0 .. BLOCK_SIZE {
        for ch in 0 .. 2 {
            let input = output.evaluate_mono(sample, ch);
            output.put_at_mono(sample, ch, crate::math::satanurate(gain * input));
        }
    }
}

// TODO: check difference on filesize, when QuadWaveShape or WaveShape is its own Operator

/*
 * QuadWaveShape is based on a waveshape ([0,1] -> [0,1]) function with three quadratic splines
 */
pub struct QuadWaveShape {
    slope_0: f32,
    x_1: f32,
    y_1: f32,
    slope_1: f32,
    x_2: f32,
    y_2: f32,
    slope_2: f32,

    quadcoeff_0: f32,
    quadcoeff_1: f32,
    quadcoeff_2: f32,
}

impl QuadWaveShape {
    pub const fn create(slope_0: f32, x_1: f32, y_1: f32, slope_1: f32, x_2: f32, y_2: f32, slope_2: f32) -> Self {
        let quadcoeff_0 = (y_1 - slope_0 * x_1) / (x_1 * x_1);
        let delta_x_1 = x_2 - x_1;
        let quadcoeff_1 = (y_2 - y_1 - slope_1 * delta_x_1) / (delta_x_1 * delta_x_1);
        let delta_x_2 = 1. - x_2;
        let quadcoeff_2 = (1. - y_2 - slope_2 * delta_x_2) / (delta_x_2 * delta_x_2);
        Self {
            slope_0,
            slope_1,
            slope_2,
            quadcoeff_0,
            quadcoeff_1,
            quadcoeff_2,
            x_1,
            x_2,
            y_1,
            y_2,
        }
    }

    pub fn evaluate(&self, x: f32) -> f32 {
        match x {
            x if x <= 0. => {
                0.
            }
            x if x < self.x_1 => {
                self.slope_0 * x + self.quadcoeff_0 * x * x
            },
            x if x < self.x_2 => {
                let delta_x = x - self.x_1;
                self.y_1 + self.slope_1 * delta_x + self.quadcoeff_1 * delta_x * delta_x
            },
            x if x <= 1. => {
                let delta_x = x - self.x_2;
                self.y_2 + self.slope_2 * delta_x + self.quadcoeff_2 * delta_x * delta_x
            }
            _ => {
                1.
            }
        }
    }
}
