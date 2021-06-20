#[inline]
pub fn slope(t: f32, t0: f32, t1: f32, y0: f32, y1: f32) -> f32 {
    y0 + (t - t0) / (t1 - t0) * (y1 - y0)
}

pub fn powerslope(t: f32, t0: f32, t1: f32, y0: f32, y1: f32, power: f32) -> f32 {
    y0 + libm::powf((t - t0) / (t1 - t0), power) * (y1 - y0)
}

#[inline]
pub fn logslope(t: f32, t0: f32, t1: f32, y0: f32, y1: f32) -> f32 {
    let f = 1./(1. + (libm::logf(t1/t)) / (libm::logf(t/t0)));
    libm::powf(y1, f) * libm::powf(y0, 1.-f)
}

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