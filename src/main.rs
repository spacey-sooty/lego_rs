extern crate ev3dev_lang_rust;
extern crate lib;

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::Ev3Result;

use lib::pid;

fn main() -> Ev3Result<()> {
    let conf = pid::PIDConfig::from(String::from("/elevator"), 1.0, 2.0, 3.0, 1.0, 1.0);
    let mut controller = pid::PIDController::from(
        conf,
        90.0,
        1.0,
        pid::System::LargeMotor(LargeMotor::get(MotorPort::OutA).unwrap()),
        0.0,
    );
    Ok(controller.on_update()?)
}
