/// 16. 3Sum Closest
///
/// 題目描述：
/// 給定一個長度為 n 的整數陣列 `nums` 和一個目標值 `target`，你需要在 `nums` 中找出三個整數，
/// 使得它們的和與 `target` 最接近。
///
/// 返回這三個數的和。題目保證每組輸入只存在唯一答案，即只有一個三元組能產生最接近目標值的和。
///
/// 解題關鍵：
/// - 必須選擇恰好三個整數
/// - 目標是最小化 |sum - target| 的絕對差值
/// - 返回實際的三數之和，而非與目標的差值
///
/// 示例 1：
/// 輸入：nums = [-1,2,1,-4], target = 1
/// 輸出：2
/// 解釋：與 target 最接近的和是 2 ((-1) + 2 + 1 = 2)
///
/// 示例 2：
/// 輸入：nums = [0,0,0], target = 1
/// 輸出：0
/// 解釋：與 target 最接近的和是 0 ((0) + 0 + 0 = 0)
///
/// 示例 3：
/// 輸入：nums = [4,0,5,-5,3,3,0,-4,-5], target = -2
/// 輸出：-2
/// 解釋：有多個三元組的和為 -2，例如 (-5) + 0 + 3 = -2
///
/// 限制條件：
/// - 3 <= nums.length <= 500
/// - -1000 <= nums[i] <= 1000
/// - -10^4 <= target <= 10^4
#[allow(dead_code)]
impl Solution {
    pub fn three_sum_closest(_nums: Vec<i32>, _target: i32) -> i32 {
        todo!("實現 3Sum Closest 的解決方案 - 請先理解題目和測試案例")
    }
}

#[allow(dead_code)]
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 基本測試案例 - 來自題目示例
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例
        // 最小陣列長度 (3個元素)
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1], 0), 3);
        assert_eq!(Solution::three_sum_closest(vec![-1, 0, 1], 0), 0);

        // 極值情況
        assert_eq!(
            Solution::three_sum_closest(vec![-1000, -1000, -1000], -10000),
            -3000
        );
        assert_eq!(
            Solution::three_sum_closest(vec![1000, 1000, 1000], 10000),
            3000
        );
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 目標值與三數之和完全相等
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 6), 6);
        assert_eq!(Solution::three_sum_closest(vec![-3, -1, 1, 2], 1), 1);

        // 包含重複元素的陣列
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], 100), 3);
        assert_eq!(Solution::three_sum_closest(vec![0, 2, 1, -3], 1), 0);

        // 負數較多的情況
        assert_eq!(
            Solution::three_sum_closest(vec![-5, -4, -3, -2, -1], -6),
            -6
        );

        // 大陣列測試
        let large_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(Solution::three_sum_closest(large_nums, 15), 15); // 4+5+6 = 15

        // 混合正負數
        assert_eq!(
            Solution::three_sum_closest(vec![-1, -5, -10, -20, -19], -24),
            -24
        );
        assert_eq!(
            Solution::three_sum_closest(vec![13, 2, -3, -5, -13], -1),
            -3
        );
    }
}
