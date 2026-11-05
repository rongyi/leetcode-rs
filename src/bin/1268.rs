#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort_unstable();
        let mut ret = Vec::new();
        let mut prefix = String::new();

        for c in search_word.chars() {
            prefix.push(c);
            let mut suggestions = Vec::new();
            let pos = products.partition_point(|p| p < &prefix);
            for i in pos..products.len() {
                if suggestions.len() >= 3 {
                    break;
                }
                if products[i].starts_with(&prefix) {
                    suggestions.push(products[i].clone());
                } else {
                    break;
                }
            }
            ret.push(suggestions);
        }

        ret
    }
}

fn main() {}
