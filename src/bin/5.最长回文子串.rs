/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
use std::iter::FromIterator;

impl Solution { // 中心扩散法
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let s: Vec<char> = s.chars().collect(); // 注意要个收集器显式的指定一个结果集，这里用的是Vec<char>
        let mut start = 0;
        let mut end = 0;
        for i in 0..n {
            let mut l = i; // 扩散窗口的左边界
            let mut r = i; // 扩散窗口的右边界
            while r + 1 < n && s[r + 1] == s[l] { // 中心也可是两个相同的字符
                r += 1;
            }
            while l > 0 && r < n - 1 && s[l - 1] == s[r + 1] {
                l -= 1;
                r += 1;
            }
            if end - start < r - l {
                start = l;
                end = r;
            }
        }
        String::from_iter(&s[start..=end])
    }
}
// @lc code=end
