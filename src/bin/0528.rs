/// LeetCode 528: Random Pick with Weight
///
/// Prefix sums + binary search. Random in [1, total], find the
/// first index where prefix[i] >= random value.
///
/// w = [1, 3] → prefix = [1, 4], total = 4
///   random 1        → prefix[0]=1 >= 1  → index 0
///   random 2,3,4    → prefix[1]=4 >= 2  → index 1
/// Probability: 0 → 1/4,  1 → 3/4  ✓
use rand::Rng;

struct Solution {
    prefix: Vec<i32>,
    total: i32,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for weight in w {
            sum += weight;
            prefix.push(sum);
        }
        Self { prefix, total: sum }
    }

    fn pick_index(&self) -> i32 {
        let target = rand::thread_rng().gen_range(1..=self.total);
        let (mut lo, mut hi) = (0, self.prefix.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.prefix[mid] < target {
                lo = mid + 1;
            } else {
                // may equal
                hi = mid;
            }
        }
        lo as i32
    }
}

fn main() {
    let s = Solution::new(vec![1, 3]);
    let mut counts = [0; 2];
    for _ in 0..100_000 {
        counts[s.pick_index() as usize] += 1;
    }
    println!(
        "w=[1,3]: index 0 → {} (~25%), index 1 → {} (~75%)",
        counts[0], counts[1]
    );
}
