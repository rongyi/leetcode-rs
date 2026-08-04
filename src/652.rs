#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// LeetCode 652: Find Duplicate Subtrees
///
/// Instead of serializing to strings (O(n²)), assign each unique subtree
/// a numeric id via interning:
///   (val, left_id, right_id) → unique id
///
/// Two subtrees are identical ⇔ they get the same id.
/// Track how many nodes share each id; ids seen ≥ 2 are duplicates.
/// O(n) time.
struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // (val, left_id, right_id) → unique id. 0 = null.
        let mut id_map: HashMap<(i32, usize, usize), usize> = HashMap::new();
        // id → list of nodes with that id
        let mut groups: HashMap<usize, Vec<Option<Rc<RefCell<TreeNode>>>>> = HashMap::new();
        let mut next_id = 1;

        Self::dfs(root.as_ref(), &mut id_map, &mut groups, &mut next_id);

        groups
            .into_iter()
            .filter(|(_, nodes)| nodes.len() > 1)
            .map(|(_, nodes)| nodes[0].clone())
            .collect()
    }

    /// Returns the interned id for the subtree rooted at `node`.
    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        id_map: &mut HashMap<(i32, usize, usize), usize>,
        groups: &mut HashMap<usize, Vec<Option<Rc<RefCell<TreeNode>>>>>,
        next_id: &mut usize,
    ) -> usize {
        let Some(node) = node else {
            return 0; // null subtree
        };

        let n = node.borrow();
        let left_id = Self::dfs(n.left.as_ref(), id_map, groups, next_id);
        let right_id = Self::dfs(n.right.as_ref(), id_map, groups, next_id);

        let key = (n.val, left_id, right_id);
        let id = *id_map.entry(key).or_insert_with(|| {
            let id = *next_id;
            *next_id += 1;
            id
        });

        groups.entry(id).or_default().push(Some(node.clone()));
        id
    }
}

fn main() {
    // Build tree:
    //       1
    //      / \
    //     2   3
    //    /   / \
    //   4   2   4
    //      /
    //     4
    let n2a = Rc::new(RefCell::new(TreeNode::new(2)));
    let n4a = Rc::new(RefCell::new(TreeNode::new(4)));
    n2a.borrow_mut().left = Some(n4a.clone());

    let n2b = Rc::new(RefCell::new(TreeNode::new(2)));
    let n4b = Rc::new(RefCell::new(TreeNode::new(4)));
    n2b.borrow_mut().left = Some(n4b.clone());

    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n4c = Rc::new(RefCell::new(TreeNode::new(4)));
    n3.borrow_mut().left = Some(n2b.clone());
    n3.borrow_mut().right = Some(n4c.clone());

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(n2a.clone());
    root.borrow_mut().right = Some(n3.clone());

    let result = Solution::find_duplicate_subtrees(Some(root));
    println!("Duplicate subtrees: {:?}", result.len());
    for tree in &result {
        if let Some(t) = tree {
            println!("  root val: {}", t.borrow().val);
        }
    }
    println!("Expected: 2 subtrees with root values 2 and 4");
}
