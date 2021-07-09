#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![windows_subsystem = "console"]
#![feature(core_intrinsics)]
#![feature(static_nobundle)]
#![feature(const_fn_floating_point_arithmetic)]
#![allow(dead_code, non_snake_case, unused_imports)]

#[cfg(windows)] extern crate winapi;

mod gl;
pub mod util;
pub mod math;
pub mod sequence;
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

use winapi::um::mmsystem::{
    MMRESULT,
    MMTIME,
    LPMMTIME,
    HWAVEOUT,
    TIME_SAMPLES,
};

use gl::*;

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
    // link(name="msvcrt", kind="static-nobundle"),
    link(name="ucrt", kind="static-nobundle"),
    link(name="user32", kind="static-nobundle")
)]
extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[link(name="msvcrt", kind="static-nobundle")]
extern "C" {
    pub fn  _chkstk() -> winapi::ctypes::c_ulong;
}

#[link(name="winmm", kind="static-nobundle")]
extern "C" {
    pub fn ExitProcess(uExitCode: UINT);
}

#[link(name="winmm", kind="static-nobundle")]
extern "C" {
    pub fn waveOutGetPosition(
        hwo: HWAVEOUT,
        pmmt: LPMMTIME,
        cbmmt: UINT
    ) -> MMRESULT;
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn create_window( ) -> ( HWND, HDC ) {
    unsafe {
        let mut devmode : DEVMODEA = core::mem::zeroed();
        devmode.dmSize = core::mem::size_of::<DEVMODEA>() as u16;
        devmode.dmFields = winapi::um::wingdi::DM_PELSWIDTH | winapi::um::wingdi::DM_PELSHEIGHT;
        devmode.dmPelsWidth  = WIDTH;
        devmode.dmPelsHeight = HEIGHT;
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

        winapi::um::wingdi::SelectObject (hdc, winapi::um::wingdi::GetStockObject (winapi::um::wingdi::SYSTEM_FONT as i32));
        winapi::um::wingdi::wglUseFontBitmapsA (hdc, 0, 255, 1000);

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
pub type ULONG64 = *mut winapi::ctypes::c_ulong;

/*
// no need to define, as it looks identical to the one given by wingdi::wglGetProcAddress
#[link(name = "Opengl32")]
extern "system" {
  /// [`wglGetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress)
  pub fn wglGetProcAddress(Arg1: LPCSTR) -> PROC;
}
*/

static buffer_a_frag: &'static str = "#version 450
uniform sampler2D iChannel0;uniform vec2 iResolution;uniform float iTime;uniform int iFrame;const vec3 v=vec3(1.,0.,-1.);const float m=3.14159,f=1.618,s=120.,y=60./s;mat3 i=mat3(1.),r=mat3(1.);float x,e;const float a=90.;float n(float v,float y,float m){float s=max(m-abs(v-y),0.)/m;return min(v,y)-s*s*s*m*(1./6.);}float p(float v,float f,float y){return v+f-n(v,f,y);}float t(float y,float v,float m){vec2 f=vec2(v,abs(y)-.5*m);return min(max(f.x,f.y),0.)+length(max(f,0.));}void h(in vec2 v,out float f,out vec2 i){vec2 y=vec2(v.x*1.2,v.y+v.x*.6),m=floor(y),s=fract(y);float x=mod(m.x+m.y,3.),e=step(1.,x),a=step(2.,x);vec2 l=step(s.xy,s.yx);f=dot(l,1.-s.yx+e*(s.x+s.y-1.)+a*(s.yx-2.*s.xy));i=m+e-a*l;i=vec2(i.x/1.2,i.y);i=vec2(i.x,i.y-i.x*.6);}mat3 h(in vec3 y){return mat3(v.xyyy,cos(y.x),sin(y.x),0.,-sin(y.x),cos(y.x))*mat3(cos(y.y),0.,-sin(y.y),v.yxy,sin(y.y),0.,cos(y.y))*mat3(cos(y.z),-sin(y.z),0.,sin(y.z),cos(y.z),v.yyyx);}float n(vec2 v){vec3 f=fract(vec3(v.xyx)*.1031);f+=dot(f,f.yzx+33.33);return fract((f.x+f.y)*f.z);}float p(vec2 f){vec2 y=floor(f);f=fract(f);f=smoothstep(v.yy,v.xx,f);vec2 i=vec2(n(y),n(y+v.xy)),s=vec2(n(y+v.yx),n(y+v.xx));i=v.zz+2.*mix(i,s,f.y);return mix(i.x,i.y,f.x);}float h(vec2 y,float m,float v,float f){float i=0.,s=1.,x=0.,c;for(float r=m;r<v;r*=2.)i+=s*p(r*y-2.*iTime),s*=f,x+=1.;return i*(1.-f)/(1.-pow(f,x));}vec3 t(vec3 v){vec4 i=vec4(1.,2./3.,1./3.,3.);vec3 y=abs(fract(v.xxx+i.xyz)*6.-i.www);return v.z*mix(i.xxx,clamp(y-i.xxx,0.,1.),v.y);}vec3 d(vec3 v){vec4 f=vec4(0.,-1./3.,2./3.,-1.),i=mix(vec4(v.zy,f.wz),vec4(v.yz,f.xy),step(v.z,v.y)),m=mix(vec4(i.xyw,v.x),vec4(v.x,i.yzx),step(i.x,v.x));float s=m.x-min(m.w,m.y),y=1e-10;return vec3(abs(m.z+(m.w-m.y)/(6.*s+y)),s/(m.x+y),m.x);}float d(vec3 y,vec3 v){vec3 f=abs(y)-v;return length(max(f,0.))+min(max(f.x,max(f.y,f.z)),0.);}float h(in vec2 v,in float y){float f=2.*m;vec2 i=mod(vec2(atan(v.y,v.x),length(v)/y),f);float x=abs(i.y-i.x);return y*min(x,f-x);}float d(in vec2 v,in vec2 y,in vec2 f){vec2 i=f-y;return length(v-mix(y,f,clamp(dot(v-y,i)/dot(i,i),0.,1.)));}float d(in vec2 y,in float f,in float i,in float x){x*=2.;float a=atan(y.y,y.x),s=m/x,c=mod(a+m,2.*s),e=mod(round((a+m-c)*.5/s),2.),l=s,r=mix(l,-l,e);vec2 z=f*vec2(cos(s-r),sin(s-r)),o=i*vec2(cos(s+r),sin(s+r)),t=o-z,d=normalize(o-z).yx*v.xz,n=length(y)*vec2(cos(c),sin(c));float S=dot(n-z,t)/dot(t,t),u=mix(1.,-1.,e)*dot(n-z,d);if(S<0.)return sign(u)*length(n-z);else if(S>1.)return sign(u)*length(n-o);else return u;}float c(vec2 v){return max(v.x,v.y);}float l(vec2 v){return min(max(max(max(max(min(max(max(c(abs(vec2(abs(abs(v.x)-.25)-.25,v.y))-vec2(.2)),-c(abs(vec2(v.x+.5,abs(abs(v.y)-.05)-.05))-vec2(.12,.02))),-c(abs(vec2(abs(v.x+.5)-.1,v.y-.05*sign(v.x+.5)))-vec2(.02,.07))),c(abs(vec2(v.x+.5,v.y+.1))-vec2(.08,.04))),-c(abs(vec2(v.x,v.y-.04))-vec2(.02,.08))),-c(abs(vec2(v.x,v.y+.1))-vec2(.02))),-c(abs(vec2(v.x-.5,v.y))-vec2(.08,.12))),-c(abs(vec2(v.x-.5,v.y-.05))-vec2(.12,.07))),c(abs(vec2(v.x-.5,v.y))-vec2(.02,.08)));}struct SceneData{float material,dist,accumulation,reflectivity,transmittivity,specular,diffuse;};SceneData S(float y){return SceneData(1.3,y,1.,.1,.1,.5,1.);}SceneData S(SceneData v,SceneData y){if(v.dist<y.dist)return v;return y;}float u;float S(vec3 v,float y,float x,float f){float i=mix(2.,12.,.5+.5*x)*y*x;mat2 m=mat2(cos(i),sin(i),-sin(i),cos(i));float s=-abs(d(m*(v.xy-vec2(x,f)*.5),abs(x+.1*y),abs(f-.1*y),round(5.+x+f)))+.01-.1*y,a=mod(s,.2)-.189;u=s-a;return a;}float c(vec3 v,float y,float f,float x){return-1.+h(v.xy-f*.3,3.,10.,.45)-3.*y;}float l(vec3 v,float y,float x,float f){mat2 i=mat2(cos(iTime),sin(iTime),-sin(iTime),cos(iTime));return-abs(h(i*i*v.xy-.3*x,mix(.05,.1,.5+.5*x)))-.3*y+.01*x;}float n(vec3 v,float y,float f,float i){float s=.3,m=-abs(mod(l(v.xy-y*.4),s)+.5*s-.4-.2*f-.5*y)+.01+.01*x+.001*y;return m;}float p(vec3 v,float y,float f,float x){vec2 i;float m=2.+2.*f,s;h(m*v.xy,s,i);return-abs(s/m)+.01-.5*y;}float t(vec3 v,float y,float f,float i){const float s=.4,e=m/6.,z=.5;vec2 a=vec2(atan(v.y,v.x),length(v.xy));float l=mod(a.x,e)-.5*e,c=a.x-l,r=mod(a.y,z)-.5*z,n=a.y-r;vec2 t=(n-.2*sin(m*y-f))*vec2(cos(c),sin(c)),o=a.y*vec2(cos(a.x),sin(a.x));float S=-length(mat2(cos(iTime-y),sin(iTime-y),-sin(iTime-y),cos(iTime-y))*(v.xy-t))+.001+.1*(.5+.5*i)+.05*(.6+.4*x)+.01*y*(.5+.5*f);return mod(S,.2)-.189;}float c(vec3 y,float f){float i=p(.5*e*v.xx-f),s=p(.5*e*v.xx+1337.-f),x=1.-clamp(iTime/a,0.,1.);const float m=6.;if(x<1.5/m)return mix(S(y,f,i,s),-abs(length(y.xy)-.3+.05*f)+.01-.5*f,smoothstep(.1/m,0.,x)*smoothstep(1.4/m,1.5/m,x));else if(x<3./m)return mix(c(y,f,i,s),-abs(length(y.xy)-.3+.05*f)+.01-.5*f,smoothstep(1.6/m,1.5/m,x)*smoothstep(2.9/m,3./m,x));else if(x<3.5/m)return mix(l(y,f,i,s),-abs(length(y.xy)-.3+.05*f)+.01-.5*f,smoothstep(3.1/m,3./m,x)*smoothstep(3.4/m,3.5/m,x));else if(x<4./m)return mix(n(y,f,i,s),-abs(length(y.xy)-.3+.05*f)+.01-.5*f,smoothstep(3.6/m,3.5/m,x)*smoothstep(3.9/m,4./m,x));else if(x<5./m)return mix(p(y,f,i,s),-abs(length(y.xy)-.3+.05*f)+.01-.5*f,smoothstep(4.1/m,4./m,x)*smoothstep(4.9/m,5./m,x));else return mix(t(y,f,i,s),-abs(length(y.xy)-.3+.05*f)+.01-.5*f,smoothstep(5.1/m,5./m,x)*smoothstep(5.9/m,6./m,x));}SceneData o(vec3 v){SceneData y=SceneData(0.,v.z+.5,0.,0.,0.,.7,1.);float f=.03,i=mod(v.z,f)-.5*f,x=v.z-i,s=x/f;if(x<=0.){vec3 m=-vec3(c(v,x-f),c(v,x),c(v,x+f));float a=n(n(t(i-f,m.x,.5*f)-.15*f,t(i,m.y,.5*f)-.15*f,.01),t(i+f,m.z,.5*f)-.15*f,.01);y=S(y,SceneData(-1.+3.*abs(s/.5*f),a,0.,0.,0.,.7,1.));}return y;}vec3 w(vec3 y){float f=o(y).dist,s=5e-05;return normalize(vec3(o(y+s*v.xyy).dist,o(y+s*v.yxy).dist,o(y+s*v.yyx).dist)-f);}vec3 z(float y){const int i=4;vec3 f[i]=vec3[i](vec3(1.,.22,.3),v.yyy,vec3(.13,.44,.66),vec3(0.,.8,.73));float x=floor(y),s=mod(x+1.,float(i));return mix(f[int(x)],f[int(s)],fract(y));}bool S(out vec3 f,out vec3 y,inout float m,vec3 x,out SceneData i,vec3 s,vec3 a,out vec3 r){for(int e=0-min(iFrame,0);e<250+min(iFrame,0);++e){y=s+m*x;i=o(y);if(i.dist<.0001){r=w(y);if(i.material==0.)f=v.yyy;else f=z(i.material+u*10.-.1*length(y.xy));f=.2*f+i.diffuse*f*max(dot(normalize(a-y),r),0.)+i.specular*f*pow(max(dot(reflect(normalize(a-y),r),x),0.),2.);return true;}m+=min(i.dist,i.dist>1.?.01:.005);}return false;}void l(out vec4 f,in vec2 s){i=h(iTime*vec3(0.,0.,.6));r=h(iTime*vec3(.7,.9,1.32));float c=mod(iTime,y)-.5*y;e=(iTime-c-.5)/y+smoothstep(-.2*y,.2*y,c);x=smoothstep(-.3*y,0.,c)*smoothstep(.3*y,0.,c);float l=0.,z;vec2 n=(s.xy-.5*iResolution.xy)/iResolution.y;vec3 u=i*v.yzx,w=v.yyy,g=v.yyy,b,T,k,D,F=i*v.xyy,R=v.yyy,C=normalize(n.x*F+n.y*cross(F,normalize(R-u))-u),q=v.zzx;SceneData Z,Y;l=-u.z/C.z;b=u+l*C;if(S(w,b,l,C,Z,u,q,k)){if(Z.reflectivity>0.){z=.002;if(S(g,T,z,reflect(C,k),Y,b,q,D))w=mix(w,g,Z.reflectivity);}if(Z.transmittivity>0.){z=.002;if(S(g,T,z,refract(C,k,.99),Y,b,q,D))w=mix(w,g,Z.transmittivity);}Y=Z;z=l;D=k;if(b.z<=.1){u=b;C=normalize(q-b);z=.01;{float X=1.,W=1e+20;for(int V=0;V<250;++V){b=u+z*C;Z=o(b);if(Z.dist<.0001){X=0.;break;}if(b.z>=.1){X=1.;break;}float U=Z.dist*Z.dist/(2.*W)/12.,Q=sqrt(Z.dist*Z.dist-U*U);X=min(X,100.*Q/max(0.,z-U));W=Z.dist;z+=min(Z.dist,Z.dist>.5?.01:.005);}w=mix(.5*w,w,X);}}}Z=Y;if(Z.material!=0.)g=d(w),g.x=m*p(.5*e*v.xx),w=mix(w,t(g),.5),w=w+w*w+w*w*w;w=mix(w,mix(w,w+w*w+w*w*w,.5),smoothstep(.9,1.4,abs(dot(v.xzx,k))));w=mix(v.yyy,w,smoothstep(0.,1.,iTime)*smoothstep(a,a-1.,iTime));f=mix(texture(iChannel0,s.xy/iResolution.xy),vec4(clamp(w,0.,1.),1.),.5);}void main(){l(gl_FragColor,gl_FragCoord.xy);}\0";

static image_frag: &'static str = "#version 450
uniform sampler2D iChannel0;uniform vec2 iResolution;uniform float iTime;uniform int iFrame;const float v=144.;const vec3 i=vec3(1.,0.,-1.);float x,y,e=120.,z=60./e;const float f=90.;float s(vec2 i){return max(i.x,i.y);}float t(vec2 i){return min(max(max(max(max(min(max(max(s(abs(vec2(abs(abs(i.x)-.25)-.25,i.y))-vec2(.2)),-s(abs(vec2(i.x+.5,abs(abs(i.y)-.05)-.05))-vec2(.12,.02))),-s(abs(vec2(abs(i.x+.5)-.1,i.y-.05*sign(i.x+.5)))-vec2(.02,.07))),s(abs(vec2(i.x+.5,i.y+.1))-vec2(.08,.04))),-s(abs(vec2(i.x,i.y-.04))-vec2(.02,.08))),-s(abs(vec2(i.x,i.y+.1))-vec2(.02))),-s(abs(vec2(i.x-.5,i.y))-vec2(.08,.12))),-s(abs(vec2(i.x-.5,i.y-.05))-vec2(.12,.07))),s(abs(vec2(i.x-.5,i.y))-vec2(.02,.08)));}float p(in float i){return smoothstep(1.5/iResolution.y,-1.5/iResolution.y,i);}float a(vec2 i){vec3 v=fract(vec3(i.xyx)*.1031);v+=dot(v,v.yzx+33.33);return fract((v.x+v.y)*v.z);}float m(vec2 v){vec2 y=floor(v);v=fract(v);v=smoothstep(i.yy,i.xx,v);vec2 x=vec2(a(y),a(y+i.xy)),z=vec2(a(y+i.yx),a(y+i.xx));x=i.zz+2.*mix(x,z,v.y);return mix(x.x,x.y,v.x);}void a(out vec4 a,in vec2 n){float s=mod(iTime,z)-.5*z;y=(iTime-s-.5)/z+smoothstep(-.2*z,.2*z,s);x=smoothstep(-.3*z,0.,s)*smoothstep(.3*z,0.,s);vec3 f=vec3(0.);float e=sqrt(v)-1.;for(float c=-.5*e;c<=.5*e;c+=1.)for(float u=-.5*e;u<=.5*e;u+=1.)f+=texture(iChannel0,n/iResolution.xy+vec2(c,u)*mix(3.,20.,2.*abs(n.y/iResolution.y-.5))*exp(-abs(.01*length(n.xy)/iResolution.y-.5))/max(e,1.)/iResolution.xy).xyz;f/=v;vec2 c=(n.xy-.5*iResolution.xy)/iResolution.y,u=c;c=n/iResolution.xy;vec2 g=1./iResolution.xy;float l=1.,o=3.,r=0.;vec4 d=texture(iChannel0,c+vec2(-g.x,-g.y)),b=texture(iChannel0,c+vec2(0.,-g.y)),h=texture(iChannel0,c+vec2(g.x,-g.y)),R=texture(iChannel0,c+vec2(-g.x,0.)),C=texture(iChannel0,c+vec2(0.,0.)),T=texture(iChannel0,c+vec2(g.x,0.)),F=texture(iChannel0,c+vec2(-g.x,g.y)),D=texture(iChannel0,c+vec2(0.,g.y)),q=texture(iChannel0,c+vec2(g.x,g.y)),Z=d*-l+b*-o+h*-l+F*l+D*o+q*l+C*r,Y=d*-l+R*-o+F*-l+h*l+T*o+q*l+C*r;a=vec4(abs(Y.xyz)*.5+abs(Z.xyz)*.5,1.);a=vec4(mix(f,a.xyz,clamp(.25+.5*m(.5*y*i.xx)+.5*x,0.,1.)),1.);float X=t(8.*(u-.5*vec2(iResolution.x/iResolution.y,1.)+vec2(.1,.04)));a.xyz=mix(a.xyz,mix(a.xyz,i.xxx,.5),p(X));}void main(){a(gl_FragColor,gl_FragCoord.xy);}\0";

const XV: f32 = -0.5;

#[no_mangle]
pub fn main() {
    let ( _, hdc ) = create_window(  );
    let iTime_location_buffer_a: gl::GLint;
    let iTime_location_image: gl::GLint;
    let iResolution_location_buffer_a: gl::GLint;
    let iResolution_location_image: gl::GLint;
    let iChannel0_location_buffer_a: gl::GLint;
    let iChannel0_location_image: gl::GLint;
    let iFrame_location_buffer_a: gl::GLint;
    let iFrame_location_image: gl::GLint;
    let mut first_pass_framebuffer: gl::GLuint = 0;
    let mut first_pass_texture: gl::GLuint = 0;
    let program_buffer_a: gl::GLuint;
    let program_image: gl::GLuint;

    unsafe {
        gl::init();
        program_buffer_a = gl::CreateShaderProgramv(gl::FRAGMENT_SHADER, 1, BUFFER_A_FRAG);

        gl::UseProgram(program_buffer_a);
        iTime_location_buffer_a = gl::GetUniformLocation(program_buffer_a, "iTime\0".as_ptr());
        iResolution_location_buffer_a = gl::GetUniformLocation(program_buffer_a, "iResolution\0".as_ptr());
        iChannel0_location_buffer_a = gl::GetUniformLocation(program_buffer_a, "iChannel0\0".as_ptr());
        iFrame_location_buffer_a = gl::GetUniformLocation(program_buffer_a, "iFrame\0".as_ptr());

        program_image = gl::CreateShaderProgramv(gl::FRAGMENT_SHADER, 1, IMAGE_FRAG);

        gl::UseProgram(program_image);
        iTime_location_image = gl::GetUniformLocation(program_image, "iTime\0".as_ptr());
        iResolution_location_image = gl::GetUniformLocation(program_image, "iResolution\0".as_ptr());
        iChannel0_location_image = gl::GetUniformLocation(program_image, "iChannel0\0".as_ptr());
        iFrame_location_image = gl::GetUniformLocation(program_image, "iFrame\0".as_ptr());

        gl::GenFramebuffers(1, &mut first_pass_framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, first_pass_framebuffer);
        gl::GenTextures(1, &mut first_pass_texture);
        gl::BindTexture(gl::TEXTURE_2D, first_pass_texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, WIDTH as i32, HEIGHT as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, 0 as *mut winapi::ctypes::c_void);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, first_pass_texture, 0);
        gl::DrawBuffer(gl::COLOR_ATTACHMENT0);
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
    unsafe {
        let mut mmtime: MMTIME = core::mem::zeroed();
        mmtime.wType = TIME_SAMPLES;
        let mut time: f32 = 0.0;
        let mut frame: i32 = 0;

        loop {

            unsafe {
                if winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_ESCAPE) != 0 || time >= sequence::SECONDS {
                    break;
                    // libc::exit(0);
                }

                waveOutGetPosition(H_WAVEOUT, &mut mmtime, core::mem::size_of::<MMTIME>() as u32);
                time = *mmtime.u.sample() as f32 / SAMPLERATE_INT as f32;

                // Buffer A
                gl::BindFramebuffer(gl::FRAMEBUFFER, first_pass_framebuffer);
                gl::UseProgram(program_buffer_a);
                gl::Uniform1f(iTime_location_buffer_a, time);
                gl::Uniform2f(iResolution_location_buffer_a, WIDTH as f32, HEIGHT as f32);
                gl::Uniform1i(iChannel0_location_buffer_a, 0);
                gl::Uniform1i(iFrame_location_buffer_a, frame);
                gl::ActiveTexture(gl::TEXTURE0);

                gl::Recti(-1,-1,1,1);
                gl::Flush();

                // Image
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::UseProgram(program_image);
                gl::Uniform1f(iTime_location_image, time);
                gl::Uniform2f(iResolution_location_image, WIDTH as f32, HEIGHT as f32);
                gl::Uniform1i(iChannel0_location_image, 0);
                gl::Uniform1i(iFrame_location_image, frame);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::Recti(-1,-1,1,1);
                gl::Flush();

                // Text
                if time > 2.
                {
                    const xv: f32 = -0.5;
                    gl::UseProgram(0);
                    gl::ListBase (1000);
                    gl::RasterPos2f(xv, 0.2);
                    gl::CallLists (41, gl::UNSIGNED_BYTE, "Team210 and The Acid Desk proudly present\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, 0.1);
                    gl::CallLists (12, gl::UNSIGNED_BYTE, "Garlic Rulez\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, 0.0);
                    gl::CallLists (12, gl::UNSIGNED_BYTE, "Code: QM^NR4\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, -0.05);
                    gl::CallLists (13, gl::UNSIGNED_BYTE, "Graphics: NR4\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, -0.1);
                    gl::CallLists (9, gl::UNSIGNED_BYTE, "Music: QM\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, -0.2);
                    gl::CallLists (41, gl::UNSIGNED_BYTE, "Rust. GLSL. New Synth. Party prod @ UC11.\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, -0.4);
                    gl::CallLists (8, gl::UNSIGNED_BYTE, "Love to:\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, -0.45);
                    gl::CallLists (117, gl::UNSIGNED_BYTE, "mercury, alcatraz, vacuum, team210, abyss-connection, k2, die wissenden, farbrausch, team210, the electronic knights,\0".as_ptr() as *const winapi::ctypes::c_void );
                    gl::RasterPos2f(xv, -0.5);
                    gl::CallLists (120, gl::UNSIGNED_BYTE, "never, copernicium, madboys unlimited virtual enterprises ltd., spacepigs, team210, spacepigs, 5711, TRBL, ctrl-alt-test\0".as_ptr() as *const winapi::ctypes::c_void );
                }

            if winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_ESCAPE) != 0 || time >= sequence::SECONDS {
                libc::exit(0);
            }

            waveOutGetPosition(H_WAVEOUT, &mut mmtime, core::mem::size_of::<MMTIME>() as u32);
            time = *mmtime.u.sample() as f32 / SAMPLERATE_INT as f32;

            // Buffer A
            gl::BindFramebuffer(gl::FRAMEBUFFER, first_pass_framebuffer);
            gl::UseProgram(program_buffer_a);
            gl::Uniform1f(iTime_location_buffer_a, time);
            gl::Uniform2f(iResolution_location_buffer_a, WIDTH as f32, HEIGHT as f32);
            gl::Uniform1i(iChannel0_location_buffer_a, 0);
            gl::Uniform1i(iFrame_location_buffer_a, frame);
            gl::ActiveTexture(gl::TEXTURE0);

            gl::Recti(-1,-1,1,1);
            gl::Flush();

            // Image
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::UseProgram(program_image);
            gl::Uniform1f(iTime_location_image, time);
            gl::Uniform2f(iResolution_location_image, WIDTH as f32, HEIGHT as f32);
            gl::Uniform1i(iChannel0_location_image, 0);
            gl::Uniform1i(iFrame_location_image, frame);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::Recti(-1,-1,1,1);
            gl::Flush();

            // Text
            if time > 2.
            {
                gl::UseProgram(0);
                gl::ListBase (1000);
                gl::RasterPos2f(XV, 0.2);
                gl::CallLists (41, gl::UNSIGNED_BYTE, "Team210 and The Acid Desk proudly present\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, 0.1);
                gl::CallLists (12, gl::UNSIGNED_BYTE, "Garlic Rulez\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, 0.0);
                gl::CallLists (12, gl::UNSIGNED_BYTE, "Code: QM^NR4\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, -0.05);
                gl::CallLists (13, gl::UNSIGNED_BYTE, "Graphics: NR4\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, -0.1);
                gl::CallLists (9, gl::UNSIGNED_BYTE, "Music: QM\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, -0.2);
                gl::CallLists (41, gl::UNSIGNED_BYTE, "Rust. GLSL. New Synth. Party prod @ UC11.\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, -0.4);
                gl::CallLists (8, gl::UNSIGNED_BYTE, "Love to:\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, -0.45);
                gl::CallLists (127, gl::UNSIGNED_BYTE, "mercury, alcatraz, vacuum, team210, abyss-connection, k2, http://die.wissen.de/n, farbrausch, team210, the electronic knights,\0".as_ptr() as *const winapi::ctypes::c_void );
                gl::RasterPos2f(XV, -0.5);
                gl::CallLists (120, gl::UNSIGNED_BYTE, "never, copernicium, madboys unlimited virtual enterprises ltd., spacepigs, team210, spacepigs, 5711, TRBL, ctrl-alt-test\0".as_ptr() as *const winapi::ctypes::c_void );
            }

            SwapBuffers(hdc);

            frame += 1;
        }

    }
}

// Compiling with no_std seems to require the following symbol to be set if there is any floating point code anywhere in the code
#[no_mangle]
pub static _fltused : i32 = 1;

// some day: get why we are not more similar to https://riptutorial.com/rust/example/5870/sharp--no-std--hello--world-
