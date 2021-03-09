use ace128_driver::Ace128;
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio};

pub fn main() {

    let encoder = Ace128::new(
        Gpio::new().unwrap().get(5).unwrap().into_input(),
        Gpio::new().unwrap().get(6).unwrap().into_input(),
        Gpio::new().unwrap().get(7).unwrap().into_input(),
        Gpio::new().unwrap().get(8).unwrap().into_input(),
        Gpio::new().unwrap().get(23).unwrap().into_input(),
        Gpio::new().unwrap().get(24).unwrap().into_input(),
        Gpio::new().unwrap().get(25).unwrap().into_input(),
        Gpio::new().unwrap().get(26).unwrap().into_input()
    );
    
    loop {
	    println!("Angle is {:?}", encoder.read_angle().unwrap());
        thread::sleep(Duration::from_millis(1000));
    }
}
