struct Solution;

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        fn f(s: &str) -> f64 {
            let i = s.find('(');
            if let Some(i) = i {
                let base = &s[..i];
                let rep = &s[i + 1..s.len() - 1];
                let mut base = base.to_string();
                for _ in 0..20 {
                    base.push_str(rep);
                }
                base.parse().unwrap()
            } else {
                s.parse().unwrap()
            }
        }

        f(&s) == f(&t)
    }
}

fn main() {}
