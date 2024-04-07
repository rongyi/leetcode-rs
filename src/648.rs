struct Solution;

impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        dictionary.sort_by(|l, r| l.len().cmp(&r.len()));
        let mut converts: Vec<String> = Vec::new();
        for word in sentence.split(' ') {
            let mut changed = false;
            for root in dictionary.iter() {
                if word.starts_with(root) {
                    changed = true;
                    converts.push(root.to_owned());
                    break;
                }
            }
            if !changed {
                converts.push(word.to_string());
            }
        }

        converts.join(" ")
    }
}

fn main() {}
