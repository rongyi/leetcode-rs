struct Solution;

impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        let primes: Vec<i32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let sz = nums.len();
        // total 10 prime and we don't use the lowest value
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; 1 << 11]; sz];

        (Self::dfs(&nums, 0, 1, &mut dp, &primes) - 1 + 1_000_000_007) % 1_000_000_007
    }

    fn dfs(
        nums: &Vec<i32>,
        idx: usize,
        cur_mask: i64,
        dp: &mut Vec<Vec<i32>>,
        primes: &Vec<i32>,
    ) -> i32 {
        if idx >= nums.len() {
            return 1;
        }
        if dp[idx][cur_mask as usize] != -1 {
            return dp[idx][cur_mask as usize];
        }
        let mask = Self::get_mask(nums[idx], primes);
        // dont take current
        let mut ret = Self::dfs(nums, idx + 1, cur_mask, dp, primes);
        // take current
        if (cur_mask & mask) == 0 {
            ret = (ret + Self::dfs(nums, idx + 1, cur_mask | mask, dp, primes)) % 1_000_000_007;
        }
        dp[idx][cur_mask as usize] = ret;
        ret
    }

    fn get_mask(mut val: i32, primes: &Vec<i32>) -> i64 {
        let mut mask = 0;

        for (i, &p) in primes.iter().enumerate() {
            let mut rep = 0;
            while val % p == 0 {
                rep += 1;
                val /= p;
            }
            if rep > 1 {
                return -1;
            }
            if rep == 1 {
                mask |= 1 << (i + 1);
            }
        }

        mask
    }
}

fn main() {
    println!("{}", 8 & -1);
}
