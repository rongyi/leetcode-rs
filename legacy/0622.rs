struct MyCircularQueue {
    data: Vec<i32>,
    n: usize,
    begin: usize,
    readable: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            n: k as usize,
            begin: 0,
            readable: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.readable == self.n {
            return false;
        }
        self.data[(self.readable + self.begin) % self.n] = value;
        self.readable += 1;

        true
    }

    fn de_queue(&mut self) -> bool {
        if self.readable == 0 {
            return false;
        }
        self.readable -= 1;
        if self.begin == self.n - 1 {
            self.begin = 0;
        } else {
            self.begin += 1;
        }

        true
    }

    fn front(&self) -> i32 {
        if self.readable == 0 {
            return -1;
        }
        self.data[self.begin]
    }

    fn rear(&self) -> i32 {
        if self.readable == 0 {
            return -1;
        }
        self.data[(self.readable + self.begin - 1) % self.n]
    }

    fn is_empty(&self) -> bool {
        self.readable == 0
    }

    fn is_full(&self) -> bool {
        self.readable == self.n
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

fn main() {}
