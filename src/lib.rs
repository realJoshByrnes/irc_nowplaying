mod mirc;

use mirc::{MircReturn, TimeoutReason, LOADINFO};
use std::sync::Mutex;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::UI::WindowsAndMessaging::GetClassNameW;

// Global variables
static M_CLIENT: Mutex<String> = Mutex::new(String::new());
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

        // Store the client name (mIRC/AdiIRC) in a global variable
        let mut class_name: Vec<u16> = vec![0; 256];
        let mut client = M_CLIENT.lock().unwrap();
        if GetClassNameW((*li).m_hwnd, &mut class_name) > 0 {
            let class_name = PCWSTR(class_name.as_ptr()).to_string().unwrap_or_default();
            if class_name == "mIRC" {
                // mIRC's main window class is "mIRC"
                *client = "mIRC".to_string();
            } else {
                // Otherwise, assume AdiIRC
                *client = "AdiIRC".to_string();
            }
        } else {
            // If we can't get the class name, assume it's an unknown client
            *client = "Unknown".to_string();
        }

        // Store the client name (mIRC/AdiIRC) in a global variable
    }
}

#[no_mangle]
pub extern "stdcall" fn UnloadDll(m_timeout: TimeoutReason) -> i32 {
    match m_timeout {
        TimeoutReason::Unload => 0,
        TimeoutReason::Inactive => {
            1 // Return return 0 to keep the DLL loaded, or 1 to allow it to be unloaded.
        }
        TimeoutReason::Exit => 0,
    }
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
    let arch = std::env::consts::ARCH;

    let m_client = M_CLIENT.lock().unwrap().clone();

    let m_version = *M_VERSION.lock().unwrap();
    let m_version_low = m_version & 0xFFFF;
    let m_version_high = m_version >> 16;

    let input = format!(
        "{} {} by {} on {} v{}.{} ({})\0",
        name, version, author, m_client, m_version_low, m_version_high, arch
    );
    let wide_input: Vec<u16> = input.encode_utf16().collect();
    let message = PCWSTR(wide_input.as_ptr());
    unsafe {
        std::ptr::copy_nonoverlapping(message.as_ptr(), data.0 as *mut u16, message.len() + 1);
    }
    MircReturn::Result // Return the result to mIRC
}
