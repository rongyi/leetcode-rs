#![allow(dead_code)]

struct Solution;

struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => {
                if self.big > 0 {
                    self.big -= 1;
                    true
                } else {
                    false
                }
            }
            2 => {
                if self.medium > 0 {
                    self.medium -= 1;
                    true
                } else {
                    false
                }
            }
            3 => {
                if self.small > 0 {
                    self.small -= 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn main() {}
