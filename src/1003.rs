struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            stack.push(c);
            // 检查栈顶是否形成 "abc"
            let n = stack.len();
            if n >= 3 && stack[n - 3] == 'a' && stack[n - 2] == 'b' && stack[n - 1] == 'c' {
                stack.pop();
                stack.pop();
                stack.pop();
            }
        }

        stack.is_empty()
    }
}
fn main() {}
