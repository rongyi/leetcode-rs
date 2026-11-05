#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        let mut prefix = vec![0; sz + 1];
        for i in 0..sz {
            prefix[i + 1] = prefix[i] ^ arr[i];
        }
        let mut ret = 0;
        for i in 0..prefix.len() {
            for j in i + 1..prefix.len() {
                if prefix[i] == prefix[j] {
                    ret += j - i - 1;
                }
            }
        }
        ret as _
    }
}

fn main() {}
