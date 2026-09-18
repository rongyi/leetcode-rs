struct Solution;

use std::collections::HashMap;
struct RandomizedSet {
    nums: Vec<i32>,
    pos: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            pos: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let Some(_) = self.pos.get(&val) {
            return false;
        }

        self.pos.insert(val, self.nums.len());
        self.nums.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.pos.get(&val) {
            Some(&pos) => {
                let last_id = self.nums.len() - 1;
                let last_val = self.nums[last_id];
                if pos != last_id {
                    self.nums.swap(pos, last_id);
                    self.pos.insert(last_val, pos);
                }
                self.nums.pop();
                self.pos.remove(&val);

                true
            }
            None => false,
        }
    }

    fn get_random(&self) -> i32 {
        use rand::Rng;
        self.nums[rand::thread_rng().gen_range(0..self.nums.len())]
    }
}

fn main() {}
