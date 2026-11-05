struct Solution;

struct RLEIterator {
    encoding: Vec<i32>,
    pos: usize,
    cur_consumed: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self {
            encoding,
            pos: 0,
            cur_consumed: 0, // consumed in current pos
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut expect = n;

        while self.pos < self.encoding.len() {
            // shit, this pos can not hold the expect number, advance
            if self.cur_consumed + expect > self.encoding[self.pos] {
                expect -= self.encoding[self.pos] - self.cur_consumed;

                self.cur_consumed = 0;
                self.pos += 2;
            } else {
                self.cur_consumed += expect;
                return self.encoding[self.pos + 1];
            }
        }

        -1
    }
}

fn main() {}
