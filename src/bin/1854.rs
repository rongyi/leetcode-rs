#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut total_populations = 0;
        let mut line_sweep = vec![0; 2051];

        for log in &logs {
            line_sweep[log[0] as usize] += 1;
            line_sweep[log[1] as usize] -= 1;
        }
        let mut acc = 0;
        let mut most_year = 0;

        for (year, &val) in line_sweep.iter().enumerate() {
            acc += val;
            if acc > total_populations {
                total_populations = acc;
                most_year = year;
            }
        }

        most_year as i32
    }
}

fn main() {}
