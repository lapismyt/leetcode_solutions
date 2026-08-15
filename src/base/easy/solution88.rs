use crate::base::Solution;

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let mut cursor1 = m - 1;
        let mut cursor1_2 = m + n - 1;
        let mut cursor2 = n - 1;

        while cursor2 > -1 {
            let value2 = nums2[cursor2 as usize];

            if cursor1 < 0 || value2 >= nums1[cursor1 as usize] {
                nums1[cursor1_2 as usize] = value2;
                cursor2 -= 1;
            } else {
                nums1[cursor1_2 as usize] = nums1[cursor1 as usize];
                cursor1 -= 1;
            }

            cursor1_2 -= 1;
        }
    }
}
