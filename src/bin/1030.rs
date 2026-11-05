struct Solution;

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut cells = Vec::with_capacity((rows * cols) as usize);

        for i in 0..rows {
            for j in 0..cols {
                cells.push(vec![i, j]);
            }
        }
        cells.sort_by(|l, r| {
            let d1 = (l[0] - r_center).abs() + (l[1] - c_center).abs();
            let d2 = (r[0] - r_center).abs() + (r[1] - c_center).abs();
            d1.cmp(&d2)
        });
        cells
    }
}

fn main() {}
