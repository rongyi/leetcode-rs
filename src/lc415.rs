struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1: Vec<char> = num1.chars().rev().collect();
        let num2: Vec<char> = num2.chars().rev().collect();
        let sz1 = num1.len();
        let sz2 = num2.len();
        let mut carry = 0;
        let mut sum: Vec<u8> = Vec::new();

        for i in 0..sz1.max(sz2) {
            let v1 = if i < sz1 {
                num1[i] as u8 - '0' as u8
            } else {
                0
            };
            let v2 = if i < sz2 {
                num2[i] as u8 - '0' as u8
            } else {
                0
            };
            let cur = v1 + v2 + carry;
            carry = cur / 10;
            sum.push(cur % 10);
        }
        if carry > 0 {
            sum.push(1);
        }
        sum.iter().rev().map(|i| i.to_string()).collect()
    }
}

fn main() {
    let val = Solution::add_strings("11".to_string(), "123".to_string());
    println!("{}", val);
}
