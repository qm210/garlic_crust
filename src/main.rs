#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![windows_subsystem = "console"]
#![feature(core_intrinsics)]
#![feature(static_nobundle)]
#![allow(dead_code, non_snake_case, unused_imports)]

#[cfg(windows)] extern crate winapi;

pub mod util;
pub mod math;
mod garlic_crust;
mod garlic_head;

// debug profile uses std library (e.g. for .wav file writing).
// this should better be a "feature" in cargo, but for now, its not.
#[cfg(debug_assertions)]
mod debug;

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
    PIXELFORMATDESCRIPTOR
};

use winapi::shared::minwindef::{
    LRESULT,
    LPARAM,
    LPVOID,
    WPARAM,
    UINT,
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
    CS_OWNDC,
    CS_HREDRAW,
    CS_VREDRAW,
    CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
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
        let h_wnd : HWND;

        let hinstance = GetModuleHandleA( 0 as *const i8 );
        let mut wnd_class : WNDCLASSA = core::mem::zeroed();
        wnd_class.style = CS_OWNDC | CS_HREDRAW | CS_VREDRAW;
        wnd_class.lpfnWndProc = Some( window_proc );
        wnd_class.hInstance = hinstance;							// The instance handle for our application which we can retrieve by calling GetModuleHandleW.
        wnd_class.lpszClassName = "MyClass\0".as_ptr() as *const i8;
        RegisterClassA( &wnd_class );

        h_wnd = CreateWindowExA(
            0,
            //WS_EX_APPWINDOW | WS_EX_WINDOWEDGE,                     // dwExStyle
            "MyClass\0".as_ptr() as *const i8,		                // class we registered.
            "GARLIC_CRUST\0".as_ptr() as *const i8,						// title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,	// dwStyle
            CW_USEDEFAULT, CW_USEDEFAULT, 1920/2, 1080/2,	// size and position
            0 as HWND,               	// hWndParent
            0 as HMENU,					// hMenu
            hinstance,                  // hInstance
            0 as LPVOID );				// lpParam

        let h_dc : HDC = GetDC(h_wnd);        // Device Context

        let mut pfd : PIXELFORMATDESCRIPTOR = core::mem::zeroed();
        pfd.nSize = core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 32;
        pfd.cAlphaBits = 8;
        pfd.cDepthBits = 32;

        let pf_id : i32 = ChoosePixelFormat(h_dc, &pfd );
        SetPixelFormat(h_dc, pf_id, &pfd);
        let gl_context : HGLRC = wglCreateContext(h_dc);    // Rendering Context
        wglMakeCurrent(h_dc, gl_context);

        // make the system font the device context's selected font
        winapi::um::wingdi::SelectObject (h_dc, winapi::um::wingdi::GetStockObject (winapi::um::wingdi::SYSTEM_FONT as i32));

        // create the bitmap display lists
        winapi::um::wingdi::wglUseFontBitmapsA (h_dc, 0, 255, 1000);

        ( h_wnd, h_dc )
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
    nAvgBytesPerSec : SAMPLERATE_INT * 4 * 2,
    nBlockAlign : 4 * 2,
    wBitsPerSample: 32,
    cbSize:0
 };

static mut WAVE_HEADER : winapi::um::mmsystem::WAVEHDR = winapi::um::mmsystem::WAVEHDR{
    lpData: 0 as *mut i8,
    dwBufferLength: 4 * (garlic_head::SAMPLES as u32),
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

static mut H_WAVEOUT: winapi::um::mmsystem::HWAVEOUT = 0 as winapi::um::mmsystem::HWAVEOUT;

//static mut mmTime: winapi::um::mmsystem::LPMMTIME = 0 as *mut winapi::um::mmsystem::MMTIME;

/*static mut MM_TIME: winapi::um::mmsystem::MMTIME = winapi::um::mmsystem::MMTIME {
    wType: winapi::um::mmsystem::TIME_MS,
    u: winapi::um::mmsystem::MMTIME_u::from(0 as winapi::um::mmsystem::MMTIME_u)
};
*/

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let ( _, hdc ) = create_window(  );

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

        unsafe { SwapBuffers(hdc); }

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
