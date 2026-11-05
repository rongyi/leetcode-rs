#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<Node>>;

fn new_link(value: i32) -> Link {
    Rc::new(RefCell::new(Node::new(value)))
}

struct Node {
    value: i32,
    right: Option<Link>,
    down: Option<Link>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            right: None,
            down: None,
        }
    }
}

struct Skiplist {
    start: Link,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Self {
            start: new_link(i32::MIN),
        }
    }

    fn find(&self, target: i32) -> Vec<Link> {
        let mut cur = Some(self.start.clone());
        let mut stack = Vec::new();
        while let Some(node) = cur {
            if node
                .borrow()
                .right
                .clone()
                .filter(|n| n.borrow().value <= target)
                .is_some()
            {
                cur = node.borrow().right.clone();
            } else {
                stack.push(node.clone());
                cur = node.borrow().down.clone();
            }
        }

        stack
    }

    fn search(&self, target: i32) -> bool {
        self.find(target)
            .last()
            .filter(|n| n.borrow().value == target)
            .is_some()
    }

    fn add(&mut self, num: i32) {
        let mut left_nodes = self.find(num);
        let mut node = new_link(num);
        let left_node = left_nodes.pop().unwrap();
        node.borrow_mut().right = left_node.borrow_mut().right.clone();
        left_node.borrow_mut().right = Some(node.clone());
        while rand::random() {
            let left_node = match left_nodes.pop() {
                Some(left_node) => left_node,
                None => {
                    let new_start = new_link(i32::MIN);
                    new_start.borrow_mut().down = Some(self.start.clone());
                    self.start = new_start;
                    self.start.clone()
                }
            };
            let new_node = new_link(num);
            new_node.borrow_mut().down = Some(node);
            new_node.borrow_mut().right = left_node.borrow_mut().right.clone();

            left_node.borrow_mut().right = Some(new_node.clone());

            node = new_node;
        }
    }

    fn erase(&self, num: i32) -> bool {
        let mut num_found = false;

        for node in self.find(num - 1) {
            let target_node_opt = node.borrow().right.clone();
            if let Some(target_node) = target_node_opt.filter(|n| n.borrow().value == num) {
                num_found = true;
                node.borrow_mut().right = target_node.borrow().right.clone();
                target_node.borrow_mut().right = None;
                target_node.borrow_mut().down = None;
            }
        }

        num_found
    }
}

fn main() {
    let mut node = Some(new_link(10));
    let val = node.filter(|x| x.borrow().value < 20);
    assert!(val.is_some());
}
