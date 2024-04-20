#![allow(dead_code)]

#[derive(Default)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur = self.head.as_ref();
        let mut i = index;
        while let Some(node) = cur {
            if i == 0 {
                return node.val;
            }
            cur = node.next.as_ref();
            i -= 1;
        }
        -1
    }

    fn add_at_head(&mut self, val: i32) {
        let mut new_node = ListNode::new(val);
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node));
    }

    fn add_at_tail(&mut self, val: i32) {
        if self.head.is_none() {
            self.head = Some(Box::new(ListNode::new(val)));
            return;
        }

        let mut cur = self.head.as_mut();
        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(Box::new(ListNode::new(val)));
                break;
            } else {
                cur = node.next.as_mut();
            }
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            let origin_head = self.head.take();
            let mut new_node = ListNode::new(val);
            new_node.next = origin_head;
            self.head = Some(Box::new(new_node));

            return;
        }

        let mut cur = self.head.as_mut();
        for _ in 0..index - 1 {
            // test contain invalid index
            if let Some(node) = cur {
                cur = node.next.as_mut();
            } else {
                // invalid index
                return;
            }
        }

        if let Some(node) = cur {
            let origin_next = node.next.take();
            let mut new_node = ListNode::new(val);
            new_node.next = origin_next;
            node.next = Some(Box::new(new_node));
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            let mut cur = self.head.take();
            if let Some(node) = cur.as_mut() {
                self.head = node.next.take();
            }
        }

        let mut cur = self.head.as_mut();
        for _ in 0..index - 1 {
            // test contain invalid index!
            if let Some(node) = cur {
                cur = node.next.as_mut();
            } else {
                // invalid index
                return;
            }
        }
        // delete cur->next
        if let Some(node) = cur {
            let mut next = node.next.take();
            if let Some(next_node) = next.as_mut() {
                node.next = next_node.next.take();
            }
        }
    }
}

fn main() {}
