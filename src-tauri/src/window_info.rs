use serde::Serialize;

#[cfg(windows)]
use windows::Win32::Foundation::{HWND, MAX_PATH, HANDLE};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
};
#[cfg(windows)]
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};
#[cfg(windows)]
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;

#[derive(Serialize, Debug, Clone)]
pub struct WindowInfo {
    pub title: String,
    pub exe_path: String,
    pub pid: u32,
}

#[cfg(windows)]
#[tauri::command]
pub fn get_foreground_window_info() -> Option<WindowInfo> {
    let hwnd: HWND = unsafe { GetForegroundWindow() };
    // 检查是否为空句柄
    if hwnd.is_invalid() {
        return None;
    }

    let mut title_buffer = [0u16; 512];
    let title_len = unsafe { GetWindowTextW(hwnd, &mut title_buffer) };
    let title = String::from_utf16_lossy(&title_buffer[..title_len as usize]);

    let mut pid: u32 = 0;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == 0 {
        return None;
    }

    let exe_path = unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid);
        
        if process_handle.is_err() {
            return Some(WindowInfo {
                title,
                exe_path: String::new(),
                pid,
            });
        }
        
        let handle = process_handle.unwrap();
        let mut path_buffer = [0u16; MAX_PATH as usize];
        
        let module_len = GetModuleFileNameExW(
            Some(handle), 
            None, 
            &mut path_buffer
        );
        
        String::from_utf16_lossy(&path_buffer[..module_len as usize])
    };

    Some(WindowInfo { title, exe_path, pid })
}

#[cfg(not(windows))]
#[tauri::command]
pub fn get_foreground_window_info() -> Option<WindowInfo> {
    None
}
