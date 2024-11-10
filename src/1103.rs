struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let round_total = num_people * (num_people + 1) / 2;
        let mut ret: Vec<i32> = vec![0; num_people as usize];
        let mut expect_candies = 1;
        let mut i = 0;
        let mut acc = 0;
        loop {
            if expect_candies <= candies {
                ret[(i % num_people) as usize] += expect_candies;
            } else {
                ret[(i % num_people) as usize] += candies;
                break;
            }
            i += 1;
            candies -= expect_candies;
            expect_candies += 1;
        }

        ret
    }
}

fn main() {}
