#![no_main]
#![no_std]

use core::marker::PhantomData;

use panic_semihosting as _;
use lpc55_pac::{NVIC, interrupt};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    NVIC::pend(interrupt::ADC0);
    loop {}
}

#[interrupt]
fn ADC0() {
    hprintln!("Goodbye!").unwrap();
}