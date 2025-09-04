
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        count.insert(0, 1);
        const MOD: i32 = 1_000_000_007;
        let mut people = 1;
        let mut growth = 0;

        for i in delay..n {
            if let Some(&cur) = count.get(&(i - delay)) {
                growth += cur;
                growth %= MOD;
            }
            if let Some(&remove) = count.get(&(i - forget)) {
                growth = (growth + MOD - remove) % MOD;
                people = (people + MOD - remove) % MOD;
            }
            count.insert(i, growth);
            people += growth;
            people %= MOD;
        }

        people
    }
}

fn main() {}
