struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let word = word.as_bytes();
        let mut ret = vec![];
        let mut prev: i64 = 0;
        let m = m as i64;
        for &w in word.iter() {
            let cur: i64 = prev * 10 + (w - b'0') as i64;
            let check = cur % m;
            if check == 0 {
                ret.push(1);
            } else {
                ret.push(0);
            }
            prev = check;
        }

        ret
    }
}

fn main() {}
