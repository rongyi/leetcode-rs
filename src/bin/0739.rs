struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            // While the stack is not empty and the current temperature is warmer
            // than the temperature at the index on top of the stack
            while !stack.is_empty() && temperatures[i] > temperatures[*stack.last().unwrap()] {
                let prev_index = stack.pop().unwrap();
                result[prev_index] = (i - prev_index) as i32;
            }
            // Push current index onto the stack
            stack.push(i);
        }

        result
    }
}

fn main() {}
