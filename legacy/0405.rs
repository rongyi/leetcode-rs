struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let table: Vec<char> = "0123456789abcdef".to_string().chars().collect();
        let mut ret = String::new();

        if num == 0 {
            return "0".to_string();
        }
        let mut cnt = 0;
        let mut num = num;
        while num != 0 && cnt < 8 {
            let cur = num & 0xf;
            cnt += 1;
            num >>= 4;
            ret.push(table[cur as usize]);
        }

        ret.chars().rev().collect()
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
