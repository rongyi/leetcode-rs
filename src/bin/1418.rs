#![allow(dead_code)]
struct Solution;

use std::collections::{BTreeMap, BTreeSet};

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Step 1: Extract unique food items and table numbers
        let mut food_items = BTreeSet::new();
        let mut table_numbers = BTreeSet::new();
        let mut order_counts: BTreeMap<i32, BTreeMap<String, i32>> = BTreeMap::new();

        for order in &orders {
            let table: i32 = order[1].parse().unwrap();
            let food = &order[2];
            food_items.insert(food.clone());
            table_numbers.insert(table.clone());
            *order_counts
                .entry(table)
                .or_default()
                .entry(food.clone())
                .or_default() += 1;
        }

        // Step 2: Build the display table
        let mut display_table = Vec::new();

        // Header row
        let mut header = vec!["Table".to_string()];
        header.extend(food_items.iter().cloned());
        display_table.push(header);

        // Data rows
        for table in &table_numbers {
            let mut row = vec![table.to_string()];
            for food in &food_items {
                let count = order_counts
                    .get(table)
                    .and_then(|m| m.get(food))
                    .copied()
                    .unwrap_or(0);
                row.push(count.to_string());
            }
            display_table.push(row);
        }

        display_table
    }
}

fn main() {}
