/// 1304. Find N Unique Integers Sum up to Zero
///
/// 題目描述：
/// 給你一個整數 n，請你返回 任意 一個由 n 個 各不相同 的整數組成的數組，
/// 並且這 n 個數的和為 0。
///
/// 核心思路：
/// - 需要生成 n 個各不相同的整數
/// - 這些整數的總和必須等於 0
/// - 可以使用數學方法：對於每個位置 i，計算 i * 2 - n + 1
/// - 也可以使用對稱方法：正數和對應的負數配對，奇數情況加入 0
///
/// 示例 1：
/// 輸入：n = 5
/// 輸出：[-7,-1,1,3,4]
/// 解釋：這些數組也是正確的 [-5,-1,1,2,3]、[-3,-1,2,-2,4]
/// 總和：-7 + (-1) + 1 + 3 + 4 = 0
///
/// 示例 2：
/// 輸入：n = 3
/// 輸出：[-1,0,1]
/// 解釋：總和：-1 + 0 + 1 = 0
///
/// 示例 3：
/// 輸入：n = 1
/// 輸出：[0]
/// 解釋：唯一的數字就是 0，總和為 0
///
/// 限制條件：
/// - 1 <= n <= 1000
/// - 返回的數組必須包含 n 個各不相同的整數
/// - 數組中所有數字的和必須為 0
#[allow(dead_code)]
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize];

        if n == 1 {
            return result;
        }

        let mid = n / 2;

        for i in 0..n {
            result[i as usize] = i - mid;
            if i >= mid && n % 2 == 0 {
                result[i as usize] += 1
            }
        }

        result
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

        // 示例 1: n = 5，返回 5 個不同整數且和為 0
        let result = Solution::sum_zero(5);
        assert_eq!(result.len(), 5);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證所有數字都不相同
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 5);

        // 示例 2: n = 3，返回 3 個不同整數且和為 0
        let result = Solution::sum_zero(3);
        assert_eq!(result.len(), 3);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證所有數字都不相同
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 3);

        // 示例 3: n = 1，返回 [0]
        let result = Solution::sum_zero(1);
        assert_eq!(result.len(), 1);
        assert_eq!(result.iter().sum::<i32>(), 0);
        assert_eq!(result[0], 0);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 最小值：n = 1
        let result = Solution::sum_zero(1);
        assert_eq!(result.len(), 1);
        assert_eq!(result.iter().sum::<i32>(), 0);

        // 偶數情況：n = 2
        let result = Solution::sum_zero(2);
        assert_eq!(result.len(), 2);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證兩個數字不同
        assert_ne!(result[0], result[1]);

        // 偶數情況：n = 4
        let result = Solution::sum_zero(4);
        assert_eq!(result.len(), 4);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證所有數字都不相同
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 4);

        // 較大的偶數：n = 6
        let result = Solution::sum_zero(6);
        assert_eq!(result.len(), 6);
        assert_eq!(result.iter().sum::<i32>(), 0);
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 6);
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 較大的奇數：n = 7
        let result = Solution::sum_zero(7);
        assert_eq!(result.len(), 7);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證所有數字都不相同
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 7);

        // 較大的偶數：n = 8
        let result = Solution::sum_zero(8);
        assert_eq!(result.len(), 8);
        assert_eq!(result.iter().sum::<i32>(), 0);
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 8);

        // 大數值測試：n = 100
        let result = Solution::sum_zero(100);
        assert_eq!(result.len(), 100);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證所有數字都不相同
        let mut sorted_result = result.clone();
        sorted_result.sort();
        sorted_result.dedup();
        assert_eq!(sorted_result.len(), 100);

        // 接近最大值：n = 1000 （限制條件上限）
        let result = Solution::sum_zero(1000);
        assert_eq!(result.len(), 1000);
        assert_eq!(result.iter().sum::<i32>(), 0);
        // 驗證所有數字都不相同（抽樣檢查前 50 個）
        let first_50: Vec<i32> = result.iter().take(50).cloned().collect();
        let mut sorted_first_50 = first_50.clone();
        sorted_first_50.sort();
        sorted_first_50.dedup();
        assert_eq!(sorted_first_50.len(), 50);
    }
}
