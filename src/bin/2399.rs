struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut prev_index: Vec<usize> = vec![usize::MAX; 26];
        for (i, c) in s.chars().enumerate() {
            let idx = (c as u8 - 'a' as u8) as usize;
            if prev_index[idx] == usize::MAX {
                prev_index[idx] = i;
            } else {
                let dist = i - prev_index[idx] - 1;
                if dist != distance[idx] as usize {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {}
