#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        // all possible subsets of courses
        let mut prereqs = vec![0; 1 << n];
        for r in relations.iter() {
            let prev = r[0] as usize - 1;
            let next = r[1] as usize - 1;
            prereqs[next] |= 1 << prev;
        }

        // init as invalid stat
        let mut dp = vec![n + 1; 1 << n];
        dp[0] = 0; // empty set

        // this is prev mask to accumulate
        for mask in 0..(1 << n) {
            // invalid prev state, ignore
            if dp[mask] == n + 1 {
                continue;
            }

            // fetch all courses that can be taken
            let mut can_take = 0;
            for course in 0..n {
                // If course is not taken (not in mask) and all prerequisites are satisfied
                // current mask is a superset of course prerequisites
                // 1. check if course is not taken, i.e. not in prev mask
                // 2. check if all prerequisites are satisfied, i.e. all prerequisites are in prev mask
                if (mask & (1 << course)) == 0 && (prereqs[course] & mask) == prereqs[course] {
                    can_take |= 1 << course;
                }
            }
            // enumerate all subsets of can_take
            let mut subset: usize = can_take;
            while subset > 0 {
                let count = subset.count_ones() as usize;
                if count <= k {
                    let new_mask = mask | subset;
                    dp[new_mask] = dp[new_mask].min(dp[mask] + 1);
                }
                subset = (subset - 1) & can_take;
            }
        }

        dp[(1 << n) - 1] as _
    }
}

fn main() {}
