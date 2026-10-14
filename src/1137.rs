struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut rolling = vec![0, 1, 1];
        let sz = n as usize;
        if sz < rolling.len() {
            return rolling[sz];
        }
        for _ in 3..=sz {
            let sum: i32 = rolling.iter().sum();
            rolling[0] = rolling[1];
            rolling[1] = rolling[2];
            rolling[2] = sum;
        }

        rolling[2]
    }
}

fn main() {}
