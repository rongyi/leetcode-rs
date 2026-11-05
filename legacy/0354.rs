struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        // (1, 1) (1, 2) (1, 3) after sort
        // (1, 3) (1, 2) (1, 1)
        envelopes.sort_unstable_by_key(|e| (e[0], -e[1]));
        let mut lis = Vec::new();

        for e in envelopes {
            let height = e[1];

            let idx = match lis.binary_search(&height) {
                Ok(pos) | Err(pos) => pos,
            };
            if idx == lis.len() {
                lis.push(height);
            } else {
                lis[idx] = height;
            }
        }

        lis.len() as i32
    }
}

fn main() {
    let a = vec![3];
    let idx = a.binary_search(&4);
    println!("{:?}", idx);
}
