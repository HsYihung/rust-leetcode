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
    pub fn minimum_operations_to_make_integer_zero(num1: i32, num2: i32) -> i32 {
        // 早期終止：如果 num2 >= num1，每次操作都會增加或不變，永遠無法減到 0
        if num2 >= num1 {
            return -1;
        }

        // 嘗試 1 到 60 次操作（2^60 已超過 i32 最大值，足夠覆蓋所有情況）
        for k in 1..=60 {
            // 計算經過 k 次操作後需要的目標值
            // target = num1 - k*num2 = 2^i₁ + 2^i₂ + ... + 2^iₖ
            let target = num1 - k * num2;

            // 檢查三個必要條件：
            // 1. target > 0: 必須為正數才能表示為 2 的冪次之和
            // 2. target >= k: 最小情況是 k 個 2^0 = k
            // 3. popcount(target) <= k: 二進制中 1 的個數不能超過操作次數
            if target > 0 && target >= k && Self::popcount(target as i64) <= k {
                return k;
            }
        }

        // 60 次操作內都無法完成，返回 -1
        -1
    }

    /// 計算二進制表示中 1 的個數（popcount/hamming weight）
    ///
    /// # 參數
    /// * `n` - 要計算的數字
    ///
    /// # 返回值
    /// * 二進制表示中 1 的個數
    fn popcount(mut n: i64) -> i32 {
        let mut count = 0;
        while n > 0 {
            count += n & 1; // 檢查最低位是否為 1
            n >>= 1; // 右移一位
        }
        count as i32
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
            3
        ); // k=3: 100-3*(-10)=130, popcount(130)=2 ≤ 3

        // 特殊組合：num1 和 num2 相等
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(8, 8), -1); // 無法達成

        // num1 可以一次操作完成的情況
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(15, -1), 1); // 15 = 2^4 - 1 = 16 - 1

        // 複雜的多步驟情況
        assert_eq!(Solution::minimum_operations_to_make_integer_zero(25, -3), 3);
        // k=3: 25-3*(-3)=34, popcount(34)=2 ≤ 3
    }
}
