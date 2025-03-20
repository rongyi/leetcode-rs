#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let max_val = *instructions.iter().max().unwrap() as usize;
        let mut bit = vec![0; max_val + 1];
        let mut cost = 0;

        // Binary Indexed Tree functions
        // A Binary Indexed Tree (BIT), also known as a Fenwick Tree, is a data structure
        // that efficiently supports dynamic prefix sum operations on an array.
        // With a BIT, we can:
        // 1. Update an element in the array in O(log n) time
        // 2. Calculate the prefix sum (sum of elements from index 1 to i) in O(log n) time
        //
        // The key insight of a BIT is storing partial sums in a way that allows efficient
        // updates and queries by using properties of binary representation of indices.
        // Each node in the tree is responsible for a range of elements determined by its
        // least significant bit.
        fn update(bit: &mut Vec<i32>, idx: usize, val: i32) {
            let mut i = idx;
            while i < bit.len() {
                bit[i] += val;
                // This line updates the BIT at position i by adding val to it.
                // In a BIT, when we update a value at position i, we need to update
                // all the positions that include i in their range.
                //
                // The formula i += i & (!i + 1) identifies the next position that needs
                // updating by adding the least significant 1 bit of i to i itself.
                //
                // For example:
                // If i = 5 (101 in binary), !i + 1 = ~101 + 1 = 010 + 1 = 011
                // So i & (!i + 1) = 101 & 011 = 001 = 1
                // Next i = 5 + 1 = 6
                //
                // This clever bit manipulation ensures we efficiently propagate
                // updates through the tree in logarithmic time.
                // More examples:
                // If i = 12 (1100 in binary), !i + 1 = ~1100 + 1 = 0011 + 1 = 0100
                // So i & (!i + 1) = 1100 & 0100 = 0100 = 4
                // Next i = 12 + 4 = 16
                //
                // If i = 7 (111 in binary), !i + 1 = ~111 + 1 = 000 + 1 = 001
                // So i & (!i + 1) = 111 & 001 = 001 = 1
                // Next i = 7 + 1 = 8
                //
                // Underlying theory:
                // In a BIT, each index is responsible for a range of elements.
                // The size of this range is determined by the least significant 1 bit.
                // When we update a value, we need to propagate this change to all
                // indices that include the current position in their range.
                // The expression (i & (!i + 1)) isolates the least significant 1 bit,
                // which helps us determine the parent node in the tree structure.
                // This creates a sparse tree where each node stores partial sums,
                // allowing for efficient updates and queries in O(log n) time.
                i += i & (!i + 1); // Add the least significant 1 bit
            }
        }

        fn query(bit: &Vec<i32>, idx: usize) -> i32 {
            let mut i = idx;
            let mut sum = 0;
            while i > 0 {
                sum += bit[i];
                i -= i & (!i + 1); // Remove the least significant 1 bit
            }
            sum
        }

        for (i, &num) in instructions.iter().enumerate() {
            let num = num as usize;

            // Count elements less than num
            let less_count = query(&bit, num - 1);

            // Count elements greater than num
            let greater_count = i as i32 - query(&bit, num);

            // Add minimum cost
            cost = (cost + less_count.min(greater_count)) % MOD;

            // Update BIT
            update(&mut bit, num, 1);
        }

        cost
    }
}

fn main() {}
