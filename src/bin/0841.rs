struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut unlocked_rooms: HashSet<i32> = HashSet::new();
        let mut q: VecDeque<i32> = VecDeque::new();
        let sz = rooms.len();

        q.push_back(0);

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            unlocked_rooms.insert(cur);
            if unlocked_rooms.len() == sz {
                return true;
            }

            for &key_for_next_room in rooms[cur as usize].iter() {
                if !unlocked_rooms.contains(&key_for_next_room) {
                    q.push_back(key_for_next_room);
                }
            }
        }

        unlocked_rooms.len() == rooms.len()
    }
}

fn main() {}
