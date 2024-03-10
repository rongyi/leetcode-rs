// yes, leetcode include this external crate
use rand::thread_rng;
use rand::Rng;
use std::f64::consts::PI;

struct Solution {
    radius: f64,
    x: f64,
    y: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x: x_center,
            y: y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        // random an angle
        let theta: f64 = 2.0 * PI * rng.gen::<f64>();
        // random an lenth
        let len = rng.gen::<f64>().sqrt() * self.radius;

        vec![self.x + len * theta.cos(), self.y + len * theta.sin()]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */

fn main() {
    let mut rng = thread_rng();

    println!("{}", rng.gen::<f64>());
}
