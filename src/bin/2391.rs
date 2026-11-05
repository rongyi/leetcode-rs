struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        // prepend 0 at the begin of travel, so prefix sum should be collect sum;
        // find right most index of one garbage
        let sz = garbage.len();
        let mut prefix = vec![0; sz];
        for i in 1..sz {
            prefix[i] += prefix[i - 1] + travel[i - 1];
        }

        let m: Vec<usize> = garbage
            .iter()
            .map(|s| s.chars().filter(|c| *c == 'M').count())
            .collect();
        let p: Vec<usize> = garbage
            .iter()
            .map(|s| s.chars().filter(|c| *c == 'P').count())
            .collect();
        let g: Vec<usize> = garbage
            .iter()
            .map(|s| s.chars().filter(|c| *c == 'G').count())
            .collect();
        let mut right_most_m = 0;
        let mut right_most_p = 0;
        let mut right_most_g = 0;

        for i in 0..sz {
            if m[i] != 0 {
                right_most_m = i;
            }
            if p[i] != 0 {
                right_most_p = i;
            }
            if g[i] != 0 {
                right_most_g = i;
            }
        }
        let collect_m = prefix[right_most_m] + m.into_iter().sum::<usize>() as i32;
        let collect_p = prefix[right_most_p] + p.into_iter().sum::<usize>() as i32;
        let collect_g = prefix[right_most_g] + g.into_iter().sum::<usize>() as i32;

        collect_m + collect_p + collect_g
    }
}

fn main() {}
