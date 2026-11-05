use std::collections::{BTreeSet, HashMap};

struct LFUCache {
    capacity: usize,
    // freq -> (ts, key)
    freq_map: HashMap<usize, BTreeSet<(usize, i32)>>, // freq -> set of keys
    values: HashMap<i32, (usize, i32, usize)>,        // key -> (freq, value, timestamp)
    min_lfu: usize,
    timestamp: usize,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            freq_map: HashMap::new(),
            values: HashMap::new(),
            min_lfu: 0,
            timestamp: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&(old_freq, value, ts)) = self.values.get(&key) {
            let mut need_delete = false;
            self.freq_map.entry(old_freq).and_modify(|set| {
                set.remove(&(ts, key));
                if set.is_empty() {
                    need_delete = true;
                }
            });
            if need_delete {
                self.freq_map.remove(&old_freq);
            }
            let new_freq = old_freq + 1;
            self.freq_map
                .entry(new_freq)
                .or_insert_with(BTreeSet::new)
                .insert((self.timestamp, key));
            self.values.insert(key, (new_freq, value, self.timestamp));
            self.timestamp += 1;

            if !self.freq_map.contains_key(&self.min_lfu)
                || self.freq_map.get(&self.min_lfu).unwrap().is_empty()
            {
                self.min_lfu += 1;
            }

            return value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if let Some(&mut (old_freq, _, ts)) = self.values.get_mut(&key) {
            let mut need_delete = false;
            self.freq_map.entry(old_freq).and_modify(|set| {
                set.remove(&(ts, key));
                if set.is_empty() {
                    need_delete = true;
                }
            });
            if need_delete {
                self.freq_map.remove(&old_freq);
            }

            let new_freq = old_freq + 1;
            self.freq_map
                .entry(new_freq)
                .or_insert_with(BTreeSet::new)
                .insert((self.timestamp, key));
            self.values.insert(key, (new_freq, value, self.timestamp));

            if !self.freq_map.contains_key(&self.min_lfu)
                || self.freq_map.get(&self.min_lfu).unwrap().is_empty()
            {
                self.min_lfu += 1;
            }
        } else {
            if self.values.len() >= self.capacity {
                let min_lfu = self.min_lfu;
                let &(ts, min_key_to_remove) =
                    self.freq_map.get(&min_lfu).unwrap().iter().next().unwrap();
                // those are all the TLE method
                // let key_timestamp_pairs: Vec<(i32, usize)> = self
                //     .freq_map
                //     .get(&min_lfu)
                //     .unwrap()
                //     .iter()
                //     .map(|key| (*key, self.values[key].2))
                //     .collect();
                // // key_timestamp_pairs.sort_by_key(|&(_, timestamp)| timestamp);
                // // let min_key_to_remove = key_timestamp_pairs[0].0;
                // let mut heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();
                // for (key, ts) in key_timestamp_pairs.into_iter() {
                //     heap.push(Reverse((ts, key)));
                // }
                // let Reverse((_, min_key_to_remove)) = heap.pop().unwrap();

                // // let min_keys = self.freq_map.get(&min_lfu).unwrap().iter().copied();
                // // let mut min_key_to_remove = 0;
                // // let mut min_timestamp = usize::MAX;
                // // for key in min_keys {
                // //     if self.values[&key].2 < min_timestamp {
                // //         min_timestamp = self.values[&key].2;
                // //         min_key_to_remove = key;
                // //     }
                // // }

                let mut need_delete = false;
                self.freq_map.entry(min_lfu).and_modify(|set| {
                    set.remove(&(ts, min_key_to_remove));
                    if set.is_empty() {
                        need_delete = true;
                    }
                });
                if need_delete {
                    self.freq_map.remove(&min_lfu);
                }

                self.values.remove(&min_key_to_remove);
            }
            let freq = 1;
            self.freq_map
                .entry(freq)
                .or_insert_with(BTreeSet::new)
                .insert((self.timestamp, key));
            self.values.insert(key, (freq, value, self.timestamp));
            self.min_lfu = 1;
        }
        self.timestamp += 1;
    }
}

fn main() {
    let mut cache = LFUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1)); // returns 1
    cache.put(3, 3); // evicts key 2
    println!("{}", cache.get(2)); // returns -1 (not found)
    println!("{}", cache.get(3)); // returns 3.
    cache.put(4, 4); // evicts key 1.
    println!("{}", cache.get(1)); // returns -1 (not found)
    println!("{}", cache.get(3)); // returns 3
    println!("{}", cache.get(4)); // returns 4
}
