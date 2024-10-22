struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let ret = Self::check(tops[0], &tops, &bottoms);

        if ret != -1 {
            return ret;
        }

        Self::check(bottoms[0], &tops, &bottoms)
    }

    fn check(x: i32, tops: &Vec<i32>, bottoms: &Vec<i32>) -> i32 {
        let mut top_rotations = 0;
        let mut bottom_rotations = 0;

        for i in 0..tops.len() {
            if tops[i] != x && bottoms[i] != x {
                return -1;
            } else if tops[i] != x {
                top_rotations += 1;
            } else if bottoms[i] != x {
                bottom_rotations += 1;
            }
        }
        top_rotations.min(bottom_rotations)
    }
}
fn main() {}
