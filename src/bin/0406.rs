struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|l, r| -> Ordering {
            if l[0] == r[0] {
                return l[1].cmp(&r[1]);
            }
            r[0].cmp(&l[0])
        });
        // println!("{:?}", people);
        let mut ret = Vec::new();
        for p in people.iter() {
            ret.insert(p[1] as usize, p.clone());
        }

        ret
    }
}

fn main() {}
