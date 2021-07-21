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

#[rtic::app(device = stm32f2xx_hal::stm32, dispatchers = [EXTI0], peripherals = true)]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use dwt_systick_monotonic::DwtSystick;
    use rtic::rtic_monotonic::Milliseconds;
    use stm32f2xx_hal::{gpio::{GpioExt, Output, PushPull, gpioa::PA5}, prelude::_embedded_hal_digital_v2_OutputPin};

    const MONO_HZ: u32 = 8_000_000; // 8 MHz

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    #[shared]
    struct Shared {
        led: PA5<Output<PushPull>>,
        toggle: bool,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init start");

        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;

        let mono = DwtSystick::new(&mut dcb, dwt, systick, 8_000_000); // maybe MONO_HZ?

        let device = cx.device;

        // let mut flash = device.FLASH.constrain();

        // power on GPIOA
        device.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
        // configure pin PA5 as output
        device.GPIOA.moder.modify(|_, w| w.moder5().bits(1));

        let gpioa = device.GPIOA.split();
        let mut led: PA5<Output<PushPull>> = gpioa.pa5.into_push_pull_output();

        led.set_high().unwrap();

        rprintln!("init end");

        // Schedule `toggle` to run 8e6 cycles (clock cycles) in the future
        toggle::spawn_after(Milliseconds(300u32)).ok();

        (
            Shared {
                led: led,
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
            rprintln!("toggle: {}  @ !", toggle);
            
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
