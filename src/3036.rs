struct Solution;

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = pattern.len();

        // 1. Create the relationship array B
        let mut b = Vec::with_capacity(n - 1);
        for i in 0..n - 1 {
            if nums[i + 1] > nums[i] {
                b.push(1);
            } else if nums[i + 1] == nums[i] {
                b.push(0);
            } else {
                b.push(-1);
            }
        }

        // 2. Combine pattern and B with a separator that won't appear in the data
        // Using a value like 2 since pattern only contains -1, 0, 1
        let mut combined = pattern.clone();
        combined.push(2);
        combined.extend(b);

        // 3. Z-Algorithm
        let total_len = combined.len();
        let mut z = vec![0; total_len];
        let (mut l, mut r) = (0, 0);

        for i in 1..total_len {
            if i <= r {
                z[i] = std::cmp::min(r - i + 1, z[i - l]);
            }
            while i + z[i] < total_len && combined[z[i]] == combined[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] - 1 > r {
                l = i;
                r = i + z[i] - 1;
            }
        }

        // 4. Count matches
        let mut count = 0;
        // Matches start after the pattern (m) and the separator (1)
        for i in (m + 1)..total_len {
            if z[i] == m {
                count += 1;
            }
        }

        count
    }
}

fn main() {}
