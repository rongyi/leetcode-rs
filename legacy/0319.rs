struct Solution;


impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let mut ret = 0;
        for i in 1.. {
            if i * i > n {
                break;
            }
            ret += 1;
        }
        ret
    }
}

fn main() {}
