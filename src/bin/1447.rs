#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn min_form(a: i32, b: i32) -> bool {
            for i in 2..=a {
                if a % i == 0 && b % i == 0 {
                    return false;
                }
            }
            true
        }
        let mut ret: Vec<String> = Vec::new();
        for i in 2..=n {
            for j in 1..i {
                if j == 1 || min_form(j, i) {
                    let val = j.to_string() + "/" + &i.to_string();
                    ret.push(val);
                }
            }
        }

        ret
    }
}

fn main() {}
