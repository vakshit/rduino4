use core;

#[derive(Clone, Copy)]
pub enum PortName {
    C,
}

#[repr(C, packed)]
pub struct Port {
    // Complete the struct below. See section 11.1.4 of the manual. Note it has continous memory representation of multiple ports but struct should only account for port C i.e. all registers beginning with PORTC_.
    pcr: [u32; 32],
    gpclr: u32,
    gpchr: u32,
    dummy: [u8; 24], // how to decide that its 1 byte long..??
    isfr: u32,
    
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        // Complete the function below. Similar to watchdog. But use a matchcase since we should only return when portname is C. See the address in section 11.1.4.
        &mut *match name {
            PortName::C => 0x4004B000 as *mut Port
        } 
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        // Given the pin mode as a 32 bit value set the register bytes to the same value for the corresponding pin. See the MUX(10-8) bits in section 11.14.1. We need to set only those bits. Again think of appropriate operations using AND,OR,XOR etc.. There are only 8 possible pin models so mode = 0 to 7. Reject if different.
        if mode >7 || mode < 0 {
            return
        }
        let mut pcr = core::ptr::read_volatile(&self.pcr[p]);
        pcr &= 0xFFFFF8FF;
        mode <<=8;
        pcr|=mode;
        core::ptr::write_volatile(&mut self.pcr[p], pcr);
    }
}

pub struct Pin {
    port: *mut Port,
    pin: usize,
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        // Complete and return a pin struct
        Pin {port: self, pin: p}
    }
}

#[repr(C, packed)]
pub struct GpioBitBand {
    // Complete using section 49.2
    pdor: u32,
    psor: u32,
    pcor: u32,
    ptor: u32,
    pdir: u32,
    pddr: u32,
}

pub struct Gpio {
    gpio: *mut GpioBitBand,
    pin: usize,
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            // Return PortName::C if the address matches the starting address of port C as specified in section 11.1.4. Reject if address is wrong and return error.
            0x4004B000 => PortName::C,
            _ => unreachable!
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            // Set pin mode to 1 to enable gpio mode (section 11.14.1 MUX bits).
            // Consume the pin into a gpio struct i.e. instantitate a gpio struct using the new function below.
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio
        }

    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            PortName::C => 0x43FE1000 as *mut GpioBitBand
        };

        // Initialize and return a gpio struct.
    }

    pub fn output(&mut self) {
        unsafe {
            //  WRITE THE  XX register of GPIO to 1 to enable this pin as output type.
            // See section 49.2 of the teensy manual to find out what is XX.
           let mut ctrl_output = core::ptr::read_volatile(&self.pdor);
           core::ptr::write_volatile(&mut self.pdor, ctrl_output |(1<<31));

        }
    }

    pub fn high(&mut self) {
        unsafe {
           //  WRITE THE  XX register of GPIO to 1 to set this pin as high.
           // See section 49.2 of the teensy manual to find out what is XX. Please not that it is not PDOR, since PDOR is never directly written.
        let mut ctrl_disable = core::ptr::read_volatile(&self.psor);
        core::ptr::write_volatile(&mut self.psor, ctrl_disable | (1<<31) );
        }
    }
}
