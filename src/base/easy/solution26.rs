use crate::base::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut unique = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[unique - 1] {
                nums[unique] = nums[i];
                unique += 1;
            }
        }

        unique as i32
    }
}
