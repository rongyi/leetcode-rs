struct Solution;

impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let mut ret = String::new();
        while a > 0 || b > 0 {
            let sz = ret.len();
            if sz >= 2 && &ret[sz - 2..] == "aa" {
                if b > 0 {
                    b -= 1;
                    ret.push('b');
                }
            } else if sz >= 2 && &ret[sz - 2..] == "bb" {
                if a > 0 {
                    a -= 1;
                    ret.push('a');
                }
            } else {
                if a > b {
                    ret.push('a');
                    a -= 1;
                } else {
                    ret.push('b');
                    b -= 1;
                }
            }
        }
        ret
    }
}

fn main() {}
