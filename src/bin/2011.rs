struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut val = 0;
        for op in operations.into_iter() {
            if op.contains('+') {
                val += 1;
            } else {
                val -= 1;
            }
        }

        val
    }
}

fn main() {}
