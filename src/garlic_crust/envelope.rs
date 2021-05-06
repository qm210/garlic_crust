use super::*;

#[derive(Debug)]
pub enum BaseEnv {
    ExpDecay,
}

pub struct Envelope {
    pub shape: BaseEnv,
    pub attack: Edge,
    pub decay: Edge,
    pub sustain: Edge,
    pub playhead: TimeFloat,
    pub seq_cursor: usize,
}

impl Operator for Envelope {
    fn handle_event(&mut self, event: &SeqEvent) {
        match &event.message {
            SeqMsg::NoteOn => {
                self.playhead = 0.;
            },
            _ => ()
        }
    }

    fn evaluate(&mut self, sample: usize, total_time: TimeFloat) -> AmpFloat {
        let attack = self.attack.evaluate(sample);
        let decay = self.decay.evaluate(sample);

        let result = match self.shape {
            BaseEnv::ExpDecay => {
                libm::exp2f(-(self.playhead-attack)/decay) * crate::math::smoothstep(0., attack, self.playhead)
            }
        };

        result.clamp(0., 1.)
    }

    fn advance(&mut self) {
        self.playhead += 1. / SAMPLERATE;
    }

    fn get_cursor(&mut self) -> usize {
        self.seq_cursor
    }

    fn inc_cursor(&mut self) {
        self.seq_cursor += 1;
    }
}
