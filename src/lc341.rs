struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut stack = Vec::new();

        let mut ret = NestedIterator { stack };
        ret.flattern(nestedList);

        ret
    }

    // not lazy
    fn flattern(&mut self, nestedList: Vec<NestedInteger>) {
        for val in nestedList.into_iter().rev() {
            match val {
                NestedInteger::Int(i) => self.stack.push(i),
                NestedInteger::List(lst) => self.flattern(lst),
            }
        }
    }

    fn next(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {}
