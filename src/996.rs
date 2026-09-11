struct Solution;

mod ai {
    struct Solution;

    use std::collections::HashMap;

    impl Solution {
        pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
            let n = nums.len();

            // Step 1: Count frequency of each unique number to handle duplicate values naturally
            let mut count: HashMap<i32, usize> = HashMap::new();
            for &num in &nums {
                *count.entry(num).or_insert(0) += 1;
            }

            // Step 2: Build an adjacency list connecting unique pairs (u, v) if u + v is a perfect square
            let unique_keys: Vec<i32> = count.keys().copied().collect();
            let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

            for i in 0..unique_keys.len() {
                for j in i..unique_keys.len() {
                    let u = unique_keys[i];
                    let v = unique_keys[j];

                    // Allow self-loops if there are at least 2 instances of the number
                    if u != v && Self::is_square(u + v) {
                        graph.entry(u).or_default().push(v);
                        graph.entry(v).or_default().push(u);
                    } else if u == v && count[&u] >= 2 && Self::is_square(u + u) {
                        graph.entry(u).or_default().push(u);
                    }
                }
            }

            // Step 3: Backtracking search starting from each unique number
            let mut total = 0;
            for &start in &unique_keys {
                *count.get_mut(&start).unwrap() -= 1;
                total += Self::dfs(start, n - 1, &mut count, &graph);
                *count.get_mut(&start).unwrap() += 1;
            }

            total
        }

        fn is_square(val: i32) -> bool {
            let root = (val as f64).sqrt() as i32;
            root * root == val
        }

        fn dfs(
            curr: i32,
            remaining: usize,
            count: &mut HashMap<i32, usize>,
            graph: &HashMap<i32, Vec<i32>>,
        ) -> i32 {
            if remaining == 0 {
                return 1;
            }

            let mut paths = 0;
            if let Some(neighbors) = graph.get(&curr) {
                for &next in neighbors {
                    // Check availability without holding a mutable reference across the recursive call
                    if count.get(&next).copied().unwrap_or(0) > 0 {
                        // 1. Mutate and immediately release borrow
                        *count.get_mut(&next).unwrap() -= 1;

                        // 2. Recursive call (count is now free to be borrowed mutably)
                        paths += Self::dfs(next, remaining - 1, count, graph);

                        // 3. Mutate back and release borrow
                        *count.get_mut(&next).unwrap() += 1;
                    }
                }
            }

            paths
        }
    }
}

impl Solution {
    pub fn num_squareful_perms(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ret = 0;
        Self::dfs(nums, 0, &mut ret);
        ret
    }

    fn dfs(mut nums: Vec<i32>, cur_idx: usize, ret: &mut i32) {
        if cur_idx >= nums.len() {
            *ret += 1;
        }
        for i in cur_idx..nums.len() {
            if i > cur_idx && nums[i] == nums[cur_idx] {
                continue;
            }
            nums.swap(i, cur_idx);
            if cur_idx == 0 || (cur_idx > 0 && Self::is_square(nums[cur_idx] + nums[cur_idx - 1])) {
                Self::dfs(nums.clone(), cur_idx + 1, ret);
            }
        }
    }

    fn is_square(v: i32) -> bool {
        let r = (v as f64).sqrt() as i32;
        r * r == v
    }
}

fn main() {}
