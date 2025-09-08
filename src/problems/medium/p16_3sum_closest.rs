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
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // 初始化最接近的和為前三個元素的和
        let mut closest = nums[0] + nums[1] + nums[2];

        if nums.len() == 3 {
            return closest;
        }

        // 排序陣列，為雙指針法做準備
        nums.sort();

        // 外層循環：固定第一個元素 (x1, y1)
        // take(nums.len() - 2) 確保至少留下兩個元素給雙指針
        for (x1, &y1) in nums.iter().enumerate().take(nums.len() - 2) {
            let mut x2 = x1 + 1; // 左指針，從第一個元素的下一位開始
            let mut x3 = nums.len() - 1; // 右指針，從陣列末尾開始

            // 優化：使用二分搜索找到更好的起始位置
            // 如果當前最小可能的和（y1 + nums[x2] + nums[x3]）小於目標
            if y1 + nums[x2] + nums[x3] < target {
                // 二分搜索找到最大的 x2，使得 y1 + nums[x2] + nums[x3] <= target
                let mut lo = x2;
                let mut hi = x3;
                while hi - lo > 1 {
                    let mid = (lo + hi) / 2;
                    if y1 + nums[mid] + nums[x3] <= target {
                        lo = mid; // mid 可能是答案，保留
                    } else {
                        hi = mid; // mid 太大，捨棄
                    }
                }
                x2 = lo; // 更新左指針到最佳位置
            } else {
                // 如果當前最小可能的和大於等於目標
                // 二分搜索找到最小的 x3，使得 y1 + nums[x2] + nums[x3] >= target
                let mut lo = x2;
                let mut hi = x3;
                while hi - lo > 1 {
                    let mid = (lo + hi) / 2;
                    if y1 + nums[x2] + nums[mid] >= target {
                        hi = mid; // mid 可能是答案，保留
                    } else {
                        lo = mid; // mid 太小，捨棄
                    }
                }
                x3 = hi; // 更新右指針到最佳位置
            }

            // 標準雙指針法：在優化後的範圍內搜索
            while x2 < x3 {
                let sum = y1 + nums[x2] + nums[x3];

                // 檢查是否找到更接近的和
                // !(x1 == x2 || x1 == x3) 確保不使用重複的索引
                if !(x1 == x2 || x1 == x3) && (sum - target).abs() < (target - closest).abs() {
                    closest = sum;

                    // 如果找到完全匹配，直接返回
                    if closest == target {
                        return closest;
                    }
                }

                // 根據當前和與目標的關係移動指針
                if sum < target {
                    x2 += 1; // 和太小，移動左指針增大和
                } else {
                    x3 -= 1; // 和太大，移動右指針減小和
                }
            }
        }

        closest
    }
}

#[allow(dead_code)]
pub(crate) struct Solution;

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
        assert_eq!(Solution::three_sum_closest(vec![-3, -1, 1, 2], 1), 0);

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
            -25
        );
        assert_eq!(
            Solution::three_sum_closest(vec![13, 2, -3, -5, -13], -1),
            -3
        );
    }
}
