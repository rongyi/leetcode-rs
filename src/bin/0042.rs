struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;
        let sz = height.len();
        let mut left_max = vec![0; sz];
        let mut right_max = vec![0; sz];

        let mut val = 0;
        for i in 0..sz {
            left_max[i] = val;
            val = val.max(height[i]);
        }

        val = 0;
        for i in (0..sz).rev() {
            right_max[i] = val;
            val = val.max(height[i]);
        }

        for i in 1..sz - 1 {
            let diff = (left_max[i].min(right_max[i]) - height[i]).max(0);
            sum += diff;
        }

        sum
    }
}

fn main() {}
