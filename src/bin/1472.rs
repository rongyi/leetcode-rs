#![allow(dead_code)]

struct Solution;

struct BrowserHistory {
    // using it like stack
    visits: Vec<String>,
    pos: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            visits: vec![homepage],
            pos: 0,
        }
    }

    fn visit(&mut self, url: String) {
        // this is the key point to clear forward history
        self.visits.truncate(self.pos + 1);
        self.visits.push(url);
        // make it to top, clear forward history
        self.pos = self.visits.len() - 1;
    }

    fn back(&mut self, steps: i32) -> String {
        let mut cur = self.pos as i32 - steps;
        cur = cur.max(0);
        self.pos = cur as usize;
        self.visits[cur as usize].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let mut cur = self.pos as i32 + steps;
        cur = cur.min(self.visits.len() as i32 - 1);
        self.pos = cur as usize;

        self.visits[cur as usize].clone()
    }
}

fn main() {}
