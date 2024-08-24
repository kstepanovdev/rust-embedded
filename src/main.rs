#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr, MagMode, MagOutputDataRate};
use microbit::{
    board::Board,
    hal::{
        uarte::{Baudrate, Parity},
        Delay, Timer, Twim, Uarte,
    },
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut app_mode = AppMode::default();

    let mut timer = Timer::new(board.TIMER0);
    let mut uart = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    let mut rx_buf = [0; 1];

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c)
        .into_mag_continuous()
        .ok()
        .unwrap();
    sensor.init().unwrap();
    let mut delay = Delay::new(board.SYST);

    loop {
        if uart.read_timeout(&mut rx_buf, &mut timer, 40_000).is_ok() {
            let c = rx_buf[0] as char;

            match c {
                'a' => {
                    sensor
                        .set_accel_mode_and_odr(
                            &mut delay,
                            AccelMode::Normal,
                            AccelOutputDataRate::Hz50,
                        )
                        .unwrap();
                    app_mode = AppMode::Accelerometer;
                    write!(uart, "Accelerometer mode set\r\n").unwrap();
                }
                'm' => {
                    sensor
                        .set_mag_mode_and_odr(
                            &mut delay,
                            MagMode::HighResolution,
                            MagOutputDataRate::Hz50,
                        )
                        .unwrap();

                    app_mode = AppMode::Magnetometer;
                    write!(uart, "Magnetometer mode set\r\n").unwrap();
                }
                'n' => {
                    sensor
                        .set_accel_mode_and_odr(
                            &mut delay,
                            AccelMode::PowerDown,
                            AccelOutputDataRate::Hz50,
                        )
                        .unwrap();
                    sensor
                        .set_mag_mode_and_odr(
                            &mut delay,
                            MagMode::LowPower,
                            MagOutputDataRate::Hz10,
                        )
                        .unwrap();
                    app_mode = AppMode::None;
                    write!(uart, "None mode set\r\n").unwrap();
                }
                _ => {}
            }
        }

        match app_mode {
            AppMode::Accelerometer => {
                while sensor.accel_status().unwrap().xyz_new_data() {
                    let acceleration = sensor.acceleration().unwrap();

                    write!(uart, "Acceleration: {:?}\r\n", acceleration).unwrap();
                }
            }
            AppMode::Magnetometer => {
                while sensor.mag_status().unwrap().xyz_new_data() {
                    let mag_field = sensor.magnetic_field().unwrap();

                    write!(uart, "Magnetometer: {:?}\r\n", mag_field).unwrap();
                }
            }
            AppMode::None => nop(),
        }
    }
}

#[derive(Default)]
pub enum AppMode {
    Accelerometer,
    Magnetometer,
    #[default]
    None,
}
