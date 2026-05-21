struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut out: Vec<char> = vec![];
        let a: Vec<u32> = a.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
        let b: Vec<u32> = b.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
        let sz = a.len().max(b.len());
        let mut carry = 0;
        for i in 0..sz {
            match (a.get(i), b.get(i)) {
                (Some(&v1), Some(&v2)) => {
                    let sum = v1 + v2 + carry;
                    carry = sum / 2;
                    out.push(char::from_digit(sum % 2, 2).unwrap());
                }
                (Some(&v1), None) => {
                    let sum = v1 + carry;
                    carry = sum / 2;
                    out.push(char::from_digit(sum % 2, 2).unwrap());
                }
                (None, Some(&v2)) => {
                    let sum = v2 + carry;
                    carry = sum / 2;
                    out.push(char::from_digit(sum % 2, 2).unwrap());
                }
                _ => unreachable!(),
            }
        }
        if carry > 0 {
            out.push(char::from_digit(carry, 2).unwrap());
        }

        out.reverse();
        out.into_iter().collect()
    }
}

fn main() {}
