struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if nums.len() % 2 == 0 {
            return true;
        }
        let mut dp = vec![vec![-1; nums.len()]; nums.len()];
        let my_best = Self::recur(&nums, &mut dp, 0, nums.len() as i32 - 1);

        // >= one half of sum
        2 * my_best >= sum
    }

    fn recur(v: &[i32], dp: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        if i > j {
            return 0;
        }
        if dp[i as usize][j as usize] != -1 {
            return dp[i as usize][j as usize];
        }

        // (i, j) 此时轮到自己走，分两种情况
        // 1. 我取 v[i] , 那么剩下 (i + 1, j) 以及轮到对手走，对手可以取 v[i + 1],
        // 也可以取 v[j]，那么取哪个呢，肯定是要剩下最小的那个
        // recur(v, dp, i + 1, j - 1) ==> 对手此时取 v[j]
        // recur(v, dp, i + 2, j) ==> 对手此时取 v[i + 1]

        // 2. I pick v[j] the save as above, opponent have (i, j - 1)
        // recur(v, dp, i, j - 2) ==> opponent pick v[j - 1]
        // recur(v, dp, i + 1, j - 1) ==> opponent pick v[i]
        // 对手和你一样强，所以自己获得的肯定是最小的
        let a = v[i as usize] + Self::recur(v, dp, i + 1, j - 1).min(Self::recur(v, dp, i + 2, j));
        // omit the next turn of our opponent
        let b = v[j as usize] + Self::recur(v, dp, i, j - 2).min(Self::recur(v, dp, i + 1, j - 1));

        dp[i as usize][j as usize] = a.max(b);

        dp[i as usize][j as usize]
    }
}

fn main() {}
