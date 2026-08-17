use crate::base::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut cursor = 0;
        let mut zeroes_count = 0;

        while cursor < nums.len() {
            let value = nums[cursor];

            if value == 0 {
                nums.remove(cursor);
                zeroes_count += 1;
            } else {
                cursor += 1;
            }
        }

        for _ in 1..=zeroes_count {
            nums.push(0);
        }
    }
}
