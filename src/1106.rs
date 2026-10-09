struct Solution;

#[derive(Debug, Clone, Copy)]
enum Expr {
    And,
    Or,
    Not,
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let s = expression.as_bytes();
        // "global shared"
        let mut pos = 0;
        Self::parse(s, &mut pos)
    }
    fn parse(expr: &[u8], pos: &mut usize) -> bool {
        match expr[*pos] {
            b't' | b'f' => Self::parse_atom(expr, pos),
            b'&' => Self::parse_list(expr, pos, Expr::And),
            b'|' => Self::parse_list(expr, pos, Expr::Or),
            b'!' => Self::parse_list(expr, pos, Expr::Not),
            _ => unreachable!(),
        }
    }

    fn parse_list(expr: &[u8], pos: &mut usize, kind: Expr) -> bool {
        // jump over left & | !
        *pos += 1;
        // jump over (
        *pos += 1;

        let mut acc = vec![];
        let sz = expr.len();
        while *pos < sz {
            match expr[*pos] {
                b't' | b'f' => {
                    let val = Self::parse_atom(expr, pos);
                    acc.push(val);
                }
                b'&' => {
                    let val = Self::parse_list(expr, pos, Expr::And);
                    acc.push(val);
                }
                b'|' => {
                    let val = Self::parse_list(expr, pos, Expr::Or);
                    acc.push(val);
                }
                b'!' => {
                    let val = Self::parse_list(expr, pos, Expr::Not);
                    acc.push(val);
                }
                b')' => break,
                b',' => {
                    *pos += 1;
                    continue;
                }

                _ => {
                    println!("idx: {}, {}", *pos, expr[*pos] as char);
                    unreachable!()
                }
            }
        }

        // right )
        *pos += 1;

        match kind {
            Expr::And => acc.into_iter().all(|t| t),
            Expr::Not => {
                if acc.len() != 1 {
                    panic!("bad expr");
                }
                !acc[0]
            }
            Expr::Or => acc.into_iter().any(|t| t),
        }
    }
    fn parse_atom(expr: &[u8], pos: &mut usize) -> bool {
        match expr[*pos] {
            b't' => {
                *pos += 1;
                true
            }
            b'f' => {
                *pos += 1;
                false
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let val = Solution::parse_bool_expr("&(|(f))".to_string());
    println!("{val}");
}
