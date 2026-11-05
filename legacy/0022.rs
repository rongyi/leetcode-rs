struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let mut cur: String = String::new();

        Self::backtrack(&mut ret, &mut cur, n, n);
        ret
    }

    fn backtrack(ret: &mut Vec<String>, cur: &mut String, open: i32, close: i32) {
        if open == 0 && close == 0 {
            ret.push(cur.clone());
            return;
        }
        if open > 0 {
            cur.push('(');
            Self::backtrack(ret, cur, open - 1, close);
            cur.pop();
        }
        // can not use too much )
        if close > open {
            cur.push(')');
            Self::backtrack(ret, cur, open, close - 1);
            cur.pop();
        }
    }
}
