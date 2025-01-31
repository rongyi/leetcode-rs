#![allow(dead_code)]
struct Solution;

struct ProductOfNumbers {
    prod: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self { prod: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prod = vec![1];
        } else {
            self.prod.push(*self.prod.last().unwrap() * num);
        }
    }

    fn get_product(&mut self, k: i32) -> i32 {
        if k as usize >= self.prod.len() {
            0
        } else {
            *self.prod.last().unwrap() / self.prod[self.prod.len() - k as usize - 1]
        }
    }
}

fn main() {}
