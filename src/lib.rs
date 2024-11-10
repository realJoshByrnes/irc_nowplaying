mod mirc;

use std::sync::Mutex;
use windows::core::{PCWSTR, w};
use windows::Win32::Foundation::{BOOL, HWND};
use mirc::{LOADINFO, MircReturn, TimeoutReason};

#[cfg(target_arch = "x86")]
static M_BITS: &str = "32-bit";
#[cfg(target_arch = "x86_64")]
static M_BITS: &str = "64-bit";
#[cfg(all(not(target_arch = "x86"), not(target_arch = "x86_64")))]
static M_BITS: &str = "Unknown Arch";

static M_VERSION: Mutex<u32> = Mutex::new(0);
static M_MAX_BYTES: Mutex<u32> = Mutex::new(0);


#[no_mangle]
pub extern "stdcall" fn LoadDll(li: *mut LOADINFO) {
  unsafe {
    (*li).m_keep = BOOL(1); // Keep the DLL loaded
    (*li).m_unicode = BOOL(1); // Use Unicode

    // Store version in a global variable
    let mut version = M_VERSION.lock().unwrap();
    *version = (*li).m_version;

    // Store the maximum bytes (for data & parms) in a global variable
    let mut max_bytes = M_MAX_BYTES.lock().unwrap();
    *max_bytes = (*li).m_bytes;
  }
}

#[no_mangle]
pub extern "stdcall" fn UnloadDll(m_timeout: TimeoutReason) -> i32 {
  match m_timeout {
    TimeoutReason::Unload => 0,
    TimeoutReason::Inactive => 1, // You can return return 0 to keep the DLL loaded, or 1 to allow it to be unloaded.
    TimeoutReason::Exit => 0,
  }
}

#[no_mangle]
pub extern "stdcall" fn MyFunction(
  _m_wnd: HWND,
  _a_wnd: HWND,
  data: PCWSTR,
  _parms: PCWSTR,
  _show: BOOL,
  _nopause: BOOL,
) -> MircReturn {
    let message = w!("Hello, World!");
    unsafe {
      std::ptr::copy_nonoverlapping(message.as_ptr(), data.0 as *mut u16, message.len() + 1);
    }
    MircReturn::Result // Return the result to mIRC
}

#[no_mangle]
pub extern "stdcall" fn version(
  _m_wnd: HWND,
  _a_wnd: HWND,
  data: PCWSTR,
  _parms: PCWSTR,
  _show: BOOL,
  _nopause: BOOL,
) -> MircReturn {
  let name = env!("CARGO_PKG_NAME");
  let version = env!("CARGO_PKG_VERSION");
  let author = env!("CARGO_PKG_AUTHORS");
  let m_version = *M_VERSION.lock().unwrap();
  let m_version_low = m_version & 0xFFFF;
  let m_version_high = m_version >> 16;
  
  let input = format!("{} {} by {} on mIRC v{}.{} ({})\0", name, version, author, m_version_low, m_version_high, M_BITS);
  let wide_input: Vec<u16> = input.encode_utf16().collect();
  let message = PCWSTR(wide_input.as_ptr());
  unsafe {
    std::ptr::copy_nonoverlapping(message.as_ptr(), data.0 as *mut u16, message.len() + 1);
  }
  MircReturn::Result // Return the result to mIRC
}