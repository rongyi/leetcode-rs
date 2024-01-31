struct NumArray {
    prefix: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let sz = nums.len();
        let mut prefix = vec![0; sz + 1];

        for (i, num) in nums.into_iter().enumerate() {
            prefix[i + 1] = prefix[i] + num as i64;
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        (self.prefix[(right + 1) as usize] - self.prefix[left as usize]) as i32
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

fn main() {
    let nums = vec![-2, 0, 3, -5, 2, -1];
    let a = NumArray::new(nums);
    println!("{}", a.sum_range(0, 2));
    println!("{}", a.sum_range(2, 5));
    println!("{}", a.sum_range(0, 5));
}
