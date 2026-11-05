struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut lst = vec![a, b, c];
        lst.sort();
        let a = lst[0];
        let b = lst[1];
        let c = lst[2];
        if c - a == 2 {
            return vec![0, 0];
        }
        let mut min_move = 2;
        if (b - a).min(c - b) <= 2 {
            min_move = 1;
        }
        vec![min_move, c - a - 2]
    }
}

fn main() {}
