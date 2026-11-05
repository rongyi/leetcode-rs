struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let tree = vec![0; n + 1];

        let mut ret = Self {
            nums: nums.clone(),
            tree,
        };
        for i in 0..n {
            ret.update_tree(i as i32, nums[i]);
        }

        ret
    }

    fn update(&mut self, index: i32, val: i32) {
        let diff = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        self.update_tree(index, diff);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query_tree(right + 1) - self.query_tree(left)
    }

    fn update_tree(&mut self, index: i32, diff: i32) {
        let mut i = index + 1;
        let n = self.tree.len();
        while (i as usize) < n {
            self.tree[i as usize] += diff;
            i += i & -i;
        }
    }
    fn query_tree(&self, index: i32) -> i32 {
        let mut i = index;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i as usize];
            i -= i & -i;
        }

        sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */

fn main() {}
