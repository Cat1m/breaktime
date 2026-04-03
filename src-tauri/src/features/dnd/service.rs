use crate::core::error::AppResult;

/// Kiem tra xem he thong co dang o che do Do Not Disturb / Focus khong
#[allow(dead_code)]
pub fn is_dnd_active() -> AppResult<bool> {
    #[cfg(target_os = "windows")]
    {
        is_dnd_active_windows()
    }

    #[cfg(target_os = "macos")]
    {
        is_dnd_active_macos()
    }

    // Linux: khong co standard DND API
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Ok(false)
    }
}

#[cfg(target_os = "windows")]
fn is_dnd_active_windows() -> AppResult<bool> {
    // Read registry directly via Windows API — no subprocess needed
    use windows::Win32::System::Registry::*;

    let mut hkey = HKEY::default();
    let subkey = windows::core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Notifications\\Settings");
    let value_name = windows::core::w!("NOC_GLOBAL_SETTING_TOASTS_ENABLED");

    let result = unsafe {
        RegOpenKeyExW(HKEY_CURRENT_USER, subkey, 0, KEY_READ, &mut hkey)
    };

    if result.is_err() {
        return Ok(false); // Key doesn't exist = DND off
    }

    let mut data: u32 = 1;
    let mut data_size = std::mem::size_of::<u32>() as u32;
    let mut data_type = REG_VALUE_TYPE::default();

    let result = unsafe {
        RegQueryValueExW(
            hkey,
            value_name,
            None,
            Some(&mut data_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        )
    };

    unsafe { RegCloseKey(hkey) };

    if result.is_err() {
        return Ok(false); // Value doesn't exist = DND off
    }

    // 0 = notifications disabled = DND on
    Ok(data == 0)
}

#[cfg(target_os = "macos")]
fn is_dnd_active_macos() -> AppResult<bool> {
    // macOS: conservative fallback for v1
    Ok(false)
}
