struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut lst = Vec::new();
        while let Some(mut node) = head {
            lst.push(node.val.clone());
            // consume this single list
            head = node.next.take();
        }
        let mut stk = Vec::new();
        let mut ret = Vec::new();
        for &val in lst.iter().rev() {
            while !stk.is_empty() && *stk.last().unwrap() <= val {
                // 小的对于后面的判断确实无用了，
                // 拿 4 3, 5 举例
                //    ^ 当前值
                // 栈里： [5, 3] <- top
                // 这个 3 会被弹走，4 之后假设两种情况： >= 4 但 3 本身就没用， < 4 那这个 4 就会被选中作为next big, 所以 3 同样没有用
                // 所以弹出的不影响后续遇到值的计算，算是某种贪心
                stk.pop();
            }
            ret.push(*stk.last().unwrap_or(&0));
            stk.push(val);
        }
        ret.into_iter().rev().collect()
    }
}

fn main() {}
