use std::collections::HashMap;

use crate::base::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut value_to_indice_map: HashMap<i32, usize> = HashMap::new();

        nums.iter()
            .enumerate()
            .find_map(|(first_i, first_num)| {
                let second_i = value_to_indice_map.get(&(target - first_num));

                let result = second_i.and_then(|second_i| {
                    if &first_i == second_i {
                        None
                    } else {
                        Some(vec![first_i as i32, *second_i as i32])
                    }
                });

                value_to_indice_map.insert(*first_num, first_i);

                result
            })
            .unwrap()
    }
}
