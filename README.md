# 🦀 LeetCode Solutions in Rust

这个项目记录我用 Rust 语言解决 LeetCode 问题的过程和代码。

## 📁 项目结构

```
leetcode-rust/
├── src/
│   ├── main.rs           # 主入口文件
│   └── solutions/        # 解决方案目录
│       ├── mod.rs        # 模块定义和通用数据结构
│       └── two_sum.rs    # Two Sum 问题解决方案
├── Cargo.toml            # 项目配置
└── README.md             # 项目说明
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

| 问题编号 | 问题名称 | 难度 | 解法 | 时间复杂度 | 空间复杂度 |
|---------|---------|------|------|-----------|-----------|
| 1 | [Two Sum](https://leetcode.com/problems/two-sum/) | Easy | 哈希表 | O(n) | O(n) |

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

- **已解决**: 1 题
- **简单**: 1 题
- **中等**: 0 题  
- **困难**: 0 题

---

Happy coding! 🦀
