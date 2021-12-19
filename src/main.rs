mod utils;
mod memory_api;

use memory_api::{read_memory_api, window_api};


fn main() {
    let sun_count = vec![0x400000 + 0x2a9ec0, 0x768, 0x5560];
    let process_data = window_api::get_process_data_by_window_name("植物大战僵尸中文版");
    let value: (u8, usize) = read_memory_api::read_memory_by_vec(process_data.0, &sun_count).unwrap();
    println!("最总值:{} 地址:0x{:X}", value.0, value.1);
}



