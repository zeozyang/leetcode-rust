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
        for (i, v) in nums.iter().enumerate() {
            let other = target - v; // 这里的v是个对i32类型的引用，但被编译器自动解引用了，也可以手动解引用为*v
            if let Some(&other_i) = map.get(&other) {
                return vec![i as i32, other_i as i32];
            }
            map.insert(v, i);
        }
        vec![]
    }
}
// @lc code=end
