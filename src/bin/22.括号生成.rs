/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];

        dfs("".to_string(), 0, 0, n, &mut res);
        return res;
    }
}

fn dfs(stack: String, num_of_left: i32, num_of_right: i32, n: i32, res: &mut Vec<String>) {
    if num_of_left == n && num_of_right == n {
        res.push(stack);
        return;
    }

    // 剪枝
    if num_of_left < num_of_right {
        return;
    }

    // 「最小单元」处理逻辑
    if num_of_left < n {
        dfs(stack.clone() + "(", num_of_left + 1, num_of_right, n, res)
    }
    if num_of_right < n {
        dfs(stack + ")", num_of_left, num_of_right + 1, n, res)
    }
}
// @lc code=end
