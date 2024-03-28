struct Solution;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut dists = vec![
            Self::dist(&p1, &p2),
            Self::dist(&p1, &p3),
            Self::dist(&p1, &p4),
            Self::dist(&p2, &p3),
            Self::dist(&p2, &p4),
            Self::dist(&p3, &p4),
        ];
        dists.sort();
        println!("{:?}", dists);

        dists[0] > 0
            && dists[0] == dists[1]
            && dists[1] == dists[2]
            && dists[2] == dists[3]
            && dists[4] == dists[5] // two diagonal
            && dists[4] == 2 * dists[0]
    }
    fn dist(p1: &[i32], p2: &[i32]) -> i32 {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        dx * dx + dy * dy
    }
}

fn main() {}
