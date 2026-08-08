use std::time::Instant;

struct Solution;

impl Solution {
    /// LeetCode 691: Stickers to Spell Word — bitmask DP
    ///
    /// mask bit i = target position i still uncovered (target.len() ≤ 15).
    /// memo[mask] = min stickers to cover the remaining mask.
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let target_bytes = target.as_bytes();
        // dont worry about the same char in target, they just use position to mask all this
        // so each postion reduction is different, wether have duplicate char is irelevant
        let tlen = target_bytes.len();

        // Dedupe stickers with identical letter counts.
        let mut seen_counts = std::collections::HashSet::new();
        let mut sticker_counts: Vec<[i32; 26]> = Vec::new();
        for s in &stickers {
            let mut cnt = [0i32; 26];
            for &c in s.as_bytes() {
                cnt[(c - b'a') as usize] += 1;
            }
            if seen_counts.insert(cnt) {
                sticker_counts.push(cnt);
            }
        }

        let full_mask = (1 << tlen) - 1;
        let mut memo = vec![-2; 1 << tlen]; // -2 = uncomputed, -1 = impossible

        Self::dfs(full_mask, &sticker_counts, &target_bytes, &mut memo)
    }

    fn dfs(mask: usize, stickers: &[[i32; 26]], target: &[u8], memo: &mut Vec<i32>) -> i32 {
        if mask == 0 {
            return 0;
        }
        if memo[mask] != -2 {
            return memo[mask];
        }

        // First uncovered position — any solution must cover it.
        let first_pos = mask.trailing_zeros() as usize;
        let first_char = (target[first_pos] - b'a') as usize;

        let mut best = i32::MAX;
        for sticker in stickers {
            if sticker[first_char] == 0 {
                continue;
            }

            // Greedily cover uncovered positions with this sticker.
            let mut remaining = *sticker;
            let mut new_mask = mask;
            for i in first_pos..target.len() {
                if (new_mask >> i) & 1 == 1 {
                    let ch = (target[i] - b'a') as usize;
                    if remaining[ch] > 0 {
                        remaining[ch] -= 1;
                        // clear this bit
                        new_mask &= !(1 << i);
                    }
                }
            }
            if new_mask == mask {
                continue; // no progress
            }

            let sub = Self::dfs(new_mask, stickers, target, memo);
            if sub != -1 {
                best = best.min(1 + sub);
            }
        }

        memo[mask] = if best == i32::MAX { -1 } else { best };
        memo[mask]
    }
}

/// String-based memo version for comparison.
mod string_based {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
            let sticker_counts: Vec<[i32; 26]> = stickers
                .iter()
                .map(|s| {
                    let mut cnt = [0i32; 26];
                    for c in s.chars() {
                        cnt[(c as u8 - b'a') as usize] += 1;
                    }
                    cnt
                })
                .collect();

            let mut dp = HashMap::new();
            dp.insert(String::new(), 0);
            Self::recur(&mut dp, &sticker_counts, &target)
        }

        fn recur(dp: &mut HashMap<String, i32>, stickers: &[[i32; 26]], target: &str) -> i32 {
            if let Some(&cached) = dp.get(target) {
                return cached;
            }
            let mut need = [0i32; 26];
            for c in target.chars() {
                need[(c as u8 - b'a') as usize] += 1;
            }
            let first = (target.as_bytes()[0] - b'a') as usize;

            let mut best = i32::MAX;
            for sticker in stickers {
                if sticker[first] == 0 {
                    continue;
                }
                let mut leftover = String::new();
                for j in 0..26 {
                    let left = need[j] - sticker[j];
                    if left > 0 {
                        leftover.push_str(
                            &((b'a' + j as u8) as char).to_string().repeat(left as usize),
                        );
                    }
                }
                let sub = Self::recur(dp, stickers, &leftover);
                if sub != -1 {
                    best = best.min(1 + sub);
                }
            }
            let ans = if best == i32::MAX { -1 } else { best };
            dp.insert(target.to_string(), ans);
            ans
        }
    }
}

fn main() {
    let tests = [
        (
            vec![
                "with".to_string(),
                "example".to_string(),
                "science".to_string(),
            ],
            "thehat".to_string(),
            3,
        ),
        (
            vec!["notice".to_string(), "possible".to_string()],
            "basicbasic".to_string(),
            -1,
        ),
        (vec!["a".to_string()], "aaa".to_string(), 3),
        (
            vec!["ab".to_string(), "b".to_string()],
            "abb".to_string(),
            2,
        ),
    ];

    for (stickers, target, expected) in &tests {
        let r1 = Solution::min_stickers(stickers.clone(), target.clone());
        let r2 = string_based::Solution::min_stickers(stickers.clone(), target.clone());
        assert_eq!(
            r1, *expected,
            "bitmask wrong for {:?} {:?}",
            stickers, target
        );
        assert_eq!(
            r2, *expected,
            "string wrong for {:?} {:?}",
            stickers, target
        );
    }
    println!("✓ correctness: both versions match expected");

    // Benchmark on a heavier input.
    let stickers: Vec<String> = vec![
        "these".into(),
        "guess".into(),
        "about".into(),
        "garden".into(),
        "club".into(),
        "bet".into(),
        "way".into(),
    ];
    let target = "dthebattheth".to_string();

    let mut t0 = Instant::now();
    for _ in 0..10 {
        string_based::Solution::min_stickers(stickers.clone(), target.clone());
    }
    let string_time = t0.elapsed();

    t0 = Instant::now();
    for _ in 0..10 {
        Solution::min_stickers(stickers.clone(), target.clone());
    }
    let mask_time = t0.elapsed();

    println!("string-based: {:?}", string_time);
    println!("bitmask:      {:?}", mask_time);
    println!(
        "speedup: {:.1}x",
        string_time.as_secs_f64() / mask_time.as_secs_f64()
    );
}
