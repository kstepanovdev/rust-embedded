#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board::Board,
    hal::{
        uarte::{Baudrate, Parity},
        Timer, Uarte,
    },
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    // let mut timer = Timer::new(board.TIMER0);

    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    // let mut tx_buf = [0; 1];
    let mut rx_buf = [0; 1];

    loop {
        // write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
        // timer.delay_ms(1000u32);

        serial.read(&mut rx_buf).unwrap();
        rprintln!("{:?}", rx_buf[0] as char);
    }
}
