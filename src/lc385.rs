struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if !s.contains('[') {
            return NestedInteger::Int(s.parse::<i32>().unwrap());
        }

        let mut pos = 1;
        let s: Vec<char> = s.chars().collect();
        Self::recur(&mut pos, &s)
    }
    fn recur(pos: &mut usize, s: &[char]) -> NestedInteger {
        let mut d = String::new();
        let mut ret = Vec::new();
        while *pos < s.len() {
            let c = s[*pos];
            if c == '[' {
                *pos += 1;
                let tmp = Self::recur(pos, s);
                ret.push(tmp);
            } else if c == ']' {
                if !d.is_empty() {
                    ret.push(NestedInteger::Int(d.parse::<i32>().unwrap()));
                }
                d.clear();
                return NestedInteger::List(ret);
            } else if c == '-' || c.is_digit(10) {
                d.push(c);
            } else if c == ',' {
                if !d.is_empty() {
                    ret.push(NestedInteger::Int(d.parse::<i32>().unwrap()));
                }
                d.clear();
            }

            *pos += 1;
        }

        NestedInteger::List(ret)
    }
}



fn main() {}
