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

use panic_halt as _;

#[rtic::app(device = lpc55_hal::raw, dispatchers = [SDIO], peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::DwtSystick;
    use lpc55_hal::{drivers::pins, drivers::pins::Level, prelude::*, typestates::pin};
    use rtic::rtic_monotonic::Milliseconds;

    type RedLed = lpc55_hal::Pin<pins::Pio1_6, pin::state::Gpio<pin::gpio::direction::Output>>;

    const MONO_HZ: u32 = 150_000_000; // 8 MHz

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    #[shared]
    struct Shared {
        led: RedLed,
        toggle: bool,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init start").unwrap();

        let mut dcb = cx.core.DCB;

        let device = cx.device;

        let mut syscon = lpc55_hal::Syscon::from(device.SYSCON);
        let mut gpio = lpc55_hal::Gpio::from(device.GPIO).enabled(&mut syscon);
        let mut iocon = lpc55_hal::Iocon::from(device.IOCON).enabled(&mut syscon);

        let mono = DwtSystick::new(&mut dcb, cx.core.DWT, cx.core.SYST, MONO_HZ); // maybe MONO_HZ?

        // let mut flash = device.FLASH.constrain();
        let pins = lpc55_hal::Pins::take().unwrap();

        let red_led = pins
            .pio1_6
            .into_gpio_pin(&mut iocon, &mut gpio)
            .into_output(Level::High);

        hprintln!("init end").unwrap();


        // Schedule `toggle` to run 8e6 cycles (clock cycles) in the future
        toggle::spawn_after(Milliseconds(300u32)).ok();

        (
            Shared {
                led: red_led,
                toggle: false,
            },
            Local {},
            init::Monotonics(mono),
        )
    }

    #[task(shared = [toggle, led])]
    fn toggle(cx: toggle::Context) {
        let toggle = cx.shared.toggle;
        let led = cx.shared.led;

        (toggle, led).lock(|toggle, led| {
            hprintln!(
                "toggle: {} @ {} !",
                toggle,
                rtic::export::DWT::get_cycle_count()
            )
            .unwrap();

            if *toggle {
                (*led).set_high().ok();
            } else {
                (*led).set_low().ok();
            }

            *toggle = !*toggle;
        });
        toggle::spawn_after(Milliseconds(300u32)).ok();
    }
}
