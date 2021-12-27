#![feature(llvm_asm)]

mod power;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[no_mangle]
#[allow(non_snake_case)]
#[warn(unused_attributes)]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    lpv_reserved: LPVOID,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            power::hello_asm();
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    TRUE
}
