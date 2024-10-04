struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut ret = HashSet::new();

        let x_max = if x == 1 {
            0
        } else {
            (bound as f64).log(x as f64) as i32
        };
        let y_max = if y == 1 {
            0
        } else {
            (bound as f64).log(y as f64) as i32
        };

        for i in 0..=x_max {
            for j in 0..=y_max {
                let val = x.pow(i as u32) + y.pow(j as u32);
                if val <= bound {
                    ret.insert(val);
                } else {
                    break;
                }
            }
            if x == 1 {
                break;
            }
        }

        ret.into_iter().collect()
    }
}

fn main() {}
