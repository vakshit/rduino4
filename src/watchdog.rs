use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {
    stctrlh: u16,    stctrll: u16,
    tovalh: u16,   tovall: u16,
    winh: u16,   winl: u16,
    refresh: u16,  unlock: u16,
    tmrouth: u16,  tmroutl: u16,
    rstcnt: u16,  presc: u16
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        // Returns the reference to the structure's first register
        &mut *(0x40052000 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            Watchdog *ptr= new();
            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            core::ptr::write_volatile(&mut self.unlock, 0xD928);
            __nop();
            let mut x = core::ptr::read_volatile(&mut self.stctrlh);
            core::ptr::write_volatile(&mut self.stctrlh, x & 0);
        }
    }
}
