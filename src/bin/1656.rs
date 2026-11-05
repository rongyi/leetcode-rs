#![allow(dead_code)]

struct Solution;

struct OrderedStream {
    ptr: usize,
    stream: Vec<Option<String>>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            ptr: 0,
            stream: vec![None; n as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let index = id_key as usize - 1; // Convert to 0-indexed
        self.stream[index] = Some(value);

        let mut result = Vec::new();

        // Check if we can return a chunk starting from ptr
        if index == self.ptr {
            // Collect consecutive values until we hit None
            while self.ptr < self.stream.len() && self.stream[self.ptr].is_some() {
                // Unwrap is safe here because we just checked that it's Some
                result.push(self.stream[self.ptr].clone().unwrap());
                self.ptr += 1;
            }
        }

        result
    }
}

fn main() {}
