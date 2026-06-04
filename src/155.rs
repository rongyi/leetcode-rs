struct Solution;

struct MinStack {
    normal_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            normal_stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.normal_stack.push(val);
        if let Some(&v) = self.min_stack.last() {
            self.min_stack.push(v.min(val));
        } else {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        self.normal_stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        if let Some(&v) = self.normal_stack.last() {
            v
        } else {
            -1
        }
    }

    fn get_min(&self) -> i32 {
        if let Some(&v) = self.min_stack.last() {
            v
        } else {
            -1
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {}
