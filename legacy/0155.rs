struct Solution;

struct MinStack {
    normal: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            normal: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.normal.push(val);
        match self.min.last() {
            Some(top) => self.min.push(val.min(*top)),
            None => {
                self.min.push(val);
            }
        }
    }

    fn pop(&mut self) {
        self.normal.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        *self.normal.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
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
