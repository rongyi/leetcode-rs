struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|l, r| l[1].cmp(&r[1]));
        let mut ret = 0;
        let mut i = 0;
        let mut arrow = 0;
        while i < points.len() {
            ret += 1;
            arrow = points[i][1];
            while i < points.len() && points[i][0] <= arrow {
                i += 1;
            }
        }

        ret
    }
}

fn main() {}
