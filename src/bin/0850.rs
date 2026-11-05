struct Solution;

mod ai {
    struct Solution;
    use std::collections::HashSet;

    impl Solution {
        pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
            const MOD: i64 = 1_000_000_007;

            // Collect all unique x-coordinates
            let mut xs: HashSet<i64> = HashSet::new();
            for rect in &rectangles {
                xs.insert(rect[0] as i64);
                xs.insert(rect[2] as i64);
            }
            let mut xs: Vec<i64> = xs.into_iter().collect();
            xs.sort();

            // Collect all unique y-coordinates
            let mut ys: HashSet<i64> = HashSet::new();
            for rect in &rectangles {
                ys.insert(rect[1] as i64);
                ys.insert(rect[3] as i64);
            }
            let mut ys: Vec<i64> = ys.into_iter().collect();
            ys.sort();

            // Use a 2D array to mark coverage
            let m = xs.len();
            let n = ys.len();
            let mut covered = vec![vec![false; n - 1]; m - 1];

            for rect in &rectangles {
                let x1 = rect[0] as i64;
                let y1 = rect[1] as i64;
                let x2 = rect[2] as i64;
                let y2 = rect[3] as i64;

                let xi1 = xs.binary_search(&x1).unwrap();
                let xi2 = xs.binary_search(&x2).unwrap();
                let yi1 = ys.binary_search(&y1).unwrap();
                let yi2 = ys.binary_search(&y2).unwrap();

                for i in xi1..xi2 {
                    for j in yi1..yi2 {
                        covered[i][j] = true;
                    }
                }
            }

            let mut area: i64 = 0;
            for i in 0..m - 1 {
                for j in 0..n - 1 {
                    if covered[i][j] {
                        let width = xs[i + 1] - xs[i];
                        let height = ys[j + 1] - ys[j];
                        area = (area + width * height) % MOD;
                    }
                }
            }

            area as i32
        }
    }
}

use std::collections::BTreeMap;

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut sortx: BTreeMap<i32, Vec<(i32, i32, i32)>> = BTreeMap::new();
        for rec in rectangles.iter() {
            let (x1, y1, x2, y2) = (rec[0], rec[1], rec[2], rec[3]);
            sortx.entry(x1).or_default().push((y1, y2, 1));
            sortx.entry(x2).or_default().push((y1, y2, -1));
        }
        let mut y_acc: BTreeMap<i32, i32> = BTreeMap::new();

        let mut ret = 0;
        let mut prev_x = -1;
        for (&cur_x, y_dots) in sortx.iter() {
            // there's a valid gap between two x
            if prev_x >= 0 && cur_x - prev_x > 0 {
                let mut sum_y = 0;
                let mut sum = 0;
                let mut start = i32::MIN;

                for (&y, &cnt) in y_acc.iter() {
                    if cnt == 0 {
                        continue;
                    }
                    if start == i32::MIN {
                        start = y;
                    }
                    sum += cnt;
                    if sum == 0 {
                        sum_y += y - start;
                        start = i32::MIN;
                    }
                }
                ret += ((cur_x - prev_x) as i64 * sum_y as i64) % Self::MOD;
                ret %= Self::MOD;
            }

            for &(y1, y2, start_end) in y_dots.iter() {
                *y_acc.entry(y1).or_default() += start_end;
                *y_acc.entry(y2).or_default() += -start_end;
            }

            prev_x = cur_x;
        }

        ret as i32
    }
}

fn main() {}
