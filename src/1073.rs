struct Solution;

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut carry = 0;
        let mut i = arr1.len() as i32 - 1;
        let mut j = arr2.len() as i32 - 1;
        while i >= 0 || j >= 0 || carry != 0 {
            let mut sum = carry;

            if i >= 0 {
                sum += arr1[i as usize];
                i -= 1;
            }
            if j >= 0 {
                sum += arr2[j as usize];
                j -= 1;
            }
            ret.push(sum.abs() % 2);
            carry = if sum < 0 {
                1
            } else if sum > 1 {
                -1
            } else {
                0
            };
        }
        while ret.len() > 1 && ret.last() == Some(&0) {
            ret.pop();
        }

        ret.reverse();
        ret
    }
}
fn main() {}
