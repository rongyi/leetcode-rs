use rand::Rng;

struct Solution {
    rects: Vec<Vec<i32>>,
    areas: Vec<i32>,
    total_area: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut total = 0;
        let mut areas = Vec::new();
        for i in 0..rects.len() {
            total += (rects[i][2] - rects[i][0] + 1) * (rects[i][3] - rects[i][1] + 1);
            areas.push(total);
        }
        Self {
            rects,
            areas,
            total_area: total,
        }
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let random_area = rng.gen_range(0..self.total_area) + 1;
        let mut i = 0;
        while i < self.areas.len() {
            if random_area <= self.areas[i] {
                break;
            }

            i += 1;
        }
        let x = rng.gen_range(0..(self.rects[i][2] - self.rects[i][0] + 1));
        let y = rng.gen_range(0..(self.rects[i][3] - self.rects[i][1] + 1));

        vec![self.rects[i][0] + x, self.rects[i][1] + y]
    }
}

fn main() {}
