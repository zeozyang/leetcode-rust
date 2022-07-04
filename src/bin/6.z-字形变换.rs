/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let mut v = vec!["".to_string(); num_rows as usize]; // 这里用[类型; 长度]的方法初始化数组时，长度要用usize表示
        let mut row: i32 = 0;
        let mut flag: i32 = -1; // flag用来表示方向，只有-1和1两个值
        for a_char in s.chars() {
            v[row as usize].push(a_char); // 用索引取值时，索引要用uszie表示。我在这里有个疑问，初始化字符串时，并没有给他标注mut，为什么它是可变的？
            if row == 0 || row == num_rows - 1 {
                flag = -flag;
            }
            row += flag;
        }

        let mut res = String::from("");
        for s in v {
            res += &s;
        }
        res
    }
}
// @lc code=end
