use std::collections::HashMap;

struct Solution;

struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut vote_cnt: HashMap<i32, i32> = HashMap::new();
        let mut max_votes = 0;
        let mut cur_leader = -1;
        let mut leaders = Vec::with_capacity(persons.len());
        for &p in persons.iter() {
            let cur = vote_cnt.entry(p).or_insert(0);
            *cur += 1;
            if *cur >= max_votes {
                max_votes = *cur;
                cur_leader = p;
            }
            leaders.push(cur_leader);
        }

        Self { times, leaders }
    }

    fn q(&self, t: i32) -> i32 {
        match self.times.binary_search(&t) {
            Ok(idx) => self.leaders[idx],
            Err(idx) => self.leaders[idx - 1],
        }
    }
}

fn main() {}
