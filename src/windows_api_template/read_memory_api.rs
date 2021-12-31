use winapi::um::memoryapi::ReadProcessMemory;
use winapi::shared::ntdef::{HANDLE};
use std::io::Error;

/**
通过地址加偏移的方式读取数据， vec的第一个数值需要 模块地址加偏移
其他数值都是偏移。泛型T是最后的物理地址要读取的数据大小对应的数据类型
u32 就是的读4字节数据。 u8 就是读单字节数据
返回值 T:读取的数据 usize:最后的物理地址
*/
pub fn read_memory_by_vec<T>(process_handle: HANDLE, address_vec: &Vec<usize>) -> Result<(T, usize), String> where
    T: Sized + Default {
    if !(address_vec.len() > 0) {
        return Result::Err(String::from("addressVec不能为空!"));
    }
    let mut result: T = Default::default();
    let mut index = 0;
    let mut memory_value: u32 = 0;
    let mut read_address: usize = 0;
    while index < address_vec.len() {
        read_address = address_vec.get(index).unwrap() + memory_value as usize;
        if index == address_vec.len() - 1 {
            result = read(process_handle, read_address).unwrap();
            break;
        }
        memory_value = read(process_handle, read_address).unwrap();
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