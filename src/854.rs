struct Solution;

mod ai {
    struct Solution;

    use std::collections::{HashSet, VecDeque};

    impl Solution {
        pub fn k_similarity(s1: String, s2: String) -> i32 {
            let s1: Vec<char> = s1.chars().collect();
            let s2: Vec<char> = s2.chars().collect();
            let n = s1.len();

            // Pre-check: if strings are already equal
            if s1 == s2 {
                return 0;
            }

            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();

            queue.push_back((s1.clone(), 0));
            visited.insert(s1);

            while let Some((mut curr, steps)) = queue.pop_front() {
                // Find first mismatch
                let mut i = 0;
                while i < n && curr[i] == s2[i] {
                    i += 1;
                }

                if i == n {
                    return steps;
                }

                // Only try swaps that put the correct character at position i
                for j in i + 1..n {
                    if curr[j] == s2[i] && curr[j] != s2[j] {
                        curr.swap(i, j);

                        if !visited.contains(&curr) {
                            visited.insert(curr.clone());
                            queue.push_back((curr.clone(), steps + 1));
                        }

                        curr.swap(i, j);
                    }
                }
            }

            -1
        }
    }
}

impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        let mut s1: Vec<char> = s1.chars().collect();
        let mut s2: Vec<char> = s2.chars().collect();

        Self::dfs(&mut s1, &mut s2)
    }

    fn dfs(s1: &mut Vec<char>, s2: &mut Vec<char>) -> i32 {
        let sz = s1.len();

        for i in 0..sz {
            if s1[i] == s2[i] {
                continue;
            }
            let mut candiate: Vec<usize> = Vec::new();

            for j in i + 1..sz {
                //     i      j
                // A  A[i]   A[j]
                // B  B[i]   B[j]
                // 不完美的情况，至少要让swap有意义
                // 那就是A[j] 换过来要和B[i]相等 而且原位 A[j] != B[j]
                // 否则换了就零和了
                if s1[j] == s2[i] && s1[j] != s2[j] {
                    candiate.push(j);
                    // perfect
                    if s1[i] == s2[j] {
                        s1.swap(i, j);
                        let mut subs1: Vec<char> = s1[i + 1..].iter().copied().collect();
                        let mut subs2: Vec<char> = s2[i + 1..].iter().copied().collect();
                        return 1 + Self::dfs(&mut subs1, &mut subs2);
                    }
                }
            }
            let mut best = sz as i32 - 1;
            for j in candiate.into_iter() {
                s1.swap(i, j);
                let mut subs1: Vec<char> = s1[i + 1..].iter().copied().collect();
                let mut subs2: Vec<char> = s2[i + 1..].iter().copied().collect();
                best = best.min(1 + Self::dfs(&mut subs1, &mut subs2));

                s1.swap(i, j);
            }
            return best;
        }

        0
    }
}

fn main() {}
