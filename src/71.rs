struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut paths = vec![];

        for chunks in path.split('/') {
            if chunks.is_empty() || chunks == "." {
                continue;
            }
            if chunks == ".." {
                paths.pop();
            } else {
                paths.push(chunks.to_string());
            }
        }

        format!("/{}", paths.join("/"))
    }
}

fn main() {}
