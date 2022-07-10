/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

use leetcode_rust::ListNode;

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
impl Solution {
    // 前后双指针(快慢双指针)
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        unsafe {
            let mut slow = &mut dummy as *mut Box<ListNode>;
            let mut fast = &mut dummy as *mut Box<ListNode>;
            // move fast n forward
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap(); // .运算符的优先级比解引用高，所以要用括号
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }

        dummy.next
    }
}
// @lc code=end
