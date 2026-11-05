struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let val = tickets[k as usize];
        let mut sum = 0;

        for &t in tickets.iter() {
            sum += t.min(val);
        }

        for i in (k + 1) as usize..tickets.len() {
            if tickets[i] >= val {
                sum -= 1;
            }
        }

        sum
    }
}

fn main() {}
