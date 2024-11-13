struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Prefix {
    Or,
    Not,
    And,
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let expr: Vec<char> = expression.chars().collect();
        let mut pos = 0;

        Self::parse(&expr, &mut pos)
    }

    fn parse(expr: &[char], pos: &mut usize) -> bool {
        match expr[*pos] {
            'f' | 't' => Self::parse_atom(expr, pos),
            '&' => {
                *pos += 1;
                Self::parse_list(expr, pos, Prefix::And)
            }
            '|' => {
                *pos += 1;
                Self::parse_list(expr, pos, Prefix::Or)
            }
            '!' => {
                *pos += 1;
                Self::parse_list(expr, pos, Prefix::Not)
            }
            _ => unreachable!(),
        }
    }

    fn parse_atom(expr: &[char], pos: &mut usize) -> bool {
        match expr[*pos] {
            'f' => {
                *pos += 1;
                false
            }
            't' => {
                *pos += 1;
                true
            }
            _ => unreachable!(),
        }
    }

    fn parse_list(expr: &[char], pos: &mut usize, prefix: Prefix) -> bool {
        // jump over (
        *pos += 1;
        let mut acc: Vec<bool> = Vec::new();

        while *pos < expr.len() {
            match expr[*pos] {
                '&' => {
                    *pos += 1;
                    let cur = Self::parse_list(expr, pos, Prefix::And);
                    acc.push(cur);
                }
                '|' => {
                    *pos += 1;
                    let cur = Self::parse_list(expr, pos, Prefix::Or);
                    acc.push(cur);
                }
                '!' => {
                    *pos += 1;
                    let cur = Self::parse_list(expr, pos, Prefix::Not);
                    acc.push(cur);
                }
                'f' | 't' => {
                    let cur = Self::parse_atom(expr, pos);
                    acc.push(cur);
                }
                ',' => {
                    *pos += 1;
                    // no value gernated
                }
                // end
                ')' => {
                    break;
                }
                _ => panic!("not ok"),
            };
        }

        // jump over )
        *pos += 1;
        println!("{:?}", acc);

        match prefix {
            Prefix::Or => acc.into_iter().any(|t| t),
            Prefix::And => acc.into_iter().all(|t| t),
            Prefix::Not => {
                if acc.len() != 1 {
                    panic!("not right")
                }
                !acc[0]
            }
        }
    }
}

fn main() {
    let val = Solution::parse_bool_expr("|(&(t,f,t),!(t))".to_string());
    println!("{val}");
}
