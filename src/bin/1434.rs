#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let sz = hats.len();
        // hats -> person
        let mut persons: Vec<Vec<usize>> = vec![vec![]; 40];

        let mut masks = vec![0; 1 << sz];
        masks[0] = 1;

        for i in 0..sz {
            for &hat in hats[i].iter() {
                persons[hat as usize - 1].push(i);
            }
        }

        for i in 0..40 {
            for j in (0..(1 << sz)).rev() {
                for &p in persons[i].iter() {
                    // different hat means
                    if (j & (1 << p)) == 0 {
                        masks[j | (1 << p)] += masks[j];
                        masks[j | (1 << p)] %= MOD;
                    }
                }
            }
        }

        masks[(1 << sz) - 1] as i32
    }
}

fn main() {}
