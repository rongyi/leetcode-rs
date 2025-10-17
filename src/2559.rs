struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let counts: Vec<usize> = words
            .iter()
            .map(|s| {
                let c = s.chars().next().unwrap();
                let c2 = s.chars().last().unwrap();
                if "aoeiu".contains(c) && "aoeiu".contains(c2) {
                    1
                } else {
                    0
                }
            })
            .collect();
        let mut prefix: Vec<usize> = vec![0; counts.len() + 1];
        for (i, &v) in counts.iter().enumerate() {
            prefix[i + 1] = prefix[i] + v;
        }
        let mut ret = vec![];

        for q in queries.iter() {
            let val = prefix[q[1] as usize + 1] - prefix[q[0] as usize];
            ret.push(val as i32);
        }

        ret
    }
}

fn main() {}
