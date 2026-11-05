struct Solution;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let sz = graph.len();

        let mouse_turn = 0;
        let cat_turn = 1;

        let mouse_win = 1;
        let cat_win = 2;
        // let draw = 0;

        let mut dp = vec![vec![[0, 0]; sz]; sz];
        let mut degress = vec![vec![[0, 0]; sz]; sz];
        let mut stack = Vec::new();

        // cat 不走0
        for cat_pos in 1..sz {
            // (0, x, 1) - mouse is at the hole so the result is mouse_win
            // 不管谁先到，猫鼠在同一个位置那就是猫赢
            // (x, x, {0, 1}) - the mouse bumped into the cat or the
            //                  cat bumped into the mouse, either way it's a cat_win
            dp[0][cat_pos][cat_turn] = mouse_win;
            dp[cat_pos][cat_pos] = [cat_win, cat_win];
            stack.push((0, cat_pos, cat_turn));
            stack.push((cat_pos, cat_pos, mouse_turn));
            stack.push((cat_pos, cat_pos, cat_turn));

            // mouse走0
            for mouse_pos in 0..sz {
                degress[mouse_pos][cat_pos][mouse_turn] = graph[mouse_pos].len();
                degress[mouse_pos][cat_pos][cat_turn] = graph[cat_pos].len();
            }
        }

        for mouse_pos in 0..sz {
            for &cat_pos in graph[0].iter() {
                // cat can not go into 0
                // so 0's neib for cat pos should minus 1
                degress[mouse_pos][cat_pos as usize][cat_turn] -= 1;
            }
        }
        // Let u = (mouse_pos, cat_pos, turn) be a vertex in the state graph
        // and let v = (previous_mouse_pos, previous_cat_pos, previous_turn)
        // be an adjacent vertex:

        // If v already has a final result associated with it(mouse_win/cat_win)
        // we skip it.

        // If previous_turn == mouse_turn && u is a mouse_win,
        // v is also a mouse_win(mouse moves from v to u and wins)

        // If previous_turn == cat_turn && u is a cat_win
        // v is also a cat_win(cat moves from v to u and wins)

        // If previous_turn == mouse_turn && all adjacent states of v
        // are cat_win, v is also a cat_win(wherever mouse goes, cat wins)

        // If previous_turn == cat_turn && all adjacent states of v
        // are mouse_win, v is also a mouse_win(wherever cat goes, mouse wins)
        while let Some((mouse_pos, cat_pos, turn)) = stack.pop() {
            let previous_turn = 1 - turn;
            let rez = dp[mouse_pos][cat_pos][turn];

            let prev_positions: Box<dyn Iterator<Item = (usize, usize)>> = if turn == mouse_turn {
                Box::new(
                    std::iter::repeat(mouse_pos).zip(
                        graph[cat_pos]
                            .iter()
                            .filter_map(|&pos| (pos != 0).then_some(pos as usize)),
                    ),
                )
            } else {
                Box::new(
                    graph[mouse_pos]
                        .iter()
                        .map(|&pos| pos as usize)
                        .zip(std::iter::repeat(cat_pos)),
                )
            };

            for (previous_mouse_pos, previouse_cat_pos) in prev_positions {
                if dp[previous_mouse_pos][previouse_cat_pos][previous_turn] != 0 {
                    continue;
                }
                let win_state = (rez == mouse_win && previous_turn == mouse_turn)
                    || (rez == cat_win && previous_turn == cat_turn);

                // for 3/4 case
                if !win_state && degress[previous_mouse_pos][previouse_cat_pos][previous_turn] != 1
                {
                    degress[previous_mouse_pos][previouse_cat_pos][previous_turn] -= 1;
                    continue;
                }

                dp[previous_mouse_pos][previouse_cat_pos][previous_turn] = rez;
                if previous_mouse_pos == 1 && previouse_cat_pos == 2 && previous_turn == mouse_turn
                {
                    return rez;
                }

                stack.push((previous_mouse_pos, previouse_cat_pos, previous_turn));
            }
        }

        dp[1][2][mouse_turn]
    }
}

fn main() {}
