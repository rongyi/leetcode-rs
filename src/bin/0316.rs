struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack = vec![];
        let mut freq = vec![0; 26];
        let s = s.as_bytes();
        for &b in s.iter() {
            let idx = (b - b'a') as usize;
            freq[idx] += 1;
        }
        let mut in_stack = vec![false; 26];

        for &b in s.iter() {
            let idx = (b - b'a') as usize;
            freq[idx] -= 1;

            if in_stack[idx] {
                continue;
            }
            while let Some(&top) = stack.last() {
                let top_idx = (top - b'a') as usize;
                // ok, arrage top to latter
                if top > b && freq[top_idx] > 0 {
                    stack.pop();
                    in_stack[top_idx] = false;
                } else {
                    break;
                }
            }

            stack.push(b);
            in_stack[idx] = true;
        }

        String::from_utf8_lossy(&stack).to_string()
    }
}

fn main() {}
