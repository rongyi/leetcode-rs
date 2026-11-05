struct Solution;

impl Solution {
    pub fn minimum_total(mut tri: Vec<Vec<i32>>) -> i32 {
        if tri.len() <= 1 {
            return tri[0][0];
        }
        tri[1][0] += tri[0][0];
        tri[1][1] += tri[0][0];

        for i in 2..tri.len() {
            let sz = tri[i].len();
            tri[i][0] += tri[i - 1][0];
            tri[i][sz - 1] += *tri[i - 1].last().unwrap();
            for j in 1..sz - 1 {
                tri[i][j] += tri[i - 1][j].min(tri[i - 1][j - 1]);
            }
        }

        tri.last().unwrap().iter().copied().min().unwrap()
    }
}

fn main() {}
