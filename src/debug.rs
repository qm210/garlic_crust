extern crate std;

use std::fs::File;

#[link(name="vcruntime")]
extern {}

pub unsafe fn write_wave_file() {
    super::printf("Waveout would happen now!\n\0".as_ptr());


}
