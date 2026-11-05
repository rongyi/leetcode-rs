use std::collections::HashMap;

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

struct RandomizedSet {
    cache: HashMap<i32, usize>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            // val -> index in nums
            cache: HashMap::new(),
            nums: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.cache.contains_key(&val) {
            return false;
        }
        self.nums.push(val);
        self.cache.insert(val, self.nums.len() - 1);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        // its a swap remove, place last one to current pos, and post last one
        if let Some(&pos) = self.cache.get(&val) {
            let &last_val = self.nums.last().unwrap();
            let last_val_index = self.nums.len() - 1;
            // last_val to new position: pos
            self.cache.insert(last_val, pos);
            // and make this val to here
            self.nums.swap(pos, last_val_index);

            self.nums.pop();
            self.cache.remove(&val);

            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        self.nums[my_rand_number() as usize % self.nums.len()]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

fn main() {}
