struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_diag_sq = 0;
        let mut max_area = 0;

        for dim in dimensions {
            let l = dim[0] as i32;
            let w = dim[1] as i32;
            let diag_sq = l * l + w * w;
            let area = l * w;
            if diag_sq > max_diag_sq || (diag_sq == max_diag_sq && area > max_area) {
                max_diag_sq = diag_sq;
                max_area = area;
            }
        }

        max_area
    }
}

fn main() {}
