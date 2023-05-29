#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use mos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world! {}", "!");

    mos::init();

    #[cfg(test)]
    test_main();

    println!("It dit not crash!");
    mos::hlt_loop();
}
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mos::test_panic_handler(info)
}
