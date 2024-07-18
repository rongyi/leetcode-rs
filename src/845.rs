#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        let mut acc = vec![1; arr.len()];
        for i in 1..sz {
            if arr[i] > arr[i - 1] {
                acc[i] += acc[i - 1];
            }
        }
        let mut acc2 = vec![1; arr.len()];
        for i in (0..sz - 1).rev() {
            if arr[i] > arr[i + 1] {
                acc2[i] += acc2[i + 1];
            }
        }

        let mut ret = 0;
        for i in 1..sz - 1 {
            if acc[i] > 1 && acc2[i] > 1 {
                ret = ret.max(acc[i] + acc2[i] - 1);
            }
        }

        ret
    }
}

fn main() {
    let input = vec![2, 1, 4, 7, 3, 2, 5];
    Solution::longest_mountain(input);
}
