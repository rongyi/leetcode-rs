struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let sz = score.len();
        let mut ret: Vec<String> = vec![String::new(); sz];
        // (score, origin_index)
        let mut score_with_idx: Vec<(i32, usize)> = Vec::with_capacity(sz);
        for (i, &s) in score.iter().enumerate() {
            score_with_idx.push((s, i));
        }
        score_with_idx.sort_by(|&l, &r| r.0.cmp(&l.0));
        for (i, &(_s, origin_index)) in score_with_idx.iter().enumerate() {
            if i == 0 {
                ret[origin_index] = "Gold Medal".to_string();
            } else if i == 1 {
                ret[origin_index] = "Silver Medal".to_string();
            } else if i == 2 {
                ret[origin_index] = "Bronze Medal".to_string();
            } else {
                ret[origin_index] = (i + 1).to_string();
            }
        }

        ret
    }
}

fn main() {}
