struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut out = vec![];
        let mut carry = 1;
        while let Some(last) = digits.pop() {
            let sum = last + carry;
            carry = sum / 10;
            out.push(sum % 10);
        }
        if carry > 0 {
            out.push(carry);
        }
        out.reverse();

        out
    }
}

fn main() {}
