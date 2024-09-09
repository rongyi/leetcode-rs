struct Solution;

struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut ret = 1;
        while !self.stack.is_empty() && self.stack.last().unwrap().0 <= price {
            ret += self.stack.last().unwrap().1;
            self.stack.pop();
        }
        self.stack.push((price, ret));
        ret
    }
}

fn main() {}
