#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut valid_by_vegan: Vec<Vec<i32>> = restaurants
            .into_iter()
            .filter(|v| v[2] >= vegan_friendly)
            .filter(|x| x[3] <= max_price)
            .filter(|x| x[4] <= max_distance)
            .collect();

        valid_by_vegan.sort_by(|r1, r2| r2[1].cmp(&r1[1]).then(r2[0].cmp(&r1[0])));

        valid_by_vegan.into_iter().map(|r| r[0]).collect()
    }
}

fn main() {}
