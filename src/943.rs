struct Solution;

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
            for j in 0..sz {
                // mask don't have current node
                if ((1 << j) & mask) == 0 {
                    continue;
                }
                for i in 0..sz {
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
