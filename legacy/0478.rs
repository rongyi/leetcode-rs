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
        // 随机一个长度
        // 我们不能把圆这样看成是很多半径的集合——或者说，这条半径上不同位置的点的密集程度是不一样的。显然距离圆心更远的一端被选择的概率应该更大。
        // 如果不用平方根的话，那么表示圆的时候
        // (len * cos(theta)) ^ 2 + (len * sin(theta) ^ 2，
        // 这里就相当于对随机出的[0, 1]
        // 中的小数平方了，那么其就不是等概率的了
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
