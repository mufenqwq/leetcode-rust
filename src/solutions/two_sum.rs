// Problem 1: Two Sum
// https://leetcode.com/problems/two-sum/
// 
// Given an array of integers nums and an integer target, return indices of the two numbers 
// such that they add up to target.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// 暴力解法 - O(n²) 时间复杂度
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return Some(vec![i as i32, j as i32]);
                }
            }
        }
        None
    }

    /// 哈希表解法 - O(n) 时间复杂度
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
        let mut map = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = map.get(&complement) {
                return Some(vec![j as i32, i as i32]);
            }
            map.insert(num, i);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15], 9),
            Some(vec![0, 1])
        );
        
        assert_eq!(
            Solution::two_sum(vec![3, 2, 4], 6),
            Some(vec![1, 2])
        );
        
        assert_eq!(
            Solution::two_sum(vec![3, 3], 6),
            Some(vec![0, 1])
        );
    }

    #[test]
    fn test_two_sum_brute_force() {
        assert_eq!(
            Solution::two_sum_brute_force(vec![2, 7, 11, 15], 9),
            Some(vec![0, 1])
        );
        
        assert_eq!(
            Solution::two_sum_brute_force(vec![3, 2, 4], 6),
            Some(vec![1, 2])
        );
    }
}
