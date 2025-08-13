struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let sz = questions.len();
        let q: Vec<Vec<i64>> = questions
            .into_iter()
            .map(|v| v.into_iter().map(|i| i as i64).collect())
            .collect();

        let mut cache: Vec<i64> = vec![-1; sz];

        Self::recur(&q, 0, &mut cache)
    }

    // TLE
    fn recur(q: &Vec<Vec<i64>>, pos: usize, cache: &mut Vec<i64>) -> i64 {
        if pos >= q.len() {
            return 0;
        }

        if cache[pos] != -1 {
            return cache[pos];
        }
        // 1. skip current
        let v1 = Self::recur(q, pos + 1, cache);
        // 2. take current
        let v2 = q[pos][0] + Self::recur(q, pos + q[pos][1] as usize + 1, cache);

        let val = v1.max(v2);
        cache[pos] = val;
        val
    }
}

fn main() {}
