struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let sz = heights.len();
        let mut ret = vec![0; sz];
        let mut stack = Vec::new();
        for i in (0..sz).rev() {
            while let Some(&top) = stack.last() {
                // so all element after(in stack means all elements after top)
                // will be shadowed
                if top >= heights[i] {
                    break;
                }
                stack.pop();
                ret[i] += 1;
            }
            // at least we can see this taller guy
            if !stack.is_empty() {
                ret[i] += 1;
            }
            stack.push(heights[i]);
        }

        ret
    }
}

fn main() {}
