struct Solution;

// shitty problem
impl Solution {
    pub fn strong_password_checker(s: String) -> i32 {
        let req_char = Self::get_required_char(&s);
        if s.len() < 6 {
            return req_char.max(6 - s.len() as i32);
        }

        // total replacements for repeated chars. e.g. "aaa" needs 1 replacement to fix
        let mut replace = 0i32;
        // total deletions for 3n repeated chars. e.g. "aaa" needs 1 deletion to fix
        let mut oned = 0;
        // total deletions for 3n+1 repeated chars. e.g. "aaaa" needs 2 deletions to fix.
        let mut twod = 0;

        let mut i = 0;
        let s: Vec<char> = s.chars().collect();
        while i < s.len() {
            let mut len = 1;
            while i + len < s.len() && s[i + len] == s[i + len - 1] {
                len += 1;
            }
            if len >= 3 {
                replace += (len / 3) as i32;
                if len % 3 == 0 {
                    oned += 1;
                }
                if len % 3 == 1 {
                    twod += 2;
                }
            }
            i += len;
        }
        if s.len() <= 20 {
            return req_char.max(replace);
        }
        let delete_count = s.len() as i32 - 20;

        replace -= delete_count.min(oned);
        replace -= twod.min((delete_count - oned).max(0)) / 2;
        replace -= (delete_count - oned - twod).max(0) / 3;

        delete_count + req_char.max(replace)
    }

    fn get_required_char(s: &str) -> i32 {
        let mut lower_case = 1;
        let mut upper_case = 1;
        let mut digit = 1;

        for c in s.chars() {
            if c.is_lowercase() {
                lower_case = 0;
            } else if c.is_uppercase() {
                upper_case = 0;
            } else if c.is_digit(10) {
                digit = 0;
            }
        }
        lower_case + upper_case + digit
    }
}

fn main() {}
