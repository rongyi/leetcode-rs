struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut carry = 0;
        let mut i = arr1.len() as i32 - 1;
        let mut j = arr2.len() as i32 - 1;
        while i >= 0 || j >= 0 || carry != 0 {
            if i >= 0 {
                carry += arr1[i as usize];
                i -= 1;
            }
            if j >= 0 {
                carry += arr2[j as usize];
                j -= 1;
            }
            ret.push(if carry.abs() & 1 == 1 { 1 } else { 0 });
            carry = -(carry >> 1);
        }
        while ret.len() > 1 && ret.last() == Some(&0) {
            ret.pop();
        }

        ret.reverse();
        ret
    }
}
fn main() {}
