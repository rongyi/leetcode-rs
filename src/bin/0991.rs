struct Solution;

impl Solution {
    // Let us consider several cases and understand, what will be the best choise in each of them. Let us go in opposite direction and divide Y by 3 or add 1, to it until we get X.

    // If X > Y, we do not have a lot of choice, we can just decrease X by one until it becomes equal to Y, so answer will be X - Y.
    // If X == Y, then we already happy and we return 1.
    // If Y % 3 == 1, then let us think, what can be the last step? It can not be multilication by 2, so the only choice is subtraction of 1 and on previous step we have configuration (X, Y + 1), for which we run our function recursively.
    // If Y % 3 == 0, let us prove that we always need to divide by 2 in this case.
    // Imagine, that we have sequence of operations, like: [+2, +1, /2, /2, /2, +1, /2, ...]. Then, if we have +1, +1, /2 sequence inside, we can always replace it with /2, +1, so we can make it shorter. So, there will be no +1, +1 subsequence, except for the very end and in general all sequence looks like: [+1, /2, .., /2, +1, /2, ..., /2, ..., +1, ..., +1]. Note, that the last part correspondes to case 1. Also we will never have two +1 in the middle and it means, that if we have even number, we must divide it by 2: if we add 1 to it, then we have no choice to add 1 again (case 3) and then we have +1, +1, /2 pattern.
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        if start_value >= target {
            return start_value - target;
        }
        if target % 2 == 1 {
            return 1 + Self::broken_calc(start_value, target + 1);
        }
        return 1 + Self::broken_calc(start_value, target / 2);
    }
}

fn main() {}
