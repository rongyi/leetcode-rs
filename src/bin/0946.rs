struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut pop_idx = 0;

        for &v in pushed.iter() {
            stack.push(v);
            while let Some(&top) = stack.last() {
                if popped[pop_idx] == top {
                    pop_idx += 1;
                    stack.pop();
                } else {
                    break;
                }
            }
        }

        stack.is_empty()
    }
}

fn main() {}
