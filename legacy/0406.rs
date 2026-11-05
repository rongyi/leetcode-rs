struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        // 矮子插队无所谓，反正高个子看不到
        // 所以这里的思路就是先把高的排好，然后矮的放在固定的位置即可
        people.sort_by(|l, r| -> Ordering {
            if l[0] == r[0] {
                return l[1].cmp(&r[1]);
            }
            r[0].cmp(&l[0])
        });
        let mut ret = Vec::new();
        for p in people.iter() {
            ret.insert(p[1] as usize, p.clone());
        }

        ret
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
