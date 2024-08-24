#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::i2c::I2c;
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit::{
    board::Board,
    hal::{Delay, Twim},
    pac::twim0::frequency::FREQUENCY_A,
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut acc = [0];
    let mut mag = [0];

    if i2c
        .write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .is_ok()
    {
        rprintln!("Accelerometer ID: {:#b}", acc[0]);
    } else {
        rprintln!("Error reading accelerometer ID");
    }

    if i2c
        .write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .is_ok()
    {
        rprintln!("Magnetometer ID: {:#b}", mag[0]);
    } else {
        rprintln!("Error reading Magnetometer ID");
    }

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    let mut delay = Delay::new(board.SYST);

    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz50)
        .unwrap();

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let acceleration = sensor.acceleration().unwrap();

            rprintln!("Acceleration: {:?}", acceleration);
        }
    }
}
