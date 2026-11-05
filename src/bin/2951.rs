struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];

        for (i, &val) in mountain.iter().enumerate().skip(1) {
            if val > mountain[i - 1] && i < mountain.len() - 1 && val > mountain[i + 1] {
                ret.push(i as i32);
            }
        }

        ret
    }
}

fn main() {}
