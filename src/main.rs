#![feature(proc_macro)] // <- IMPORTANT! Feature gate for procedural macros
#![feature(const_unsafe_cell_new)]
#![no_std]

extern crate f3;
extern crate cortex_m_rtfm as rtfm; // <- this rename is required
extern crate cortex_m_semihosting as semihosting;

use core::fmt::Write;
use rtfm::app; // <- this is a procedural macro
use semihosting::hio;

// This macro expands into the `main` function
app! {
    // this is a path to a _device_ crate, a crate generated using svd2rust
    device: f3::stm32f30x,
}

// INITIALIZATION
fn init(_p: init::Peripherals) {
    // Nothing to initialize in this example ...
}

// IDLE LOOP
fn idle() -> ! {
    writeln!(hio::hstdout().unwrap(), "Hello, world!").unwrap();

    // Go to sleep
    loop {
        rtfm::wfi();
    }
} 