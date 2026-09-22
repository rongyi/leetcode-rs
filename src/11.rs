struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() as i32 - 1;
        let mut max_area = 0;

        while i < j {
            let width = j - i;
            let v1 = height[i as usize];
            let v2 = height[j as usize];
            let height = v1.min(v2);
            max_area = max_area.max(width * height);
            if v1 < v2 {
                i += 1;
            } else {
                j -= 1;
            }
        }

        max_area
    }
}

fn main() {}
