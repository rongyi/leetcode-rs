struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let prev = Self::count_and_say(n - 1).chars().collect::<Vec<_>>();

        let mut ret = String::new();

        let mut count = 1;
        for i in 0..prev.len() {
            if i + 1 < prev.len() && prev[i + 1] == prev[i] {
                count += 1;
            } else {
                ret.push_str(&count.to_string());
                ret.push(prev[i]);
                count = 1;
            }
        }

        ret
    }
}
