struct Solution;

mod ai {
    struct Solution;
    impl Solution {
        pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
            let mut row = row;
            let n = row.len();
            let mut pos = vec![0; n];

            // Store the position of each person
            for (i, &person) in row.iter().enumerate() {
                pos[person as usize] = i;
            }

            let mut swaps = 0;

            for i in (0..n).step_by(2) {
                // Current person and their partner
                let current = row[i];
                let partner = if current % 2 == 0 {
                    current + 1
                } else {
                    current - 1
                };

                // If partner is not in the adjacent seat
                if row[i + 1] != partner {
                    // Find where the partner is
                    let partner_pos = pos[partner as usize];

                    // Swap the partner with the person in the adjacent seat
                    row.swap(i + 1, partner_pos);

                    // Update positions
                    pos[row[partner_pos] as usize] = partner_pos;
                    pos[row[i + 1] as usize] = i + 1;

                    swaps += 1;
                }
            }

            swaps
        }
    }
}
impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let mut ret = 0;
        let sz = row.len() / 2;

        for i in 0..sz {
            let one = row[2 * i];
            let another = Self::my_couple(one);
            // already sit together
            if another == row[2 * i + 1] {
                continue;
            }
            ret += 1;

            for j in 2 * (i + 1)..2 * sz {
                if row[j] == another {
                    row[j] = row[2 * i + 1];
                    break;
                }
            }
        }

        ret
    }

    fn my_couple(i: i32) -> i32 {
        if i & 1 == 1 {
            i - 1
        } else {
            i + 1
        }
    }
}

fn main() {}
