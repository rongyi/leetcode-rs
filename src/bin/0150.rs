struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut vals = vec![];
        for tk in tokens.into_iter() {
            match tk.as_ref() {
                "+" => {
                    let a: i32 = vals.pop().unwrap();
                    let b = vals.pop().unwrap();
                    vals.push(b + a);
                }
                "-" => {
                    let a: i32 = vals.pop().unwrap();
                    let b = vals.pop().unwrap();
                    vals.push(b - a);
                }
                "*" => {
                    let a: i32 = vals.pop().unwrap();
                    let b = vals.pop().unwrap();
                    vals.push(b * a);
                }
                "/" => {
                    let a: i32 = vals.pop().unwrap();
                    let b = vals.pop().unwrap();
                    vals.push(b / a);
                }
                _ => {
                    let v: i32 = tk.parse().unwrap();
                    vals.push(v);
                }
            }
        }
        *vals.last().unwrap()
    }
}

fn main() {}
