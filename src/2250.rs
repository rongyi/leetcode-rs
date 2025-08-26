struct Solution;

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut group_by_y = vec![vec![]; 101];
        for r in rectangles.iter() {
            let (x, y) = (r[0], r[1]);
            group_by_y[y as usize].push(x);
        }
        for xs in group_by_y.iter_mut() {
            xs.sort_unstable();
        }

        let mut ret = vec![0; points.len()];

        for (i, p) in points.iter().enumerate() {
            let mut sum = 0;
            for j in points[i][1] as usize..group_by_y.len() {
                // all the points x >= p[0] is valid, so partition point should be *x < p[0], which
                // is cpp's lower_bound... of p[0]
                let it = group_by_y[j].partition_point(|x| *x < p[0]);
                sum += (group_by_y[j].len() - it) as i32;
            }
            ret[i] = sum;
        }

        ret
    }
}

fn main() {
    let input = vec![1, 2, 2, 2, 3, 3, 4];
    let idx = input.partition_point(|x| *x < 2);
    println!("idx: {}", idx);
}
