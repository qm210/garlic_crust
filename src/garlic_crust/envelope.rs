use super::*;
use crate::math::EPSILON;

// I tried to have struct enums, but THIS WAS HORRIBLY SLOW
#[derive(Debug, Clone, Copy)]
pub enum BaseEnv {
    ExpDecay,
    Swell,
}

#[derive(Clone, Copy)]
pub enum EnvShape{
    Common {
        base: BaseEnv,
        decay: Edge,
        sustain: Edge,
        attack: Edge,
    },
    Const (MonoSample),
    Sinc {
        period: Edge,
        gain: Edge,
        suppression: Edge,
    },
    Generic {
        func: MonoFunc
    }
}

pub struct Envelope {
    pub shape: EnvShape,
    pub playhead: TimeFloat,
    pub note_vel: MonoSample,
    pub note_vel_mod: MonoSample,
    pub seq_cursor: usize,
}

impl Operator for Envelope {
    fn handle_message(&mut self, message: &SeqMsg) {
        match &message {
            SeqMsg::NoteOn(_, note_vel) => {
                self.playhead = 0.;
                self.note_vel = self.note_vel_mod * (*note_vel as MonoSample / 127.);
            },
            SeqMsg::Loop | SeqMsg::Init => {
                self.playhead = 0.;
            },
            _ => ()
        }
    }

    fn evaluate(&mut self, sample: usize) -> Sample {
        let mut result = ZERO_SAMPLE;
        for ch in 0 .. 2 {
            result[ch] = match &self.shape {
                EnvShape::Common { base, attack, decay, sustain } => {
                    let _attack = attack.evaluate_mono(sample, ch);
                    let _decay = decay.evaluate_mono(sample, ch);
                    let _sustain = sustain.evaluate_mono(sample, ch);
                    let norm_result = match base {
                        BaseEnv::ExpDecay => {
                            (_sustain + (1. - _sustain) * libm::exp2f(-(self.playhead - _attack)/_decay))
                            * crate::math::smoothstep(0., _attack + EPSILON, self.playhead)
                        },
                        BaseEnv::Swell => {
                            libm::powf(crate::math::smoothstep(-_attack, _attack, self.playhead), _sustain) // _sustain is rather a power
                        },
                    };

                    norm_result.clamp(0., 1.)
                },
                EnvShape::Const(value) => {
                    *value
                },
                EnvShape::Sinc { period, gain, suppression } => {
                    let _period = period.evaluate_mono(sample, ch);
                    let _gain = gain.evaluate_mono(sample, ch);
                    let _suppression = suppression.evaluate_mono(sample, ch);
                    let x = crate::math::PI * self.playhead / _period;
                    if libm::fabsf(x) < 0.001 {
                        _gain
                    } else {
                        _gain * crate::math::sin(x) / libm::powf(x, _suppression)
                    }
                },
                EnvShape::Generic { func } => {
                    func(self.playhead)
                },
            };
        }
        result
    }

    fn advance(&mut self, _: usize) {
        self.playhead += INV_SAMPLERATE;
    }

    fn get_cursor(&mut self) -> usize {
        self.seq_cursor
    }

    fn inc_cursor(&mut self) {
        self.seq_cursor += 1;
    }
}

impl Default for Envelope {
    fn default() -> Envelope {
        Envelope {
            shape: EnvShape::Const(0.),
            playhead: 0.,
            note_vel: 0.,
            note_vel_mod: 0.,
            seq_cursor: 0,
        }
    }
}
