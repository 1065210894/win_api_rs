/**
转化无符数据类型的数据为 无符单字节长度的Vec
 */
pub fn usize_conversion_u8_array(conversion_value: usize) -> Vec<u8> {
    let mut size_of: usize = std::mem::size_of_val(&conversion_value);
    let mut result: Vec<u8> = Vec::new();
    let mut index_bit = 0;
    while size_of > 0 {
        size_of = size_of - 1;
        let value = (conversion_value >> index_bit) as u8;
        index_bit = index_bit + 8;
        if value == 0 {
            continue;
        }
        result.push(value);
    }
    return result;
}
/**
转化有符数据类型的数据为无符单字节长度的Vec
 */
pub fn isize_conversion_u8_array(conversion_value: isize) -> Vec<u8> {
    let mut size_of: usize = std::mem::size_of_val(&conversion_value);
    let mut result: Vec<u8> = Vec::new();
    let mut index_bit = 0;
    while size_of > 0 {
        size_of = size_of - 1;
        let value = (conversion_value >> index_bit) as u8;
        index_bit = index_bit + 8;
        if value == 0 {
            continue;
        }
        result.push(value);
    }
    return result;
}