struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut out = vec![];

        Self::backtract(
            num.as_bytes(),
            target as i64,
            0,
            0,
            0,
            "".to_string(),
            &mut out,
        );
        out
    }
    fn backtract(
        num: &[u8],
        target: i64,
        acc: i64,
        prev: i64,
        start: usize,
        acc_expr: String,
        out: &mut Vec<String>,
    ) {
        if start == num.len() {
            if acc == target {
                out.push(acc_expr.clone());
            }
            return;
        }
        let mut cur_num = 0;
        for i in start..num.len() {
            // no leading zero values
            if i > start && num[start] == b'0' {
                break;
            }
            cur_num = cur_num * 10 + (num[i] as u8 - b'0' as u8) as i64;
            if start == 0 {
                Self::backtract(
                    num,
                    target,
                    cur_num,
                    cur_num,
                    i + 1,
                    format!("{}", cur_num),
                    out,
                );
            } else {
                // +
                Self::backtract(
                    num,
                    target,
                    acc + cur_num,
                    cur_num,
                    i + 1,
                    format!("{}+{}", acc_expr, cur_num),
                    out,
                );
                // -
                Self::backtract(
                    num,
                    target,
                    acc - cur_num,
                    -cur_num,
                    i + 1,
                    format!("{}-{}", acc_expr, cur_num),
                    out,
                );
                // *
                Self::backtract(
                    num,
                    target,
                    acc - prev + prev * cur_num,
                    prev * cur_num, // the prev unit for next round
                    i + 1,
                    format!("{}*{}", acc_expr, cur_num),
                    out,
                );
            }
        }
    }
}

fn main() {
    let out = Solution::add_operators("123".to_string(), 6);
    println!("{:?}", out);
}
