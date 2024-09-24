struct Solution;

use std::collections::VecDeque;
struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        if t < 3000 {
            return self.queue.len() as i32;
        }
        let last = t - 3000;
        while let Some(&val) = self.queue.front() {
            if val < last {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}

fn main() {}
