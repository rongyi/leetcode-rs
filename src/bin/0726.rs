struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let s = formula.as_bytes();
        let sz = s.len();
        let mut i = 0;

        let mut stack: Vec<BTreeMap<String, i32>> = Vec::new();
        // just like global env
        stack.push(BTreeMap::new());

        while i < sz {
            match s[i] {
                b'(' => {
                    // sub block env
                    stack.push(BTreeMap::new());
                    i += 1;
                }
                b')' => {
                    let cur = stack.pop().unwrap();
                    // merge cur to parent
                    let parent = stack.last_mut().unwrap();
                    let mut j = i + 1;
                    while j < sz && s[j].is_ascii_digit() {
                        j += 1;
                    }
                    let val = formula[i + 1..j].parse::<i32>().unwrap_or(1);
                    // merge
                    for (k, v) in cur.into_iter() {
                        *parent.entry(k).or_insert(0) += v * val;
                    }

                    i = j;
                }
                _ => {
                    // 1. atom
                    // 2. count
                    let mut j = i + 1;
                    while j < sz && s[j].is_ascii_lowercase() {
                        j += 1;
                    }
                    let atom: String = String::from_utf8(s[i..j].to_vec()).unwrap();

                    i = j;

                    while j < sz && s[j].is_ascii_digit() {
                        j += 1;
                    }
                    let val: i32 = formula[i..j].parse().unwrap_or(1);

                    *stack.last_mut().unwrap().entry(atom).or_insert(0) += val;

                    i = j;
                }
            }
        }

        let mut ret = String::new();
        for (k, v) in stack.pop().unwrap().into_iter() {
            ret.push_str(&k);

            if v > 1 {
                ret.push_str(&v.to_string());
            }
        }

        ret
    }
}

fn main() {}
