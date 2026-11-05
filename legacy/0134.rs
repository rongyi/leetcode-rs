struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_gas = 0;
        let mut start_station = 0;
        let mut cur_gas = 0;

        for i in 0..n {
            let diff = gas[i] - cost[i];
            total_gas += diff;
            cur_gas += diff;

            if cur_gas < 0 {
                cur_gas = 0;
                start_station = (i + 1) % n;
            }
        }

        if total_gas < 0 {
            return -1;
        }
        return start_station as i32;
    }
}

fn main() {}
