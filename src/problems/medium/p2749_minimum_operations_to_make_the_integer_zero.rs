/// 2749. Minimum Operations to Make the Integer Zero
///
/// 題目描述：
/// 給你兩個整數 num1 和 num2。在一次操作中，你可以選擇範圍 [0, 60] 內的任意整數 i，
/// 然後從 num1 中減去 2^i + num2。
///
/// 返回使 num1 等於 0 所需的最少操作次數。如果無法使 num1 等於 0，則返回 -1。
///
/// 核心思路：
/// - 每次操作減去 (2^i + num2)，其中 i ∈ [0, 60]
/// - 目標是找到最少操作次數使 num1 變為 0
/// - 如果無法實現則返回 -1
///
/// 示例 1：
/// 輸入：num1 = 3, num2 = -2
/// 輸出：3
/// 解釋：可以通過以下操作使 3 變為 0：
/// - 選擇 i = 2，從 3 中減去 2^2 + (-2) = 4 - 2 = 2，得到 3 - 2 = 1
/// - 選擇 i = 2，從 1 中減去 2^2 + (-2) = 4 - 2 = 2，得到 1 - 2 = -1  
/// - 選擇 i = 0，從 -1 中減去 2^0 + (-2) = 1 - 2 = -1，得到 -1 - (-1) = 0
///   可以證明 3 是執行操作的最小次數。
///
/// 示例 2：
/// 輸入：num1 = 5, num2 = 7
/// 輸出：-1
/// 解釋：可以證明無法通過給定操作使 5 變為 0。
///
/// 限制條件：
/// - 1 <= num1 <= 10^9
/// - -10^9 <= num2 <= 10^9
#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations_to_make_integer_zero(_num1: i32, _num2: i32) -> i32 {
        todo!(
            "實現 Minimum Operations to Make the Integer Zero 的解決方案 - 請先理解題目和測試案例"
        )
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

        // 示例 1: num1 = 3, num2 = -2, 期望輸出 = 3
        // 解釋：3 -> 1 -> -1 -> 0，共需要 3 次操作
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(3, -2), 3);

        // 示例 2: num1 = 5, num2 = 7, 期望輸出 = -1
        // 解釋：無法通過操作使 5 變為 0
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(5, 7), -1);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // num2 為 0 的情況：只能減去 2^i
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(1, 0), 1); // 減去 2^0 = 1
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(2, 0), 1); // 減去 2^1 = 2
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(4, 0), 1); // 減去 2^2 = 4

        // num1 為 1 的最小值情況
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(1, -1), 1); // 減去 2^0 + (-1) = 0

        // num2 為負數的大值情況
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(10, -5), 2);
        // 可通過兩次操作達成
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 當 num2 非常大且為正數時，可能無法減少到 0
        assert_eq!(
            Solution::minimum_operations_to_make_integer_zero(1, 1000000000),
            -1
        );

        // 當 num1 較大但 num2 為負數時的情況
        assert_eq!(
            Solution::minimum_operations_to_make_integer_zero(100, -10),
            7
        ); // 需要多次操作

        // 特殊組合：num1 和 num2 相等
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(8, 8), -1); // 無法達成

        // num1 可以一次操作完成的情況
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(15, -1), 1); // 15 = 2^4 - 1 = 16 - 1

        // 複雜的多步驟情況
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(25, -3), 4);
        // 需要多次精確計算
    }
}
