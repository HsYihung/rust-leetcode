/// 1317. Convert Integer to the Sum of Two No-Zero Integers
///
/// 題目描述：
/// 給定一個正整數 n，需要將其分解為兩個正整數 a 和 b，使得這兩個數都是「無零整數」且它們的和等於 n。
///
/// 無零整數的定義：
/// 正整數，且其十進制表示中不包含數字 0。例如，123 是無零整數，但 105 不是，因為它包含數字 0。
///
/// 要求：
/// - 找到兩個正整數 a 和 b
/// - a 和 b 都必須是無零整數（不包含數字 0）
/// - a + b = n
/// - 返回這兩個數組成的陣列 [a, b]
/// - 如果存在多個有效解，返回任意一個即可
///
/// 示例 1：
/// 輸入：n = 2
/// 輸出：[1, 1]
/// 解釋：1 + 1 = 2，且 1 和 1 都是無零整數
///
/// 示例 2：
/// 輸入：n = 11
/// 輸出：[2, 9]
/// 解釋：2 + 9 = 11，且 2 和 9 都是無零整數。也可以返回 [3, 8]、[4, 7] 等
///
/// 示例 3：
/// 輸入：n = 10000
/// 輸出：[1, 9999]
/// 解釋：1 + 9999 = 10000，且 1 和 9999 都是無零整數
///
/// 限制條件：
/// - 2 <= n <= 10^4
/// - 題目保證至少存在一個有效解
#[allow(dead_code)]
impl Solution {
    pub fn get_no_zero_integers(_n: i32) -> Vec<i32> {
        todo!("實現 Convert Integer to the Sum of Two No-Zero Integers 的解決方案 - 請先理解題目和測試案例")
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
        let result1 = Solution::get_no_zero_integers(2);
        assert_eq!(result1[0] + result1[1], 2);
        assert!(!result1[0].to_string().contains('0'));
        assert!(!result1[1].to_string().contains('0'));

        let result2 = Solution::get_no_zero_integers(11);
        assert_eq!(result2[0] + result2[1], 11);
        assert!(!result2[0].to_string().contains('0'));
        assert!(!result2[1].to_string().contains('0'));

        let result3 = Solution::get_no_zero_integers(10000);
        assert_eq!(result3[0] + result3[1], 10000);
        assert!(!result3[0].to_string().contains('0'));
        assert!(!result3[1].to_string().contains('0'));
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 最小值 n = 2
        let result = Solution::get_no_zero_integers(2);
        assert_eq!(result[0] + result[1], 2);
        assert!(!result[0].to_string().contains('0'));
        assert!(!result[1].to_string().contains('0'));

        // 較大的值
        let result = Solution::get_no_zero_integers(9999);
        assert_eq!(result[0] + result[1], 9999);
        assert!(!result[0].to_string().contains('0'));
        assert!(!result[1].to_string().contains('0'));

        // 包含很多 0 的數字
        let result = Solution::get_no_zero_integers(1000);
        assert_eq!(result[0] + result[1], 1000);
        assert!(!result[0].to_string().contains('0'));
        assert!(!result[1].to_string().contains('0'));
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 包含 0 的數字測試
        for n in [10, 20, 30, 100, 200, 300, 1001, 2020, 5050] {
            let result = Solution::get_no_zero_integers(n);
            assert_eq!(result[0] + result[1], n, "Sum should equal {}", n);
            assert!(
                !result[0].to_string().contains('0'),
                "First number {} should not contain 0",
                result[0]
            );
            assert!(
                !result[1].to_string().contains('0'),
                "Second number {} should not contain 0",
                result[1]
            );
            assert!(
                result[0] > 0 && result[1] > 0,
                "Both numbers should be positive"
            );
        }

        // 隨機測試一些中間值
        for n in [123, 456, 789, 1234, 5678, 8765] {
            let result = Solution::get_no_zero_integers(n);
            assert_eq!(result[0] + result[1], n);
            assert!(!result[0].to_string().contains('0'));
            assert!(!result[1].to_string().contains('0'));
            assert!(result[0] > 0 && result[1] > 0);
        }
    }
}
