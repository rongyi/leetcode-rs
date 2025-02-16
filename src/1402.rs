#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut sum = 0;
        let mut ret = 0;
        for cur_statis in satisfaction.iter().rev() {
            if cur_statis + ret <= 0 {
                break;
            }
            ret += cur_statis;
            sum += ret;
        }
        sum
    }
}

fn main() {}
