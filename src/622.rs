struct Solution;

struct MyCircularQueue {
    data: Vec<i32>,
    capacity: usize,
    begin: usize, // read pos
    sz: usize,    // also the next write position
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            capacity: k as usize,
            begin: 0,
            sz: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.sz == self.capacity {
            return false;
        }
        self.data[(self.sz + self.begin) % self.capacity] = value;
        self.sz += 1;

        true
    }

    fn de_queue(&mut self) -> bool {
        if self.sz == 0 {
            return false;
        }
        self.sz -= 1;
        self.begin = (self.begin + 1) % self.capacity;

        true
    }

    fn front(&self) -> i32 {
        if self.sz == 0 {
            return -1;
        }
        self.data[self.begin]
    }

    fn rear(&self) -> i32 {
        if self.sz == 0 {
            return -1;
        }
        self.data[(self.sz + self.begin - 1) % self.capacity]
    }

    fn is_empty(&self) -> bool {
        self.sz == 0
    }

    fn is_full(&self) -> bool {
        self.sz == self.capacity
    }
}

fn main() {}
