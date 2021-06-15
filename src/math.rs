use core::mem::MaybeUninit;

pub const TAU: f32 = 3.14159265358979323846264338327950288 * 2.0;
const TWO_OVER_PI: f32 = 0.63661977236f32;

pub const EPSILON: f32 = 3.0e-4;

#[inline(always)]
pub fn sin(a: f32) -> f32 {

    let mut res: f32 = unsafe { MaybeUninit::<f32>::uninit().assume_init() }; // todo: see if this changes anything over mem::uninitialized()

    unsafe { llvm_asm!(
        r##"
            flds $1;
            fsin;
            fstps $0;
        "##
        : "=*m"(&mut res as *mut f32)
        : "*m"(&a as *const f32)
    ) };

    res
}

#[inline(always)]
pub fn cos(a: f32) -> f32 {

    let mut res: f32 = unsafe { MaybeUninit::<f32>::uninit().assume_init() };

    unsafe { llvm_asm!(
        r##"
            flds $1;
            fcos;
            fstps $0;
        "##
        : "=*m"(&mut res as *mut f32)
        : "*m"(&a as *const f32)
    ) };

    res
}

pub fn approx(a: f32, b: f32, prec: u8) -> bool {
    libm::fabsf(a-b) < libm::powf(10., -(prec as f32))
}

#[inline(always)]
pub fn approx4(a: f32, b: f32) -> bool {
    libm::fabsf(a-b) < 0.0001
}

#[inline]
pub fn linstep(a: f32, b: f32, x: f32) -> f32 {
    ((x-a)/(b-a)).clamp(0., 1.)
}

// try out: does core::intrinsics::likely change anything??
pub fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let x_clip = linstep(a, b, x);
    x_clip * x_clip * (3. - 2. * x_clip)
}

pub fn smootherstep(a: f32, b: f32, x: f32) -> f32 {
    let x_clip = linstep(a, b, x);
    x_clip * x_clip * x_clip * (x_clip * (x_clip * 6. - 15.) + 10.)
}

pub fn satanurate(s: f32) -> f32 {
    TWO_OVER_PI * libm::atanf(s)
}
