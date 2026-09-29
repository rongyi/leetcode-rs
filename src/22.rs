struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = vec![];
        let mut cur: String = "".to_string();

        Self::recur(&mut ret, n, n, &mut cur);

        ret
    }

    fn recur(output: &mut Vec<String>, left: i32, right: i32, cur: &mut String) {
        if left == 0 && right == 0 {
            output.push(cur.clone());
            return;
        }

        if left < 0 || right < 0 {
            return;
        }
        // put more ) before (, which is invalid
        if left > right {
            return;
        }

        // 1. push (
        cur.push('(');
        Self::recur(output, left - 1, right, cur);
        cur.pop();

        // 2. push ), only when left < right, not equal
        if left < right {
            cur.push(')');
            Self::recur(output, left, right - 1, cur);
            cur.pop();
        }
    }
}

fn main() {}
