struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let locked: Vec<char> = locked.chars().collect();
        let sz = s.len();
        // must be even
        if sz % 2 == 1 {
            return false;
        }
        let mut balance = 0;
        for i in 0..sz {
            if s[i] == '(' || locked[i] == '0' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                return false;
            }
        }

        balance = 0;
        for i in (0..sz).rev() {
            if s[i] == ')' || locked[i] == '0' {
                balance += 1;
            } else {
                balance -= 1;
            }

            if balance < 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    for i in (0..9).rev() {
        println!("{}", i);
    }
}
