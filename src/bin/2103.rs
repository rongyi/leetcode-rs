struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let rings: Vec<char> = rings.chars().collect();
        let sz = rings.len();
        let mut red = vec![0; 10];
        let mut green = vec![0; 10];
        let mut blue = vec![0; 10];

        for i in (0..sz).step_by(2) {
            let idx = rings[i + 1].to_digit(10).unwrap() as usize;
            match rings[i] {
                'R' => red[idx] += 1,
                'G' => green[idx] += 1,
                'B' => blue[idx] += 1,
                _ => unreachable!(),
            }
        }
        let mut res = 0;
        for i in 0..10 {
            if red[i] > 0 && green[i] > 0 && blue[i] > 0 {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    for i in (0..10).step_by(2) {
        println!("{}", i);
    }
}
