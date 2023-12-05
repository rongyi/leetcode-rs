struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let sz = height.len();
        let mut left_max = vec![0; sz];
        let mut right_max = vec![0; sz];
        let mut cur_max = 0;

        for i in 0..sz {
            left_max[i] = cur_max;
            cur_max = cur_max.max(height[i]);
        }
        cur_max = 0;
        for i in (0..sz).rev() {
            right_max[i] = cur_max;
            cur_max = cur_max.max(height[i]);
        }
        for i in 0..sz {
            let min_height = left_max[i].min(right_max[i]);
            if min_height > height[i] {
                water += min_height - height[i];
            }
        }

        water
    }
}
