#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = requests.len();
        let mut max_fulfilled = 0;

        // Generate all possible combinations of requests
        for mask in 0..(1i32 << m) {
            let bit_count = mask.count_ones() as i32;

            // Skip if we've already found a better solution
            if bit_count <= max_fulfilled {
                continue;
            }

            // Calculate net change for each building
            let mut balance = vec![0; n];
            for i in 0..m {
                if (mask >> i) & 1 == 1 {
                    let from = requests[i][0] as usize;
                    let to = requests[i][1] as usize;
                    balance[from] -= 1;
                    balance[to] += 1;
                }
            }

            // Check if all buildings have a net change of 0
            if balance.iter().all(|&x| x == 0) {
                max_fulfilled = max_fulfilled.max(bit_count);
            }
        }

        max_fulfilled
    }
}
fn main() {}
