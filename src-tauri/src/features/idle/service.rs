use crate::core::error::AppResult;

/// Tra ve so giay ke tu input cuoi cung cua user
pub fn get_idle_seconds() -> AppResult<u64> {
    #[cfg(target_os = "windows")]
    {
        get_idle_seconds_windows()
    }

    #[cfg(target_os = "macos")]
    {
        get_idle_seconds_macos()
    }

    #[cfg(target_os = "linux")]
    {
        get_idle_seconds_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(0) // fallback: never idle
    }
}

#[cfg(target_os = "windows")]
fn get_idle_seconds_windows() -> AppResult<u64> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

    let mut last_input = LASTINPUTINFO {
        cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };

    unsafe {
        let success = GetLastInputInfo(&mut last_input);
        if !success.as_bool() {
            return Err(crate::core::error::AppError::General(
                "GetLastInputInfo failed".to_string(),
            ));
        }
        // GetTickCount returns u32 milliseconds since system start
        let tick_count = windows::Win32::System::SystemInformation::GetTickCount();
        let idle_ms = tick_count.wrapping_sub(last_input.dwTime);
        Ok((idle_ms / 1000) as u64)
    }
}

#[cfg(target_os = "macos")]
fn get_idle_seconds_macos() -> AppResult<u64> {
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    let idle_secs = CGEventSource::seconds_since_last_event_type(
        CGEventSourceStateID::CombinedSessionState,
        core_graphics::event::CGEventType::Null,
    );
    Ok(idle_secs as u64)
}

#[cfg(target_os = "linux")]
fn get_idle_seconds_linux() -> AppResult<u64> {
    use x11::xlib;
    use x11::xss;

    unsafe {
        let display = xlib::XOpenDisplay(std::ptr::null());
        if display.is_null() {
            return Ok(0);
        }
        let mut info: xss::XScreenSaverInfo = std::mem::zeroed();
        let root = xlib::XDefaultRootWindow(display);
        xss::XScreenSaverQueryInfo(display, root, &mut info);
        xlib::XCloseDisplay(display);
        Ok((info.idle / 1000) as u64)
    }
}
