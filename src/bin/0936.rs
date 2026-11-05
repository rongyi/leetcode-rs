struct Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp: Vec<char> = stamp.chars().collect();
        let mut target: Vec<char> = target.chars().collect();

        let sz = target.len();
        let mut visited = vec![false; sz];

        let mut ret = Vec::new();
        let mut stars = 0;

        while stars < sz {
            let mut has_replace = false;
            for i in 0..=sz - stamp.len() {
                if !visited[i] && Self::can_replace(&target, i, &stamp) {
                    stars = Self::do_replace(&mut target, i, stamp.len(), stars);
                    has_replace = true;
                    visited[i] = true;
                    ret.push(i as i32);
                    if stars == sz {
                        break;
                    }
                }
            }

            if !has_replace {
                return vec![];
            }
        }

        ret.reverse();
        ret
    }

    fn can_replace(target: &Vec<char>, start: usize, stamp: &Vec<char>) -> bool {
        for i in 0..stamp.len() {
            if target[i + start] != '*' && target[i + start] != stamp[i] {
                return false;
            }
        }

        return true;
    }

    fn do_replace(target: &mut Vec<char>, start: usize, len: usize, mut star_cnt: usize) -> usize {
        for i in 0..len {
            if target[i + start] != '*' {
                target[i + start] = '*';
                star_cnt += 1;
            }
        }

        star_cnt
    }
}

fn main() {}
