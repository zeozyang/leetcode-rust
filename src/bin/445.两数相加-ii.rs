/*
 * @lc app=leetcode.cn id=445 lang=rust
 *
 * [445] 两数相加 II
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use leetcode_rust::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 将两个链表入栈
        let (mut l1_p, mut l2_p, mut stack1, mut stack2) = (&l1, &l2, vec![], vec![]);
        while l1_p.is_some() || l2_p.is_some() { // 这里用l1,l2的引用，而不是l1,l2。方便以后再次查询l1,l2.
            if let Some(node1) = l1_p {
                stack1.push(node1.val);
                l1_p = &node1.next;
            }
            if let Some(node2) = l2_p {
                stack2.push(node2.val);
                l2_p = &node2.next;
            }
        }
    
        let (mut root, mut carry) = (ListNode::new(0), 0);
        while !stack1.is_empty() || !stack2.is_empty() || carry > 0 {
            let sum = stack1.pop().unwrap_or(0) + stack2.pop().unwrap_or(0) + carry;
            let mut node = ListNode::new(sum % 10);
            node.next = root.next;
            root.next = Some(Box::new(node));
            carry = sum / 10;
        }
        root.next
    }
}
// @lc code=end
