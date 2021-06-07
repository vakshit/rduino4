use core;

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

#[repr(C, packed)]
pub struct Sim {
    // Complete code here
// See section 12.2 of the teensy manual for the register sizes and memory locations and do similar to the watchdog struct.
// Note that there are some empty bits between some registers and they are not continous, how do we resolve that ? Padding, eh ?
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        // Complete code here (similar to watchdog), see memory location from section 12.2
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    // Use the teensy manual to find out which register controls Port C. Then implement this function to enable port C. Scroll through section 12.2 to find which bit of which register needs to be changed to enable clock gate to Port C. Note that all other bits of that register must remain unchanged.
                }
            }
        }
    }
}
