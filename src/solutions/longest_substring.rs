// Problem 3: Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// 
// Given a string s, find the length of the longest substring without repeating characters.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// 滑动窗口解法 - O(n) 时间复杂度
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut left: usize = 0;
        let mut max_len: usize = 0;

        for (end, ch) in s.chars().enumerate() {
            if let Some(&prev_pos) = map.get(&ch) {
                left = left.max(prev_pos + 1);
            }
            map.insert(ch, end);
            max_len = max_len.max(end - left + 1);
        }
        
        max_len as _
    }

    /// 滑动窗口解法 - O(n) 时间复杂度
    /// 使用数组来记录字符出现的位置
    pub fn length_of_longest_substring_array(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut left = 0;
        let mut windows = [false; 128];
        for (end, &ch) in s.iter().enumerate() {
            let c = ch as usize;
            while windows[c] {
                windows[s[left] as usize] = false;
                left += 1;
            }
            windows[c] = true;
            ans = ans.max(end - left + 1)
        }
        
        ans as _
    }

    /// 暴力解法 - O(n²) 时间复杂度
    pub fn length_of_longest_substring_brute_force(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut max_length = 0;
        
        for i in 0..chars.len() {
            let mut seen = std::collections::HashSet::new();
            for j in i..chars.len() {
                if seen.contains(&chars[j]) {
                    break;
                }
                seen.insert(chars[j]);
                max_length = max_length.max(j - i + 1);
            }
        }
        
        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        // 测试用例 1: "abcabcbb" -> 3 ("abc")
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        
        // 测试用例 2: "bbbbb" -> 1 ("b")
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        
        // 测试用例 3: "pwwkew" -> 3 ("wke")
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        
        // 测试用例 4: "" -> 0
        assert_eq!(
            Solution::length_of_longest_substring("".to_string()),
            0
        );
        
        // 测试用例 5: "a" -> 1
        assert_eq!(
            Solution::length_of_longest_substring("a".to_string()),
            1
        );
    }

    #[test]
    fn test_length_of_longest_substring_brute_force() {
        assert_eq!(
            Solution::length_of_longest_substring_brute_force("abcabcbb".to_string()),
            3
        );
        
        assert_eq!(
            Solution::length_of_longest_substring_brute_force("bbbbb".to_string()),
            1
        );
        
        assert_eq!(
            Solution::length_of_longest_substring_brute_force("pwwkew".to_string()),
            3
        );
    }
} 