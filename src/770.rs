#![allow(dead_code)]

struct Solution;

use std::{cmp::Ordering, collections::HashMap};
impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        let mut exps: Vec<String> = expression
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let m: HashMap<String, i32> = evalvars.into_iter().zip(evalints.into_iter()).collect();

        // replace
        for exp in exps.iter_mut() {
            if m.contains_key(exp) {
                *exp = (*m.get(exp).unwrap()).to_string();
            }
        }

        // imagine exp is surrunded by ()
        exps.push(")".to_string());
        let mut st: Vec<String> = Vec::new();
        st.push("(".to_string());

        let mut u: Vec<String> = Vec::new();

        // conert to RPN
        for s in exps.iter() {
            if Self::is_number(s) || Self::is_variable(s) {
                u.push(s.clone());
            } else if s == "(" {
                st.push(s.clone());
            } else if s == ")" {
                while !st.is_empty() && *st.last().unwrap() != "(" {
                    let cur = st.pop().unwrap();
                    u.push(cur);
                }

                // the left (
                st.pop();
            } else {
                // op
                let p = Self::priority(s);
                while !st.is_empty() && Self::priority(st.last().unwrap()) >= p {
                    let cur = st.pop().unwrap();
                    u.push(cur);
                }
                st.push(s.clone());
            }
        }

        let mut ste: Vec<Vec<Vec<String>>> = Vec::new();
        for s in u.iter() {
            if Self::is_number(s) {
                let cur = vec![vec![s.clone()]];
                ste.push(cur);
            } else if Self::is_variable(s) {
                // first element is coeffcient
                let cur = vec![vec!["1".to_string(), s.clone()]];
                ste.push(cur);
            } else {
                let mut second = ste.pop().unwrap();
                let mut first = ste.pop().unwrap();
                let val = Self::calculate(&mut first, s, &mut second);
                ste.push(val);
            }
        }
        let inner = ste.pop().unwrap();
        let mut ret: Vec<String> = Vec::new();
        for p in inner.iter() {
            if p[0] == "0" {
                continue;
            }
            let cur = p.join("*");
            ret.push(cur);
        }

        ret
    }

    fn calculate(a: &mut Vec<Vec<String>>, op: &str, b: &mut Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut c: Vec<Vec<String>> = Vec::new();
        if op == "+" {
            for s in a.iter() {
                c.push(s.clone());
            }
            for s in b.iter() {
                c.push(s.clone());
            }
        } else if op == "-" {
            for s in a.iter() {
                c.push(s.clone());
            }

            for s in b.iter() {
                let mut cur = s.clone();
                cur[0] = (-cur[0].parse::<i32>().unwrap()).to_string();
                c.push(cur);
            }
        } else {
            for s in a.iter() {
                for t in b.iter() {
                    let mut o: Vec<String> = Vec::new();
                    let num1 = s[0].parse::<i32>().unwrap();
                    let num2 = t[0].parse::<i32>().unwrap();
                    o.push((num1 * num2).to_string());
                    for i in 1..s.len() {
                        o.push(s[i].clone());
                    }
                    for i in 1..t.len() {
                        o.push(t[i].clone());
                    }
                    c.push(o);
                }
            }
        }
        // 1. sort every * expression
        for s in c.iter_mut() {
            s[1..].sort();
        }
        c.sort_by(|p, q| {
            if p.len() != q.len() {
                return q.len().cmp(&p.len());
            }

            for i in 1..p.len() {
                if p[i] != q[i] {
                    return p[i].cmp(&q[i]);
                }
            }

            Ordering::Equal
        });
        let mut ret: Vec<Vec<String>> = Vec::new();
        for s in c.iter() {
            if !ret.is_empty() && Self::equal(ret.last().unwrap(), s) {
                let val1 = ret.last().unwrap()[0].parse::<i32>().unwrap();
                let val2 = s[0].parse::<i32>().unwrap();
                ret.last_mut().unwrap()[0] = (val1 + val2).to_string();
            } else {
                ret.push(s.clone());
            }

            if ret.last().unwrap()[0] == "0" {
                ret.pop();
            }
        }

        ret
    }

    fn is_number(s: &str) -> bool {
        let c = s.chars().last().unwrap();
        c.is_ascii_digit()
    }

    fn is_variable(s: &str) -> bool {
        let c = s.chars().next().unwrap();
        c.is_ascii_alphabetic()
    }

    fn priority(s: &str) -> i32 {
        match s {
            "+" => 1,
            "-" => 1,
            "*" => 2,
            _ => -1, // (
        }
    }
    fn equal(a: &Vec<String>, b: &Vec<String>) -> bool {
        if a.len() != b.len() {
            return false;
        }
        for i in 1..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }

        true
    }
}

fn main() {
    let input1 = vec![];
    let input2 = vec![];
    let val = Solution::basic_calculator_iv("(e + 8) * (e - 8)".to_string(), input1, input2);
    println!("{:?}", val);
}
