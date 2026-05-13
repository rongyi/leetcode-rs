struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut cur: Vec<i32> = vec![1];
        let mut next_round: Vec<i32> = vec![];

        for _ in 1..n {
            let sz = cur.len();
            let mut i = 0;
            while i < sz {
                let mut j = i + 1;
                while j < sz && cur[j] == cur[i] {
                    j += 1;
                }
                let cur_cnt = (j - i) as i32;
                next_round.push(cur_cnt);
                next_round.push(cur[i]);

                i = j;
            }

            cur = next_round;
            next_round = vec![];
        }

        cur.into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }
}

fn main() {}
