struct Solution;

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

pub struct MyLinkedList {
    head: Option<Box<Node>>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList { head: None, len: 0 }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index as usize >= self.len {
            return -1;
        }

        let mut curr = self.head.as_ref();
        for _ in 0..index {
            if let Some(node) = curr {
                curr = node.next.as_ref();
            }
        }

        curr.map_or(-1, |node| node.val)
    }

    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len as i32, val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index as usize > self.len {
            return;
        }
        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut curr = self.head.as_mut();
        for _ in 0..(index - 1) {
            if let Some(node) = curr {
                curr = node.next.as_mut();
            }
        }

        if let Some(node) = curr {
            let new_node = Box::new(Node {
                val,
                next: node.next.take(),
            });
            node.next = Some(new_node);
            self.len += 1;
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index as usize >= self.len {
            return;
        }
        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
            self.len -= 1;
            return;
        }

        let mut curr = self.head.as_mut();
        for _ in 0..(index - 1) {
            if let Some(node) = curr {
                curr = node.next.as_mut();
            }
        }

        if let Some(node) = curr {
            let node_to_remove = node.next.take();
            if let Some(mut target) = node_to_remove {
                node.next = target.next.take();
                self.len -= 1;
            }
        }
    }
}

fn main() {}
