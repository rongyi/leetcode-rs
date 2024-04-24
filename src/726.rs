#![allow(dead_code)]


struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let s = formula.chars().collect::<Vec<_>>();
        let sz = s.len();

        let mut stack: Vec<BTreeMap<String, i32>> = Vec::new();
        // global env
        stack.push(BTreeMap::new());

        let mut i = 0;
        while i < sz {
            // TODO: update i
            match s[i] {
                '(' => {
                    // simply add a new env entry to stack
                    stack.push(BTreeMap::new());
                    i += 1;
                }
                ')' => {
                    let cur = stack.pop().unwrap();
                    let parent = stack.last_mut().unwrap();
                    // TODO: add cur env to parent
                    // find multiply value e.g. (H2O)
                    let mut j = i + 1;
                    while j < sz && s[j].is_ascii_digit() {
                        j += 1;
                    }
                    let num_str: String = s[i + 1..j].iter().collect();
                    let val = num_str.parse::<i32>().unwrap_or(1);

                    for (k, v) in cur.into_iter() {
                        *parent.entry(k).or_insert(0) += v * val;
                    }

                    i = j;
                }
                _ => {
                    // 1. atom
                    // 2. count: default 1
                    let mut j = i + 1;
                    while j < sz && s[j].is_ascii_lowercase() {
                        j += 1;
                    }
                    let atom: String = s[i..j].iter().collect();

                    // pull i to j
                    i = j;
                    // consume digit
                    while j < sz && s[j].is_ascii_digit() {
                        j += 1;
                    }
                    let num_str: String = s[i..j].iter().collect();
                    let val = num_str.parse::<i32>().unwrap_or(1);
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
