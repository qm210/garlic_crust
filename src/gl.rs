#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use winapi::um::wingdi::wglGetProcAddress;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::libloaderapi::GetProcAddress;
use core::mem;

pub type GLboolean = u8;
pub type GLchar = u8;
pub type GLfloat = f32;
pub type GLenum = u32;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLsizeiptr = isize;

pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const LINEAR: GLenum = 0x2601;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
pub const RGBA: GLenum = 0x1908;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const TEXTURE0: GLenum = 0x84C0;

static mut addr_glCreateShaderProgramv: usize = 0;
static mut addr_glUseProgram: usize = 0;
static mut addr_glRecti: usize = 0;
static mut addr_glGetUniformLocation: usize = 0;
static mut addr_glUniform1f: usize = 0;
static mut addr_glUniform1i: usize = 0;
static mut addr_glUniform2f: usize = 0;
static mut addr_glFlush: usize = 0;
static mut addr_glGenFramebuffers: usize = 0;
static mut addr_glBindFramebuffer: usize = 0;
static mut addr_glFramebufferTexture2D: usize = 0;
static mut addr_glGenTextures: usize = 0;
static mut addr_glBindTexture: usize = 0;
static mut addr_glTexParameteri: usize = 0;
static mut addr_glTexImage2D: usize = 0;
static mut addr_glDrawBuffer: usize = 0;
static mut addr_glActiveTexture: usize = 0;

// FUCKING real code here.
fn getTheFuckingAddress(name: &str) -> usize {
    let addr = unsafe { wglGetProcAddress(name.as_ptr() as *const i8) as usize };
    if addr != 0 {
        return addr;
    }
    unsafe {
        let handle = LoadLibraryA("Opengl32.dll\0".as_ptr() as *const i8);
        return GetProcAddress( handle, name.as_ptr() as *const i8 ) as usize;
    }
}

pub unsafe fn init() {
    addr_glCreateShaderProgramv = getTheFuckingAddress("glCreateShaderProgramv\0");
    addr_glUseProgram = getTheFuckingAddress("glUseProgram\0");
    addr_glRecti = getTheFuckingAddress("glRecti\0");
    addr_glGetUniformLocation = getTheFuckingAddress("glGetUniformLocation\0");
    addr_glUniform1f = getTheFuckingAddress("glUniform1f\0");
    addr_glUniform1i = getTheFuckingAddress("glUniform1i\0");
    addr_glUniform2f = getTheFuckingAddress("glUniform2f\0");
    addr_glFlush = getTheFuckingAddress("glFlush\0");
    addr_glBindFramebuffer = getTheFuckingAddress("glBindFramebuffer\0");
    addr_glFramebufferTexture2D = getTheFuckingAddress("glFramebufferTexture2D\0");
    addr_glGenFramebuffers = getTheFuckingAddress("glGenFramebuffers\0");
    addr_glGenTextures = getTheFuckingAddress("glGenTextures\0");
    addr_glBindTexture = getTheFuckingAddress("glBindTexture\0");
    addr_glTexParameteri = getTheFuckingAddress("glTexParameteri\0");
    addr_glTexImage2D = getTheFuckingAddress("glTexImage2D\0");
    addr_glDrawBuffer = getTheFuckingAddress("glDrawBuffer\0");
    addr_glActiveTexture = getTheFuckingAddress("glActiveTexture\0");
}

pub unsafe fn CreateShaderProgramv(shader_type: u32, count: u32, strings: &str) -> u32 {
    core::mem::transmute::<_, extern "system" fn(u32, u32, &str) -> u32>(addr_glCreateShaderProgramv)(shader_type, count, strings)
}

pub unsafe fn UseProgram(program: u32) -> bool {
    core::mem::transmute::<_, extern "system" fn(u32) -> bool>(addr_glUseProgram)(program)
}

pub unsafe fn Recti(x1: i32, y1: i32, x2: i32, y2: i32 ) -> () {
    core::mem::transmute::<_, extern "system" fn(i32, i32, i32, i32) -> ()>(addr_glRecti)(x1,y1,x2,y2)
}

pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(addr_glGetUniformLocation)(program, name)
}

pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(addr_glUniform1f)(location, v0)
}

pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(addr_glUniform1i)(location, v0)
}

pub unsafe fn Uniform2f(location: GLint, x: GLfloat, y: GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(addr_glUniform2f)(location, x, y)
}

pub unsafe fn Flush() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(addr_glFlush)()
}

pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () {
    mem::transmute::<_, extern "system" fn (GLenum, GLuint) -> ()>(addr_glBindFramebuffer)(target, framebuffer);
}

pub unsafe fn GenFramebuffers(n: GLsizei, ids: *mut GLuint) -> () {
    mem::transmute::<_, extern "system" fn (GLsizei, *mut GLuint) -> ()>(addr_glGenFramebuffers)(n, ids);
}

pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () {
    mem::transmute::<_, extern "system" fn (GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(addr_glFramebufferTexture2D)(target, attachment, textarget, texture, level);
}

pub unsafe fn GenTextures(n: GLenum, textures: *mut GLuint) -> () {
    mem::transmute::<_, extern "system" fn (GLenum, *mut GLuint) -> ()>(addr_glGenTextures)(n, textures);
}

pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () {
    mem::transmute::<_, extern "system" fn (GLenum, GLuint) -> ()>(addr_glBindTexture)(target, texture);
}

pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () {
    mem::transmute::<_, extern "system" fn (GLenum, GLenum, GLint) -> ()>(addr_glTexParameteri)(target, pname, param);
}

pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, target_type: GLenum, data: *mut winapi::ctypes::c_void) -> () {
    mem::transmute::<_, extern "system" fn (GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *mut winapi::ctypes::c_void) -> ()>(addr_glTexImage2D)(target, level, internalformat, width, height, border, format, target_type, data);
}

pub unsafe fn DrawBuffer(buf: GLenum) -> () {
    mem::transmute::<_, extern "system" fn (GLenum) -> ()>(addr_glDrawBuffer)(buf);
}
pub unsafe fn ActiveTexture(texture: GLenum) -> () {
    mem::transmute::<_, extern "system" fn (GLenum) -> ()>(addr_glActiveTexture)(texture);
}
