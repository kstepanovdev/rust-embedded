#![no_std]
#![no_main]

use hal::{
    pac::{self},
    prelude::*,
    rcc::RccExt,
};
use stm32f4xx_hal as hal;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    let gpiod = dp.GPIOD.split();

    let mut green_led = gpiod.pd12.into_push_pull_output();
    let mut orange_led = gpiod.pd13.into_push_pull_output();
    let mut red_led = gpiod.pd14.into_push_pull_output();
    let mut blue_led = gpiod.pd15.into_push_pull_output();

    rtt_init_print!();
    loop {
        green_led.set_high();
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
        green_led.set_low();

        orange_led.set_high();
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
        orange_led.set_low();

        green_led.set_high();
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
        green_led.set_low();

        red_led.set_high();
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
        red_led.set_low();

        blue_led.set_high();
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
        blue_led.set_low();
    }
}
