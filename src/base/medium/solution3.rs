use crate::base::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = [-1i32; 128];
        let mut left = 0;
        let mut max_len = 0;

        for (right, char) in s.bytes().enumerate() {
            let idx = char as usize;
            if last_seen[idx] > -1 && last_seen[idx] >= left {
                left = last_seen[idx] + 1;
            }

            last_seen[idx] = right as i32;
            max_len = max_len.max(right as i32 - left + 1);
        }

        max_len
    }
}
