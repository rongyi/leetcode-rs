struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Self::recur(&expression.as_bytes())
    }
    fn recur(expr: &[u8]) -> Vec<i32> {
        let mut ret = vec![];
        for (i, &c) in expr.iter().enumerate() {
            if c == b'+' || c == b'-' || c == b'*' {
                let ls = Self::recur(&expr[0..i]);
                let rs = Self::recur(&expr[i + 1..]);
                for &vl in ls.iter() {
                    for &vr in rs.iter() {
                        let cur = match c {
                            b'+' => vl + vr,
                            b'-' => vl - vr,
                            b'*' => vl * vr,
                            _ => unreachable!(),
                        };
                        ret.push(cur);
                    }
                }
            }
        }

        if ret.is_empty() {
            ret.push(String::from_utf8_lossy(expr).parse().unwrap());
        }

        ret
    }
}
fn main() {}
