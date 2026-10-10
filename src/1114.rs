struct Solution;

fn main() {}

use std::sync::{Condvar, Mutex};

pub struct Foo {
    pair: (Mutex<u8>, Condvar),
}

impl Foo {
    pub fn new() -> Self {
        Self {
            pair: (Mutex::new(1), Condvar::new()),
        }
    }

    pub fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        print_first();

        let (lock, cvar) = &self.pair;
        let mut state = lock.lock().unwrap();
        *state = 2;
        cvar.notify_all(); // Wake up sleeping threads
    }

    pub fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let (lock, cvar) = &self.pair;
        let mut state = lock.lock().unwrap();

        // Wait until state becomes 2
        while *state != 2 {
            state = cvar.wait(state).unwrap();
        }

        print_second();

        *state = 3;
        cvar.notify_all();
    }

    pub fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let (lock, cvar) = &self.pair;
        let mut state = lock.lock().unwrap();

        // Wait until state becomes 3
        while *state != 3 {
            state = cvar.wait(state).unwrap();
        }

        print_third();
    }
}
