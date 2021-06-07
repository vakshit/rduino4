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

## Disabling the Watchdog

The first bit of hardware setup we’ll do is disabling the watchdog. The watchdog’s control is done through a series of 12 16-bit registers at address `0x40052000`. This can be represented in Rust as a packed structure.

```rust
#[repr(C,packed)]
pub struct Watchdog {
    // Complete here
}
```

We’ll add this struct to a new file - `src/watchdog.rs`. The fields of this struct use the same names that the manufacturer does for these registers. They’re hard to read here, but being consistent makes searching for their documentation much easier.

Once we have a struct representing the hardware, we need to build our functions to access it safely. To design this abstraction, we need to think about the invariants of accessing these registers. An invariant is any rule or condition that our unsafe code must take into account, in order for it to be safely callable by safe code. Fortunately the watchdog is pretty simple - it looks just like a struct in memory, and can be treated as such. The biggest invariants here are Rust’s rules about reference aliasing. There can only be one mutable reference to the watchdog struct.

For now, we will say that acquiring a reference to the watchdog is an unsafe operation. This puts the responsibility on the calling code to verify there is only one mutable reference. Once we have that reference, all the functions to update the watchdog will be safe - after all, we’re just changing some fields in memory.

In reality, using the watchdog to its full potential could introduce additional invariants. For example, requiring that a certain value be written to a watchdog register during your main loop. This is not a memory safety issue, and thus strictly falls outside of Rust’s idea of safety. It could cause correctness issues, though, and good API design will try to minimize correctness errors - even if they’re technically “safe”.

The watchdog’s implementation looks like this. Note that new is unsafe, but disable is safe.

```rust
use core::arch::arm::__NOP;

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
