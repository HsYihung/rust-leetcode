/// 679. 24 Game
///
/// 給定一個長度為4的正整數陣列 nums，你需要用 +、-、*、/ 四種運算符
/// 和括號將這四個數字連接起來，使得最終結果等於24。
///
/// 注意：
/// - 除法運算是真除法，不是整數除法
/// - 每個數字必須用且只能用一次
/// - 可以使用括號改變運算順序
/// - 如果可以得到24，返回 true；否則返回 false
///
/// 示例 1：
/// 輸入：nums = [4, 1, 8, 7]
/// 輸出：true
/// 解釋：(8-4) * (7-1) = 4 * 6 = 24
///
/// 示例 2：
/// 輸入：nums = [1, 2, 1, 2]
/// 輸出：false
///
/// 限制條件：
/// - nums.length == 4
/// - 1 <= nums[i] <= 9
#[allow(dead_code)]
const EPS: f64 = 1e-6;
#[allow(dead_code)]
const TARGET: f64 = 24.0;

#[allow(dead_code)]
impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        let mut cards: Vec<f64> = nums.iter().map(|&x| x as f64).collect();
        Self::backtrack(&mut cards)
    }

    fn backtrack(nums: &mut [f64]) -> bool {
        if nums.len() == 1 {
            return (nums[0] - TARGET).abs() < EPS;
        }

        let n = nums.len();
        // 嘗試每一對數字
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let a = nums[i];
                let b = nums[j];

                // 移除這兩個數字，創建新的數組
                let mut new_nums = Vec::new();
                for (k, &value) in nums.iter().enumerate() {
                    if k != i && k != j {
                        new_nums.push(value);
                    }
                }

                // 嘗試四種運算
                let operations = [a + b, a - b, a * b];
                for op in operations {
                    new_nums.push(op);
                    if Self::backtrack(&mut new_nums) {
                        return true;
                    }
                    new_nums.pop();
                }

                // 除法需要檢查分母不為0
                if b.abs() > EPS {
                    new_nums.push(a / b);
                    if Self::backtrack(&mut new_nums) {
                        return true;
                    }
                    new_nums.pop();
                }
            }
        }

        false
    }
}

#[allow(dead_code)]
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_point24() {
        // 示例 1: [4, 1, 8, 7] -> true
        // (8-4) * (7-1) = 4 * 6 = 24
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);

        // 示例 2: [1, 2, 1, 2] -> false
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);

        // 額外測試用例
        // [1, 1, 8, 8] -> true
        // (8 / 8 + 1) * 1 * 24 = 24 (錯誤，重新計算)
        // (1 + 1 + 8) * 8 / 4 = 24 (無4)
        // 8 / (1 - 1/8) = 8 / (7/8) = 64/7 ≠ 24
        // 實際: (8-1+1)*8/8 = 8, 不對
        // 正確: 8/(1-1/8) 無法用給定數字實現
        // 讓我們用一個確定的例子: (1+1)*8+8 = 24
        assert_eq!(Solution::judge_point24(vec![1, 1, 8, 8]), true);

        // [3, 3, 8, 8] -> true
        // 8/(3-8/3) = 8/(9/3-8/3) = 8/(1/3) = 24
        assert_eq!(Solution::judge_point24(vec![3, 3, 8, 8]), true);

        // [1, 1, 1, 1] -> false
        // 無論如何運算都無法得到24
        assert_eq!(Solution::judge_point24(vec![1, 1, 1, 1]), false);

        // [2, 2, 2, 2] -> false
        // 2+2+2+2=8, 2*2*2*2=16, 都不等於24
        assert_eq!(Solution::judge_point24(vec![2, 2, 2, 2]), false);

        // [6, 6, 6, 6] -> false
        // 6+6+6+6=24 但需要三個加號，我們只能用兩個運算符處理四個數
        // 實際上: 6+6+6+6確實=24，但在我們的運算規則下需要驗證
        // (6+6)+(6+6) = 12+12 = 24，這是可行的
        assert_eq!(Solution::judge_point24(vec![6, 6, 6, 6]), true);
    }

    #[test]
    fn test_edge_cases() {
        // 包含1的情況 - 實際上 [1,2,3,4] 可以得到24: (1+2+3)*4 = 6*4 = 24
        assert_eq!(Solution::judge_point24(vec![1, 2, 3, 4]), true);

        // 大數字情況
        assert_eq!(Solution::judge_point24(vec![9, 9, 9, 9]), false);

        // 混合數字
        assert_eq!(Solution::judge_point24(vec![1, 3, 4, 6]), true); // 6/(1-3/4) = 6/(1/4) = 24

        // 確實無解的情況
        assert_eq!(Solution::judge_point24(vec![1, 1, 2, 2]), false);
    }
}
