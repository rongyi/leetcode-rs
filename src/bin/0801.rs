#![allow(dead_code)]

struct Solution;

impl Solution {
    // 递推公式有点相对运动的感觉
    // 1. 用两个数组swap, noswap，swap[i] 表示数组有序，且当前节点i需要互换，noswap表示不需要置换i节点
    // swap[0] == 1 because it need swap
    // noswap[0] == 0

    // 如果当前节点和前一个节点在两个数组中都有序了，所以按道理不需要置换当前节点，所以 noswap[i] = noswap[i - 1]
    // 但如果非要强制转换，那之前的节点也得跟着换，所以swap[i] = swap[i - 1] + 1
    // 如果是置换当前节点后有序，则swap[i] = noswap[i - 1] + 1, 如果非要不换此节点（某种程度的相对运动）,
    // 那就换前一个节点，noswap[i] = swap[i - 1]
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sz = nums1.len();
        let mut swap = vec![sz as i32; sz];
        let mut no_swap = vec![sz as i32; sz];
        swap[0] = 1;
        no_swap[0] = 0;

        for i in 1..sz {
            // already ordered
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                swap[i] = swap[i - 1] + 1;
                no_swap[i] = no_swap[i - 1];
            }
            // can swap
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                // swap current, only
                swap[i] = swap[i].min(no_swap[i - 1] + 1);
                // swap prev, only
                no_swap[i] = no_swap[i].min(swap[i - 1]);
            }
        }

        swap[sz - 1].min(no_swap[sz - 1])
    }
}

fn main() {}
