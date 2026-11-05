struct Solution;

use std::collections::HashMap;
#[derive(Debug, Clone)]
enum Exp {
    List(Vec<Exp>),
    Symbol(String),
    Number(i32),
}

fn parse(tokens: &[String]) -> (Exp, &[String]) {
    let (first, rest) = tokens.split_first().unwrap();
    match &first[..] {
        "(" => parse_list(rest),
        _ => (parse_atom(first), rest),
    }
}

fn parse_list(mut tokens: &[String]) -> (Exp, &[String]) {
    let mut ret: Vec<Exp> = Vec::new();
    loop {
        match &tokens[0][..] {
            ")" => return (Exp::List(ret), &tokens[1..]),
            _ => {
                let (cur, rest) = parse(tokens);
                ret.push(cur);

                tokens = rest;
            }
        }
    }
}

fn parse_atom(token: &str) -> Exp {
    match token.parse::<i32>() {
        Ok(val) => Exp::Number(val),
        _ => Exp::Symbol(token.to_string()),
    }
}

fn eval(exp: &Exp, env: &mut Vec<HashMap<String, i32>>) -> Exp {
    match exp {
        Exp::Number(v) => Exp::Number(*v),
        Exp::Symbol(s) => {
            for e in env.iter().rev() {
                if e.contains_key(s) {
                    return Exp::Number(*e.get(s).unwrap());
                }
            }
            unreachable!()
        }
        Exp::List(lst) => {
            if let Exp::Symbol(s) = &lst[0] {
                match &s[..] {
                    "let" => {
                        env.push(HashMap::new());
                        let mut i = 1;
                        while i < lst.len() - 1 {
                            if let Exp::Symbol(s) = &lst[i] {
                                if let Exp::Number(v) = eval(&lst[i + 1], env) {
                                    env.last_mut().unwrap().insert(s.to_string(), v);
                                }
                            }
                            i += 2;
                        }
                        let val = eval(lst.last().unwrap(), env);

                        env.pop();
                        return val;
                    }
                    "add" => {
                        let mut nums: Vec<i32> = Vec::new();
                        for e in lst.iter().skip(1) {
                            if let Exp::Number(v) = eval(e, env) {
                                nums.push(v);
                            }
                        }
                        let val = nums.into_iter().sum();
                        return Exp::Number(val);
                    }
                    "mult" => {
                        let mut nums: Vec<i32> = Vec::new();
                        for e in lst.iter().skip(1) {
                            if let Exp::Number(v) = eval(e, env) {
                                nums.push(v);
                            }
                        }
                        let val = nums.into_iter().fold(1, |acc, cur| acc * cur);
                        return Exp::Number(val);
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
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
        // println!("{:?}", tokens);

        let (e, _) = parse(&tokens);
        // println!("{:?}", e);

        let mut env = Vec::new();
        if let Exp::Number(v) = eval(&e, &mut env) {
            return v;
        }

        unreachable!()
    }
}

fn main() {}
