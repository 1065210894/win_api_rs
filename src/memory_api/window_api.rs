use crate::utils::string_utils;
use core::ptr::null_mut;
use winapi::shared::minwindef;
use winapi::um::winuser::{FindWindowW, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::{OpenProcess, PROCESS_ALL_ACCESS};
use winapi::shared::ntdef::{HANDLE};


/**
通过窗口名称获取窗口进程句柄与Id
@param: 窗口名称
@return: 窗口的进程句柄,窗口的进程Id
 */
pub fn get_process_data_by_window_name(window_name: &str) -> (HANDLE, usize) {
    unsafe {
        let window_handle = FindWindowW(null_mut(), string_utils::get_string16_ptr(window_name));
        let mut win_pid: u32 = 0;
        GetWindowThreadProcessId(window_handle, &mut win_pid as *mut u32 as minwindef::LPDWORD);
        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false as minwindef::BOOL, win_pid as minwindef::DWORD);
        return (process_handle, win_pid as usize);
    }
}