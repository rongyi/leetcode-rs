#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut in_block = false;
        let mut cur = String::new();
        let mut ret = Vec::new();
        for s in source.into_iter() {
            let s: Vec<char> = s.chars().collect();
            let mut i = 0;
            let sz = s.len();
            while i < sz {
                if in_block {
                    if i + 1 < sz && s[i] == '*' && s[i + 1] == '/' {
                        in_block = false;
                        i += 2;
                    } else {
                        i += 1;
                    }
                } else {
                    if i + 1 < sz && s[i] == '/' && s[i + 1] == '*' {
                        in_block = true;
                        i += 2;
                    } else if i + 1 < sz && s[i] == '/' && s[i + 1] == '/' {
                        break;
                    } else {
                        cur.push(s[i]);
                        i += 1;
                    }
                }
            }

            if cur.len() > 0 && !in_block {
                ret.push(cur.clone());
                cur.clear();
            }
        }

        ret
    }
}

fn main() {
    let input = [
        "/*Test program */",
        "int main()",
        "{ ",
        "  // variable declaration ",
        "int a, b, c;",
        "/* This is a test",
        "   multiline  ",
        "   comment for ",
        "   testing */",
        "a = b + c;",
        "}",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    Solution::remove_comments(input);
}
