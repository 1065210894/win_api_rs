mod utils;
mod memory_api;
mod macros;

use winapi::um::winnt::HANDLE;
use memory_api::{read_memory_api, init_api, write_memory_api};


fn main() {
    let process_data = init_api::get_process_data_by_window_name("植物大战僵尸中文版");
    let module_address = init_api::get_module_address(process_data.1, "game.exe");
    let sun_address = vec![module_address + 0x2a9ec0, 0x768, 0x5560];
    let value: (u32, usize) = read_memory_api::read_memory_by_vec(process_data.0, &sun_address).unwrap();
    println!("最总值:{} 地址:0x{:X}", value.0, value.1);
    let ret = write_memory_api::write_process_memory(process_data.0, &sun_address, 10000);
    print!("写入结果:{}", ret);
}



