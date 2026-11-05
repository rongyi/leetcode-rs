struct Solution;

use std::collections::HashMap;
struct FreqStack {
    max_freq: i32,
    val_freq: HashMap<i32, i32>,
    // freq-> [val]
    freq_stack: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            max_freq: 0,
            val_freq: HashMap::new(),
            freq_stack: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let cnt = self.val_freq.entry(val).or_insert(0);
        *cnt += 1;
        if *cnt > self.max_freq {
            self.max_freq = *cnt;
        }

        self.freq_stack.entry(*cnt).or_default().push(val);
    }

    fn pop(&mut self) -> i32 {
        if let Some(vals) = self.freq_stack.get_mut(&self.max_freq) {
            if let Some(val) = vals.pop() {
                self.val_freq.entry(val).and_modify(|x| *x -= 1);
                if vals.is_empty() {
                    self.max_freq -= 1;
                }

                return val;
            }
        }

        unreachable!()
    }
}

fn main() {}
