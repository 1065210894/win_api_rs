mod windows_api_template;
mod utils;
mod macros;

use libloading::Library;
use windows_api_template::{read_memory_api, init_api, write_memory_api};
use crate::init_api::loop_module;

fn main() {

    unsafe {
        let lib = libloading::Library::new("C:\\Users\\Administrator\\Desktop\\leaker.dll").unwrap();
    }

    // let process_data = init_api::get_process_data_by_window_name("新建 文本文档.txt - 记事本");
    // println!("{:#?}", process_data);
    // loop_module(process_data.1)


    // let process_data = init_api::get_process_data_by_window_name("植物大战僵尸中文版");
    // let module_address = init_api::get_module_address(process_data.1, "game.exe");
    // let sun_address = vec![module_address + 0x2a9ec0, 0x768, 0x5560];
    // let value = read_memory_api::read_memory_by_vec::<u32>(process_data.0, &sun_address).unwrap();
    // println!("阳光值:{} 地址:0x{:X}", value.0, value.1);
    // let ret = write_memory_api::write_process_memory(process_data.0, &sun_address, 10000);
    // print!("写入结果:{}", ret);
}



