struct Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name: Vec<char> = name.chars().collect();
        let typed: Vec<char> = typed.chars().collect();
        let sz1 = name.len();
        let sz2 = typed.len();
        let mut i = 0;
        let mut j = 0;
        while i < sz1 && j < sz2 {
            if name[i] != typed[j] {
                return false;
            }
            let c = name[i];
            let mut ipeek = i;
            while ipeek < sz1 && name[ipeek] == c {
                ipeek += 1;
            }
            let mut jpeek = j;
            while jpeek < sz2 && typed[jpeek] == c {
                jpeek += 1;
            }
            if (ipeek - i) > (jpeek - j) {
                return false;
            }
            i = ipeek;
            j = jpeek;
        }

        i == sz1 && j == sz2
    }
}

fn main() {
    Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string());
}
