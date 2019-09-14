use core::ffi::c_void;

use super::gx::types::GXRModeObj;

pub static SYS_BASE_UNCACHED: u32 = 0xC0000000;
pub static SYS_BASE_CACHED: u32 = 0x80000000; 

// Most functions are missing

#[link(name="ogc")]
extern {
    pub fn SYS_AllocateFramebuffer(rmode: *mut GXRModeObj) -> *mut c_void;
}

pub unsafe fn mem_k0_to_k1<T>(ptr: *mut T) -> *mut T {
    let old_address = ptr as u32;

    let new_address = old_address + (SYS_BASE_UNCACHED - SYS_BASE_CACHED);

    new_address as *mut T
}
