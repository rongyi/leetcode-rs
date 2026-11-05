struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let sz = blocks.len();
        let win_sz = k as usize;
        let mut max_black_in_win = 0;
        let mut cur_black_in_win = 0;

        for j in 0..win_sz {
            // black?
            if blocks[j] == b'B' {
                cur_black_in_win += 1;
            }
        }
        max_black_in_win = max_black_in_win.max(cur_black_in_win);

        for j in win_sz..sz {
            if blocks[j] == b'B' {
                cur_black_in_win += 1;
            }
            if blocks[j - win_sz] == b'B' {
                cur_black_in_win -= 1;
            }
            max_black_in_win = max_black_in_win.max(cur_black_in_win);
        }

        win_sz as i32 - max_black_in_win
    }
}

fn main() {}
