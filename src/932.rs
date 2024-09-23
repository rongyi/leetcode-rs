struct Solution;

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(n as usize);
        ret.push(1);
        while ret.len() < n as usize {
            ret = ret
                .iter()
                .map(|i| i * 2 - 1)
                .chain(ret.iter().map(|i| i * 2))
                .filter(|&i| i <= n)
                .collect();
        }
        ret
    }
}

fn main() {}
