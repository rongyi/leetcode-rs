struct Solution;

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n1 = nums1.len() as i32;
        let n2 = nums2.len() as i32;
        let mut ret: Vec<i32> = Vec::new();

        for k1 in (k - n2).max(0)..=k.min(n1) {
            println!("round--");
            let a = Self::selectk(&nums1, k1);
            let b = Self::selectk(&nums2, k - k1);
            println!("{:?}", a);
            println!("{:?}", b);
            ret = Self::max(ret, Self::merge(a, b));
        }

        ret
    }

    fn selectk(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut drop = nums.len() as i32 - k;

        for &num in nums {
            while !ret.is_empty() && drop > 0 && *ret.last().unwrap() < num {
                ret.pop();
                drop -= 1;
            }
            ret.push(num);
        }

        ret.truncate(k as usize);
        ret
    }

    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::with_capacity(nums1.len() + nums2.len());
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        while nums1.len() + nums2.len() > 0 {
            let cur = if Self::cmp(&nums1, &nums2) {
                &mut nums1
            } else {
                &mut nums2
            };
            ret.push(cur[0]);
            cur.remove(0);
        }
        ret
    }

    fn cmp(nums1: &Vec<i32>, nums2: &Vec<i32>) -> bool {
        for i in 0..nums1.len().min(nums2.len()) {
            if nums1[i] > nums2[i] {
                return true;
            } else if nums1[i] < nums2[i] {
                return false;
            }
        }

        if nums1.len() > nums2.len() {
            true
        } else {
            false
        }
    }

    fn max(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        for i in 0..nums1.len().min(nums2.len()) {
            if nums1[i] > nums2[i] {
                return nums1;
            } else if nums1[i] < nums2[i] {
                return nums2;
            }
        }
        if nums1.len() > nums2.len() {
            nums1
        } else {
            nums2
        }
    }
}

fn main() {
    let nums1 = vec![6, 7];
    let nums2 = vec![6, 0, 4];
    // let k = 5;
    // Solution::max_number(nums1, nums2, k);
    let c = Solution::merge(nums1, nums2);
    println!("{:?}", c);
}
