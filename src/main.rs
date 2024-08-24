#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod calibration;

use calibration::calc_calibration;
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

    // let mut grid = [
    //     [0, 0, 1, 0, 0],
    //     [0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0],
    // ];

    // display.show(&mut timer, grid, 200);

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

    // let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);
    // rprintln!("Calibration: {:?}", calibration);
    // rprintln!("Calibration done, entering busy loop");

    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let mag_field = sensor.magnetic_field().unwrap();
            rprintln!(
                "x: {}, y: {}, z: {}\n",
                mag_field.x_raw(),
                mag_field.y_raw(),
                mag_field.z_raw()
            );
        }
        timer.delay(200);
    }
}
