#[cfg(any(target_arch = "x86", target_arch = "x86_64"))] //编译的cpu架构
pub fn hello_asm() {
    unsafe {
        let result: i32;
        unsafe {
            llvm_asm!("mov eax, 2" : "={eax}"(result) : : : "intel")
        }
        println!("eax is currently {}", result);
    }
}