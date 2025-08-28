struct Solution;

impl Solution {
    pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        tiles.sort_unstable();
        let sz = tiles.len();
        let mut j = 0;
        let mut chunk_len = 0;
        let mut ret = 0;

        for i in 0..sz {
            let cur_start = tiles[i][0];
            let cover_end = cur_start + carpet_len;
            // totally covered
            while j < sz && tiles[j][1] < cover_end {
                chunk_len += tiles[j][1] - tiles[j][0] + 1;
                j += 1;
            }
            // partial cover
            let mut partial_len = 0;
            if j < sz && tiles[j][0] < cover_end {
                partial_len = cover_end - tiles[j][0];
            }
            ret = ret.max(chunk_len + partial_len);

            chunk_len -= tiles[i][1] - tiles[i][0] + 1;
        }

        ret
    }
}

fn main() {}
