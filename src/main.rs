mod lc46;
mod lc45;
mod lc44;
mod lc43;
mod lc42;
mod lc41;
mod lc40;
mod lc39;
mod lc38;
mod lc37;
mod lc36;
mod lc35;
mod lc34;
mod lc33;
mod lc32;
mod lc31;
mod lc30;
mod lc29;
mod lc28;
mod lc27;
mod lc26;
mod lc25;
mod lc24;
mod lc23;
mod lc22;
mod lc21;
mod lc20;
mod lc19;
mod lc18;
mod lc17;
mod lc16;
mod lc15;
mod lc14;
mod lc13;
mod lc12;
mod lc11;
mod lc10;
mod lc09;
mod lc08;
mod lc07;
mod lc06;
mod lc05;
mod lc04 {

    struct Solution;

    impl Solution {
        // 1 2 3 | 4 5
        // 1 2 3 | 4 5 6 7 8
        pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
            let sz1 = nums1.len() as i32;
            let sz2 = nums2.len() as i32;
            if sz1 > sz2 {
                return Self::find_median_sorted_arrays(nums2, nums1);
            }
            let sz = sz1 + sz2;
            let half = (sz + 1) / 2;
            let mut low = 0;
            let mut high = sz1;
            while low <= high {
                let mid1 = (low + high) / 2;
                let mid2 = half - mid1;
                let r1 = if mid1 < sz1 {
                    nums1[mid1 as usize]
                } else {
                    i32::MAX
                };
                let r2 = if mid2 < sz2 {
                    nums2[mid2 as usize]
                } else {
                    i32::MAX
                };
                let l1 = if mid1 - 1 >= 0 {
                    nums1[(mid1 - 1) as usize]
                } else {
                    i32::MIN
                };
                let l2 = if mid2 - 1 >= 0 {
                    nums2[(mid2 - 1) as usize]
                } else {
                    i32::MIN
                };
                if l1 <= r2 && l2 <= r1 {
                    if sz % 2 == 1 {
                        return l1.max(l2) as f64;
                    }
                    return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
                } else if l1 > r2 {
                    high = mid1 - 1;
                } else {
                    low = mid1 + 1;
                }
            }
            unreachable!()
        }
    }
}

fn main() {
    // Test cases
    //println!("{}", lc10::is_match("aaa".to_string(), ".*".to_string())); // Output: false
    for i in (0..6 - 1).step_by(2) {
        println!("{i}");
    }
}
