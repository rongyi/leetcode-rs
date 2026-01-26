struct Solution;

const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut prim_divisors = vec![0; max_num + 1];
        // sieve filter
        for i in 2..=max_num {
            if prim_divisors[i] == 0 {
                for j in (i..=max_num).step_by(i) {
                    prim_divisors[j] += 1;
                }
            }
        }
        let scores: Vec<i32> = nums.iter().map(|&x| prim_divisors[x as usize]).collect();

        let sz = nums.len();
        let mut left = vec![-1i32; sz];
        let mut right = vec![sz as i32; sz];
        let mut stack = Vec::new();

        for i in 0..sz {
            // find current score[i] as max, so little bitch need to fuckoff
            while !stack.is_empty() && scores[*stack.last().unwrap()] < scores[i] {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                left[i] = top as i32;
            }
            stack.push(i);
        }

        stack.clear();

        for i in (0..sz).rev() {
            // notice the difference? with equal case, so max dont include tie condition, let
            // them be alone in a seprated group,
            while !stack.is_empty() && scores[*stack.last().unwrap()] <= scores[i] {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                right[i] = top as i32;
            }
            stack.push(i);
        }
        let mut items: Vec<(i32, usize)> =
            nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        items.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut ret: i64 = 1;
        let mut k = k as i64;
        for (val, i) in items.into_iter() {
            let count = (i as i32 - left[i]) as i64 * (right[i] - i as i32) as i64;
            let take = count.min(k);
            if take > 0 {
                ret = (ret * Self::pow(val as i64, take)) % MOD;
                k -= take;
            }
            if k <= 0 {
                break;
            }
        }

        ret as i32
    }
    fn pow(mut base: i64, mut exp: i64) -> i64 {
        let mut ret = 1;
        base %= MOD;

        while exp > 0 {
            if exp % 2 == 1 {
                ret = (ret * base) % MOD;
            }
            base = (base * base) % MOD;
            exp /= 2;
        }

        ret
    }
}
fn main() {}
