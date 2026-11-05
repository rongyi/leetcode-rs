struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();

        Self::dfs(&price, &special, needs, &mut memo)
    }

    fn dfs(
        price: &Vec<i32>,
        special: &Vec<Vec<i32>>,
        needs: Vec<i32>,
        memo: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if let Some(&ret) = memo.get(&needs) {
            return ret;
        }
        let sz = price.len();
        let mut ret = 0;
        for i in 0..sz {
            ret += needs[i] * price[i];
        }
        for cur_offer in special.iter() {
            let mut can_use = true;
            let mut next_needs = needs.clone();

            for i in 0..sz {
                // must have this need
                if cur_offer[i] > next_needs[i] {
                    can_use = false;
                    break;
                }
                next_needs[i] -= cur_offer[i];
            }
            if can_use {
                let mut bundle_buy = cur_offer[sz];
                bundle_buy += Self::dfs(price, special, next_needs, memo);
                ret = ret.min(bundle_buy);
            }
        }
        memo.insert(needs.clone(), ret);

        ret
    }
}

fn main() {}
