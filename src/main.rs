#![no_std]
#![no_main]

use cortex_m_rt::entry;
extern crate panic_halt;

#[entry]
fn main() -> ! {
    loop {}
}
