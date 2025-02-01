#![allow(dead_code)]


struct Solution;

use std::collections::HashMap;
struct Cashier {
    prices: HashMap<i32, f64>,
    serial_number: i64,
    discount: f64,
    interval: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut id_price: HashMap<i32, f64> = HashMap::new();
        for (id, &price) in products.into_iter().zip(prices.iter()) {
            id_price.insert(id, price as f64);
        }

        Self {
            prices: id_price,
            serial_number: 1,
            discount: discount as f64,
            interval: n as i64,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut total = 0.0;
        for (id, &amt) in product.into_iter().zip(amount.iter()) {
            total += self.prices[&id] * amt as f64;
        }
        if self.serial_number % self.interval == 0 {
            total -= self.discount * total / 100.0;
        }

        self.serial_number += 1;

        total
    }
}

fn main() {}
