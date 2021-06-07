#![feature(stdsimd)]
#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::empty_loop)]

mod port;
mod sim;
mod watchdog;

extern "C" fn main() {
    loop {}
}

extern "C" {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];

const FSEC: u8 = 0xDE;
const FOPT: u8 = 0xF9;

#[link_section = ".flashconfig"]
#[no_mangle]
// Complete the code below
// pub static _FLASHCONFIG:
#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {}
}
