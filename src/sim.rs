use core;

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

#[repr(C, packed)]
pub struct Sim {
    sopt1: u32,  sopt1cfg: u32,
    empty1: [u32,1023],  
    sopt2: u32,  empty2: u32,
    sopt4: u32,  sopt5: u32,  empty3: u32,
    sopt7: u32, empty4: u16,
    sdid: u32,  scgc1: u32,  scgc2: u32,
    scgc3: u32,  scgc4: u32,  scgc5: u32,
    scgc6: u32,  scgc7: u32,  clkdiv1: u32,
    clkdiv2: u32,  fcfg1: u32,  fcfg2: u32,
    uidh: u32,  uidmh: u32,  uidml: u32,
    uidl: u32
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
