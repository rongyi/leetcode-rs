struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut carry = 0;

        let mut ret = Vec::new();
        let mut i = num.len() as i32 - 1;
        while i >= 0 || k > 0 {
            let cur = if i >= 0 { num[i as usize] } else { 0 } + k % 10 + carry;
            ret.push(cur % 10);
            carry = cur / 10;
            k /= 10;
            i -= 1;
        }

        if carry > 0 {
            ret.push(carry);
        }

        ret.reverse();

        ret
    }
}

fn main() {}
