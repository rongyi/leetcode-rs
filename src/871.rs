struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0i64; 501];
        dp[0] = start_fuel as i64;
        //              index, fuel
        for (i, station) in stations.iter().enumerate() {
            let mut t = i as i32;
            while t >= 0 && dp[t as usize] >= station[0] as i64 {
                dp[t as usize + 1] = dp[t as usize + 1].max(dp[t as usize] + station[1] as i64);
                t -= 1;
            }
        }

        for t in 0..=stations.len() {
            if dp[t] >= target as i64 {
                return t as i32;
            }
        }

        -1
    }
}

fn main() {}
