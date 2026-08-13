struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut canbemet: HashMap<i32, i32> = HashMap::new();
        let mut ret = 0;

        for &num in answers.iter() {
            if num == 0 {
                ret += 1;
                continue;
            }
            if canbemet.contains_key(&num) {
                if let Some(v) = canbemet.get_mut(&num) {
                    *v -= 1;
                    if *v == 0 {
                        canbemet.remove(&num);
                    }
                }
            } else {
                ret += num + 1;
                canbemet.insert(num, num);
            }
        }

        ret
    }
}

fn main() {}
