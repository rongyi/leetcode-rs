struct Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let pos_sum: i32 = nums.iter().filter(|&&x| x >= 0).sum();
        let mut negs: Vec<i32> = nums.iter().filter(|&&x| x < 0).copied().collect();
        let neg_sz = negs.len() as i32;
        let min_abs = nums.iter().map(|&x| x.abs()).min().unwrap();

        if k == neg_sz {
            return pos_sum - negs.into_iter().sum::<i32>();
        }

        // k has some left
        if neg_sz < k {
            let left_flips = k - neg_sz;

            if left_flips % 2 == 1 {
                return pos_sum - negs.into_iter().sum::<i32>() - min_abs * 2;
            }

            return pos_sum - negs.into_iter().sum::<i32>();
        }
        // consume all k
        negs.sort_unstable();
        for i in 0..k as usize {
            negs[i] *= -1;
        }

        pos_sum + negs.into_iter().sum::<i32>()
    }
}

fn main() {}
