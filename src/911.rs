struct Solution;

struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let n = persons.len();
        let mut votes = std::collections::HashMap::new();
        let mut leaders = Vec::with_capacity(n);
        let mut current_leader = -1;
        let mut max_votes = 0;

        for i in 0..n {
            let person = persons[i];
            let count = votes.entry(person).or_insert(0);
            *count += 1;

            // Update leader if this person now has more votes
            if *count >= max_votes {
                // If tie, the most recent vote wins (this happens automatically
                // because we check >= and update when equal)
                max_votes = *count;
                current_leader = person;
            }

            leaders.push(current_leader);
        }

        TopVotedCandidate { times, leaders }
    }

    fn q(&self, t: i32) -> i32 {
        // Binary search to find the rightmost time <= t
        // let mut left = 0;
        // let mut right = self.times.len();

        // while left < right {
        //     let mid = left + (right - left) / 2;
        //     if self.times[mid] <= t {
        //         left = mid + 1;
        //     } else {
        //         right = mid;
        //     }
        // }

        // match self.times.binary_search(&t) {
        //     Ok(idx) => self.leaders[idx],
        //     Err(idx) => self.leaders[idx - 1],
        // }

        let idx = self.times.partition_point(|&time| time <= t);
        self.leaders[idx - 1]
    }
}

fn main() {}
