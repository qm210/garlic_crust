use super::*;

#[derive(Debug)]
pub enum FilterType {
    LowPass,
}

pub struct Filter {
    pub shape: FilterType,
    pub cutoff: Edge,
    pub input: BlockArray, // TODO: this could be Edge too??
    pub output: Edge,
}

impl Operator for Filter {
    fn handle_event(&mut self, event: &SeqEvent) {
    }

    fn evaluate(&mut self, sample: usize, total_time: TimeFloat) -> AmpFloat {
        let cutoff = self.cutoff.evaluate(sample);

        // only FilterType::LowPass implemented right now, and cutoff doesn't do anything
        let mut result: AmpFloat = 0.;
        for k in 0..24 {
            if sample + k >= BLOCK_SIZE {
                break;
            }
            result += 1./24. * self.input[sample + k];
        }
        result
    }

    fn advance(&mut self) {
    }

    fn get_cursor(&mut self) -> usize {
        0
    }

    fn inc_cursor(&mut self) {
    }
}

impl Filter {
    pub fn apply(&mut self, input: &BlockArray, ) {
        //self.input = input.clone();
    }
}