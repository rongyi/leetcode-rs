struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        // , means union
        // {x}{y} means product (x..)*(y..)
        fn evaluate(expr: &[char], pos: &mut usize) -> HashSet<String> {
            let mut union_set = HashSet::new();
            let mut product_set = HashSet::new();
            // just make iteration work
            product_set.insert(String::new());

            while *pos < expr.len() && expr[*pos] != '}' {
                match expr[*pos] {
                    '{' => {
                        *pos += 1;
                        let nested = evaluate(expr, pos);
                        let mut new_set = HashSet::new();
                        for p in &product_set {
                            for n in &nested {
                                new_set.insert(format!("{}{}", p, n));
                            }
                        }
                        product_set = new_set;
                        // jump the }
                        *pos += 1;
                    }
                    ',' => {
                        union_set.extend(product_set.drain());
                        product_set.insert(String::new());
                        *pos += 1;
                    }
                    _ => {
                        let mut new_set = HashSet::new();
                        for s in &product_set {
                            new_set.insert(format!("{}{}", s, expr[*pos]));
                        }
                        product_set = new_set;
                        *pos += 1;
                    }
                }
            }

            union_set.extend(product_set);
            union_set
        }
        let chars: Vec<char> = expression.chars().collect();
        let mut pos = 0;
        let mut ret: Vec<String> = evaluate(&chars, &mut pos).into_iter().collect();
        ret.sort_unstable();
        ret
    }
}

fn main() {}
