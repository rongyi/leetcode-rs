struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut l: String = s.clone();
        let r: String = s.clone().chars().rev().collect();
        l.push('#');
        l.push_str(&r);
        let l: Vec<char> = l.chars().collect();
        let r: Vec<char> = r.chars().collect();

        // kmp prefix/suffix len
        let mut prefix = vec![0; l.len()];

        for i in 1..l.len() {
            let mut j = prefix[i - 1];
            while j > 0 && l[i] != l[j as usize] {
                j = prefix[j - 1];
            }
            prefix[i] = j + if l[i] == l[j as usize] { 1 } else { 0 };
        }

        format!(
            "{}{}",
            r[0..(s.len() - prefix[l.len() - 1])]
                .iter()
                .collect::<String>(),
            s
        )
    }
}

fn main() {}
