struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let m = 1e9 as i64 + 7;
        let corridor = corridor.as_bytes();

        let seat_count: i32 = corridor.iter().filter(|x| **x == b'S').count() as _;

        if seat_count == 0 || seat_count % 2 != 0 {
            return 0;
        }

        let mut cur_count = 0;
        let mut prev_end = -1;

        let mut ret = 1;

        for i in 0..corridor.len() {
            if corridor[i] == b'S' {
                cur_count += 1;
                if cur_count == 2 {
                    cur_count = 0;
                    prev_end = i as i32;
                } else {
                    if prev_end != -1 {
                        ret *= (i as i32 - prev_end) as i64;
                        ret %= m;
                    }
                }
            }
        }

        (ret % m) as _
    }
}

fn main() {}
