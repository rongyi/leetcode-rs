struct Solution;

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let ps = Self::primes();
        let sz = nums.len();
        let mut next = nums[sz - 1];
        for i in (0..sz - 1).rev() {
            if nums[i] < next {
                next = nums[i];
                continue;
            }
            let mut valid = false;

            for &p in ps.iter() {
                if p >= nums[i] {
                    break;
                }
                if nums[i] - p < next {
                    valid = true;
                    next = nums[i] - p;
                    break;
                }
            }
            if !valid {
                return false;
            }
        }
        true
    }
    // prime less then 1001
    fn primes() -> Vec<i32> {
        let mut flags = vec![true; 1001];
        flags[0] = false;
        flags[1] = false;

        for i in 2.. {
            if i * i >= 1001 {
                break;
            }
            let mut j = 2 * i;
            while j < 1001 {
                flags[j] = false;

                j += i;
            }
        }
        let mut ret = vec![];
        for (idx, is_prime) in flags.into_iter().enumerate() {
            if is_prime {
                ret.push(idx as i32);
            }
        }

        ret
    }
}
fn main() {}
