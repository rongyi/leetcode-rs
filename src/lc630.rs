struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by(|l, r| l[1].cmp(&r[1]));
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        let mut sum = 0;
        for c in courses.iter() {
            pq.push(c[0]);
            sum += c[0];
            if sum > c[1] {
                sum -= pq.pop().unwrap();
            }
        }
        pq.len() as i32
    }
}

fn main() {
    let mut pq: BinaryHeap<i32> = BinaryHeap::new();
    pq.push(10);
    pq.push(1);
    let cur = *pq.peek().unwrap();
    println!("{}", cur);
}
