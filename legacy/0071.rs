struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dirs: Vec<String> = Vec::new();


        for dir in path.split("/") {
            match dir {
                "" | "." => continue,
                ".." => {
                    dirs.pop();
                },
                _ => dirs.push(dir.to_string()),
            }
        }

        let mut formated_path = dirs.into_iter().collect::<Vec<_>>().join("/");
        formated_path.insert(0, '/');
        formated_path
    }
}
