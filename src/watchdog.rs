use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {}

    pub fn disable(&mut self) {
        unsafe {}
    }
}
