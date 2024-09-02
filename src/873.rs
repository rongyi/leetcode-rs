
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let s: HashSet<i32> = arr.iter().copied().collect();
        let mut ret = 0;
        let sz = arr.len();
        for i in 0..sz {
            for j in i + 1..sz {
                let mut a = arr[i];
                let mut b = arr[j];
                let mut l = 2;
                while s.contains(&(a + b)) {
                    let tmp = b;
                    b = a + b;
                    a = tmp;
                    l += 1;
                }
                ret = ret.max(l);
            }
        }

        if ret > 2 {
            ret
        } else {
            0
        }
    }
}

fn main() {}
