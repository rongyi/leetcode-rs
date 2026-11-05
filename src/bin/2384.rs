struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut cnt: BTreeMap<char, usize> = BTreeMap::new();

        let mut center: Option<char> = None;
        for c in num.chars() {
            *cnt.entry(c).or_default() += 1;
        }
        let mut half: Vec<char> = Vec::new();
        for (&k, &v) in cnt.iter().rev() {
            if v % 2 == 1 && center.is_none() {
                center = Some(k);
            }
            if v / 2 > 0 {
                half.push(k);
            }
        }
        let mut ret: String = String::new();

        for &d in half.iter() {
            let tmp = d.to_string().repeat(*cnt.get(&d).unwrap() / 2);
            ret.push_str(&tmp);
        }

        if ret.len() > 0 && ret.as_bytes()[0] == b'0' {
            match center {
                Some(v) => return v.to_string(),
                // just return the max char in this string
                None => return num.chars().max().unwrap().to_string(),
            }
        }

        match center {
            Some(v) => {
                let tail: String = ret.clone().chars().rev().collect();
                ret.push(v);
                ret.push_str(&tail);
                ret
            }
            None => {
                let tail: String = ret.clone().chars().rev().collect();
                ret.push_str(&tail);
                ret
            }
        }
    }
}

fn main() {}
