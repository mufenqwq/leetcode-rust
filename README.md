# 🦀 LeetCode Solutions in Rust

这个项目记录我用 Rust 语言解决 LeetCode 问题的过程和代码。每个解决方案都包含详细的注释、多种解法比较和完整的测试用例。

## 📁 项目结构

```
leetcode-rust/
├── src/
│   ├── main.rs                    # 主入口文件
│   └── solutions/                 # 解决方案目录
│       ├── mod.rs                 # 模块定义和通用数据结构
│       ├── two_sum.rs             # Two Sum 问题解决方案
│       ├── add_two_numbers.rs     # Add Two Numbers 问题解决方案
│       ├── longest_substring.rs   # Longest Substring Without Repeating Characters 问题解决方案
│       └── kth_largest_element.rs # Kth Largest Element in an Array 问题解决方案
├── Cargo.toml                     # 项目配置
└── README.md                      # 项目说明
```

## 🚀 使用方法

### 运行项目
```bash
cargo run
```

### 运行测试
```bash
cargo test
```

### 运行特定测试
```bash
cargo test two_sum
```

## 📝 已解决的问题

| 问题编号 | 问题名称 | 难度 | 解法 | 时间复杂度 | 空间复杂度 | 状态 |
|---------|---------|------|------|-----------|-----------|------|
| 1 | [Two Sum](https://leetcode.com/problems/two-sum/) | Easy | 哈希表 | O(n) | O(n) | ✅ 已完成 |
| 2 | [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/) | Medium | 链表遍历 | O(max(M,N)) | O(max(M,N)) | ✅ 已完成 |
| 3 | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | Medium | 滑动窗口 | O(n) | O(min(m,n)) | ✅ 已完成 |
| 215 | [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/) | Medium | 快速选择 | O(n) 平均 | O(1) | ✅ 已完成 |

## 🛠️ 添加新问题

1. 在 `src/solutions/` 目录下创建新的 `.rs` 文件
2. 在 `src/solutions/mod.rs` 中添加模块声明
3. 按照以下模板编写代码：

```rust
// Problem X: Problem Name
// https://leetcode.com/problems/problem-name/
// 
// Problem description...

pub struct Solution;

impl Solution {
    pub fn solution_function(/* parameters */) -> /* return type */ {
        // 实现代码
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // 测试用例
    }
}
```

## 📊 常用数据结构

项目在 `src/solutions/mod.rs` 中定义了 LeetCode 常用的数据结构：

- `ListNode`: 链表节点
- `TreeNode`: 二叉树节点

## 🎯 学习目标

- [ ] 掌握 Rust 的所有权系统
- [ ] 熟练使用 Rust 标准库
- [ ] 提高算法和数据结构能力
- [ ] 学习 Rust 的性能优化技巧

## 📈 进度统计

- **已解决**: 4 题
- **简单**: 1 题
- **中等**: 3 题  
- **困难**: 0 题

## 🛠️ 技术栈

- **语言**: Rust
- **测试框架**: Rust 内置测试
- **序列化**: serde + serde_json
- **性能测试**: criterion (开发依赖)

## 🎯 项目特色

- ✅ **详细注释**: 每个解决方案都有详细的中文注释
- ✅ **多种解法**: 提供暴力解法和优化解法的对比
- ✅ **完整测试**: 包含多个测试用例确保代码正确性
- ✅ **性能分析**: 使用 criterion 进行性能基准测试
- ✅ **代码规范**: 遵循 Rust 编码规范和最佳实践

## 📚 参考资料

- [Rust 官方文档](https://doc.rust-lang.org/)
- [LeetCode 官网](https://leetcode.com/)
- [Rust 算法与数据结构](https://github.com/TheAlgorithms/Rust)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

---

Happy coding! 🦀

*用 Rust 解决 LeetCode 问题，提升编程技能*
