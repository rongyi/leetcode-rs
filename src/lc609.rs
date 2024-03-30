struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut merge: HashMap<String, Vec<String>> = HashMap::new();
        for s in paths.iter() {
            let cur = Self::parse(s);
            for (k, v) in cur.iter() {
                merge
                    .entry(k.to_owned())
                    .or_insert(Vec::new())
                    .extend(v.to_owned());
            }
        }
        merge
            .values()
            .into_iter()
            .filter(|x| x.len() > 1)
            .cloned()
            .collect()
    }
    // content-> [filepath]
    pub fn parse(s: &str) -> HashMap<String, Vec<String>> {
        let mut ret: HashMap<String, Vec<String>> = HashMap::new();
        let chunks: Vec<&str> = s.split(' ').collect();
        // first pos is path
        let path = chunks[0].to_string();
        // all file
        for &f in chunks.iter().skip(1) {
            let fchunks: Vec<&str> = f.split('(').collect();
            let full_path = path.clone() + "/" + &fchunks[0].to_string();
            // content have end ), but don't need to pop
            let content = fchunks[1].to_string();
            ret.entry(content).or_insert(Vec::new()).push(full_path);
        }
        ret
    }
}

fn main() {
    let ret = Solution::parse("root/a 1.txt(abcd) 2.txt(efgh)");
    for (k, v) in ret.iter() {
        println!("{:?} -> {:?}", k, v);
    }
}
