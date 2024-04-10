struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for c in moves.chars() {
            match c {
                'L' => x -= 1,
                'R' => x += 1,
                'U' => y += 1,
                'D' => y -= 1,
                _ => unreachable!(),
            }
        }
        x == 0 && y == 0
    }
}
fn main() {}
