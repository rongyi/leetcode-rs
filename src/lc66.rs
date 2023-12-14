struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut c = 1;
        for i in digits.iter().rev() {
            let cur = i + c;
            ret.push(cur % 10);
            c = cur / 10;
        }
        if c > 0 {
            ret.push(c);
        }

        ret.reverse();
        ret
    }
}
