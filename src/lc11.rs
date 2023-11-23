struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;
        while l < r {
            let cur_area = (r - l) as i32 * height[l].min(height[r]);

            max_area = max_area.max(cur_area);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_area
    }
}
