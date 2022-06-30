/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let other = target - num;
            if let Some(&j) = map.get(&other) {
                return vec![i as i32, j as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}
// @lc code=end
