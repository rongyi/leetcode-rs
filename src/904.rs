
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut ret = 0;

        let mut i = 0;
        let sz = fruits.len();

        for j in 0..sz {
            *cnt.entry(fruits[j]).or_insert(0) += 1;
            while cnt.len() > 2 {
                let left = cnt.get_mut(&fruits[i]).unwrap();
                *left -= 1;
                if *left == 0 {
                    cnt.remove(&fruits[i]);
                }

                i += 1;
            }
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}

fn main() {}
