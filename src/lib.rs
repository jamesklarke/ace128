#![no_std]

use embedded_hal as hal;
use core::f64::consts::PI;

const ACE128_MAP: [Option<u8>; 256] = [
    None     , Some( 56), Some( 40), Some( 55), Some( 24), None     , Some( 39), Some( 52), Some(  8), Some( 57), None     , None     , Some( 23), None     , Some( 36), Some( 13),
    Some(120), None     , Some( 41), Some( 54), None     , None     , None     , Some( 53), Some(  7), None     , None     , None     , Some( 20), Some( 19), Some(125), Some( 18),
    Some(104), Some(105), None     , None     , Some( 25), Some(106), Some( 38), None     , None     , Some( 58), None     , None     , None     , None     , Some( 27), Some( 14),
    Some(119), Some(118), None     , None     , None     , Some(107), None     , None     , Some(  4), None     , Some(  3), None     , Some(109), Some(108), Some(  2), Some(  1),
    Some( 88), None     , Some( 89), None     , None     , None     , None     , Some( 51), Some(  9), Some( 10), Some( 90), None     , Some( 22), Some( 11), None     , Some( 12),
    None     , None     , Some( 42), Some( 43), None     , None     , None     , None     , None     , None     , None     , None     , Some( 21), None     , Some(126), Some(127),
    Some(103), None     , Some(102), None     , None     , None     , None     , None     , None     , None     , Some( 91), None     , None     , None     , None     , None     ,
    Some(116), Some(117), None     , None     , Some(115), None     , None     , None     , Some( 93), Some( 94), Some( 92), None     , Some(114), Some( 95), Some(113), Some(  0),
    Some( 72), Some( 71), None     , Some( 68), Some( 73), None     , None     , Some( 29), None     , Some( 70), None     , Some( 69), None     , None     , Some( 35), Some( 34),
    Some(121), None     , Some(122), None     , Some( 74), None     , None     , Some( 30), Some(  6), None     , Some(123), None     , None     , None     , Some(124), Some( 17),
    None     , None     , None     , Some( 67), Some( 26), None     , Some( 27), Some( 28), None     , Some( 59), None     , None     , None     , None     , None     , Some( 15),
    None     , None     , None     , None     , None     , None     , None     , None     , Some(  5), None     , None     , None     , Some(110), None     , Some(111), Some( 16),
    Some( 87), Some( 84), None     , Some( 45), Some( 86), Some( 85), None     , Some( 50), None     , None     , None     , Some( 46), None     , None     , None     , Some( 33),
    None     , Some( 83), None     , Some( 44), Some( 75), None     , None     , Some( 31), None     , None     , None     , None     , None     , None     , None     , Some( 32),
    Some(100), Some( 61), Some(101), Some( 66), None     , Some( 62), None     , Some( 49), Some( 99), Some( 60), None     , Some( 47), None     , None     , None     , Some( 48),
    Some( 77), Some( 82), Some( 78), Some( 65), Some( 76), Some( 63), None     , Some( 64), Some( 98), Some( 81), Some( 79), Some( 80), Some( 97), Some( 96), Some(112), None     ,
];

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn correct_constants() {
        let count = ACE128_MAP.iter().filter_map(|&x| x).count();
        assert_eq!(count, 128);
    }
}


pub struct Ace128<GPIO> {
    p1: GPIO,
    p2: GPIO,
    p3: GPIO,
    p4: GPIO,
    p5: GPIO,
    p6: GPIO,
    p7: GPIO,
    p8: GPIO,
    map: [Option<u8>; 256]
}

impl<GPIO> Ace128<GPIO> 
where
    GPIO: hal::digital::v2::InputPin
{
    pub fn new(
        p1: GPIO,
        p2: GPIO, 
        p3: GPIO,
        p4: GPIO,
        p5: GPIO,
        p6: GPIO,
        p7: GPIO,
        p8: GPIO,
    ) -> Self {
        Self {
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            map: ACE128_MAP,
        }
    }

    pub fn position_to_angle(position: u8) -> f64 {
        ((2.0 * PI  / 127.0) * position as f64) - PI
    }

    pub fn read_angle(&self) -> Result<f64, <GPIO as embedded_hal::digital::v2::InputPin>::Error> {
        match self.read()? {
            Some(position) => Ok(Self::position_to_angle(position)),
            None => Ok(10.0),
        }
    }

    fn read(&self) -> Result<Option<u8>, <GPIO as embedded_hal::digital::v2::InputPin>::Error> {
        let states = self.pin_states()?;
        let byte = self.convert_pin_states(states);

        Ok(self.map[byte as usize])
    }

    fn convert_pin_states(&self, states: [bool; 8]) -> u8 {
        states.iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ bit as u8
        })
    }

    fn pin_states(&self) -> Result<[bool; 8], <GPIO as embedded_hal::digital::v2::InputPin>::Error> {
            Ok([
                self.p8.is_high()?, //msb
                self.p7.is_high()?,
                self.p6.is_high()?,
                self.p5.is_high()?,
                self.p4.is_high()?,
                self.p3.is_high()?,
                self.p2.is_high()?,
                self.p1.is_high()?, //lsb
            ])
    }
}
