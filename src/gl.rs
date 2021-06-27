#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::libloaderapi::GetProcAddress;
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
static mut GL_API: [usize; 696] = [0; 696];

const RectiIdx: u16 = 136;
const GetUniformLocationIdx: u16 = 313;
const Uniform1fIdx: u16 = 539;

static LOAD_DESC: &'static [(u16, &'static str)] = &[

    ( wglSwapIntervalIdx, "wglSwapIntervalEXT\0" ),
//    (DrawArraysIdx, "glDrawArrays\0"),
    (RectiIdx, "glRecti\0"),


    // Program functions
    (CreateProgramIdx, "glCreateProgram\0"),
    #[cfg(feature = "logger")]
    (GetProgramivIdx, "glGetProgramiv\0"),
    (AttachShaderIdx, "glAttachShader\0"),
    #[cfg(feature = "logger")]
    (DetachShaderIdx, "glDetachShader\0"),

    (UseProgramIdx, "glUseProgram\0"),

    (LinkProgramIdx, "glLinkProgram\0"),
    (CreateShaderIdx, "glCreateShader\0"),
    (ShaderSourceIdx, "glShaderSource\0"),
    (CompileShaderIdx, "glCompileShader\0"),

    #[cfg(feature = "logger")]
    (GetShaderivIdx, "glGetShaderiv\0"),
    #[cfg(feature = "logger")]
    (GetShaderInfoLogIdx, "glGetShaderInfoLog\0"),
    #[cfg(feature = "logger")]
    (GetProgramInfoLogIdx, "glGetProgramInfoLog\0"),

    (GetUniformLocationIdx, "glGetUniformLocation\0"),
    (Uniform4fvIdx, "glUniform4fv\0"),

    // Texture
    (GenTexturesIdx, "glGenTextures\0"),
    (BindTextureIdx, "glBindTexture\0"),
    (ActiveTextureIdx, "glActiveTexture\0"),
    (TexImage2DIdx, "glTexImage2D\0"),
    (TexParameteriIdx, "glTexParameteri\0"),


    (ListBaseIdx, "glListBase\0"),
    (CallListsIdx, "glCallLists\0"),
    (RasterPos2fIdx, "glRasterPos2f\0"),

];

pub unsafe fn UseProgram(program: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint) -> ()>(*GL_API.get_unchecked(UseProgramIdx as usize))(program)
}

pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(*GL_API.get_unchecked(Uniform4fvIdx as usize))(location, count, value)
}

pub unsafe fn Recti(x1: GLint, y1: GLint, x2: GLint, y2: GLint ) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(*GL_API.get_unchecked(RectiIdx as usize))(x1,y1,x2,y2)
}

pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(*GL_API.get_unchecked(GetUniformLocationIdx as usize))(program, name)
}

pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(*GL_API.get_unchecked(Uniform1fIdx as usize))(location, v0)
}

pub fn init() {
    let handle : HMODULE;
    unsafe { handle = LoadLibraryA( "Opengl32.dll\0".as_ptr() as *const i8);  }
    for &(index, name) in LOAD_DESC {
        let mut prc = wglGetProcAddress(name.as_ptr() as *const i8) as usize;
        *GL_API.get_unchecked_mut( index as usize ) =  prc;
    }
}
