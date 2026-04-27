struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut i = 0;
        let mut j = height.len() - 1;
        while i < j {
            let w = (j - i) as i32;
            let h = height[j].min(height[i]);
            let cur = w * h;
            max_area = max_area.max(cur);

            // keep large one
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        max_area
    }
}

fn main() {}
