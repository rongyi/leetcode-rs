#![allow(dead_code)]

struct Solution;

// bruteforce
struct SubrectangleQueries {
    vals: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { vals: rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for row in row1..=row2 {
            for col in col1..=col2 {
                self.vals[row as usize][col as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.vals[row as usize][col as usize]
    }
}

fn main() {}
