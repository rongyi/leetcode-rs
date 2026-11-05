use std::collections::BinaryHeap;

struct MedianFinder {
    // find bigger one in small
    small: BinaryHeap<i32>,
    // find small one in large
    large: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        self.large.push(-self.small.pop().unwrap());

        if self.large.len() > self.small.len() {
            self.small.push(-self.large.pop().unwrap());
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else {
            (*self.small.peek().unwrap() - *self.large.peek().unwrap()) as f64 / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    let mut a = BinaryHeap::new();
    a.push(1);
    a.push(2);
    let cur = a.pop().unwrap();
    println!("{}", cur);
}
