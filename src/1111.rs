struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let seq: Vec<char> = seq.chars().collect();
        let mut ret = vec![0; seq.len()];
        let mut level = 0;
        let mut index = 0;
        while index < seq.len() {
            if seq[index] == '(' {
                level += 1;
                ret[index] = level % 2;
            } else {
                ret[index] = level % 2;
                level -= 1;
            }
            index += 1;
        }

        ret
    }
}

fn main() {}
