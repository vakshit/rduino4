use core;

#[derive(Clone, Copy)]
pub enum Clock {
    // We have a enum which displays the clock.
    // We consider only PortC in our assignment
    PortC,
}

#[repr(C, packed)]
pub struct Sim {
    // This is the structure containing 22 registers of 32 bit which are resposible for controlling the SIM module.
    // There are various memory location gaps in the SIM registers unlike the WatchDog which had contiguous storage.
    // To deal with that we would apply memory padding. 
    // [ in C the concept is that each memory unit starts from a location whose gap from the start is divisible by the sizeof(data) ]
    // So here in RUST we would be using empty slots [ empty1,empty2,empty3,empty4 to represent storage places].
    sopt1: u32,  sopt1cfg: u32,
    empty1: [u32,1023],  // First empty pad slot  
    sopt2: u32,  
    empty2: u32,  // Second empty pad slot 
    sopt4: u32,  sopt5: u32,  
    empty3: u32,  // Third empty pad slot
    sopt7: u32, 
    empty4: u16,  // Fourth empty pad slot
    sdid: u32,  scgc1: u32,  scgc2: u32,
    scgc3: u32,  scgc4: u32,  scgc5: u32,
    scgc6: u32,  scgc7: u32,  clkdiv1: u32,
    clkdiv2: u32,  fcfg1: u32,  fcfg2: u32,
    uidh: u32,  uidmh: u32,  uidml: u32,
    uidl: u32
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        // Creates a reference to the mutable structure to control the hardware of SIM.
        // Using this we would implement the clock gating feature in our code.
        &mut *(0x40047000 as *mut Sim)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    // According to the Teensy Manual, the last(most unsignificant bit) of the scgc register
                    // denotes the status of the Clock Gating feature.
                    // So we need to change that from 0 to 1 to enable clock gating feature.
                    let mut scgc = core::ptr::read_volatile(&self.scgc5);
                    // Could also be done by BITWISE move-left operator.
                    // [ scgc = scgc | (1<<11) ]
                    scgc = scgc | 0x00000900;
                    // [ core::ptr::write_volatile(&mut self.scgc5, 0x00040982); ]
                    // Not confirm whether the scgc register could be wriiten like this or not ?? ( Should Work )
                    core::ptr::write_volatile(&mut self.scgc5, scgc);
                }
            }
        }
    }
}
