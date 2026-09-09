struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn shortest_superstring(words: Vec<String>) -> String {
            let n = words.len();

            // 1. Precalculate overlap matrix:
            // overlap[i][j] = length of the longest suffix of words[i]
            //                 that matches a prefix of words[j]
            let mut overlap = vec![vec![0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        let w1 = words[i].as_bytes();
                        let w2 = words[j].as_bytes();
                        let max_k = w1.len().min(w2.len());

                        for k in (1..=max_k).rev() {
                            if w1[w1.len() - k..] == w2[..k] {
                                overlap[i][j] = k;
                                break;
                            }
                        }
                    }
                }
            }

            // 2. DP table and Parent table for path reconstruction
            let num_states = 1 << n;
            let mut dp = vec![vec![usize::MAX / 2; n]; num_states];
            let mut parent = vec![vec![usize::MAX; n]; num_states];

            // Base cases: starting with a single word
            for i in 0..n {
                dp[1 << i][i] = words[i].len();
            }

            // 3. Populate DP table across all bitmask states
            for mask in 1..num_states {
                for last in 0..n {
                    if (mask & (1 << last)) == 0 || dp[mask][last] == usize::MAX / 2 {
                        continue;
                    }

                    for next in 0..n {
                        if (mask & (1 << next)) == 0 {
                            let next_mask = mask | (1 << next);
                            let added_len = words[next].len() - overlap[last][next];
                            let cost = dp[mask][last] + added_len;

                            if cost < dp[next_mask][next] {
                                dp[next_mask][next] = cost;
                                parent[next_mask][next] = last;
                            }
                        }
                    }
                }
            }

            // 4. Find the ending word that gives the shortest full superstring
            let full_mask = num_states - 1;
            let mut last = 0;
            let mut min_len = usize::MAX;

            for i in 0..n {
                if dp[full_mask][i] < min_len {
                    min_len = dp[full_mask][i];
                    last = i;
                }
            }

            // 5. Backtrack using parent matrix to recover word order
            let mut path = Vec::with_capacity(n);
            let mut current_mask = full_mask;

            while last != usize::MAX {
                path.push(last);
                let prev = parent[current_mask][last];
                current_mask ^= 1 << last;
                last = prev;
            }

            path.reverse();

            // 6. Build final superstring by merging overlapping segments
            let mut result = words[path[0]].clone();
            for i in 1..n {
                let prev = path[i - 1];
                let curr = path[i];
                let ol = overlap[prev][curr];
                result.push_str(&words[curr][ol..]);
            }

            result
        }
    }
}

impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        let sz = words.len();
        let mut overlap = vec![vec![0; sz]; sz];
        Self::build_overlap(&mut overlap, &words);

        // dp[i][j]
        // i ==> 访问过的node的mask， 每一个node占一位
        // j ==> 表示最后一个访问元素是node j
        let mut dp = vec![vec!["".to_string(); sz]; 1 << sz];
        for i in 0..sz {
            let cur: String = words[i].iter().copied().collect();
            dp[1 << i][i].push_str(&cur);
        }
        for mask in 1..(1 << sz) {
            // end with j
            for j in 0..sz {
                // mask don't have current node
                if ((1 << j) & mask) == 0 {
                    continue;
                }
                // from i to j
                for i in 0..sz {
                    // mask contains i
                    if i != j && (mask & (1 << i)) > 0 {
                        let tmp: String =
                            words[j][overlap[i][j] as usize..].iter().copied().collect();
                        let merge = dp[mask ^ (1 << j)][i].clone() + &tmp;
                        if dp[mask][j].is_empty() || merge.len() < dp[mask][j].len() {
                            dp[mask][j] = merge;
                        }
                    }
                }
            }
        }
        let last = (1 << sz) - 1;
        let mut ret = dp[last][0].clone();
        for i in 1..sz {
            if dp[last][i].len() < ret.len() {
                ret = dp[last][i].clone();
            }
        }

        ret
    }
    fn build_overlap(overlap: &mut Vec<Vec<i32>>, words: &Vec<Vec<char>>) {
        let sz = overlap.len();
        for i in 0..sz {
            for j in 0..sz {
                if i == j {
                    continue;
                }
                let from = &words[i];
                let to = &words[j];
                for k in (0..from.len().min(to.len())).rev() {
                    let to_sub: Vec<char> = to[0..k].iter().copied().collect();
                    let from_sub: Vec<char> = from[from.len() - k..].iter().copied().collect();
                    if to_sub == from_sub {
                        overlap[i][j] = k as i32;
                        break;
                    }
                }
            }
        }
    }
}

fn main() {}
