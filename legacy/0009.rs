struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let cs = x.to_string().chars().collect::<Vec<char>>();
        let sz = cs.len();
        for i in 0..sz / 2 {
            if cs[i] != cs[sz - i - 1] {
                return false;
            }
        }

        true
    }
}
