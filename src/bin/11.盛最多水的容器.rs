/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution { // 相撞双指针
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max: i32 = 0;
        while l < r {
            max = i32::max(max, i32::min(height[l], height[r]) * (r - l) as i32);
            if height[l] <= height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max
    }
}
// @lc code=end
