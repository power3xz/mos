#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unimplemented!();
}

#[panic_handler]
fn panic_info(info: &PanicInfo) -> ! {
    mos::test_panic_handler(info)
}
