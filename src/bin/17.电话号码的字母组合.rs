/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digits: Vec<char> = digits.chars().collect();
        let map: HashMap<char, Vec<char>> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .map(|(d, v)| (*d, v.chars().collect()))
        .collect();
        let mut stack: Vec<char> = vec![];
        let mut res: Vec<String> = vec![];
        Self::dfs(&map, &digits, &mut stack, 0, &mut res);
        res
    }

    fn dfs(
        map: &HashMap<char, Vec<char>>,
        digits: &Vec<char>,
        stack: &mut Vec<char>,
        begin: usize,
        res: &mut Vec<String>,
    ) {
        if begin == digits.len() {
            res.push(stack.iter().collect());
        } else {
            let d = digits[begin];
            for &c in map[&d].iter() {
                stack.push(c);
                Self::dfs(map, digits, stack, begin + 1, res);
                stack.pop();
            }
        }
    }
}
// @lc code=end
