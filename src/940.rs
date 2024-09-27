struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut end_with = vec![0; 26];
        let mut total = 0;

        for c in s.bytes() {
            let index = (c - b'a') as usize;
            let new_count = (total + 1) % MOD;
            total = ((total + new_count - end_with[index]) % MOD + MOD) % MOD;
            end_with[index] = new_count;
        }

        total
    }
}

fn main() {}
