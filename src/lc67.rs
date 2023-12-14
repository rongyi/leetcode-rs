struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut c = 0;
        let mut a = a;
        let mut b = b;
        let mut ret = String::new();

        while !a.is_empty() || !b.is_empty() || c != 0 {
            let n1 = a.pop().unwrap_or('0');
            let n2 = b.pop().unwrap_or('0');
            let sum = (n1 as u8 - '0' as u8) + (n2 as u8 - '0' as u8) + c;
            ret.push(((sum & 1) + '0' as u8) as char);
            c = sum / 2;
        }

        ret.chars().rev().collect()
    }
}
