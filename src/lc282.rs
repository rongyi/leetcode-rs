struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut ret = Vec::new();
        Self::backtrack(
            &num.chars().collect::<Vec<char>>(),
            target as i64,
            0,
            "".to_string(),
            0,
            0,
            &mut ret,
        );

        ret
    }
    fn backtrack(
        nums: &[char],
        target: i64,
        index: usize,
        expression: String,
        acc: i64,
        prev: i64,
        ret: &mut Vec<String>,
    ) {
        if index == nums.len() {
            if acc == target {
                ret.push(expression.clone());
            }
            return;
        }
        let mut cur_num = 0;
        for i in index..nums.len() {
            // not start with leading '0'
            if i > index && nums[index] == '0' {
                break;
            }
            cur_num = cur_num * 10 + (nums[i] as i64 - '0' as i64);

            if index == 0 {
                Self::backtrack(
                    nums,
                    target,
                    i + 1,
                    cur_num.to_string(),
                    cur_num,
                    cur_num,
                    ret,
                );
            } else {
                Self::backtrack(
                    nums,
                    target,
                    i + 1,
                    format!("{}+{}", expression, cur_num),
                    acc + cur_num,
                    cur_num,
                    ret,
                );

                Self::backtrack(
                    nums,
                    target,
                    i + 1,
                    format!("{}-{}", expression, cur_num),
                    acc - cur_num,
                    -cur_num,
                    ret,
                );

                // e.g. 3 + 2 * 3
                //            ^
                //            |____we are here
                // now acc is 5 ==> 3 + 2
                //     prev is 2
                // so acc - prev we get the left part 3 then add prev * cur (2 * 3)
                // final is 9
                Self::backtrack(
                    nums,
                    target,
                    i + 1,
                    format!("{}*{}", expression, cur_num),
                    acc - prev + prev * cur_num,
                    prev * cur_num,
                    ret,
                );
            }
        }
    }
}

fn main() {
    unimplemented!();
}
