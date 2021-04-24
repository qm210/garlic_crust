use core::mem::MaybeUninit;

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
