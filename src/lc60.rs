struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut ret = String::new();
        let mut nums = (1..=n).collect::<Vec<_>>();

        let mut facts = vec![1; (n + 1) as usize];

        for i in 1..=n {
            facts[i as usize] = facts[i as usize - 1] * i;
        }

        // start from zero
        let mut k = k - 1;

        for i in (1..=n).rev() {
            let index = k / facts[(i - 1) as usize];
            ret.push_str(&nums[index as usize].to_string());
            nums.remove(index as usize);
            k %= facts[(i - 1) as usize];
        }

        ret
    }
}
