struct Solution;

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut max3 = i32::MIN;
        let mut max4 = i32::MIN;
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        let mut min3 = i32::MAX;
        let mut min4 = i32::MAX;

        // Calculate all possible combinations of +/- for the expression
        for i in 0..n {
            let idx = i as i32;

            // arr1[i] + arr2[i] + i
            let sum1 = arr1[i] + arr2[i] + idx;
            max1 = max1.max(sum1);
            min1 = min1.min(sum1);

            // arr1[i] + arr2[i] - i
            let sum2 = arr1[i] + arr2[i] - idx;
            max2 = max2.max(sum2);
            min2 = min2.min(sum2);

            // arr1[i] - arr2[i] + i
            let sum3 = arr1[i] - arr2[i] + idx;
            max3 = max3.max(sum3);
            min3 = min3.min(sum3);

            // arr1[i] - arr2[i] - i
            let sum4 = arr1[i] - arr2[i] - idx;
            max4 = max4.max(sum4);
            min4 = min4.min(sum4);
        }

        // Find maximum difference among all combinations
        let diff1 = max1 - min1;
        let diff2 = max2 - min2;
        let diff3 = max3 - min3;
        let diff4 = max4 - min4;

        diff1.max(diff2).max(diff3).max(diff4)
    }
}

fn main() {}
