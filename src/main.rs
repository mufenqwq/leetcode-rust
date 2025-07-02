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
}
