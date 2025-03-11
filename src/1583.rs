#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut rank: Vec<Vec<i32>> = vec![vec![0; n]; n];

        // Calculate the preference rank for each person
        for i in 0..n {
            for j in 0..n - 1 {
                let person = preferences[i][j] as usize;
                rank[i][person] = j as i32;
            }
        }

        // Create a mapping from person to their paired friend
        let mut paired_with = vec![0; n];
        for pair in pairs {
            let x = pair[0] as usize;
            let y = pair[1] as usize;
            paired_with[x] = y;
            paired_with[y] = x;
        }

        let mut unhappy_count = 0;

        // Check each person
        for x in 0..n {
            let y = paired_with[x];

            // Check each potential u (friend that x prefers over y)
            for u in 0..n - 1 {
                let u_person = preferences[x][u] as usize;

                // If x prefers y over u, then x is happy with all remaining people
                if u_person == y {
                    break;
                }

                let v = paired_with[u_person];

                // Check if u prefers x over v
                if rank[u_person][x] < rank[u_person][v] {
                    unhappy_count += 1;
                    break;
                }
            }
        }

        unhappy_count
    }
}

fn main() {}
