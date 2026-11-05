#![allow(dead_code)]

struct Solution;

struct CustomFunction;
impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        0
    }
}

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];

        for i in 1..=1000 {
            let mut lo = 1;
            let mut high = 1000;
            let mut mid = lo + (high - lo) / 2;
            while lo < high {
                if customfunction.f(i, mid) == z {
                    ret.push(vec![i, mid]);
                    break;
                } else if customfunction.f(i, mid) > z {
                    high = mid;
                } else {
                    lo = mid + 1;
                }
                mid = (high + lo) / 2;
            }
        }

        ret
    }
}

fn main() {}
