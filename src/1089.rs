struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut ret = Vec::with_capacity(arr.len());

        for &v in arr.iter() {
            if v == 0 {
                ret.extend(vec![0, 0]);
            } else {
                ret.push(v);
            }
            if ret.len() >= arr.len() {
                break;
            }
        }
        ret.truncate(arr.len());

        arr.copy_from_slice(&ret);
    }
}

fn main() {}
