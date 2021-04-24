
static mut sounds : [[f32;44100*9];7] = [[0.0;44100*9];7];
// const FREQ_F4: f32 = 349.;
// const FREQ_AB4: f32 = 415.;
const FREQ_C5: f32 = 523.;
// const FREQ_Db5: f32 = 554.;
// const FREQ_EB5: f32 = 622.;
// const FREQ_F5: f32 = 698.;
// const FREQ_AB5: f32 = 831.;

fn play( dst: &mut [f32;44100*120], dst_offset : usize, signal : &[f32;44100*9], sample_duration : f32 ) {
    let mut dst_pos = 0;
    let mut position : f32 = 0.0;
    unsafe{
        loop{
            let src_val = signal.get_unchecked(dst_pos);
            let in_pos = position/4.5-2f32;
            let val = (in_pos*in_pos)*position/4.5;
            *dst.get_unchecked_mut( dst_pos + dst_offset) += src_val*val;

            position += sample_duration;
            dst_pos += 1;
            if dst_pos == 44100*9 {
                return;
            }
        }
    }
}

pub fn make_music( music: &mut [f32;44100*120]) {
    super::log!( "Make instruments!");

    let mut i = 0;
    let mut scale = 1.0;

    unsafe{
        loop{
            let mut d = 0;
            loop{
                let frequency : f32 = FREQ_C5/scale;
                let mut position : f32 = 0.0;
                let mut sample_no = 0;
                loop {
                    let sample_duration : f32 = frequency / 44100.0f32;
                    position = position + sample_duration;
                    if position > 0.5 {
                        position -= 1.0f32;
                    }
                    let val = core::intrinsics::fabsf32(position)*4f32-1.0f32;
                    *sounds.get_unchecked_mut(i).get_unchecked_mut(sample_no) += val/55.0f32;
                    sample_no += 1;
                    if sample_no == 44100*9 {
                        break;
                    }
                }
                d += 1;
                if d == 11 {
                    break;
                }
            }
            scale *= 2.0f32;
            if scale >= 32.0 {
                break;
            }
        }
    }

    unsafe{
        let mut dst : usize = 0;
        let mut s = 0;
        loop {
            let mut i = 0;
            let nt = 1.;
            if nt > 0.9 {
                play( music, dst, &sounds[i], 1.0 / 44100.0 );
            }
            dst += 44100;
            s += 1;
            if s == 110 {
                break;
            }
        }
    }
}
