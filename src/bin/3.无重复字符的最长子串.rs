/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
use std::collections::HashMap;

impl Solution { // 滑动窗口的应用
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new(); // k：字符。v：字符最后出现的索引
        let mut max = 0;
        let mut l = 0;
        for (r, c) in s.char_indices() {
            if let Some(end) = map.insert(c, r) { // end：上一次该字符最后出现的索引
                l = usize::max(l, end + 1); // 只有当end+1出现在窗口内时，滑动窗口的左边界才更新为end+1
            }
            max = usize::max(r - l + 1, max);
        }
        max as i32
    }
}
// @lc code=end
