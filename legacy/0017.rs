struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let mut ret = Vec::new();
        // 0 1 is empty, see the picture for detail
        let digit_map = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut cur_gen = String::new();
        let digits: Vec<char> = digits.chars().collect();

        Self::backtrack(&digits, &digit_map, 0, &mut cur_gen, &mut ret);

        return ret;
    }

    fn backtrack(
        digits: &[char],
        digit_map: &[&str],
        cur_idx: usize,
        cur_gen: &mut String,
        ret: &mut Vec<String>,
    ) {
        if cur_idx == digits.len() {
            ret.push(cur_gen.clone());
            return;
        }
        let d = digits[cur_idx].to_digit(10).unwrap() as usize;
        for c in digit_map[d].chars() {
            // take current
            cur_gen.push(c);

            Self::backtrack(digits, digit_map, cur_idx + 1, cur_gen, ret);

            // don't take current
            cur_gen.pop();
        }
    }
}
