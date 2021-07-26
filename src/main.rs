// run with gdb, either from terminal or in vscode
//
// from terminal:
// start openocd in separate terminal
// > openocd -f openocd.cfg
//
// start gdb
// > arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/examples/rtic_blinky -x openocd.gdb

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lpc55_hal::raw, dispatchers = [SDIO], peripherals = true)]
mod app {
    use dwt_systick_monotonic::DwtSystick;
    use rtic::rtic_monotonic::Milliseconds;
    use cortex_m_semihosting::hprintln;
    use lpc55_hal::{
        prelude::*,
        drivers::pins::Level,
        drivers::pins,
        typestates::pin,
    };
    
    type RedLed = lpc55_hal::Pin<pins::Pio1_6, pin::state::Gpio<pin::gpio::direction::Output>>;

    const MONO_HZ: u32 = 150_000_000; // 8 MHz

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    #[shared]
    struct Shared {
        led: RedLed,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init start").unwrap();

        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;
        
        let device = cx.device;        

        let mut syscon = lpc55_hal::Syscon::from(device.SYSCON);
        let mut gpio = lpc55_hal::Gpio::from(device.GPIO).enabled(&mut syscon);
        let mut iocon = lpc55_hal::Iocon::from(device.IOCON).enabled(&mut syscon);

        let mono = DwtSystick::new(&mut dcb, dwt, systick, MONO_HZ);

        let pins = lpc55_hal::Pins::take().unwrap();

        let red_led = pins.pio1_6
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::Low);

        hprintln!("init end").unwrap();

        toggle::spawn_after(Milliseconds(300u32)).unwrap();

        (
            Shared {
                led: red_led,
            },
            Local {},
            init::Monotonics(mono),
        )
    }

    #[task(shared = [led])]
    fn toggle(cx: toggle::Context) {
        let mut led = cx.shared.led;

        hprintln!("attempt lock").unwrap();

        led.lock(|led| {
            let toggle = (*led).is_set_low().unwrap();
            hprintln!("toggle: {}  @ !", toggle).unwrap();

            if toggle {
                (*led).set_high().ok();
            } else {
                (*led).set_low().ok();
            }
        });
        toggle::spawn_after(Milliseconds(300u32)).ok();
    }
}
