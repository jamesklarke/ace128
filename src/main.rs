use ace128_driver::Ace128;
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio};

pub fn main() {

    let encoder = Ace128::new(
        Gpio::new().unwrap().get(1).unwrap().into_input_pullup(),
        Gpio::new().unwrap().get(2).unwrap().into_input_pullup(),
        Gpio::new().unwrap().get(3).unwrap().into_input_pullup(),
        Gpio::new().unwrap().get(4).unwrap().into_input_pullup(),
        Gpio::new().unwrap().get(5).unwrap().into_input_pullup(),
        Gpio::new().unwrap().get(6).unwrap().into_input_pullup(),
        Gpio::new().unwrap().get(7).unwrap().into_input_pullup(),
	Gpio::new().unwrap().get(8).unwrap().into_input_pullup()
    );
    
    loop {
	    println!("Angle is {:?}", encoder.read_angle().unwrap());
        thread::sleep(Duration::from_millis(1000));
    }
}
