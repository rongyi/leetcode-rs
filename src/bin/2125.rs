struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut sum = 0;
        let mut prev = 0;
        for i in 0..bank.len() {
            let cur_cnt = bank[i].chars().filter(|c| *c == '1').count();
            if cur_cnt == 0 {
                continue;
            }

            sum += prev * cur_cnt;
            prev = cur_cnt;
        }

        sum as _
    }
}

fn main() {}
