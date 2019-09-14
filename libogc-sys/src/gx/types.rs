#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GXRModeObj {
    pub viTVMode: u32,
    pub fbWidth: u16,
    pub efbHeight: u16,
    pub xfbHeight: u16,
    pub viXOrigin: u16,
    pub viYOrigin: u16,
    pub viWidth: u16,
    pub viHeight: u16,
    pub xfbMode: u32,
    pub field_rendering: u8,
    pub aa: u8,
    pub sample_pattern: [[u8; 2usize]; 12usize],
    pub vfilter: [u8; 7usize],
}
