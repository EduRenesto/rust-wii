use core::ffi::c_void;

use super::gx::types::GXRModeObj;

#[link(name="ogc")]
extern {
    pub fn CON_Init(framebuffer: *mut c_void, xstart: i32, ystart: i32, 
                    xres: i32, yres: i32, stride: i32);

    pub fn CON_InitEx(rmode: *mut GXRModeObj, conXOrigin: i32, 
                      conYOrigin: i32, conWidth: i32, conHeight: i32) -> i32;

    pub fn CON_GetMetrics(cols: *mut i32, rows: *mut i32);

    pub fn CON_GetPosition(cols: *mut i32, rows: *mut i32);

    pub fn CON_EnableGecko(channel: i32, safe: i32);
}
