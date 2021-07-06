#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use winapi::um::wingdi::wglGetProcAddress;
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

static mut addr_glCreateShaderProgramv: usize = 0;
static mut addr_glUseProgram: usize = 0;
static mut addr_glRecti: usize = 0;
static mut addr_glGetUniformLocation: usize = 0;
static mut addr_glUniform1f: usize = 0;
static mut addr_glFlush: usize = 0;

pub unsafe fn init() {
    addr_glCreateShaderProgramv = wglGetProcAddress("glCreateShaderProgramv\0".as_ptr() as *const i8) as usize;
    addr_glUseProgram = wglGetProcAddress("glUseProgram\0".as_ptr() as *const i8) as usize;
    addr_glRecti = wglGetProcAddress("glRecti\0".as_ptr() as *const i8) as usize;
    addr_glGetUniformLocation = wglGetProcAddress("glGetUniformLocation\0".as_ptr() as *const i8) as usize;
    addr_glUniform1f = wglGetProcAddress("glUniform1f\0".as_ptr() as *const i8) as usize;
    addr_glFlush = wglGetProcAddress("glFlush\0".as_ptr() as *const i8) as usize;
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

pub unsafe fn Flush() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(addr_glFlush)()
}
