struct Solution;

impl Solution {
    /// LeetCode 331: Verify Preorder Serialization of a Binary Tree
    ///
    /// A preorder string like "9,3,4,#,#,1,#,#,2,#,6,#,#" encodes a
    /// binary tree. '#' means null. Verify if it's valid.
    ///
    /// # Approach: slot counting
    ///
    /// Think of "slots" = places where a node can go.
    ///   - Start with 1 slot (the root).
    ///   - Every node consumes 1 slot.
    ///   - A real node creates 2 new slots (left + right child).
    ///   - A null node (#) creates 0 new slots.
    ///
    /// If slots ever go negative → invalid.
    /// At the end, slots must be exactly 0 (tree fully built).
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut slots = 1;

        for token in preorder.split(',') {
            slots -= 1; // consume a slot for this node
            if slots < 0 {
                return false;
            }
            if token != "#" {
                slots += 2; // real node → add left + right slots
            }
        }

        slots == 0
    }
}

fn main() {
    let tests = [
        ("9,3,4,#,#,1,#,#,2,#,6,#,#", true),
        ("1,#", false),
        ("9,#,#,1", false),
        ("#", true),    // empty tree
        ("1,#,#", true), // single node
    ];

    for (s, expected) in &tests {
        let result = Solution::is_valid_serialization(s.to_string());
        println!(
            "{} {:40} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            s,
            result,
            expected
        );
    }
}
