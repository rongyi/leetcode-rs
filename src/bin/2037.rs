pub struct Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();

        let mut moves = 0;
        for (seat, student) in seats.iter().zip(students.iter()) {
            moves += (seat - student).abs();
        }
        moves
    }
}

fn main() {}
