struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        // by start time, all start time is unique
        meetings.sort_unstable();
        let mut taken: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
        let mut free: BinaryHeap<Reverse<i32>> = (0..n).map(|v| Reverse(v)).collect();
        let mut cnt: Vec<i32> = vec![0; n as usize];

        for m in meetings.iter() {
            let mut off = 0i64;
            while !taken.is_empty() && taken.peek().unwrap().0 .0 <= m[0] as i64 {
                let fresh = taken.pop().unwrap().0;
                free.push(Reverse(fresh.1));
            }
            // consume this meeting at root r
            let r = if let Some(r) = free.pop() {
                r.0
            } else {
                // wait outside
                let p = taken.pop().unwrap().0;
                off = p.0 - m[0] as i64;
                p.1
            };
            cnt[r as usize] += 1;
            taken.push(Reverse((off + m[1] as i64, r)));
        }

        let mut max_idx = 0;
        let mut max_freq = 0;
        for i in (0..n).rev() {
            if cnt[i as usize] >= max_freq {
                max_freq = cnt[i as usize];
                max_idx = i;
            }
        }
        max_idx
    }
}

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        // make meeting in time order
        meetings.sort_unstable();
        // initial: all room is free
        let mut free: BinaryHeap<Reverse<i32>> = (0..n).map(|v| Reverse(v)).collect();
        // (next_free time, index)
        let mut taken: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
        let n = n as usize;
        let mut cnt: Vec<i32> = vec![0; n];

        for m in meetings.iter() {
            let (start, end) = (m[0] as i64, m[1] as i64);
            let duration = end - start;
            // frist: fetch all used root by this tart time
            while !taken.is_empty() && taken.peek().unwrap().0 .0 <= start {
                let cur = taken.pop().unwrap().0 .1;
                free.push(Reverse(cur));
            }
            if !free.is_empty() {
                // take one from free
                let cur_room = free.pop().unwrap().0;
                // usage accumulation
                cnt[cur_room as usize] += 1;
                // and push to take with next free time
                taken.push(Reverse((end, cur_room)));
            } else {
                // wait outside for the smallest ending room
                let (room_end, cur_room) = taken.pop().unwrap().0;
                cnt[cur_room as usize] += 1;
                taken.push(Reverse((room_end + duration, cur_room)));
            }
        }
        let mut max_val = 0;
        let mut max_idx = 0;
        for i in (0..n).rev() {
            if cnt[i] >= max_val {
                max_val = cnt[i];
                max_idx = i;
            }
        }

        max_idx as _
    }
}


fn main() {
    let input: Vec<Vec<i32>> = [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect();
    Solution::most_booked(3, input);
}
