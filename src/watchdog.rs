use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {
    // 12 registers of 16 bit each to control the WatchDog. 
    // The standard naming procedure is used to name all the registers.    
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
            // New instance of the Structure WatchDog created for implementation.
            Watchdog *ptr= new();
            // Unlock register is written with a specific sequence of C250 followed by D928
            // This sequence enables us to write into the writable bits of registers in the struct for some time.
            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            core::ptr::write_volatile(&mut self.unlock, 0xD928);
            // As soon as the unlock sequence is carried we need to wait for 2 clock cycles.
            // This is the time the WatchDog takes to unlock itself.
            __nop();
            __nop();
            // Now we need to read the stctrlh register and check from the manual
            // We get that the last(most unsignifant bit) is the Enable bit for the WatchDog timer.
            // So just use some logic(XOR,OR,AND) and converted only the last bit to 0 from 1.
            let mut x = core::ptr::read_volatile(&mut self.stctrlh);
            // Here 0 means only the last bit is zero, rest are not considered. So the last bit is converted to 0 by this.
            // However we might also use direct writing of the register to 0.
            //  [ core::ptr::write_volatile(&mut self.stctrlh, 0x01D2); ]
            core::ptr::write_volatile(&mut self.stctrlh, x & 0);
        }
    }
}
