#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut x = 0;
    loop {
        x += 1;
        rprintln!("X: {}", x);
        for _ in 0..1_000_00 {
            cortex_m::asm::nop();
        }
    }
}
