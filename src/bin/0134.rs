struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let sz = gas.len();
        let mut start_pos = 0;
        let mut acc = 0;
        let mut start_acc = 0;

        for i in 0..sz {
            let diff = gas[i] - cost[i];
            acc += diff;
            start_acc += diff;
            if start_acc < 0 {
                start_pos = (i + 1) % sz;
                start_acc = 0;
            }
        }

        if acc < 0 {
            return -1;
        }

        start_pos as _
    }
}

fn main() {}
