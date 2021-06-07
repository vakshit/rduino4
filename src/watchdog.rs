use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {
    stctrlh: u16,
    stctrll: u16,
    // Complete the rest of the registers here using section 23.7 of the manual.
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        // You can see the starting address in section 23.7 of the manual i.e. 4005_2000.
        &mut *(0x40052000 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            // Disable the watchdog. This has 2 parts, unlocking the watchdog for modification and then disabling the watchdog.
            // See section 23.3.1 for unlocking the watchdog. Ignore point 3 there.
            // To disable the watchdog, see section 23.7.1 and scroll down to the last item in the table the 0th bit to understand how to disable the watchdog. This makes it clear that your operation should only change the 0th bit in the 16-bit value, keeping others same. How would you do that? (Think XOR,AND,OR etc.)
        }
    }
}
