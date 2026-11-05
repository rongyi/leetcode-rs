struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let one_cnt: usize = nums.iter().filter(|&&x| x == 1).count();
        if one_cnt != 0 {
            return (sz - one_cnt) as _;
        }

        let mut ret = usize::MAX;
        for i in 0..sz {
            let mut cur = nums[i];
            for j in i + 1..sz {
                cur = Self::gcd(nums[j], cur);
                if cur == 1 {
                    ret = ret.min(j - i + sz - 1);
                }
            }
        }
        if ret == usize::MAX {
            return -1;
        }

        ret as _
    }
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }
}

fn main() {}
