mod mirc;

use debug_print::debug_eprintln as deprintln;
use mIRC::MircReturn;
use mirc as mIRC;
use std::sync::atomic::{AtomicI32, Ordering};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND};

// Global variables
static M_WAITING_FOR_MEDIA_CHANGE: AtomicI32 = AtomicI32::new(0);

#[no_mangle]
pub extern "stdcall" fn wait_for_media(
    _m_wnd: HWND,
    _a_wnd: HWND,
    data: PCWSTR,
    _parms: PCWSTR,
    _show: BOOL,
    _nopause: BOOL,
) -> MircReturn {
    if !mIRC::is_dllcall() {
        // Synchronous calls using $dll or /dll block the GUI thread, prevent this.
        deprintln!("Prevented attempt to block GUI thread in wait_for_media()");
        let result =
            w!("!echo -estgc info * The DLL routine you attempted to use is not allowed on the GUI thread, try using $ $+ dllcall() instead.");
        unsafe {
            std::ptr::copy_nonoverlapping(result.as_ptr(), data.0 as *mut u16, result.len() + 1);
        }
        return MircReturn::Command;
    }

    if M_WAITING_FOR_MEDIA_CHANGE.load(Ordering::SeqCst) != 0 {
        // If we're already looping in another thread, return immediately.
        return MircReturn::Continue;
    }

    M_WAITING_FOR_MEDIA_CHANGE.store(1, Ordering::SeqCst);
    while M_WAITING_FOR_MEDIA_CHANGE.load(Ordering::SeqCst) != 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    MircReturn::Continue // Return the result to mIRC
}

#[no_mangle]
pub extern "stdcall" fn halt(
    _m_wnd: HWND,
    _a_wnd: HWND,
    data: PCWSTR,
    _parms: PCWSTR,
    _show: BOOL,
    _nopause: BOOL,
) -> MircReturn {
    M_WAITING_FOR_MEDIA_CHANGE.store(0, Ordering::SeqCst);
    let result = w!("!echo -estgc info * All $dllcall() calls have completed.");
    unsafe {
        std::ptr::copy_nonoverlapping(result.as_ptr(), data.0 as *mut u16, result.len() + 1);
    }
    MircReturn::Command // Return the result to mIRC
}
