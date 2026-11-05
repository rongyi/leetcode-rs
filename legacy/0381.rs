use std::collections::{HashMap, HashSet};

// import external C code
extern "C" {
    fn srand() -> u32;
    fn rand() -> u32;
}

// random number function
fn my_rand_number() -> u32 {
    unsafe {
        srand();
        rand()
    }
}
struct RandomizedCollection {
    cache: HashMap<i32, HashSet<usize>>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            nums: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.nums.push(val);
        let e = self.cache.entry(val).or_insert(HashSet::new());
        e.insert(self.nums.len() - 1);

        e.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx_set) = self.cache.get_mut(&val) {
            if let Some(&idx) = idx_set.iter().next() {
                idx_set.remove(&idx);
                if idx_set.is_empty() {
                    self.cache.remove(&val);
                }
                let last_val = *self.nums.last().unwrap();
                if idx != self.nums.len() - 1 {
                    let last_idx_set = self.cache.get_mut(&last_val).unwrap();
                    last_idx_set.remove(&(self.nums.len() - 1));
                    last_idx_set.insert(idx);
                    self.nums[idx] = last_val;
                }

                self.nums.pop();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        self.nums[my_rand_number() as usize % self.nums.len()]
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

fn main() {}
