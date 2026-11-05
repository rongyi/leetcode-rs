struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut sorted: Vec<char> = s.chars().collect();
        let mut voews: Vec<char> = sorted
            .iter()
            .filter(|&&x| "aoeiuAOEIU".contains(x))
            .copied()
            .collect();
        voews.sort_unstable();
        let mut ret = String::new();
        let mut it = voews.into_iter();
        for c in sorted.into_iter() {
            if "aoeiuAOEIU".contains(c) {
                ret.push(it.next().unwrap());
            } else {
                ret.push(c);
            }
        }
        ret
    }
}

fn main() {
    Solution::sort_vowels("lEetcOde".to_string());
}
