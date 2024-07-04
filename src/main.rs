#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
use core::panic::PanicInfo;
mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";  //byte string 
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    

    loop {}
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // "!" is never type, since function should never return anything
    loop {}
}

#[cfg(test)]
fn test_runner(_test: &[&dyn Fn()]) {
    loop {}
}
