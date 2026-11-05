struct MyCircularDeque {
    data: Vec<i32>,
    cap: i32,
    len: i32,
    front: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            data: vec![0; k as usize],
            cap: k,
            len: 0,
            front: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.front as usize] = value;
        if self.front == self.cap - 1 {
            self.front = 0;
        } else {
            self.front += 1;
        }

        self.len += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        let start = (self.front - self.len + self.cap) % self.cap;
        let idx = (start - 1 + self.cap) % self.cap;
        self.data[idx as usize] = value;

        self.len += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.front == 0 {
            self.front = self.cap - 1;
        } else {
            self.front -= 1;
        }

        self.len -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.len -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let idx = (self.front - 1 + self.cap) % self.cap;

        self.data[idx as usize]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        let idx = (self.front - self.len + self.cap) % self.cap;
        self.data[idx as usize]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }
}

// ["MyCircularDeque","insertFront","deleteLast","getRear","getFront","getFront","deleteFront","insertFront","insertLast","insertFront","getFront","insertFront"]
// [[4],                [9],           [],           [],       [],        [],         [],[6],[5],[9],[],[6]]
//
fn main() {}
