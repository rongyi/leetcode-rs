use std::collections::{BTreeMap, HashMap};

struct AllOne {
    // string -> cnt
    count_map: HashMap<String, usize>,
    bucket_map: BTreeMap<usize, Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            count_map: HashMap::new(),
            bucket_map: BTreeMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let cur_cnt = self.count_map.entry(key.clone()).or_insert(0);
        let mut need_delete_empty = false;
        self.bucket_map.entry(*cur_cnt).and_modify(|v| {
            v.retain(|k| k != &key);
            if v.is_empty() {
                need_delete_empty = true;
            }
        });
        if need_delete_empty {
            self.bucket_map.remove(cur_cnt);
        }
        *cur_cnt += 1;
        self.bucket_map
            .entry(*cur_cnt)
            .or_insert(Vec::new())
            .push(key);
    }

    fn dec(&mut self, key: String) {
        if let Some(cnt) = self.count_map.get_mut(&key) {
            let mut delete_empty_cnt = false;
            self.bucket_map.entry(*cnt).and_modify(|v| {
                v.retain(|k| k != &key);
                if v.is_empty() {
                    delete_empty_cnt = true;
                }
            });
            if delete_empty_cnt {
                self.bucket_map.remove(cnt);
            }
            *cnt -= 1;
            if *cnt > 0 {
                self.bucket_map.entry(*cnt).or_insert(Vec::new()).push(key);
            } else {
                self.count_map.remove(&key);
            }
        }
    }

    fn get_max_key(&self) -> String {
        self.bucket_map
            .iter()
            .rev()
            .next()
            .map(|(_, v)| v[0].clone())
            .unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.bucket_map
            .iter()
            .next()
            .map(|(_, v)| v[0].clone())
            .unwrap_or_default()
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
fn main() {}
