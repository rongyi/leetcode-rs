struct Solution;

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut acc = 0;
        for &d in differences.iter() {
            acc += d as i64;
            l = l.min(acc);
            r = r.max(acc);
        }

        let mut valid_range = 0;

        for i in lower..=upper {
            let i = i as i64;
            if i + l < lower as i64 || i + r > upper as i64 {
                continue;
            }
            valid_range += 1;
        }

        valid_range
    }
}

fn main() {}
