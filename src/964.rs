struct Solution;

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        if target < x {
            return (target * 2 - 1).min((x - target) * 2);
        }

        if x == target {
            return 0;
        }
        let mut multi: i64 = x as i64;
        let mut num_op = 0;
        let target = target as i64;
        while multi < target {
            num_op += 1;
            multi *= x as i64;
        }
        if multi == target {
            return num_op;
        }

        let mut l = i32::MAX;
        let mut r = i32::MAX;
        if multi - target < target {
            l = Self::least_ops_express_target(x, (multi - target) as i32) + num_op;
        }

        r = Self::least_ops_express_target(x, (target - (multi / x as i64)) as i32) + num_op - 1;

        (l.min(r) + 1) as i32
    }
}

fn main() {}
