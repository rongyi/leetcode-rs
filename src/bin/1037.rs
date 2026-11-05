struct Solution;

impl Solution {
    pub fn is_boomerang(p: Vec<Vec<i32>>) -> bool {
        let area = p[0][0] * (p[1][1] - p[2][1])
            + p[1][0] * (p[2][1] - p[0][1])
            + p[2][0] * (p[0][1] - p[1][1]);
        return area != 0;
    }
}

fn main() {}
