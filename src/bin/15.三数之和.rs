/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    // 这题和第一题两数之和最大的区别就是要去重，典型的去重就是排序后判断nums[i] == nums[i-1]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let tow_sum = -nums[i];
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                if nums[l] + nums[r] < tow_sum {
                    l += 1;
                } else if nums[l] + nums[r] > tow_sum {
                    r -= 1;
                } else {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] { // 找到了一个解之后，将左指针往右移到第一个与当前值不同的值上，去找其他的解
                        l += 1;
                    }
                    l += 1;
                }
            }
        }
        res
    }
}
// @lc code=end
