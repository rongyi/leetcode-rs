/// LeetCode 341: Flatten Nested List Iterator
///
/// Each element is either an integer or a nested list.
/// The iterator flattens the structure and yields integers in order.
///
/// # Approach: flatten upfront (DFS)
///
/// Recursively walk the nested list in `new()` and collect all integers
/// into a flat Vec. Then iterate with a simple index.
/// O(n) time to construct, O(1) per next/has_next, O(n) space.

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    nums: Vec<i32>,
    pos: usize,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut iter = Self {
            nums: vec![],
            pos: 0,
        };
        for v in nested_list.into_iter() {
            iter.flatten(v);
        }
        iter
    }

    fn next(&mut self) -> i32 {
        let val = self.nums[self.pos];
        self.pos += 1;
        val
    }

    fn has_next(&self) -> bool {
        self.pos < self.nums.len()
    }

    /// DFS: recursively collect all integers into `self.nums`.
    fn flatten(&mut self, item: NestedInteger) {
        match item {
            NestedInteger::Int(val) => self.nums.push(val),
            NestedInteger::List(lst) => {
                for v in lst.into_iter() {
                    self.flatten(v);
                }
            }
        }
    }
}

fn main() {
    // [[1,1],2,[1,1]]  →  [1,1,2,1,1]
    let list = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    let mut iter = NestedIterator::new(list);
    let mut result = vec![];
    while iter.has_next() {
        result.push(iter.next());
    }
    println!("Flattened: {:?}", result);
    println!("Expected:  [1, 1, 2, 1, 1]");
    println!("Pass: {}", result == vec![1, 1, 2, 1, 1]);
}
