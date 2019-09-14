use core::ffi::c_void;

use super::gx::types::GXRModeObj;

// Some functions are missing, specially the ones that use
// callbacks.

#[link(name="ogc")]
extern {
    pub fn VIDEO_GetNextFramebuffer() -> *mut c_void;
    pub fn VIDEO_GetCurrentFramebuffer() -> *mut c_void;

    pub fn VIDEO_Init();
    pub fn VIDEO_Flush();
    pub fn VIDEO_SetBlack(black: bool);
    pub fn VIDEO_GetNextField() -> u32;
    pub fn VIDEO_GetCurrentLine() -> u32;
    pub fn VIDEO_GetCurrentTvMode() -> u32;

    pub fn VIDEO_Configure(rmode: *mut GXRModeObj);
    pub fn VIDEO_GetFrameBufferSize(rmode: *mut GXRModeObj) -> u32;
    pub fn VIDEO_ClearFrameBuffer(rmode: *mut GXRModeObj, fb: *mut c_void, color: u32);

    pub fn VIDEO_WaitVSync();
    pub fn VIDEO_SetNextFramebuffer(fb: *mut c_void);
    pub fn VIDEO_SetNextRightFramebuffer(fb: *mut c_void);

    pub fn VIDEO_HaveComponentCable() -> u32;

    pub fn VIDEO_GetPreferredMode(mode: *mut GXRModeObj) -> *mut GXRModeObj;
}
