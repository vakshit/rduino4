use core;

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

#[repr(C, packed)]
pub struct Sim {
    /** Complete code here
     *  See section 12.2 of the teensy manual for the register sizes and memory locations.
     *  */ 
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        // Complete code here (similar to watchdog)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    // Use the teensy manual to find out which register controls PortC and what values neeeds to be written to that register to enable port C. Then implement this function to enable port C.
                }
            }
        }
    }
}
