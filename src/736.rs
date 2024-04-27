#![allow(dead_code)]

struct Solution;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum RispExp {
    List(Vec<RispExp>),
    Symbol(String),
    Number(i32),
}

fn parse(tokens: &[String]) -> (RispExp, &[String]) {
    let (first, rest) = tokens.split_first().unwrap();
    match &first[..] {
        "(" => parse_list(rest),
        _ => (parse_atom(first), rest),
    }
}

fn parse_list(mut tokens: &[String]) -> (RispExp, &[String]) {
    let mut ret = Vec::new();

    loop {
        match &tokens.first().unwrap()[..] {
            ")" => {
                return (RispExp::List(ret), &tokens[1..]);
            }
            _ => {
                let (cur, rest) = parse(tokens);

                ret.push(cur);
                tokens = rest;
            }
        }
    }
}

fn parse_atom(token: &str) -> RispExp {
    match token.parse::<i32>() {
        Ok(val) => RispExp::Number(val),
        _ => RispExp::Symbol(token.to_string()),
    }
}

fn eval(exp: &RispExp, env: &mut Vec<HashMap<String, i32>>) -> RispExp {
    match exp {
        RispExp::List(lst) => match &lst[0] {
            RispExp::Symbol(s) => {
                println!("here");
                match &s[..] {
                    "let" => {
                        println!("bind");
                        env.push(HashMap::new());
                        let mut i = 1;
                        // bind
                        while i < lst.len() - 1 {
                            if let RispExp::Symbol(s) = &lst[i] {
                                if let RispExp::Number(v) = eval(&lst[i + 1], env) {
                                    env.last_mut().unwrap().insert(s.to_string(), v);
                                }
                            }
                            i += 2;
                        }

                        let val = eval(lst.last().unwrap(), env);

                        env.pop();
                        val
                    }
                    "add" => {
                        let mut i = 1;
                        let mut nums = Vec::new();
                        while i < lst.len() {
                            if let RispExp::Number(v) = eval(&lst[i], env) {
                                nums.push(v);
                            }

                            i += 1;
                        }

                        RispExp::Number(nums.iter().sum::<i32>())
                    }
                    "mult" => {
                        let mut i = 1;
                        let mut nums = Vec::new();
                        while i < lst.len() {
                            if let RispExp::Number(v) = eval(&lst[i], env) {
                                nums.push(v);
                            }

                            i += 1;
                        }
                        let val = nums.iter().fold(1, |acc, &cur| acc * cur);

                        RispExp::Number(val)
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => {
                unreachable!()
            }
        },
        RispExp::Number(num) => RispExp::Number(*num),
        RispExp::Symbol(s) => {
            for e in env.iter().rev() {
                if e.contains_key(s) {
                    let val = e.get(s).unwrap();
                    return RispExp::Number(*val);
                }
            }
            unreachable!()
        }
    }
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let tokens: Vec<String> = expression
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for token in tokens.iter() {
            println!("{}", token);
        }

        let exp = parse(&tokens);
        println!("{:?}", exp);
        let mut env: Vec<HashMap<String, i32>> = Vec::new();
        if let RispExp::Number(val) = eval(&exp.0, &mut env) {
            return val;
        }
        unreachable!()
    }
}

fn main() {
    // let input = "(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string();
    let input = "(let x 3 x 2 x)".to_string();
    Solution::evaluate(input);
}
