struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut fact = vec![1; n as usize];
        for i in 1..n as usize {
            fact[i] = i * fact[i - 1];
        }
        let mut ret = String::new();

        let mut table: Vec<char> = (1..=n)
            .map(|i| char::from_digit(i as u32, 10).unwrap())
            .collect();

        // 1 index to 0index
        let mut k = k as usize - 1;

        for i in (1..=n as usize).rev() {
            let cur_idx = k / fact[i - 1];
            k %= fact[i - 1];
            ret.push(table.remove(cur_idx as usize));
        }

        ret
    }
}

fn main() {}
