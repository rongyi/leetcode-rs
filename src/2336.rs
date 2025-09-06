struct Solution;

use std::collections::BTreeSet;
struct SmallestInfiniteSet {
    range_lower: i32,
    dots: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            range_lower: 1,
            dots: BTreeSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.dots.is_empty() {
            let val = self.range_lower;
            self.range_lower += 1;
            return val;
        } else {
            let val = *self.dots.iter().next().unwrap();
            self.dots.remove(&val);

            return val;
        }
    }

    fn add_back(&mut self, num: i32) {
        // no op
        if num >= self.range_lower {
            return;
        }
        if num == self.range_lower - 1 {
            self.range_lower -= 1;
            // also need to ensure dots dont containt this num
            self.dots.remove(&num);
            return;
        }
        // put in dots
        self.dots.insert(num);
    }
}

fn main() {}
