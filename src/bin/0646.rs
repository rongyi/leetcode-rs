struct Solution;

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by(|l, r| {
            if l[1] == r[1] {
                return l[0].cmp(&r[0]);
            }
            l[1].cmp(&r[1])
        });
        let mut prev_end = pairs[0][1];
        let mut ret = 0;
        for i in 0..pairs.len() {
            if i == 0 || pairs[i][0] > prev_end {
                prev_end = pairs[i][1];
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
