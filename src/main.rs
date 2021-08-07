#![no_main]
#![no_std]
// #![feature(asm)]

use cortex_m::interrupt::InterruptNumber;
use panic_semihosting as _;
use lpc55_hal::raw::{interrupt, NVIC};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {

    // disable interrupt to avoid potential conflicts with initilization
    cortex_m::interrupt::disable();
    
    // ADC0 is pending even without calling pend() if interrupts are not disabled first.
    // hprintln!("Why is this pending before we ask it to? pending: {}", NVIC::is_pending(interrupt::ADC0)).unwrap();

    // this does not seem to affect anything different then using cortex_m::interrupt::enable()
    // unsafe { 
    //     asm!(
    //         "cpsie i
    //           isb"
    //     ) // as per ARM spec the ISB ensures triggering pending interrupts
    //  }

    // prove code is running first
    hprintln!("Hello, world!").unwrap();

    let mut per = lpc55_hal::new();

    // enable ADC to ensure ADC0 interrupt isnt blocked because of it for some reason
    // not working even without this.
    let adc = per.adc.enabled(&mut per.pmc, &mut per.syscon);
    hprintln!("adc is enabled").unwrap();


    // === DEVICE SETUP DONE === //


    // // does not work when setting this
    // cortex_m::register::basepri_max::write(32);

    hprintln!("this should be 22 for ADC0 :{}", interrupt::ADC0.number()).unwrap();
    
    // not working even without this unsafe block
    unsafe {
        // 3.4.16 ADC 0 interrupt priority. 0 = highest priority. 7 = lowest priority.

        per.NVIC.set_priority(interrupt::ADC0, 1);
        // // this should be the same as above
        // per.NVIC.ipr[interrupt::ADC0 as usize].write(16);

        NVIC::unmask(interrupt::ADC0);
    }

    // re-enable interrupts 
    unsafe { cortex_m::interrupt::enable() }


    // === INTERRUPT SETUP DONE === //


    // request
    per.NVIC.request(interrupt::ADC0);

    // interrupt number directly to STIR 
    unsafe {
        per.NVIC.stir.write(interrupt::ADC0 as u32);
    }

    // // pend still does nothing
    // NVIC::pend(interrupt::ADC0);
    
    // not working with wfi, wfe, or a loop with nop
    // cortex_m::asm::wfi();
    loop {
        lpc55_hal::wait_at_least(500000);

        // everything we know about the interrupt
        let pri = NVIC::get_priority(interrupt::ADC0); // This stays at 0 no matter what for some reason
        let enabled = NVIC::is_enabled(interrupt::ADC0);
        let pend = NVIC::is_pending(interrupt::ADC0);
        hprintln!("priority: {}\nenabled: {}\npending: {}", pri, enabled, pend).unwrap();
    }
}

#[interrupt]
fn ADC0() {
    hprintln!("            I EXIST!").unwrap();
}

#[exception]
fn DefaultHandler(int: i16) {
    hprintln!("E X C E P T I O N : {}", int).unwrap();
}