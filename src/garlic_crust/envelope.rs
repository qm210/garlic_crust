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
    pub note_vel: AmpFloat,
    pub seq_cursor: usize,
    pub min: Edge,
    pub max: Edge,
}

impl Operator for Envelope {
    fn handle_message(&mut self, message: &SeqMsg) {
        match &message {
            SeqMsg::NoteOn(_, note_vel) => {
                self.playhead = 0.;
                self.note_vel = *note_vel as AmpFloat / 128.;
            },
            _ => ()
        }
    }

    fn evaluate(&mut self, sample: usize, _: TimeFloat) -> AmpFloat {
        let attack = self.attack.evaluate(sample);
        let decay = self.decay.evaluate(sample);

        let norm_result = match self.shape {
            BaseEnv::ExpDecay => {
                //self.note_vel * // what to do with note_vel ? rather a note_vel_dependency?
                libm::exp2f(-(self.playhead - attack)/decay) *
                    crate::math::smoothstep(-1.0e-5, attack, self.playhead)
            }
        };
        let min = self.min.evaluate(sample);
        let max = self.max.evaluate(sample);

        min + (max - min) * norm_result.clamp(0., 1.)
    }

    fn advance(&mut self, _: usize) {
        self.playhead += 1. / SAMPLERATE;
    }

    fn get_cursor(&mut self) -> usize {
        self.seq_cursor
    }

    fn inc_cursor(&mut self) {
        self.seq_cursor += 1;
    }
}
