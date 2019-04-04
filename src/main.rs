#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..30 {
        println!("Hello, World{} Line {}", "!", i);
    }
    panic!("Panicked at thee end");
    loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop{}
}