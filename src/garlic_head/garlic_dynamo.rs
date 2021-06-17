use super::{DynamoArray, TimeFloat, as_time};

pub struct Dynamo {
    pub times: DynamoArray,
    pub beats: DynamoArray,
    pub factors: DynamoArray,
    pub slopes: DynamoArray,
}

impl Dynamo {

    pub fn beat(self: &Dynamo, sample: usize) -> TimeFloat {
        let time = as_time(sample);
        let mut cursor = 0;
        while cursor < (super::DYNAMO_BREAKPOINTS - 1) && self.times[cursor + 1] < time {
            cursor += 1;
        }

        if self.slopes[cursor] == 0. { // TODO: evaluate whether that == 0. is actually met.
            self.beats[cursor] + (time - self.times[cursor]) * self.factors[cursor]
        } else {
            self.beats[cursor] + self.factors[cursor] * (libm::expf(self.slopes[cursor] * (time - self.times[cursor])) - 1.)
        }
    }

}