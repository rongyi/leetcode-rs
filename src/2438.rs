struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut bits = vec![];
        for i in 0..31 {
            if (1 << i) & n != 0 {
                bits.push(i);
            }
        }
        let mut prefix = vec![0; bits.len() + 1];
        for (i, &b) in bits.iter().enumerate() {
            prefix[i + 1] = prefix[i] + b;
        }
        let mut ret = vec![];

        fn twexpmod(n: i32) -> i32 {
            let mut ret = 1i64;

            for _i in 1..=n {
                ret *= 2;
                ret %= 1_000_000_007;
            }

            ret as _
        }

        for q in queries.into_iter() {
            let sum = prefix[q[1] as usize + 1] - prefix[q[0] as usize - 1 + 1];
            ret.push(twexpmod(sum));
        }

        ret
    }
}

fn main() {}
