#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut count: Vec<i32> = vec![0; 10];
        let mut sum = 0;
        for &d in digits.iter() {
            count[d as usize] += 1;
            sum += d;
        }
        // return delete success?
        fn remove_digit(count: &mut Vec<i32>, start: usize) -> bool {
            for i in (start..10).step_by(3) {
                if count[i] > 0 {
                    count[i] -= 1;
                    return true;
                }
            }

            false
        }
        match sum % 3 {
            1 => {
                if !remove_digit(&mut count, 1) {
                    remove_digit(&mut count, 2);
                    remove_digit(&mut count, 2);
                }
            }
            2 => {
                if !remove_digit(&mut count, 2) {
                    remove_digit(&mut count, 1);
                    remove_digit(&mut count, 1);
                }
            }
            _ => {}
        }
        let mut ret = String::new();
        for i in (0..10).rev() {
            for _ in 0..count[i] {
                ret.push_str(&i.to_string());
            }
        }

        if ret.is_empty() {
            "".to_string()
        } else if ret.chars().next().unwrap() == '0' {
            "0".to_string()
        } else {
            ret
        }
    }
}

fn main() {}
