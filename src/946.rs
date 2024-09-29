struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let sz = pushed.len();
        let mut i = 0;

        let mut stk: Vec<i32> = Vec::new();
        let mut prev_set: HashSet<i32> = HashSet::new();

        for &num in popped.iter() {
            if prev_set.contains(&num) {
                if num != *stk.last().unwrap() {
                    return false;
                }
                stk.pop();
            } else {
                while i < sz && pushed[i] != num {
                    stk.push(pushed[i]);
                    prev_set.insert(pushed[i]);
                    i += 1;
                }
                // num not found in pushed
                if i == sz {
                    return false;
                }
                // now pushed[i] == num
                // shift i point to next element
                i += 1;
            }
            prev_set.insert(num);
        }

        true
    }
}

fn main() {}
