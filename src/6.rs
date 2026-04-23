struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut grid: Vec<Vec<u8>> = vec![vec![]; num_rows as usize];

        let mut is_down = true;
        let mut cur_row = 0;
        for &c in s.as_bytes().iter() {
            grid[cur_row].push(c);
            if is_down {
                cur_row += 1;
                if cur_row == grid.len() {
                    is_down = false;
                    cur_row = grid.len() - 2;
                }
            } else {
                // try to decrease usize
                if cur_row == 0 {
                    cur_row = 1;
                    is_down = true;
                } else {
                    cur_row -= 1;
                }
            }
        }

        grid.into_iter()
            .map(|r| String::from_utf8(r).unwrap())
            .fold(String::new(), |mut acc, cur| {
                acc.push_str(&cur);
                acc
            })
    }
}

fn main() {
    Solution::convert("PAYPALISHIRING".to_string(), 3);
}
