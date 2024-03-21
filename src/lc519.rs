use std::collections::HashMap;

use rand::Rng;

struct Solution {
    num_row: usize,
    num_col: usize,
    swap_history: HashMap<usize, usize>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            num_row: m as usize,
            num_col: n as usize,
            swap_history: HashMap::new(),
            size: m as usize * n as usize,
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        // random target to swap
        // 举例
        // 1 2 3 4 5 6 7 8
        // 假设随机摇中3的位置，我们此时要把该位置给flip了。 此时 size_--,
        // 把最后位置和位置3上的数字对换， 下一轮再摇到位置3时，我们此时就不是
        // 把原来的3给flip而是把8给flip了。这种关系就保存在map中。
        // 记录 pos ==> flip_index
        let pos = rng.gen_range(0..self.size);
        self.size -= 1;

        let mut flip_index = pos;
        if self.swap_history.contains_key(&pos) {
            flip_index = self.swap_history[&pos];
        }

        // the postion size_ has been swapped, we need to find the real
        // index to flip
        // 这种情况产生于上一轮选到的是 size_ - 2这个位置(倒数第二个)的
        // e.g.
        // 1 2 3 4 5 6 7 8
        // 摇到7的位置，flip之后变成
        // 1 2 3 4 5 6 8 7
        // 所以再摇到相同位置选择的应该是8不是7
        if self.swap_history.contains_key(&self.size) {
            self.swap_history.insert(pos, self.swap_history[&self.size]);
        } else {
            self.swap_history.insert(pos, self.size);
        }

        vec![flip_index / self.num_col, flip_index % self.num_col]
            .into_iter()
            .map(|v| v as i32)
            .collect()
    }

    fn reset(&mut self) {
        self.swap_history.clear();
        self.size = self.num_row * self.num_col;
    }
}

fn main() {}
