#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![windows_subsystem = "console"]
#![feature(core_intrinsics)]
#![feature(static_nobundle)]
#![feature(const_fn_floating_point_arithmetic)]
#![allow(dead_code, non_snake_case, unused_imports)]

#[cfg(windows)] extern crate winapi;

pub mod util;
pub mod math;
mod garlic_crust;
mod garlic_head;
mod garlic_helper;

// debug profile uses std library (e.g. for .wav file writing).
// this should better be a "feature" in cargo, but for now, its not.
#[cfg(debug_assertions)]
mod debug;

// TODO (NR4): Remove the unused uses.
use core::panic::PanicInfo;

use winapi::um::wingdi::{
    ChoosePixelFormat,
    SwapBuffers,
    wglMakeCurrent,
    wglCreateContext,
    SetPixelFormat,

    PFD_TYPE_RGBA,
    PFD_DOUBLEBUFFER,
    PFD_SUPPORT_OPENGL,
    PFD_DRAW_TO_WINDOW,
    PIXELFORMATDESCRIPTOR,

    PFD_MAIN_PLANE,

    DEVMODEA,
    //wglGetProcAddress,
};

use winapi::shared::minwindef::{
    LRESULT,
    LPARAM,
    LPVOID,
    WPARAM,
    UINT,
    HINSTANCE,
};

use winapi::shared::windef::{
    HDC,
    HGLRC,
    HWND,
    HMENU,
};

use winapi::um::libloaderapi::GetModuleHandleA;

use winapi::um::winuser::{
    CreateWindowExA,
    DefWindowProcA,
    GetDC,
    PostQuitMessage,
    RegisterClassA,

    WNDCLASSA,

    WS_POPUP,
    WS_VISIBLE,
    WS_MAXIMIZE,
    CW_USEDEFAULT,
    CDS_FULLSCREEN,

    ShowCursor,
};

pub unsafe extern "system" fn window_proc(hwnd: HWND,
    msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

    match msg {
        winapi::um::winuser::WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => { return DefWindowProcA(hwnd, msg, w_param, l_param); }
    }
    return 0;
}

#[cfg(feature = "logger")]
fn show_error( message : *const i8 ) {
    unsafe {
        MessageBoxA(0 as HWND, message, "Window::create\0".as_ptr() as *const i8, MB_ICONERROR);
    }
}

// import printf() from C
#[cfg_attr(all(windows, target_env="msvc"),
    link(name="legacy_stdio_definitions", kind="static-nobundle"),
    link(name="msvcrt", kind="static-nobundle"),
    link(name="ucrt", kind="static-nobundle"),
    link(name="user32", kind="static-nobundle")
)]
extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

fn create_window( ) -> ( HWND, HDC ) {
    unsafe {
        let mut devmode : DEVMODEA = core::mem::zeroed();
        devmode.dmSize = core::mem::size_of::<DEVMODEA>() as u16;
        devmode.dmFields = winapi::um::wingdi::DM_PELSWIDTH | winapi::um::wingdi::DM_PELSHEIGHT;
        devmode.dmPelsWidth  = 1920;
        devmode.dmPelsHeight = 1080;
        winapi::um::winuser::ChangeDisplaySettingsA(&mut devmode, CDS_FULLSCREEN);

        let hwnd : HWND = CreateWindowExA(0, 0xc019 as *const i8, 0 as *const i8, WS_POPUP | WS_VISIBLE | WS_MAXIMIZE, 0, 0, 0, 0, 0 as HWND, 0 as HMENU, 0 as HINSTANCE, 0 as LPVOID);
        ShowCursor(0);
        let hdc : HDC = GetDC(hwnd);

        let mut pfd : PIXELFORMATDESCRIPTOR = core::mem::zeroed();
        pfd.nSize = core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 32;
        pfd.cAlphaBits = 8;
        pfd.cDepthBits = 32;
        pfd.iLayerType = PFD_MAIN_PLANE;

        SetPixelFormat(hdc, ChoosePixelFormat(hdc, &pfd), &pfd);
        wglMakeCurrent(hdc, wglCreateContext(hdc));

        ( hwnd, hdc )
    }
}

#[cfg(not(debug_assertions))]
#[panic_handler]
#[no_mangle]
pub extern fn panic( _info: &PanicInfo ) -> ! { loop {} }

#[no_mangle]
pub unsafe extern fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = c as u8;
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
        i += 1;
    }
    dest
}

pub const SAMPLERATE_INT: u32 = garlic_crust::SAMPLERATE as u32;

static WAVE_FORMAT : winapi::shared::mmreg::WAVEFORMATEX = winapi::shared::mmreg::WAVEFORMATEX{
    wFormatTag : winapi::shared::mmreg::WAVE_FORMAT_IEEE_FLOAT, // winapi::shared::mmreg::WAVE_FORMAT_PCM, //
    nChannels : 2,
    nSamplesPerSec : SAMPLERATE_INT,
    nAvgBytesPerSec : SAMPLERATE_INT * 4 * 2, // should be sizeof(sample type) * samplerate * 2
    nBlockAlign : 4 * 2, // should be sizeof(sample type) * 2
    wBitsPerSample: 32, // should be sizeof(sample type) * 8
    cbSize:0
 };

static mut WAVE_HEADER : winapi::um::mmsystem::WAVEHDR = winapi::um::mmsystem::WAVEHDR{
    lpData: 0 as *mut i8,
    dwBufferLength: 8 * (garlic_head::SAMPLES as u32), // SHOULD BE (check that!) max_samples * sizeof(float) * 2
    dwBytesRecorded: 0,
    dwUser: 0,
    dwFlags: 0,
    dwLoops: 0,
    lpNext: 0 as *mut winapi::um::mmsystem::WAVEHDR,
    reserved: 0,
};

// 2 because of WAVE_FORMAT.nChannels
static mut GARLIC_DATA : garlic_head::StereoTrack = [0.0; garlic_head::SAMPLES_TWICE];

/*
static mut MMTIME: winapi::um::mmsystem::MMTIME = winapi::um::mmsystem::MMTIME {
    wType: winapi::um::mmsystem::TIME_MS,
    u: ?? how to get something like winapi::um::mmsystem::MMTIME_u ??
};
*/

/*
// if you need, for debug reasons, the check that the waveout works as intended
pub unsafe fn render_track(data: &mut StereoTrack) {
    for sample in 0 .. data.len() / 2 {
        let debug_sine = crate::math::sin(crate::math::TAU * 440. * sample as f32 * INV_SAMPLERATE);
        data[2 * sample] = debug_sine;
        data[2 * sample + 1] = debug_sine;
    }
}
*/

static mut H_WAVEOUT: winapi::um::mmsystem::HWAVEOUT = 0 as winapi::um::mmsystem::HWAVEOUT;

//static mut mmTime: winapi::um::mmsystem::LPMMTIME = 0 as *mut winapi::um::mmsystem::MMTIME;

/*static mut MM_TIME: winapi::um::mmsystem::MMTIME = winapi::um::mmsystem::MMTIME {
    wType: winapi::um::mmsystem::TIME_MS,
    u: winapi::um::mmsystem::MMTIME_u::from(0 as winapi::um::mmsystem::MMTIME_u)
};
*/


/// Pointer to an ANSI string.
pub type LPCSTR = *const winapi::ctypes::c_char;
/// Pointer to a procedure of unknown type.
pub type PROC = *mut winapi::ctypes::c_void;

#[link(name = "Opengl32")]
extern "system" {
  /// [`wglGetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress)
  pub fn wglGetProcAddress(Arg1: LPCSTR) -> PROC;
}

//static mut program : u32 = 0;
type glCreateShaderProgramv_type = unsafe extern "system" fn(u32, usize, &str) -> usize;
type glUseProgram_type = unsafe extern "system" fn(usize) -> bool;
type glRecti_type = unsafe extern "system" fn(i32, i32, i32, i32) -> ();
pub const FRAGMENT_SHADER: u32 = 0x8B30;

pub unsafe fn UseProgram(program: u32) -> () {
    core::mem::transmute::<_, extern "system" fn(u32) -> ()>(
        wglGetProcAddress("glUseProgram\0".as_ptr().cast())
    )(program)
}

#[link(name = "Opengl32")]
extern "system" {
    fn glRecti(x1: i32, y1: i32, x2: i32, y2: i32) -> ();
    fn glFlush() -> ();
}

static gfx_frag: &'static str = "
#version 430

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(/*iTime*/0.0+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}

void main(){mainImage(gl_FragColor, gl_FragCoord.xy);}
\0";

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let ( _, hdc ) = create_window(  );

    unsafe {
/*
        GLint program = ((PFNGLCREATESHADERPROGRAMVPROC)wglGetProcAddress("glCreateShaderProgramv"))(GL_FRAGMENT_SHADER, 1, &gfx_frag);
        ((PFNGLUSEPROGRAMPROC)wglGetProcAddress("glUseProgram"))(program);
        GLint iTime_location = ((PFNGLGETUNIFORMLOCATIONPROC)wglGetProcAddress("glGetUniformLocation"))(program, VAR_ITIME);
*/
        let glCreateShaderProgramv: glCreateShaderProgramv_type = core::mem::transmute(
            wglGetProcAddress("glCreateShaderProgramv\0".as_ptr().cast())
        );
        //program = gl::CreateShaderProgramv(gl::FRAGMENT_SHADER, 1, gfx_frag.as_ptr()); // program is return of gl::GetShaderiv?

        let glUseProgram: glUseProgram_type = core::mem::transmute(
            wglGetProcAddress("glUseProgram\0".as_ptr().cast())
        );

        let program = glCreateShaderProgramv(FRAGMENT_SHADER, 1, gfx_frag);

        let whatevers = glUseProgram(program);
        // gl::UseProgram(program);

        // let iTime_location = gl::GetUniformLocation(program, "iTime".as_ptr());

    }

    unsafe {
        garlic_head::render_track(&mut GARLIC_DATA);
    }
    log!("Render finished\n\0");

    unsafe {
        WAVE_HEADER.lpData = GARLIC_DATA.as_mut_ptr() as *mut i8;
        winapi::um::mmeapi::waveOutOpen( &mut H_WAVEOUT, winapi::um::mmsystem::WAVE_MAPPER, &WAVE_FORMAT, 0, 0, winapi::um::mmsystem::CALLBACK_NULL);
        winapi::um::mmeapi::waveOutPrepareHeader(H_WAVEOUT, &mut WAVE_HEADER, core::mem::size_of::<winapi::um::mmsystem::WAVEHDR>() as u32 );
        winapi::um::mmeapi::waveOutWrite(H_WAVEOUT, &mut WAVE_HEADER, core::mem::size_of::<winapi::um::mmsystem::WAVEHDR>() as u32 );

        //(*mmTime).wType = winapi::um::mmsystem::TIME_MS; // Illegal Instruction
        #[cfg(debug_assertions)] {
            debug::write_wave_file(&GARLIC_DATA);
        }
    }

    // debugging
    /*
    for i in 0 .. 100 {
        let x: f32 = i as f32 / 100.;
        unsafe {
            printf("x=%.3f sm=%3f\n\0".as_ptr(), x as f64, math::smoothstep(0.25, 0.75, x) as f64);
        }
    }
    */

    let mut time_ms: u32 = 0;
    let start_ms = unsafe { winapi::um::timeapi::timeGetTime() };
    let end_ms = start_ms + (garlic_head::SECONDS * 1000.) as u32 + 100;

    loop {

        unsafe {
            if winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_ESCAPE) != 0 {
                break;
            }
        }

        unsafe {

            // ((PFNGLUNIFORM1FPROC)

            //let mut prc = wglGetProcAddress("glUniform1f".as_ptr() as *const i8)(iTime_location, time_ms as f32) as usize;

            glRecti(-1, -1, 1, 1);
            //gl::Recti(-1, -1, 1, 1);
            glFlush();

            SwapBuffers(hdc);

        }

        // qm: this loop is obviously lame because we render the whole track beforehand. maybe we do the block-splitting later on

        time_ms = unsafe {
            //let result = winapi::um::mmeapi::waveOutGetPosition(h_waveout, mmTime, core::mem::size_of::<winapi::um::mmsystem::MMTIME>() as u32);
            // (*mmTime).u.ms() // or something??
            winapi::um::timeapi::timeGetTime()
        };

        // No idea how to read MMTIME out here, yet. Instead, just count some time upwards.
        //time += 1.0 / 60.0;

        if time_ms > end_ms {
            break;
        }
    }

    unsafe {
        printf("Playback Finished! %d ms\n\0".as_ptr(), time_ms - start_ms);
        winapi::um::processthreadsapi::ExitProcess(0);
    }
}

// Compiling with no_std seems to require the following symbol to be set if there is any floating point code anywhere in the code
#[no_mangle]
pub static _fltused : i32 = 1;

// some day: get why we are not more similar to https://riptutorial.com/rust/example/5870/sharp--no-std--hello--world-
