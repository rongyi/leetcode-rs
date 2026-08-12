struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut ret = 0;

        let mut cur_max = 0;
        // Key insight: A chunk can end at position i iff
        // all elements up to i are exactly the numbers 0..i
        // This is true iff max(arr[0..=i]) == i
        for i in 0..arr.len() {
            cur_max = cur_max.max(arr[i]);
            if i as i32 == cur_max {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
