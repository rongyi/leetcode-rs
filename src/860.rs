
struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut changes: BTreeMap<i32, i32> = BTreeMap::new();

        for &b in bills.iter() {
            if b > 5 {
                let mut cur_change = b - 5;
                match cur_change {
                    5 => {
                        if let Some(cnt) = changes.get_mut(&cur_change) {
                            *cnt -= 1;
                            if *cnt == 0 {
                                changes.remove(&cur_change);
                            }
                        } else {
                            // dont have change
                            return false;
                        }
                    }
                    10 => {
                        if let Some(cnt) = changes.get_mut(&cur_change) {
                            *cnt -= 1;
                            if *cnt == 0 {
                                changes.remove(&cur_change);
                            }
                        } else {
                            // try 2 5
                            if let Some(cnt) = changes.get_mut(&5) {
                                if *cnt >= 2 {
                                    *cnt -= 2;
                                    if *cnt == 0 {
                                        changes.remove(&5);
                                    }
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                    }
                    15 => {
                        // 3 * 5 or 5 + 10
                        let mut cnt10 = 0;
                        let mut cnt5 = 0;
                        if let Some(v) = changes.get(&10) {
                            cnt10 = *v;
                        }
                        if let Some(v) = changes.get(&5) {
                            cnt5 = *v;
                        }
                        if cnt10 > 0 && cnt5 > 0 {
                            changes.entry(10).and_modify(|v| *v -= 1);
                            changes.entry(5).and_modify(|v| *v -= 1);
                            changes.retain(|_k, v| *v > 0);
                        } else if cnt5 >= 3 {
                            changes.entry(5).and_modify(|v| *v -= 3);
                            changes.retain(|_k, v| *v > 0);
                        } else {
                            return false;
                        }
                    }

                    _ => unreachable!(),
                }
            }

            *changes.entry(b).or_default() += 1;
        }

        true
    }
}

fn main() {}
