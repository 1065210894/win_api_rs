use core::ptr::null_mut;
use winapi::um::winuser::{FindWindowW, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPMODULE, MODULEENTRY32, Module32First, Module32Next, TH32CS_SNAPMODULE32};
use crate::utils::string_utils::get_string16_ptr;
use crate::init_api;
use winapi::shared::{minwindef};
use std::mem::zeroed;
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use winapi::um::handleapi::CloseHandle;


/**
通过窗口名称获取窗口进程句柄与Id
@param: 窗口名称
@return: 窗口的进程句柄,窗口的进程Id
 */
pub fn get_process_data_by_window_name(window_name: &str) -> (HANDLE, usize) {
    unsafe {
        let window_handle = FindWindowW(null_mut(), get_string16_ptr(window_name));
        let mut win_pid: u32 = 0;
        GetWindowThreadProcessId(window_handle, &mut win_pid as *mut u32 as minwindef::LPDWORD);
        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false as minwindef::BOOL, win_pid as minwindef::DWORD);
        return (process_handle, win_pid as usize);
    }
}

/**
打印进程中的模块
 */
pub fn loop_module(process_id: usize) {
    let mut module_iterator = init_api::ModuleIterator::of(process_id as u32);
    loop {
        let module_data = module_iterator.next();
        match module_data {
            None => {
                break;
            }
            Some(value) => {
                println!("{:#?}", value);
            }
        }
    }
}

/// 通过某块名称获取模块地址
pub fn get_module_address(process_id: usize, module_name: &str) -> usize {
    let mut module_iterator = init_api::ModuleIterator::of(process_id as u32);
    let mut ret: usize = 0;
    loop {
        let module_data = module_iterator.next();
        match module_data {
            None => {
                break;
            }
            Some(value) => {
                let name_vec: Vec<&str> = value.exe_path.split("\\").collect();
                if name_vec[name_vec.len() - 1] == module_name && value.module_base > 0 {
                    ret = value.module_base as usize;
                    break;
                }
            }
        }
    }
    return ret;
}

#[allow(missing_debug_implementations)]
pub struct ModuleIterator {
    h_snap: HANDLE,
    entry: MODULEENTRY32,
    end: i32,
}

#[derive(Debug)]
pub struct ModuleEntry {
    /// Base address of module.
    pub module_base: u64,
    /// Size of the module.
    pub module_size: u32,
    /// Path to a module.
    pub exe_path: String,
}

impl Iterator for ModuleIterator {
    type Item = ModuleEntry;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.end == -1 {
                self.end = Module32First(self.h_snap, &mut self.entry);
                Some(ModuleEntry {
                    module_base: 0,
                    exe_path: crate::to_c_str!(self.entry.szExePath).to_string(),
                    module_size: 0,
                })
            } else if self.end != 0 {
                let entry = Some(ModuleEntry {
                    module_base: self.entry.modBaseAddr as u64,
                    exe_path: crate::to_c_str!(self.entry.szExePath).to_string(),
                    module_size: self.entry.modBaseSize,
                });

                self.end = Module32Next(self.h_snap, &mut self.entry);
                entry
            } else {
                CloseHandle(self.h_snap);
                None
            }
        }
    }
}

impl ModuleIterator {
    pub fn of(process_id: u32) -> Self {
        unsafe {
            let mut entry: MODULEENTRY32 = zeroed();
            entry.dwSize = crate::size_of!(@ entry) as u32;

            Self {
                h_snap: CreateToolhelp32Snapshot(
                    TH32CS_SNAPMODULE32 | TH32CS_SNAPMODULE,
                    process_id,
                ),
                end: -1,
                entry,
            }
        }
    }
}