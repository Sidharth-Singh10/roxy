#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
use core::panic::PanicInfo;


static HELLO: &[u8] = b"Hello World!";  //byte string 
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;// VGA text mode ka address

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            //Writes the character byte at position i * 2. 
            //Each character cell in VGA text mode consists of two bytes: one for the character and one for the attribute (color).
            *vga_buffer.offset(i as isize * 2) = byte;
            //Writes the color attribute byte next to the character byte
            //0xb represents a light cyan foreground color on a black background.
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

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
