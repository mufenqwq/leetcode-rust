// LeetCode Rust Solutions
// 主入口文件，用于测试和运行解决方案

mod solutions;

fn main() {
    println!("🦀 Welcome to LeetCode Rust Solutions!");
    println!("This project contains my LeetCode problem solutions implemented in Rust.");
    
    // 示例：运行 Two Sum 问题
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    match solutions::two_sum::Solution::two_sum(nums.clone(), target) {
        Some(result) => println!("Two Sum result for {:?} with target {}: {:?}", nums, target, result),
        None => println!("No solution found for Two Sum problem")
    }
    
    // 示例：运行 Longest Substring Without Repeating Characters 问题
    let test_strings = vec![
        "abcabcbb".to_string(),
        "bbbbb".to_string(),
        "pwwkew".to_string(),
    ];
    
    for s in test_strings {
        let result = solutions::longest_substring::Solution::length_of_longest_substring(s.clone());
        println!("Longest substring length for '{}': {}", s, result);
    }
}
