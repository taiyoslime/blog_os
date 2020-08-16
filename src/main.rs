#![no_std]
#![no_main]

extern crate rlibc;
use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("{}", HELLO);
	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}