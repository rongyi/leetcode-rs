struct Solution;

impl Solution {
    pub fn min_moves(mut target: i32, mut max_doubles: i32) -> i32 {
        let mut op_count = 0;
        while max_doubles > 0 && target > 1 {
            if target % 2 != 0 {
                target -= 1;
                op_count += 1;
            }
            max_doubles -= 1;
            op_count += 1;
            target /= 2;
        }

        op_count + (target - 1)
    }
}

fn main() {}
