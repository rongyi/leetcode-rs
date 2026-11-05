struct Solution;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let arr: Vec<i64> = arr.into_iter().map(|v| v as i64).collect();
        let k = k as i64;
        let sz = arr.len() as i64;
        let k = Self::gcd(sz, k);
        let mut ret = 0;
        for i in 0..k {
            let mut group = vec![];
            for j in (i..sz).step_by(k as usize) {
                group.push(arr[j as usize]);
            }
            group.sort();
            let mid = group.len() / 2;
            for i in 0..group.len() {
                ret += (group[mid] - group[i]).abs();
            }
        }

        ret
    }
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {}
