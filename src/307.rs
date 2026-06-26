struct NumArray {
    nums: Vec<i32>,
    bit: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let sz = nums.len();
        let mut ret = Self {
            nums: nums.clone(),
            bit: vec![0; sz + 1],
        };

        for (i, &v) in nums.iter().enumerate() {
            ret.update_at(i + 1, v);
        }

        ret
    }

    fn update(&mut self, index: i32, val: i32) {
        let diff = val - self.nums[index as usize];
        // for next round
        self.nums[index as usize] = val;

        self.update_at(index as usize + 1, diff);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query(right + 1) - self.query(left)
    }
    fn query(&self, mut idx: i32) -> i32 {
        let mut acc = 0;

        while idx > 0 {
            acc += self.bit[idx as usize];
            idx -= idx & -idx;
        }

        acc
    }
    fn update_at(&mut self, mut idx: usize, diff: i32) {
        let sz = self.bit.len();
        while idx < sz {
            self.bit[idx] += diff;
            idx += idx & (!idx + 1);
        }
    }
}

fn main() {}
