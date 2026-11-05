struct Solution;

impl Solution {
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut miles = 0;

        while main_tank > 0 {
            if main_tank < 5 {
                miles += main_tank * 10;
                main_tank = 0;
            } else {
                let round = main_tank / 5;
                miles += 50 * round;
                main_tank = main_tank % 5;
                let sink = round.min(additional_tank);
                main_tank += sink;
                additional_tank -= sink;
            }
        }

        miles
    }
}

fn main() {}
