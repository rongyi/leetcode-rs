struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut ret = vec![];
        let mut placement = vec![0; n as usize];
        Self::backtrack(&mut placement, 0, &mut ret);

        ret
    }

    fn backtrack(placement: &mut [i32], cur_row: usize, ret: &mut Vec<Vec<String>>) {
        if cur_row == placement.len() {
            let mut one_solution = Vec::new();
            for i in 0..placement.len() {
                let mut dots = vec!['.'; placement.len()];
                dots[placement[i] as usize] = 'Q';
                let s: String = dots.iter().collect();
                one_solution.push(s);
            }
            ret.push(one_solution);
            return;
        }

        for col in 0..placement.len() {
            if Self::is_valid(placement, cur_row, col) {
                placement[cur_row] = col as i32;

                Self::backtrack(placement, cur_row + 1, ret);
                placement[cur_row] = 0;
            }
        }
    }

    fn is_valid(placement: &mut [i32], cur_row: usize, cur_col: usize) -> bool {
        for i in 0..cur_row {
            if placement[i] == cur_col as i32
                || (i as i32 - cur_row as i32).abs() == (cur_col as i32 - placement[i]).abs()
            {
                return false;
            }
        }

        true
    }
}
