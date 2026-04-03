use serde::Serialize;

#[cfg(windows)]
use windows::Win32::Foundation::{HWND, MAX_PATH, RECT, FILETIME};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    GetWindowRect, IsWindowVisible, GetWindowLongW, GetParent,
    GWL_STYLE, GWL_EXSTYLE, SW_MAXIMIZE, SW_MINIMIZE,
};
#[cfg(windows)]
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, GetProcessTimes};
#[cfg(windows)]
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;

fn get_window_rect(hwnd: HWND) -> RECT {
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect) };
    rect
}

#[derive(Serialize, Debug, Clone)]
pub struct WindowInfo {
    pub title: String,
    pub exe_path: String,
    pub pid: u32,
    pub hwnd: u64,
    pub visible: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub is_maximized: bool,
    pub is_minimized: bool,
    pub style: u64,
    pub extended_style: u64,
    pub parent_hwnd: Option<u64>,
    pub process_start_time: Option<u64>,
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
            let rect = get_window_rect(hwnd);
            return Some(WindowInfo {
                title,
                exe_path: String::new(),
                pid,
                hwnd: hwnd.0 as u64,
                visible: false,
                position: (rect.left, rect.top),
                size: (0, 0),
                is_maximized: false,
                is_minimized: false,
                style: 0,
                extended_style: 0,
                parent_hwnd: None,
                process_start_time: None,
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

    let rect = unsafe { get_window_rect(hwnd) };
    let visible = unsafe { IsWindowVisible(hwnd) }.into();
    
    let style = unsafe { GetWindowLongW(hwnd, GWL_STYLE) as u64 };
    let extended_style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) as u64 };
    
    let is_maximized = style as u32 & SW_MAXIMIZE.0 as u32 != 0;
    let is_minimized = style as u32 & SW_MINIMIZE.0 as u32 != 0;
    
    let parent_hwnd = unsafe {
        let parent = GetParent(hwnd);
        if parent.is_err() || parent.as_ref().unwrap().is_invalid() {
            None
        } else {
            Some(parent.unwrap().0 as u64)
        }
    };
    
    let process_start_time = unsafe {
        use windows::Win32::System::Threading::PROCESS_VM_READ;
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if process_handle.is_err() {
            None
        } else {
            let mut create_time = FILETIME::default();
            let mut exit_time = FILETIME::default();
            let mut kernel_time = FILETIME::default();
            let mut user_time = FILETIME::default();
            if GetProcessTimes(
                process_handle.unwrap(),
                &mut create_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time
            ).is_ok() {
                let time = ((create_time.dwHighDateTime as u64) << 32) | (create_time.dwLowDateTime as u64);
                Some(time)
            } else {
                None
            }
        }
    };

    Some(WindowInfo { 
        title, 
        exe_path, 
        pid,
        hwnd: hwnd.0 as u64,
        visible,
        position: (rect.left, rect.top),
        size: ((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
        is_maximized,
        is_minimized,
        style,
        extended_style,
        parent_hwnd,
        process_start_time,
    })
}

#[cfg(not(windows))]
#[tauri::command]
pub fn get_foreground_window_info() -> Option<WindowInfo> {
    None
}
