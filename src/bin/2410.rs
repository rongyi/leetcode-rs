struct Solution;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();
        let mut ret = 0;
        for &p in players.iter() {
            let idx = trainers.partition_point(|&x| x < p);
            // no more match
            println!("{idx}");
            if idx == trainers.len() {
                break;
            }
            ret += 1;
            // last trainer used
            if idx + 1 == trainers.len() {
                break;
            }

            trainers = trainers[idx + 1..].iter().copied().collect();
        }

        ret
    }
}

fn main() {
    let players = vec![4, 7, 9];
    let trainers = vec![8, 2, 5, 8];
    Solution::match_players_and_trainers(players, trainers);
}
