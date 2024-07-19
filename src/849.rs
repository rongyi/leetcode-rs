#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let sz = seats.len();
        let mut dist1 = vec![i32::MAX; sz];
        let mut dist2 = vec![i32::MAX; sz];
        let mut prev = -1;
        for i in 0..sz {
            if seats[i] == 1 {
                prev = i as i32;
            } else {
                if prev != -1 {
                    dist1[i] = i as i32 - prev;
                }
            }
        }
        let mut ret = 0;

        prev = -1;
        for i in (0..sz).rev() {
            if seats[i] == 1 {
                prev = i as i32;
            } else {
                if prev != -1 {
                    dist2[i] = prev - i as i32;
                }
            }
        }

        for i in 0..sz {
            if seats[i] == 0 {
                ret = ret.max(dist1[i].min(dist2[i]));
            }
        }

        ret
    }
}

fn main() {}
