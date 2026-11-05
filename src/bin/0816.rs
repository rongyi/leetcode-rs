#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut ret = Vec::new();
        // skip (
        let mut s: Vec<char> = s.chars().skip(1).collect();
        // and last )
        s.pop();

        let sz = s.len();

        let junkzero = |lst: &Vec<char>| -> bool {
            lst.clone()
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
                == 0
                && lst.len() > 1
        };
        for i in 1..sz {
            let first = s[0..i].to_vec();
            let second = s[i..].to_vec();

            if junkzero(&first) || junkzero(&second) {
                continue;
            }
            let all_first = Self::all_possibles(first);
            let all_second = Self::all_possibles(second);

            for first in all_first.iter() {
                for second in all_second.iter() {
                    ret.push(format!("({}, {})", first, second));
                }
            }
        }

        ret
    }

    fn all_possibles(s: Vec<char>) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        if s.len() == 1 || s[0] != '0' {
            ret.push(s.clone().into_iter().collect());
        }
        let sz = s.len();
        for i in 1..sz {
            let int_part = s[0..i].to_vec();
            let float_part = s[i..].to_vec();

            if int_part.len() > 1 && int_part[0] == '0' {
                continue;
            }

            if float_part
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
                == 0
                || *float_part.last().unwrap() == '0'
            {
                continue;
            }

            let join: String = int_part.into_iter().collect::<String>()
                + "."
                + &float_part.into_iter().collect::<String>();
            ret.push(join);
        }

        ret
    }
}

fn main() {}
