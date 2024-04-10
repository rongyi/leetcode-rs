struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut lo = 0;
        let mut hi = (arr.len() - 1) as i32;
        while hi - lo >= k {
            if (arr[lo as usize] - x).abs() > (arr[hi as usize] - x).abs() {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        let mut ret = Vec::new();
        for i in lo..=hi {
            ret.push(arr[i as usize]);
        }
        ret
    }
}

fn main() {}
