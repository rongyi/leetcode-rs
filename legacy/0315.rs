struct SolutionTLE;

#[derive(Default)]
struct BSTNode {
    val: i32,
    count: i32,
    left_count: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

impl BSTNode {
    fn new(val: i32) -> Self {
        BSTNode {
            val,
            count: 1,
            ..Default::default()
        }
    }

    fn insert(&mut self, val: i32, count: &mut i32) {
        if self.val == val {
            self.count += 1;
            *count += self.left_count;
        } else if val < self.val {
            // go to left
            self.left_count += 1;
            if let Some(left) = &mut self.left {
                left.insert(val, count);
            } else {
                self.left = Some(Box::new(BSTNode::new(val)));
            }
        } else {
            *count += self.count + self.left_count;
            if let Some(right) = &mut self.right {
                right.insert(val, count);
            } else {
                self.right = Some(Box::new(BSTNode::new(val)));
            }
        }
    }
}

impl SolutionTLE {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut root: Option<Box<BSTNode>> = None;
        let mut smaller_count = vec![0; sz];

        for i in (0..sz).rev() {
            let mut count = 0;
            if let Some(root_node) = &mut root {
                root_node.insert(nums[i], &mut count);
            } else {
                root = Some(Box::new(BSTNode::new(nums[i])));
            }
            smaller_count[i] = count;
        }
        smaller_count
    }
}

struct Solution;
impl Solution {
    // translated from cpp
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut index: Vec<i32> = (0..sz as i32).collect();

        let mut ret = vec![0; sz];
        let mut nums = nums;
        Self::mergesort(&mut nums, 0, sz as i32, &mut index, &mut ret);

        ret
    }
    fn mergesort(
        nums: &mut Vec<i32>,
        start: i32,
        end: i32,
        index: &mut Vec<i32>,
        ret: &mut Vec<i32>,
    ) {
        let len = end - start;
        if len <= 1 {
            return;
        }
        let step = len / 2;
        let mid = start + step;
        Self::mergesort(nums, start, mid, index, ret);
        Self::mergesort(nums, mid, end, index, ret);

        let mut tmp_idx = Vec::with_capacity(len as usize);
        let mut idx1 = start;
        let mut idx2 = mid;
        let mut low_cnt = 0;
        while idx1 < mid || idx2 < end {
            if idx2 == end
                || (idx1 < mid
                    && nums[index[idx1 as usize] as usize] <= nums[index[idx2 as usize] as usize])
            {
                tmp_idx.push(index[idx1 as usize]);
                ret[index[idx1 as usize] as usize] += low_cnt;
                idx1 += 1;
            } else {
                tmp_idx.push(index[idx2 as usize]);
                low_cnt += 1;
                idx2 += 1;
            }
        }
        index[start as usize..start as usize + tmp_idx.len()].copy_from_slice(&tmp_idx);
    }
}

fn main() {}
