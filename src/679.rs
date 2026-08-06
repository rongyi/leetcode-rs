struct Solution;

const TARGET: f64 = 24.0;
const EPSILON: f64 = 1e-6;

impl Solution {
    /// LeetCode 679: 24 Game
    ///
    /// Use all 4 cards once with + - * / and parentheses to make 24.
    ///
    /// # Approach: backtracking
    ///
    /// Pick any two numbers, combine them with an operation, recurse with
    /// the remaining numbers. Base case: one number left ≈ 24.
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let nums: Vec<f64> = cards.into_iter().map(|v| v as f64).collect();
        Self::solve(&nums)
    }

    fn solve(nums: &[f64]) -> bool {
        if nums.len() == 1 {
            return (nums[0] - TARGET).abs() < EPSILON;
        }

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }

                // Remaining numbers, keeping nums[i] and nums[j] aside.
                let rest: Vec<f64> = nums
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| *k != i && *k != j)
                    .map(|(_, &v)| v)
                    .collect();

                // Try all 6 results: a+b, a-b, b-a, a*b, a/b, b/a.
                let mut results = vec![
                    nums[i] + nums[j],
                    nums[i] - nums[j],
                    nums[j] - nums[i],
                    nums[i] * nums[j],
                ];
                if nums[j].abs() > EPSILON {
                    results.push(nums[i] / nums[j]);
                }
                if nums[i].abs() > EPSILON {
                    results.push(nums[j] / nums[i]);
                }

                for r in results {
                    let mut next = rest.clone();
                    next.push(r);
                    if Self::solve(&next) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn main() {
    let tests = [
        (vec![4, 1, 8, 7], true), // (8-4) * (7-1) = 24
        (vec![1, 2, 1, 2], false),
        (vec![8, 1, 6, 6], true), // 6 / (1 - 6/8) = 24
        (vec![3, 3, 8, 8], true), // 8 / (3 - 8/3) = 24  ← classic hard one
        (vec![1, 5, 9, 1], false),
    ];

    for (cards, expected) in &tests {
        let result = Solution::judge_point24(cards.clone());
        println!(
            "{} cards={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            cards,
            result,
            expected
        );
    }
}
