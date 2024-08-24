#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod calibration;

use calibration::{calc_calibration, calibrated_measurement, Calibration, Measurement};
use cortex_m_rt::entry;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr, MagMode, MagOutputDataRate};
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Delay, Timer, Twim},
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c)
        .into_mag_continuous()
        .ok()
        .unwrap();
    sensor.init().unwrap();
    let mut delay = Delay::new(board.SYST);
    sensor
        .set_mag_mode_and_odr(&mut delay, MagMode::HighResolution, MagOutputDataRate::Hz50)
        .unwrap();
    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz50)
        .unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);

    // let calibration = Calibration {
    //     center: Measurement {
    //         x: 1220,
    //         y: 11672,
    //         z: -33828,
    //     },
    //     scale: Measurement {
    //         x: 1094,
    //         y: 1111,
    //         z: 1159,
    //     },
    //     radius: 49508,
    // };

    // hardcode in order to avoid the calibration process

    rprintln!("Calibration: {:?}", calibration);

    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.magnetic_field().unwrap().xyz_nt();
            let measurement = Measurement { x, y, z };
            let Measurement { x, y, z } = calibrated_measurement(measurement, &calibration);

            let direction = match (x > 0, y > 0) {
                (true, true) => Direction::NorthEast,
                (true, false) => Direction::SouthEast,
                (false, true) => Direction::NorthWest,
                (false, false) => Direction::SouthWest,
            };

            display.show(&mut timer, direction.into_leds(), 100);
        }
        timer.delay(200);
    }
}

#[derive(Debug, Default)]
enum Direction {
    #[default]
    None,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    // TODO: why are they inverted?
    fn into_leds(&self) -> [[u8; 5]; 5] {
        match self {
            Direction::NorthEast => [
                [1, 1, 1, 0, 0],
                [1, 1, 0, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 0, 0, 1, 0],
                [0, 0, 0, 0, 1],
            ],
            Direction::NorthWest => [
                [0, 0, 1, 1, 1],
                [0, 0, 0, 1, 1],
                [0, 0, 1, 0, 1],
                [0, 1, 0, 0, 0],
                [1, 0, 0, 0, 0],
            ],
            Direction::SouthEast => [
                [0, 0, 0, 0, 1],
                [0, 0, 0, 1, 0],
                [1, 0, 1, 0, 0],
                [1, 1, 0, 0, 0],
                [1, 1, 1, 0, 0],
            ],
            Direction::SouthWest => [
                [1, 0, 0, 0, 0],
                [0, 1, 0, 0, 0],
                [0, 0, 1, 0, 1],
                [0, 0, 0, 1, 1],
                [0, 0, 1, 1, 1],
            ],
            Direction::None => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
        }
    }
}
