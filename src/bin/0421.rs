/// LeetCode 421: Maximum XOR of Two Numbers in an Array
///
/// Build a binary Trie (0/1 branches) from all numbers' 32-bit
/// representations. Then for each number, walk the Trie choosing
/// the opposite bit at each step to maximize XOR.
///
/// Time: O(n * 32), Space: O(n * 32)

#[derive(Default)]
struct Trie {
    child: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut node = self;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as usize;
            node = node.child[bit].get_or_insert_with(|| Box::new(Trie::default()));
        }
    }

    /// Returns the max XOR of `num` with any number already in the Trie.
    fn max_xor(&self, num: i32) -> i32 {
        let mut ret = 0;
        let mut node = self;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as usize;
            let opposite = 1 - bit;
            if let Some(next) = &node.child[opposite] {
                // Opposite bit exists → this bit contributes to XOR.
                ret |= 1 << i;
                node = next;
            } else {
                // Must take the same bit (always exists since num was inserted).
                node = node.child[bit].as_ref().unwrap();
            }
        }
        ret
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::default();
        for &num in &nums {
            trie.insert(num);
        }

        let mut ans = 0;
        for &num in &nums {
            ans = ans.max(trie.max_xor(num));
        }
        ans
    }
}

struct Solution;

fn main() {
    let tests = [
        (vec![3, 10, 5, 25, 2, 8], 28),
        (vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70], 127),
        (vec![0], 0),
    ];

    for (nums, expected) in &tests {
        let result = Solution::find_maximum_xor(nums.clone());
        println!(
            "{} nums={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            nums,
            result,
            expected
        );
    }
}
