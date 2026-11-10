struct Solution;

use std::collections::BinaryHeap;

mod rongyi {
    struct Solution;

    use std::collections::BinaryHeap;
    impl Solution {
        pub fn is_possible(target: Vec<i32>) -> bool {
            let target: Vec<i64> = target.into_iter().map(|x| x as i64).collect();
            let mut s: i64 = target.iter().sum();

            let mut q: BinaryHeap<i64> = target.iter().copied().collect();

            while s > 1 && *q.peek().unwrap() > s / 2 {
                let cur = q.pop().unwrap();

                s -= cur;
                if s <= 1 {
                    if s == 1 {
                        return true;
                    } else {
                        return false;
                    }
                }

                // 这样去理解就可以了
                // q.push(cur - s);
                // s += cur - s;
                // 遇到有个数特别大的情况，需要碾掉,因为余下的那些数被加到这个数上很多次
                // 这是我见过 % 用的最巧妙的地方了
                q.push(cur % s);
                s += cur % s;
            }

            s == target.len() as i64
        }
    }
    fn main() {}
}

/// 1354. Construct Target Array With Multiple Sums
///
/// Work backwards. Every forward step replaces one element with the *total sum*,
/// so in `target` the last-touched element is the maximum, and
///
///     max = prev + rest        (rest = sum of all the other elements)
///
/// Therefore `prev = max - rest`, and a state is only reachable if `max > rest`
/// (otherwise `prev <= 0`, which no array of positive integers can contain).
///
/// Peeling one `rest` at a time is too slow for inputs like `[1, 1, 1e9]`, so
/// hop straight to the remainder: `prev = max % rest`. That is only sound when
/// the remainder is non-zero — see the `max % rest == 0` guard below.
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        // The running total can reach 5e4 * 1e9, so i64.
        let mut total: i64 = target.iter().map(|&x| x as i64).sum();
        let mut values: BinaryHeap<i64> = target.into_iter().map(|x| x as i64).collect();

        loop {
            let max = values.pop().unwrap();
            let rest = total - max;

            // Nothing left to unwind: every element is 1 (the starting array).
            if max == 1 {
                return true;
            }
            // `rest == 1` means the array is [1, max], which unwinds one by one
            // for free. Must come before the divisibility check, since
            // `max % 1 == 0` always holds.
            if rest == 1 {
                return true;
            }
            // Unreachable states:
            //   rest == 0            -> single element, can't grow from [1] to >1
            //   max <= rest          -> prev would be <= 0
            //   max % rest == 0      -> prev would be 0 at last, if you follow this chain, e.g max: 8, rest is 4, [8, 2, 2] -> [4, 2, 2] -> [0, 2, 2], right? and we will get rest is 0; the chain down would have
            //                           to pass through `rest`, breaking max > rest
            if rest == 0 || max <= rest || max % rest == 0 {
                return false;
            }

            let reduced = max % rest;
            total = rest + reduced;
            values.push(reduced);
        }
    }
}

fn main() {
    let cases = [
        (vec![9, 3, 5], true),
        (vec![1, 1, 1, 2], false),
        (vec![8, 5], true),
        // regression: max % rest == 0 must not push a 0 into the heap
        (vec![8, 1, 3], false),
        (vec![1, 1, 2, 100], false),
        // single element: only [1] is reachable
        (vec![1], true),
        (vec![5], false),
        // huge value, one subtraction chain the modulo short-circuits
        (vec![1, 1_000_000_000], true),
    ];

    for (target, want) in cases {
        let got = Solution::is_possible(target.clone());
        println!("{target:?} -> {got} (want {want})");
        assert_eq!(got, want, "{target:?}");
    }
    println!("all cases passed");
}
