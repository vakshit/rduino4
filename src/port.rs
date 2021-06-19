use core;

#[derive(Clone, Copy)]
pub enum PortName {
    C,
}

#[repr(C, packed)]
pub struct Port {
    // Complete the struct below. See section 11.1.4 of the manual. Note it has continous memory representation of multiple ports but struct should only account for port C i.e. all registers beginning with PORTC_.
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        // Complete the function below. Similar to watchdog. But use a matchcase since we should only return when portname is C. See the address in section 11.1.4.
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        // Given the pin mode as a 32 bit value set the register bytes to the same value for the corresponding pin. See the MUX(10-8) bits in section 11.14.1. We need to set only those bits. Again think of appropriate operations using AND,OR,XOR etc.. There are only 8 possible pin models so mode = 0 to 7. Reject if different.
    }
}

pub struct Pin {
    port: *mut Port,
    pin: usize,
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        // Complete and return a pin struct
    }
}

#[repr(C, packed)]
struct GpioBitBand {
    // Complete using section 49.2
}

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            // Return PortName::C if the address matches the starting address of port C as specified in section 11.1.4. Reject if address is wrong and return error.
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            // Set pin mode to 1 to enable gpio mode (section 11.14.1 MUX bits).
            // Consume the pin into a gpio struct i.e. instantitate a gpio struct using the new function below.
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {-++++
        let gpio = match port {
            PortName::C => 0x43FE1000 as *mut GpioBitband
        };

        // Initialize and return a gpio struct.
    }

    pub fn output(&mut self) {
        unsafe {
            //  WRITE THE  XX register of GPIO to 1 to enable this pin as output type.
            // See section 49.2 of the teensy manual to find out what is XX.
        }
    }

    pub fn high(&mut self) {
        unsafe {
           //  WRITE THE  XX register of GPIO to 1 to set this pin as high.
           // See section 49.2 of the teensy manual to find out what is XX. Please not that it is not PDOR, since PDOR is never directly written.
        }
    }
}


use core;

#[derive(Clone,Copy)]
pub enum PortName {
    C
}

#[repr(C,packed)]
pub struct Port {
    pcr: [u32; 32],
    gpclr: u32,
    gpchr: u32,
    reserved_0: [u8; 24],
    isfr: u32,
}

pub struct Pin {
    port: *mut Port,
    pin: usize
}

#[repr(C,packed)]
struct GpioBitband {
    pdor: [u32; 32],
    psor: [u32; 32],
    pcor: [u32; 32],
    ptor: [u32; 32],
    pdir: [u32; 32],
    pddr: [u32; 32]
}

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        &mut * match name {
            PortName::C => 0x4004B000 as *mut Port
        }
    }

    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin { port: self, pin: p }
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        let mut pcr = core::ptr::read_volatile(&self.pcr[p]);
        pcr &= 0xFFFFF8FF;
        mode &= 0x00000007;
        mode <<= 8;
        pcr |= mode;
        core::ptr::write_volatile(&mut self.pcr[p], pcr);
    }

    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            0x4004B000 => PortName::C,
            _ => unreachable!()
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            PortName::C => 0x43FE1000 as *mut GpioBitband
        };

        Gpio { gpio, pin }
    }

    pub fn output(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).pddr[self.pin], 1);
        }
    }

    pub fn high(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).psor[self.pin], 1);
        }
    }
}
