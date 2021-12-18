use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use std::vec;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::{HANDLE, NULL};
use winapi::shared::minwindef::{LPVOID, LPCVOID, BOOL, DWORD, LPDWORD};
use winapi::shared::basetsd::SIZE_T;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::{OpenProcess, PROCESS_ALL_ACCESS};
use winapi::um::winuser::{MessageBoxW, MB_OK, FindWindowW, GetWindowThreadProcessId};

fn main() {
    let sun_count = vec![0x400000 + 0x2a9ec0, 0x768, 0x5560];
    let process_data = get_process_data_by_window_name("植物大战僵尸中文版");
    let value: (u8, usize) = read_memory_by_vec(process_data.0, &sun_count).unwrap();
    println!("最总值:{} 地址:0x{:X}", value.0, value.1);
}

fn read_memory_by_vec<T>(processHandle: HANDLE, addressVec: &Vec<usize>) -> Result<(T, usize), String> where
    T: Sized + Default {
    if !(addressVec.len() > 0) {
        return Result::Err(String::from("addressVec不能为空!"));
    }
    let mut result: T = Default::default();
    let mut index = 0;
    let mut memory_value: u32 = 0;
    let mut read_address: usize = 0;
    while index < addressVec.len() {
        read_address = addressVec.get(index).unwrap() + memory_value as usize;
        if index == addressVec.len() - 1 {
            result = read(processHandle, read_address).unwrap();
            break;
        }
        memory_value = read(processHandle, read_address).unwrap();
        index = index + 1;
    }
    return Ok((result, read_address));
}

/**
读取当内存地址：读取大小根据传入的类型判断
 */
pub fn read<T>(handle: HANDLE, address: usize) -> Result<T, Error>
    where
        T: Sized + Default,
{
    let mut buf: T = Default::default();
    let len_required = std::mem::size_of::<T>();
    let mut len_actual: usize = 0;

    let success = unsafe {
        ReadProcessMemory(
            handle,
            address as _,
            &mut buf as *mut _ as _,
            len_required,
            &mut len_actual,
        )
    };

    if success == 0 || len_actual != len_required {
        Err(Error::last_os_error())
    } else {
        Ok(buf)
    }
}

/**
读取字符串 utf8 [u8] utf16 [u16]
 */
pub fn read_slice<T>(handle: HANDLE, address: usize, buf: &mut [T]) -> Result<(), Error>
    where
        T: Sized,
{
    let len_required = buf.len() * std::mem::size_of::<T>();
    let mut len_actual: usize = 0;

    let success = unsafe {
        ReadProcessMemory(
            handle,
            address as _,
            buf as *mut _ as _,
            len_required,
            &mut len_actual,
        )
    };

    if success == 0 || len_actual != len_required {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

/**
读取内存字节数组
 */
pub fn read_buf(handle: HANDLE, address: usize, length: usize) -> Result<Vec<u8>, Error> {
    let mut result: Vec<u8> = Vec::with_capacity(length);
    unsafe { result.set_len(length) };

    read_slice(handle, address, &mut result[..])?;

    Ok(result)
}

/**
通过窗口名称获取窗口进程句柄与Id
@param: 窗口名称
@return: 窗口的进程句柄,窗口的进程Id
 */
fn get_process_data_by_window_name(window_name: &str) -> (HANDLE, usize) {
    unsafe {
        let window_handle = FindWindowW(null_mut(), get_string16_ptr(window_name));
        let mut win_pid: u32 = 0;
        GetWindowThreadProcessId(window_handle, &mut win_pid as *mut u32 as LPDWORD);
        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false as BOOL, win_pid as DWORD);
        return (process_handle, win_pid as usize);
    }
}

/**
将字符串变成 unicode编码 双字节数组 并获取指针
 */
fn get_string16_ptr(str: &str) -> *const u16 {
    let wide: Vec<u16> = OsStr::new(str).encode_wide().chain(once(0)).collect();
    wide.as_ptr()
}