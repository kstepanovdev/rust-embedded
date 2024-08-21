#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use heapless::Vec;
use microbit::{
    board::Board,
    hal::{
        uarte::{Baudrate, Parity},
        Uarte,
    },
};
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    let mut message_buf: Vec<char, 8> = Vec::new();
    let mut rx_buf = [0; 1];

    loop {
        serial.read(&mut rx_buf).unwrap();

        if rx_buf[0] as char == '\r' {
            for c in message_buf.iter().chain(&['\n', '\r']) {
                serial.write_char(*c).unwrap();
            }
            message_buf.clear();
        } else {
            if message_buf.len() == 8 {
                serial
                    .write_str("MAX BUF LEN REACHED, PRESS ENTER \r\n")
                    .unwrap();
            } else {
                message_buf.push(rx_buf[0] as char).unwrap();
            }
        }
    }
}
