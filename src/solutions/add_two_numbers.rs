// Problem 2: Add Two Numbers
// https://leetcode.com/problems/add-two-numbers/
//
// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains a single digit.
// Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// 使用 mod.rs 中定义的 ListNode
use super::ListNode;

pub struct Solution;

impl Solution {
    /// 添加两个数字（链表形式）
    /// 
    /// 时间复杂度: O(max(M, N)) - M 和 N 是两个链表的长度
    /// 空间复杂度: O(max(M, N)) - 需要创建一个新的链表存储结果
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut carry = 0;
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        
        while p1.is_some() || p2.is_some() || carry > 0 {
            let x = p1.map_or(0, |node| node.val);
            let y = p2.map_or(0, |node| node.val);
            let sum = x + y + carry;
            carry = sum / 10;
            
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
            
            p1 = p1.and_then(|node| node.next.as_ref());
            p2 = p2.and_then(|node| node.next.as_ref());
        }
        
        dummy.next
    }
}

// 辅助函数：从数组创建链表
pub fn create_list_from_array(arr: &[i32]) -> Option<Box<ListNode>> {
    if arr.is_empty() {
        return None;
    }
    
    let mut head = Box::new(ListNode::new(arr[0]));
    let mut current = &mut head;
    
    for &val in &arr[1..] {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    Some(head)
}

// 辅助函数：将链表转换为数组（用于测试）
pub fn list_to_array(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = list;
    
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers_basic() {
        // 测试用例 1: l1 = [2,4,3], l2 = [5,6,4]
        // 预期输出: [7,0,8]
        // 解释: 342 + 465 = 807
        let l1 = create_list_from_array(&[2, 4, 3]);
        let l2 = create_list_from_array(&[5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_array(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_zeros() {
        // 测试用例 2: l1 = [0], l2 = [0]
        // 预期输出: [0]
        let l1 = create_list_from_array(&[0]);
        let l2 = create_list_from_array(&[0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_array(result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        // 测试用例 3: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        // 预期输出: [8,9,9,9,0,0,0,1]
        let l1 = create_list_from_array(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list_from_array(&[9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_array(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_with_carry() {
        // 测试用例 4: l1 = [5], l2 = [5]
        // 预期输出: [0, 1]
        let l1 = create_list_from_array(&[5]);
        let l2 = create_list_from_array(&[5]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_array(result), vec![0, 1]);
    }
}
