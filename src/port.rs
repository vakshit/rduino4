use core;

#[derive(Clone,Copy)]
pub enum PortName {
    // We will control all the important features of portC.
    // So we will convert a pin of PortC into a GPIO pin.
    C
}

#[repr(C,packed)]
pub struct Port {
    // Registers to be used for PortC of the port module.
    // pcr are taken as the array and they would have respective serial numbers as well
    // We could have enumerated them as pcr0,pcr1... as well but that would have been
    // lengthy unnecessarily.

    // One PCR controls a whole PIN of the PORT-C.
    pcr: [u32; 32],   gpclr: u32,
    gpchr: u32,    
    // Empty space to take care of the Padding in the program.
    empty: [u32; 6],
    isfr: u32,
}

pub struct Pin {
    // This structure is used to control a single pin of each port.
    // The data required for each pin is the structure to it's Port and location of the pin.
    port: *mut Port,   
    pin: usize    // The size of a pointer so 64 bits on a 64 bit system
}

#[repr(C,packed)]
struct GpioBitband {
    // This is the structure used for controlling the GPIO pins
    // We have to take into consideration that we have the whole PORT-C which has 32 pins (according to struct port)
    // So we will be making GpioBitBand structure for the whole port and control the registers corresponding to the pin
    // which is converted to GPIO mode.
    pdor: [u32; 32],   psor: [u32; 32],
    pcor: [u32; 32],   ptor: [u32; 32],
    pdir: [u32; 32],   pddr: [u32; 32]
}

pub struct Gpio {
    // As we have to control only one pin out of the whole structure we would be controlling the pin
    // using structure GPIO.
    // This has the structure GpioBitBand reference which shows data for the whole port and the address of the GPIO pin.
    gpio: *mut GpioBitband,   
    pin: usize // Index of the pin converted to GPIO mode.
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        // We use the match function for enums so that we can return a reference to the
        // structure containing the registers in order.
        // Return is given only when it is a Port-C 
        &mut *(match name {
            PortName::C => 0x4004B000 as *mut Port
        })
    }

    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        // This will take the address of the pin which is to be controlled in GPIO mode.
        // This should be a pin of Port-C.
        // And then it returns a structure to the pin which could be used to control the pin.
        // As we use it for some port, if Port-C is the calling structure than the pin of port-C is returned.
        Pin { port: self, pin: p }
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        // This will be called by a struct which contains the information about the Port we are working on.
        // The function also requires the address of the pin whose mode is to be set to GPIO.
        // Mode is the current PCR of the pin whose address is given to us.

        // it might be done this way -- 
        // let mut pcr = core::ptr::read_volatile(&self.pcr[p]);
        // pcr = pcr & 0xFFFFF8FF
        // pcr = pcr | 0x00000100
        // core::ptr::write_volatile(&mut self.pcr[p], pcr);

        // there is a doubt in this code as of now lets see
        // Ok after reading the latter part of the code it is clear that from this code basically we can
        // put the last 3 bits of mode into the MUX bits of "pcr" so to make it a GPIO pin use mode = 1
        // The same code can be used for all configurations of the pin so it is a good code.
        let mut pcr = core::ptr::read_volatile(&self.pcr[p]);
        pcr = pcr & 0xFFFFF8FF;
        mode = mode & 0x00000007;
        mode = mode << 8;
        pcr = pcr | mode;
        core::ptr::write_volatile(&mut self.pcr[p], pcr);
    }

    pub fn name(&self) -> PortName {
        // This function helps us in identifying the port of the instance given to us.
        // We take the address of the structure given to us and check it against the reference of the Port-C.

        // It returns the address of the start of given port structure.
        // it maybe   let addr = (self as *const Port) as u64;  NOPES it would be 8*4=32
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
            // This function is to be called by a Pin and it will set the pin in GPIO mode and return the
            // GPIO structure which let's one control the pin through it in GPIO mode.
            // Creates a Mutable static reference to port structure.
            let port = &mut *self.port;
            // The port structure is passed through set_pin_mode function and it passes the pin value
            // of the pin which called the make_gpio function
            // mode = 1 enables it to be a GPIO pin as 1 would end like "001"
            port.set_pin_mode(self.pin, 1);
            // A GPIO structure with the calling pin's type and pin index sent as parameter
            Gpio::new(port.name(), self.pin)
        }
    }
}

impl Gpio {
    // pub unsafe fn new(port: PortName,pin: usize) -> &'static mut GpioBitBand {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            // While creating a new instance for the GPIO pin controling structure
            // If the pin is of Port-C then we return a GpioBitBand structure with the suitable start address.

            // PortName::C => 0x400FF080 as *mut GpioBitBand
            PortName::C => 0x43FE1000 as *mut GpioBitband
        };

        Gpio { gpio, pin }
    }

    pub fn output(&mut self) {
        unsafe {
            // Make the specified pin in output mode so convert the bit like 0 -> 1
            core::ptr::write_volatile(&mut (*self.gpio).pddr[self.pin], 1);
        }
    }

    pub fn high(&mut self) {
        unsafe {
            // PSOR -> Port Set Output Register is for setting port output as 1 (HIGH)
            core::ptr::write_volatile(&mut (*self.gpio).psor[self.pin], 1);
        }
    }
}
