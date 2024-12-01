struct Solution;

use std::collections::{BTreeMap, HashMap};
#[derive(Debug, Default)]
struct SnapshotArray {
    snap_id: i32,
    snaps: HashMap<i32, BTreeMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            snap_id: 0,
            snaps: HashMap::new(),
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.snaps
            .entry(index)
            .or_insert(BTreeMap::new())
            .insert(self.snap_id, val);
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;

        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        if let Some(cur_idx_snap) = self.snaps.get(&index) {
            match cur_idx_snap.range(..=snap_id).next_back() {
                Some((&_, &val)) => val,
                None => 0,
            }
        } else {
            0
        }
    }
}

fn main() {}
