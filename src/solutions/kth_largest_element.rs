use rand::seq::SliceRandom;

pub struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // 检查边界条件
        if nums.is_empty() || k <= 0 || k > nums.len() as i32 {
            return -1; 
        }
        
        fn quick_select(nums: Vec<i32>, k: i32) -> i32 {
            // 如果只有一个元素，直接返回
            if nums.len() == 1 {
                return nums[0];
            }
            
            let mut rng = rand::thread_rng();
            let pivot = *nums.choose(&mut rng).unwrap();
            
            let (mut big, mut equal, mut small) = (Vec::new(), Vec::new(), Vec::new());
            
            for &num in &nums {
               if num > pivot {
                   big.push(num);
               } else if num < pivot {
                   small.push(num);
               } else {
                   equal.push(num);
               } 
            }
            
            let big_count = big.len() as i32;
            let equal_count = equal.len() as i32;
            
            // 如果第k大的元素在big部分
            if k <= big_count {
                quick_select(big, k)
            } 
            // 如果第k大的元素在equal部分（即等于pivot）
            else if k <= big_count + equal_count {
                pivot
            } 
            // 如果第k大的元素在small部分
            else {
                quick_select(small, k - big_count - equal_count)
            }
        }
        
        quick_select(nums, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest_basic() {
        // 基本测试用例
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn test_find_kth_largest_single_element() {
        // 单个元素测试
        assert_eq!(Solution::find_kth_largest(vec![1], 1), 1);
    }

    #[test]
    fn test_find_kth_largest_duplicates() {
        // 包含重复元素的测试
        assert_eq!(Solution::find_kth_largest(vec![2, 2, 2, 2], 1), 2);
        assert_eq!(Solution::find_kth_largest(vec![2, 2, 2, 2], 2), 2);
        assert_eq!(Solution::find_kth_largest(vec![2, 2, 2, 2], 3), 2);
        assert_eq!(Solution::find_kth_largest(vec![2, 2, 2, 2], 4), 2);
    }

    #[test]
    fn test_find_kth_largest_sorted() {
        // 已排序数组测试
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5], 1), 5);
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5], 2), 4);
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5], 3), 3);
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5], 4), 2);
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5], 5), 1);
    }

    #[test]
    fn test_find_kth_largest_reverse_sorted() {
        // 逆序数组测试
        assert_eq!(Solution::find_kth_largest(vec![5, 4, 3, 2, 1], 1), 5);
        assert_eq!(Solution::find_kth_largest(vec![5, 4, 3, 2, 1], 2), 4);
        assert_eq!(Solution::find_kth_largest(vec![5, 4, 3, 2, 1], 3), 3);
    }

    #[test]
    fn test_find_kth_largest_negative_numbers() {
        // 包含负数测试
        assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5], 1), -1);
        assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5], 2), -2);
        assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5], 5), -5);
    }

    #[test]
    fn test_find_kth_largest_mixed_numbers() {
        // 混合正负数测试
        assert_eq!(Solution::find_kth_largest(vec![-1, 2, -3, 4, -5], 1), 4);
        assert_eq!(Solution::find_kth_largest(vec![-1, 2, -3, 4, -5], 2), 2);
        assert_eq!(Solution::find_kth_largest(vec![-1, 2, -3, 4, -5], 3), -1);
        assert_eq!(Solution::find_kth_largest(vec![-1, 2, -3, 4, -5], 4), -3);
        assert_eq!(Solution::find_kth_largest(vec![-1, 2, -3, 4, -5], 5), -5);
    }

    #[test]
    fn test_find_kth_largest_edge_cases() {
        // 边界情况测试
        // 空数组应该返回-1
        assert_eq!(Solution::find_kth_largest(vec![], 1), -1);
        
        // k超出范围应该返回-1
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3], 0), -1);
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3], 4), -1);
    }

    #[test]
    fn test_find_kth_largest_large_array() {
        // 较大数组测试
        let mut nums = vec![];
        for i in 1..=100 {
            nums.push(i);
        }
        assert_eq!(Solution::find_kth_largest(nums.clone(), 1), 100);
        assert_eq!(Solution::find_kth_largest(nums.clone(), 50), 51);
        assert_eq!(Solution::find_kth_largest(nums.clone(), 100), 1);
    }
}