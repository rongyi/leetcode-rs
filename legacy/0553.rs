struct Solution;

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let mut ret = String::new();
        let sz = nums.len();
        ret.push_str(&nums[0].to_string());
        if sz == 1 {
            return ret;
        }
        if sz == 2 {
            ret.push('/');
            ret.push_str(&nums[1].to_string());
            return ret;
        }
        ret.push_str("/(");
        ret.push_str(&nums[1].to_string());

        for i in 2..sz {
            ret.push('/');
            ret.push_str(&nums[i].to_string());
        }

        ret.push(')');
        ret
    }
}

fn main() {}
