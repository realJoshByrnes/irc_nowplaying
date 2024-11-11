use windows::Win32::Foundation::{BOOL, HWND};

#[repr(C)]
pub struct LOADINFO {
    pub m_version: u32,
    pub m_hwnd: HWND,
    pub m_keep: BOOL,
    pub m_unicode: BOOL,
    pub m_beta: u32,
    pub m_bytes: u32,
}

#[repr(i32)]
pub enum MircReturn {
    Halt = 0,
    Continue = 1,
    Command = 2,
    Result = 3,
}

#[repr(i32)]
pub enum TimeoutReason {
    Unload = 0,
    Inactive = 1,
    Exit = 2,
}
