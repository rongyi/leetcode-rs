
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut prev = 0;
        let mut prefix_set: HashSet<i32> = HashSet::new();
        let sz = a.len();
        let mut ret = vec![0; sz];

        for i in 0..sz {
            if a[i] == b[i] {
                prev += 1;
                ret[i] = prev;
            } else {
                if prefix_set.contains(&a[i]) {
                    prev += 1;
                }
                if prefix_set.contains(&b[i]) {
                    prev += 1;
                }
                ret[i] = prev;
            }
            prefix_set.insert(a[i]);
            prefix_set.insert(b[i]);
        }

        ret
    }
}

fn main() {}
