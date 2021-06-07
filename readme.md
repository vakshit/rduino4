# Introduction

The Teensy family is a set inexpensive embedded development boards, originally designed to be programmed using the Arduino environment. The Teensy 3.2 that we’ll be targeting is based on a Freescale (NXP) MK20DX256 ARM Cortex-M4 microcontroller.

This tutorial is written mostly for Linux; specifically Arch. You may have to adjust commands for other OSes, or even for other Linux distros. If anything is broken for you, please feel free to ask on discord.

## A Short Introduction to Embedded Programming

Unlike with typical desktop or server applications, embedded programs do not have an operating system to provide them with hardware control. Instead, they must access the hardware directly. The exact process for hardware control varies depending on the type of processor in use. For the ARM microcontroller that we’re using, we access the hardware through memory mapped registers.

Memory mapping is assigning a special memory address which, when read from or written to, interacts with a hardware device instead of RAM. For example, address `0x4006A007` is the UART Data Register _(see below)_. Writing a byte to this address will cause that data to be sent across the serial port.

Writing to arbitrary memory addresses requires unsafe Rust. One of our goals through this series will be to use Rust’s language features to create safe interfaces for these unsafe memory accesses. If you would have read chapter 19 by now, unsafe should be fammiliar to you. Please refer to the resources for understanding about some embedded programming terms. **Note that we only need a basic idea about them and not the full knowledge**.`

| Topic                           | Resource                                                                                                                       | Comments |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | -------- |
| Interrupts                      | https://en.wikipedia.org/wiki/Interrupt                                                                                        |          |
| Memory-Mapped I/O               | https://en.wikipedia.org/wiki/Memory-mapped_I/O                                                                                |          |
| SPI, UART, RS232, USB, I2C, TTL | https://electronics.stackexchange.com/questions/37814/usart-uart-rs232-usb-spi-i2c-ttl-etc-what-are-all-of-these-and-how-do-th |          |
| Registers                       | https://en.wikipedia.org/wiki/Processor_register                                                                               |          |
| WatchDog                        | https://os.mbed.com/cookbook/WatchDog-Timer                                                                                    |          |

## Development Environment

Currently, embedded development requires the use of nightly Rust to be practical. While many things can now be done with stable rust, we will still need a nightly version to access some specific hardware instructions. We’ll use Rustup to install nightly Rust.

```bash
rustup toolchain install nightly
rustup component add --target thumbv7em-none-eabi rust-std --toolchain=nightly
```

We need to add the appropriate stdlib for the architecture we’re targeting. For the Teensy 3.2, this is `thumbv7em-none-eabi`. This provides the core crate that our embedded application will be linked against.

Modern nightly versions of rust provide `lld`, the LLVM linker. However, we still require binutils in order to convert our binary to a format which can be loaded onto the teensy. For debian, we install binutils like so:

```
sudo apt-get update && apt-get upgrade
sudo apt-get install build-essential
```

## Code Overview

### Bootup Sequence

The MK20DX256 starts up by loading an initial stack pointer and reset vector from the beginning of flash memory. The reset vector is the equivalent of main in a normal desktop application - it is the first bit of our code that will execute.

Once our main function has control, it will have to perform some basic hardware setup - disabling the watchdog and enabling the clock gate for any peripherals that the application needs.

The watchdog is a piece of hardware which will reset the microcontroller unless the running application “checks in” in a certain interval. It’s designed to restart crashed or hung programs. For our needs in this tutorial it just adds complexity, so we will disable it.

The other part of hardware initialization is clock gating. This term comes from implementation details of how microcontrollers are constructed. You should think of a clock gate as an on/off switch for a piece of functionality. As we progress, we will need to enable the clocks for a number of hardware features.

### Application Setup

We’ll start by creating a new application with cargo, and setting it to use nightly Rust.

```
$ cargo new --bin teensy
$ cd teensy
$ rustup override set nightly
```

The first thing to do is make our program embedded-friendly. There are a few major changes to src/main.rs that we’ll need to make. Here’s the new code, with explanations below:

```rust
#![feature(stdsimd)]
#![no_std]
#![no_main]

#[no_mangle]
pub extern fn main() {
loop{}
}
```

The first line enables the use of intrinsics, and is the reason we need nightly Rust. The next two lines actually disable features of the Rust environment - the standard library, and the main wrapper. The Rust standard library relies on a full operating system, and can’t typically be used for embedded development. Instead, we will have access to libcore, which is the subset of std that is available without an OS. Similarly, the main wrapper is used for application setup tasks that aren’t necessary in embedded programs.

Lastly, we’ve marked main as an extern function, and added an infinite loop to it. Extern tells the Rust compiler that this function follows the C calling convention. The details of what this does vary by target, and are beyond the scope of this post. The important effect of the change is that it’s now safe to use main as our reset vector. Adding the infinite loop ensures that main will never return. There’s no code for main to return to in this embedded environment.

Language Items
The Rust compiler relies on certain functionality to be defined by the standard library. Unfortunately for us, we just disabled it. This means that we are responsible for providing these features.

For now, the only language feature we’re responsible for is the panic handler. This is the function that gets called to display a message when our code panics. We will eventually want to pass these messages along to the user, but initially we will ignore them and hang the program.

```rust
#[panic_handler]
fn teensy_panic(\_pi: &core::panic::PanicInfo) -> ! {
// Complete code here
}
```

### Static Data

There are two arrays of data the the hardware expects. The first is the interrupt table. This contains the initial stack pointer and reset vector that was mentioned earlier. The second is the flash configuration. This is a block of 16 bytes which control how the flash can be read and written. The Teensy bootloader makes assumptions about these values, so we will use the same set of bytes as the [Teensy Arduino tooling](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx128.c#L654-L658). Specifically, we disable all flash security through the FSEC field, and tell the processor to boot into high-power mode with FOPT.

```rust
extern {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern fn(); 2] = [
    _stack_top,
    main,
];

#[link_section = ".flashconfig"]
#[no_mangle]
// Complete the code below
// pub static _FLASHCONFIG:
```

We will use the link_section attributes in a minute to control where in the flash memory these arrays end up. The no_mangle attribute is needed to tell Rust that these arrays have special meaning at link time. Without it, the data will not appear in our final executable.

\_stack_top is not really a function. It is a memory address representing the initial stack pointer. We pretend that it is a function so that our \_VECTORS array is easier to write. Fortunately calling it from our own code is unsafe, so we can be pretty sure that only the hardware will read these values.

## Compiling and Linking

Our program now contains the important data tables, as well as a `main` that can be called by the microcontroller. We will now turn our attention to building the project for the Teensy. We’ll use a Makefile to handle the build process. Laying out the code and data in the Teensy’s flash memory is done with a linker script.

## Accessing the hardware

Accessing The Hardware
Our first steps here will be some basic hardware initialization tasks. We’ll build accessors for the watchdog and for the System Integration Module, or SIM. The SIM handles clock gating as well as most other global configuration of the microcontroller. Once we have those in place, we’ll turn to the I/O functions necessary to turn on the LED.

## Registers

Read [here](http://www.mathcs.emory.edu/~cheung/Courses/255/Syllabus/2-C-adv-data/struct-pointer.html) about pointers and structs in C before moving ahead.

Note that a register is nothing but a memory block. Remeber ESC101? Memory blocks can be simply interpreted as integers i.e. a 32 bit integer represents 32 bits of memory. Setting the register to 1 is equivalent to making that integer equal to 1.
A struct is no different from array in C, just take it as named indexes, i.e. instead of `A[i]` which is actually `A + i*sizeof(int)` is instead represented as `A.i` but points to the same memory location. So if we have a series of 2 64 bit integers in C, I can represent them as:

```C
struct register_representation {
    long long int register1;
    long long int register2;
}
```

Now if I already know that such a series of registers exist at a specific point in memory then I can simply assume that a struct exists there, since it is a mere representation. Suppose the manual says that the registers are located at 00-31 and 32-63 (regions of memory). In C we could do as:

```C
register_representation *ptr = 0x00;
// Set the bits of register 1 to 10 and register 2 to 0;
ptr->register1 = 3; // 10 in integer representation is 3
ptr->register2 = 0;
```

## Disabling the Watchdog

The first bit of hardware setup we’ll do is disabling the watchdog. The watchdog’s control is done through a series of 12 16-bit registers at address `0x40052000`. This can be represented in Rust as a packed structure

```rust
#[repr(C,packed)]
pub struct Watchdog {
    stctrlh: u16,
    stctrll: u16,
    // Complete the rest of the registers here.
}
```

We’ll add this struct to a new file - `src/watchdog.rs`. The fields of this struct use the same names that the manufacturer does for these registers. They’re hard to read here, but being consistent makes searching for their documentation much easier.

Once we have a struct representing the hardware, we need to build our functions to access it safely. To design this abstraction, we need to think about the invariants of accessing these registers. An invariant is any rule or condition that our unsafe code must take into account, in order for it to be safely callable by safe code. Fortunately the watchdog is pretty simple - it looks just like a struct in memory, and can be treated as such. The biggest invariants here are Rust’s rules about reference aliasing. There can only be one mutable reference to the watchdog struct.

For now, we will say that acquiring a reference to the watchdog is an unsafe operation. This puts the responsibility on the calling code to verify there is only one mutable reference. Once we have that reference, all the functions to update the watchdog will be safe - after all, we’re just changing some fields in memory.

In reality, using the watchdog to its full potential could introduce additional invariants. For example, requiring that a certain value be written to a watchdog register during your main loop. This is not a memory safety issue, and thus strictly falls outside of Rust’s idea of safety. It could cause correctness issues, though, and good API design will try to minimize correctness errors - even if they’re technically “safe”.

The watchdog’s implementation looks like this. Note that new is unsafe, but disable is safe.

```rust
use core::arch::arm::__nop;

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x40052000 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            // disable the watchodg
        }
    }
}
```

The disable function is following the procedure set forth in the manufacturer’s data sheet. The watchdog is protected against being accidentally disabled by a random write to memory, so our code must “unlock” it first, by writing special values to the unlock register. Once that’s done, we need to wait for the watchdog to actually unlock itself. The `_nop` function tells the processor to briefly do nothing. This introduces our necessary 2-cycle delay. Finally, we read the control register and un-set the “enable” bit.

All of our memory access are volatile. This tells the Rust compiler that the read (or write) has an effect that it can’t see from our program code. In this case, that effect is a hardware access. Without marking our memory accesses volatile, the Rust compiler would be free to say “You never read from unlock, so I will optimize away the unneeded write to it”. This would, naturally, cause our code to fail.

This disable process shows why we must have only one mutable reference to the watchdog. If an interrupt were to occur partway through this function and write to the watchdog, our attempt to disable it would fail. Knowing that an interrupt cannot change watchdog settings gives us confidence that this code will execute as we expect.

### Clock Gating

The other piece of hardware involved in the microcontroller setup is the System Integration Module. We’ll use this to enable the appropriate clock gate to enable our I/O port. Just like the watchdog, the SIM is controlled through a block of memory, which also will be represent as a struct. It has the same basic memory safety rules as the watchdog does, and for now has no extra memory-safety invariants.

There is a potential correctness issue involved with the SIM - it’s possible to use a mutable reference to the SIM to disable a hardware function that another section of code relies on. We can design an API that keeps better track of which functional units are needed, but we will save that for a future post. For now, we’ll just have to trust ourselves.

The complete code for src/sim.rs is here:

```rust
use core;

#[derive(Clone,Copy)]
pub enum Clock {
    PortC,
}

#[repr(C,packed)]
pub struct Sim {
    // Complete code here
    // See section 12.2 of the teensy manual for the register sizes and memory locations.
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
```

The simple match-based clock management we have here would get unwieldy pretty quickly if we intended to use it to manage a large number of hardware functions. We’ll get rid of it when we look in to more robust ways to manage clock gates.

## I/O Ports

With the initial hardware setup out of the way, we can turn our attention to achieving that led blink that we’ve been working towards. We will put a pin into GPIO mode, and use it to turn on the LED. GPIO stands for “General Purpose I/O”. When a pin is in GPIO mode, software has control over the high/low state of an output pin and direct read access to the state of an input pin. This is in contrast to the pin being controlled by a dedicated hardware function, such as a serial port.

Pins are grouped into ports, and all of a pin’s settings are controlled from the port’s register block. This poses a bit of a challenge for us. We’d like each pin to be a self-contained struct, so that ownership of it can be passed from one software module to another, and only the owning module can mutate its pins. This follows Rust’s one-owner rule for pins, but would require that each pin be able to mutate its settings in the Port register block. We all know how Rust feels about shared mutable state.

Fortunately, each pin has a separate control register in the port’s block. That means there’s no actual overlap of memory locations that might be written. We’ll take advantage of this to write some very, very careful unsafe code that allows each pin instance to modify its own control settings.

We’ll start out with a port implementation in src/port.rs.

```rust
use core;

#[derive(Clone,Copy)]
pub enum PortName {
    C
}

#[repr(C,packed)]
pub struct Port {
    // Complete the struct below
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        // Complete the function below.
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        // Given the pin mode as a 32 bit value set the register bytes to the same value for the corresponding pin.
    }
}
```

The set_pin_mode function is responsible for switching a single pin into GPIO (or any other) mode. The only memory it touches is the PCR associated with a single pin, and is unsafe to call. It’s unsafety is because calling it for a pin that you do not own could cause a race condition. An interrupt that changes a PCR between the read and write in this function could have its changes overwritten.

The pin struct is next on our list. A pin is not a reference to any particular register. Instead, it is a concept in our code that represents a piece of a port. It will have a mutable reference to its containing port, as well as an integer representing which index in the PCR array it is associated with.

In order for this mutable port reference to be safe, Pin instances must only call methods of Port that affect the correct PCR. We can’t really enforce this, but to encourage it, Pin’s Port reference will actually be a pointer. This makes it impossible to call Port methods without an unsafe block, and reinforces the peculiarity of this arrangement.

```rust
pub struct Pin {
    port: *mut Port,
    pin: usize
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        // Complete and return a pin struct
    }
}
```

### GPIO and the Bit-Band

There are two ways to access the GPIO registers. The first is through a block of 32-bit registers, associated with a port. Second way will be discussed later.

```rust
#[repr(C,packed)]
struct GpioBitBand {
    // Complete using section 49.2
}
```

This is what we will use to control the GPIO. Just like with Pins and the PCR registers, we will have individual GPIO structures that represent a single GPIO pin. They will ensure safety by only writing to the register words associated with their pin index. Let’s look at all that code now, then walk through it.

```rust
pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            // Return PortName::C if the address matches the starting address of port C as specified in section 11.1.4
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            // Set pin mode to 1 to enable gpio mode.
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
```

The Gpio struct, just like the Port struct, holds a pointer to the shared data block, as well as an index of its pin number. It has two functions: one to set itself as an output, and one to set its output value to high. Thanks to the bit-band, these functions can be implemented with a single write, eliminating the potential race condition that a read-modify-write of a shared memory address would create.

Converting a Pin into a Gpio consumes the Pin. This prevents having more than one reference to a single hardware pin. Getting another copy of a pin from the port is unsafe, so we can be confident that safe code will never make a second copy of a pin that is in use as a GPIO

## Putting it Together

We now have all the pieces for our first program. Going back to the beginning, our application will do the following:

- disable the watchdog
- turn on the clock gate for Port C
- grab pin 5 from that port, and make it a GPIO
- set that GPIO as output and then high to light the LED
- Can you make the led blink periodically?

You are now suppossed to complete main.rs.
