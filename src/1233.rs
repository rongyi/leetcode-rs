#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
#[derive(Debug, Default)]
struct Node {
    vals: HashMap<String, Node>,
    ending: bool,
}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut root = Node::default();
        for path in folder.iter() {
            // without root
            let dirs: Vec<String> = path.split('/').skip(1).map(|s| s.to_string()).collect();
            let mut cur = &mut root;
            for dir in dirs.into_iter() {
                if !cur.vals.contains_key(&dir) {
                    cur.vals.insert(dir.clone(), Node::default());
                }

                cur = cur.vals.get_mut(&dir).unwrap();
            }
            // sealed to here
            cur.ending = true;
        }

        let mut ret: Vec<String> = Vec::new();
        Self::dfs(&root, &mut ret, "/".to_string());

        ret
    }

    fn dfs(cur_node: &Node, ret: &mut Vec<String>, base: String) {
        for (dir, sub) in cur_node.vals.iter() {
            let mut cur_path = base.clone();
            cur_path.push_str(dir);
            // keep going
            if !sub.ending {
                Self::dfs(sub, ret, cur_path.clone() + &"/".to_string());
            } else {
                // there's one final result here
                ret.push(cur_path.clone());
            }
        }
    }
}

fn main() {
    let input = vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let ret = Solution::remove_subfolders(input);
    println!("{:?}", ret);
}
