#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        // Step 1: Find the bounding box of each color
        let m = target_grid.len();
        let n = target_grid[0].len();

        // Map to store min/max coordinates for each color
        let mut color_bounds: HashMap<i32, (usize, usize, usize, usize)> = HashMap::new();

        for i in 0..m {
            for j in 0..n {
                let color = target_grid[i][j];

                if let Some((min_row, min_col, max_row, max_col)) = color_bounds.get_mut(&color) {
                    *min_row = (*min_row).min(i);
                    *min_col = (*min_col).min(j);
                    *max_row = (*max_row).max(i);
                    *max_col = (*max_col).max(j);
                } else {
                    color_bounds.insert(color, (i, j, i, j));
                }
            }
        }

        // Step 2: Build a directed graph (dependencies between colors)
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

        for (&color, &(min_row, min_col, max_row, max_col)) in &color_bounds {
            for i in min_row..=max_row {
                for j in min_col..=max_col {
                    let current_color = target_grid[i][j];
                    if current_color != color {
                        // Color at this position is different, so it must be printed after 'color'
                        graph
                            .entry(color)
                            .or_insert_with(HashSet::new)
                            .insert(current_color);
                    }
                }
            }
        }

        // Step 3: Check for cycles in the graph using DFS
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();

        let colors: Vec<i32> = color_bounds.keys().cloned().collect();

        for &color in &colors {
            if !visited.contains(&color) {
                if Self::has_cycle(&graph, color, &mut visited, &mut in_stack) {
                    return false;
                }
            }
        }

        true
    }

    // Helper function to detect cycles in a directed graph
    fn has_cycle(
        graph: &HashMap<i32, HashSet<i32>>,
        node: i32,
        visited: &mut HashSet<i32>,
        in_stack: &mut HashSet<i32>,
    ) -> bool {
        visited.insert(node);
        in_stack.insert(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if Self::has_cycle(graph, neighbor, visited, in_stack) {
                        return true;
                    }
                } else if in_stack.contains(&neighbor) {
                    // Found a cycle
                    return true;
                }
            }
        }

        in_stack.remove(&node);
        false
    }
}

fn main() {}
