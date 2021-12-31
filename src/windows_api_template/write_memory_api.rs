use std::io::Error;
use std::panic;
use winapi::um::winnt::HANDLE;
use std::{io, os::windows::io::AsRawHandle};
use std::mem::zeroed;
use winapi::shared::minwindef::FALSE;
use crate::read_memory_api::read_memory_by_vec;
use winapi::um::memoryapi::WriteProcessMemory;
use crate::utils::vec_utils;

/**
写入数据
address_vec：[模块地址+偏移， 偏移， 偏移]
write_value:要写入的数据， 无需关心写的是多少字节的数据，
在调用该方法的时候可以将 u32 as usize 传给 write_value 实际写入的依然是4字节数据（根据数据真实长度写入）。
*/
pub fn write_process_memory(process_handle: HANDLE, address_vec: &Vec<usize>, write_value: usize) -> bool {
    let mut value = read_memory_by_vec::<u8>(process_handle, address_vec).unwrap();
    unsafe {
        let write_value_vec = vec_utils::usize_conversion_u8_array(write_value);
        let mut write_struct = WriteProcessMemory {
            process: &process_handle,
            base_address: value.1,
            number_of_bytes_written: 0,
            buffer: write_value_vec.as_slice(),
        };
        return write_struct.write();
    }
}


/// Write process memory.
pub struct WriteProcessMemory<'a> {
    process: &'a HANDLE,
    base_address: usize,
    buffer: &'a [u8],
    number_of_bytes_written: usize,
}

impl WriteProcessMemory<'_> {
    fn write(&mut self) -> bool {
        let lp_base_address = self.base_address as _;
        let lp_buffer = self.buffer.as_ptr() as _;
        let n_size = self.buffer.len();
        let lp_number_of_bytes_written = &mut self.number_of_bytes_written;
        let write_ret = unsafe {
            WriteProcessMemory(
                *self.process,
                lp_base_address,
                lp_buffer,
                n_size,
                lp_number_of_bytes_written,
            )
        };

        if write_ret == FALSE {
            println!("写入地址失败:{:x}, {}", self.base_address, Error::last_os_error());
            false
        } else {
            true
        }
    }
}
