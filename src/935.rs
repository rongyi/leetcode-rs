struct Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let m = 1e9 as i64 + 7;
        let mut cur = vec![1; 10];
        let mut next = cur.clone();

        let n = n as usize;

        for i in 2..=n {
            next[0] = (cur[4] + cur[6]) % m;
            next[1] = (cur[6] + cur[8]) % m;
            next[2] = (cur[7] + cur[9]) % m;
            next[3] = (cur[4] + cur[8]) % m;
            next[4] = (cur[0] + cur[3] + cur[9]) % m;
            next[5] = 0;
            next[6] = (cur[0] + cur[1] + cur[7]) % m;
            next[7] = (cur[2] + cur[6]) % m;
            next[8] = (cur[1] + cur[3]) % m;
            next[9] = (cur[4] + cur[2]) % m;
            cur = next;
            next = cur.clone();
        }

        (cur.into_iter().sum::<i64>() % m) as i32
    }
}

fn main() {}
