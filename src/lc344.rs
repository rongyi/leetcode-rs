struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let sz = s.len();
        let (mut i, mut j) = (0, sz - 1);
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

fn main() {}
