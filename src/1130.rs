struct Soution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut stack: Vec<i32> = vec![i32::MAX];

        for &num in arr.iter() {
            while *stack.last().unwrap() <= num {
                let mid = stack.pop().unwrap();
                // only smaller get in first
                // then new value will be checked again all smaller value to see if this is
                // their first bigger number, if it is(cur <= newnumber aka num)
                // pop this value and combine this two as one true
                ret += mid * num.min(*stack.last().unwrap());
            }

            stack.push(num);
        }

        for i in 2..stack.len() {
            ret += stack[i] * stack[i - 1];
        }

        ret
    }
}

fn main() {}
