
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        // domain -> [email]
        let mut domain_emails: HashMap<String, HashSet<String>> = HashMap::new();

        for email in emails.into_iter() {
            let chunk: Vec<String> = email.split('@').map(|s| s.to_string()).collect();
            if chunk.len() != 2 {
                continue;
            }
            let mut normal_address = String::new();
            for c in chunk[0].chars() {
                if c == '+' {
                    break;
                }
                if c == '.' {
                    continue;
                }
                normal_address.push(c);
            }
            domain_emails
                .entry(chunk[1].clone())
                .or_default()
                .insert(normal_address);
        }

        domain_emails.values().map(|c| c.len() as i32).sum()
    }
}

fn main() {}
