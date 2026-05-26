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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-200));
        let mut tail = &mut dummy;
        let mut out: Vec<i32> = vec![];
        Self::dfs(head.as_ref(), &mut out);
        let mut uniq: Vec<i32> = vec![];
        let mut should_pop = false;
        for v in out.into_iter() {
            if let Some(&last) = uniq.last() {
                if v == last {
                    should_pop = true;
                } else {
                    if should_pop {
                        uniq.pop();
                        should_pop = false;
                    }
                    uniq.push(v);
                }
            } else {
                uniq.push(v);
            }
        }
        if should_pop {
            uniq.pop();
        }
        for v in uniq.into_iter() {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next.take()
    }

    fn dfs(p: Option<&Box<ListNode>>, out: &mut Vec<i32>) {
        match p {
            None => return,
            Some(cur) => {
                out.push(cur.val);
                Self::dfs(cur.next.as_ref(), out);
            }
        }
    }
}
fn main() {}
