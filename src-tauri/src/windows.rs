#[cfg(windows)]
pub fn detach_console() {
    use windows_sys::Win32::System::Console::FreeConsole;

    unsafe {
        let _ = FreeConsole();
    }
}

#[cfg(windows)]
pub fn was_started_from_console() -> bool {
    use windows_sys::Win32::{
        Foundation::HWND,
        System::{Console::GetConsoleWindow, Threading::GetCurrentProcessId},
        UI::WindowsAndMessaging::GetWindowThreadProcessId,
    };

    unsafe {
        let console_wnd: HWND = GetConsoleWindow();
        let mut dw_process_id = 0;
        let _ = GetWindowThreadProcessId(console_wnd, &mut dw_process_id);
        let current_pid = GetCurrentProcessId();
        current_pid == dw_process_id
    }
}
