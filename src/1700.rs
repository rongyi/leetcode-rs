#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut student_count = [0, 0];
        for &s in &students {
            student_count[s as usize] += 1;
        }
        // This problem is about students in a queue and sandwiches in a stack.
        // Each student has a preference (0 or 1) and each sandwich has a type (0 or 1).
        // A student will take the top sandwich if it matches their preference,
        // otherwise they will go to the end of the queue.
        // We want to count how many students are left when no one can take any more sandwiches.

        // student_count[0] represents how many students prefer type 0 sandwiches
        // student_count[1] represents how many students prefer type 1 sandwiches

        // We'll now iterate through the sandwiches and see if there's a student who wants it.
        // If yes, we decrement the count. If no, we're stuck and remaining students can't eat.
        for &sandwich in &sandwiches {
            if student_count[sandwich as usize] > 0 {
                student_count[sandwich as usize] -= 1;
            } else {
                break;
            }
        }

        student_count[0] + student_count[1]
    }
}

fn main() {}
