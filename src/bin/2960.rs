struct Solution;

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut tested = 0;
        let mut decrement = 0;

        for &battery in battery_percentages.iter() {
            if battery - decrement > 0 {
                tested += 1;
                decrement += 1;
            }
        }

        tested
    }
}

fn main() {}
