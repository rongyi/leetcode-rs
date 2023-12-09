struct Solution;


use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = vec![0; 26];
            for c in s.bytes() {
                count[(c - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(s);
        }

        groups.into_iter().map(|(_, v)| v).collect()
    }
}
