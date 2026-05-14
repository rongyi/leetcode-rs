struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let sz = height.len();
        let mut left_bars = vec![0; sz];
        let mut right_bars = vec![0; sz];
        let mut max_left = 0;
        for (i, &cur_height) in height.iter().enumerate() {
            left_bars[i] = max_left;
            max_left = max_left.max(cur_height);
        }
        let mut acc = 0;

        // from right to left
        let mut max_right = 0;
        for (i, &cur_height) in height.iter().enumerate().rev() {
            right_bars[i] = max_right;

            max_right = max_right.max(cur_height);
        }
        for i in 1..sz - 1 {
            let min_heiht = left_bars[i].min(right_bars[i]);
            if min_heiht > height[i] {
                acc += min_heiht - height[i];
            }
        }

        acc
    }
}

fn main() {
    let input = vec![1, 2, 3];
    for (i, &val) in input.iter().enumerate().rev() {
        println!("{}, {}", i, val);
    }
}
