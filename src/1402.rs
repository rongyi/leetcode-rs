#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut sum = 0;
        let mut ret = 0;
        // where is the time tick? *1 *2 *3?
        // oh we are tranverse reverse
        // so the muliply is contained in rest processing
        // like every accumulated value is added again
        //Input: satisfaction = [-1, -8, 0, 5, -9]
        // Sorted: [-9, -8, -1, 0, 5]
        // Steps:
        // Start with 5:
        // current_sum = 5
        // total_satisfaction = 5
        // Add 0:
        // current_sum = 5 + 0 = 5
        // total_satisfaction = 5 + 5 = 10
        // Add -1:
        // current_sum = 5 + (-1) = 4
        // total_satisfaction = 10 + 4 = 14
        // Add -8:
        // current_sum = 4 + (-8) = -4 (negative, so we stop)
        // Output: 14
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
