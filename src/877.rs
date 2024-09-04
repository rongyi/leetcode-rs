struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        // Alex is first to pick pile.
        // piles.length is even, and this lead to an interesting fact:
        // Alex can always pick odd piles or always pick even piles!

        // For example,
        // If Alex wants to pick even indexed piles piles[0], piles[2], ....., piles[n-2],
        // he picks first piles[0], then Lee can pick either piles[1] or piles[n - 1].
        // Every turn, Alex can always pick even indexed piles and Lee can only pick odd indexed piles.

        // In the description, we know that sum(piles) is odd.
        // If sum(piles[even]) > sum(piles[odd]), Alex just picks all evens and wins.
        // If sum(piles[even]) < sum(piles[odd]), Alex just picks all odds and wins.

        // So, Alex always defeats Lee in this game.
        true
    }
}

fn main() {}
