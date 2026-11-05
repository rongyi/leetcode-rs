struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = vec![0; 60];
        let mut ret = 0;

        for t in time.into_iter() {
            ret += count[(((60 - t) % 60 + 60) % 60) as usize];
            count[(t % 60) as usize] += 1;
        }

        ret
    }
}

fn main() {}
