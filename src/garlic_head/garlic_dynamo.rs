use super::{DynamoArray, TimeFloat, SAMPLERATE};

pub struct Dynamo {
    pub times: DynamoArray,
    pub beats: DynamoArray,
    pub factors: DynamoArray,
    pub slopes: DynamoArray,
}

pub const TICK: f32 = 2. / SAMPLERATE;

impl Dynamo {

    pub fn beat(self: &Dynamo, sample: usize) -> TimeFloat {
        let time = (sample as TimeFloat) / SAMPLERATE;
        let mut cursor = 0;
        while cursor < 2 && self.times[cursor + 1] < time {
            cursor += 1;
        }

        if self.slopes[cursor] == 0. {
            self.beats[cursor] + (time - self.times[cursor]) * self.factors[cursor]
        } else {
            self.beats[cursor] + self.factors[cursor] * (libm::expf(self.slopes[cursor]*(time - self.times[cursor])) - 1.)
        }
    }

}