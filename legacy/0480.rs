
struct Solution;

use std::collections::BTreeMap;
struct Heap {
    h: BTreeMap<i32, i32>,
    is_min: bool,
    size: i32,
}

impl Heap {
    fn new(is_min: bool) -> Self {
        Self {
            h: BTreeMap::new(),
            is_min,
            size: 0,
        }
    }
    fn add(&mut self, x: i32) {
        *self.h.entry(x).or_insert(0) += 1;
        self.size += 1;
    }
    fn remove(&mut self, x: i32) -> bool {
        if let Some(cnt) = self.h.get_mut(&x) {
            *cnt -= 1;
            if *cnt == 0 {
                self.h.remove(&x);
            }
            self.size -= 1;
            true
        } else {
            false
        }
    }
    fn pop(&mut self) -> i32 {
        if self.is_min {
            let (&key, _) = self.h.iter().next().unwrap();
            self.remove(key);
            key
        } else {
            let (&key, _) = self.h.iter().rev().next().unwrap();
            self.remove(key);
            key
        }
    }

    fn top(&self) -> i32 {
        if self.is_min {
            let (&key, _) = self.h.iter().next().unwrap();
            key
        } else {
            let (&key, _) = self.h.iter().rev().next().unwrap();
            key
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut ret: Vec<f64> = Vec::new();
        let mut min_heap = Heap::new(true);
        let mut max_heap = Heap::new(false);
        let k = k as usize;

        for (i, &num) in nums.iter().enumerate() {
            min_heap.add(num);
            // 通过 min_heap 弹出来的都是小值
            // max_heap 里尽放小矮个
            max_heap.add(min_heap.pop());
            // 走出window了，要移除一个数，那么这个数在哪呢？
            if i >= k {
                // min_heap.Top()是min里最小的，如果滑动窗口要移除的那个数字比
                // 这个最小的还大，那么这个删除的target就在 min_heap里无疑
                // 因为min_heap是高的里面挑矮的，比最矮的还高，那肯定算高个
                // 肯定在min_heap里面
                if min_heap.size > 0 && nums[i - k] >= min_heap.top() {
                    min_heap.remove(nums[i - k]);
                } else {
                    max_heap.remove(nums[i - k]);
                }
            }
            // 互相倒腾一下
            // 小矮个太多了，挑一些高的去min_heap里
            while max_heap.size > min_heap.size {
                min_heap.add(max_heap.pop());
            }
            // 其实这保证了 max_heap 和  min_heap 要么相等，要么 max_heap 大 1
            // 大高个这一组又太多了，挑一些矮的去max_heap里
            while min_heap.size > max_heap.size {
                max_heap.add(min_heap.pop());
            }
            if i >= k - 1 {
                if max_heap.size == min_heap.size {
                    ret.push(max_heap.top() as f64 / 2.0 + min_heap.top() as f64 / 2.0);
                } else {
                    ret.push(max_heap.top() as f64);
                }
            }
        }

        ret
    }
}

fn main() {
    let mut h = Heap::new(false);
    h.add(1);
    h.add(2);
    h.add(3);
    let k = h.top();
    println!("{}", k);
}
