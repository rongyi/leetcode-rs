struct Solution;
impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let n = nums.len();

        // Step 1: Find the median
        let median = if n % 2 == 1 {
            nums[n / 2] as i64
        } else {
            // Either of the two middle values works as a starting point
            // to search for the nearest palindrome.
            (nums[n / 2] + nums[n / 2 - 1]) as i64 / 2
        };

        // Step 2: Find the closest palindromes to the median
        let mut candidates = Vec::new();
        let s = median.to_string();
        let len = s.len();

        // Generate a palindrome by mirroring the left half
        let half = &s[0..(len + 1) / 2];
        let h_val = half.parse::<i64>().unwrap();

        // Check the mirrored version of h_val, h_val - 1, and h_val + 1
        // to cover cases like 100 -> 99 or 99 -> 101
        for i in [h_val - 1, h_val, h_val + 1] {
            let res = i.to_string();
            let mut rev = res.chars().rev().collect::<String>();

            let p = if len % 2 == 0 {
                format!("{}{}", res, rev)
            } else {
                // If odd length, skip the last char of the prefix when mirroring
                format!("{}{}", res, &rev[1..])
            };

            if let Ok(p_val) = p.parse::<i64>() {
                candidates.push(p_val);
            }
        }

        // Edge cases: handles 10^n - 1 (like 999) and 10^n + 1 (like 1001)
        candidates.push(10i64.pow(len as u32 - 1) - 1);
        candidates.push(10i64.pow(len as u32) + 1);

        // Step 3: Calculate the minimum cost
        candidates
            .into_iter()
            .map(|p| nums.iter().map(|&num| (num as i64 - p).abs()).sum::<i64>())
            .min()
            .unwrap_or(0)
    }
}

fn main() {}
