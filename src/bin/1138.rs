struct Solution;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut tgt = vec!['a'];
        tgt.extend(target.chars());
        let mut ret: Vec<char> = Vec::new();
        for i in 1..tgt.len() {
            let r = Self::route(tgt[i - 1], tgt[i]);
            ret.extend(r);
            ret.push('!');
        }
        ret.into_iter().collect()
    }

    fn route(from: char, to: char) -> Vec<char> {
        if from == to {
            return vec![];
        }
        let mut ret: Vec<char> = Vec::new();
        if from == 'z' {
            let mut up = vec!['U'];
            up.extend(&Self::route('u', to)[..]);
            return up;
        }
        if to == 'z' {
            let mut tou = Self::route(from, 'u');
            tou.push('D');
            return tou;
        }
        let cold = (to as i8 - 'a' as i8) % 5 - (from as i8 - 'a' as i8) % 5;
        let rowd = (to as i8 - 'a' as i8) / 5 - (from as i8 - 'a' as i8) / 5;
        if rowd != 0 {
            let val: Vec<char> = (0..rowd.abs())
                .map(|_| if rowd > 0 { 'D' } else { 'U' })
                .collect();
            ret.extend(val);
        }
        if cold != 0 {
            let val: Vec<char> = (0..cold.abs())
                .map(|_| if cold > 0 { 'R' } else { 'L' })
                .collect();
            ret.extend(val);
        }

        ret
    }
}

fn main() {}
