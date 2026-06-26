struct Solution;

struct NumArray {
    prefix: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len() + 1];
        for (i, &v) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + v as i64;
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let v = self.prefix[right as usize + 1] - self.prefix[left as usize];
        v as _
    }
}

fn main() {}
