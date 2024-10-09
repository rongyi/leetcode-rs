struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut carry = 0;

        let mut ret = Vec::new();
        for i in num.into_iter().rev() {
            let cur = i + k % 10 + carry;
            ret.push(cur % 10);
            carry = cur / 10;
            k /= 10;
        }
        while k > 0 {
            let cur = k % 10 + carry;
            ret.push(cur % 10);
            k /= 10;
            carry = cur / 10;
        }

        if carry > 0 {
            ret.push(carry);
        }

        ret.reverse();
        ret
    }
}

fn main() {}
