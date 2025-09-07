/// 338. Counting Bits (計算二進制位數)
///
/// 題目描述：
/// 給定一個整數 n，返回一個長度為 n + 1 的陣列 ans，對於每個 i（0 <= i <= n），
/// ans[i] 是 i 的二進制表示中 1 的個數。
///
/// 示例 1：
/// 輸入：n = 2
/// 輸出：[0,1,1]
/// 解釋：
/// - 0 的二進制：0，包含 0 個 1
/// - 1 的二進制：1，包含 1 個 1  
/// - 2 的二進制：10，包含 1 個 1
///
/// 示例 2：
/// 輸入：n = 5
/// 輸出：[0,1,1,2,1,2]
/// 解釋：
/// - 0 的二進制：0，包含 0 個 1
/// - 1 的二進制：1，包含 1 個 1
/// - 2 的二進制：10，包含 1 個 1
/// - 3 的二進制：11，包含 2 個 1
/// - 4 的二進制：100，包含 1 個 1
/// - 5 的二進制：101，包含 2 個 1
///
/// 限制條件：
/// - 0 <= n <= 10^5
///
/// 進階挑戰：
/// - 能否在線性時間 O(n) 內解決？
/// - 能否不使用內建函數（如 C++ 的 __builtin_popcount）來解決？
#[allow(dead_code)]
impl Solution {
    pub fn count_bits(_n: i32) -> Vec<i32> {
        todo!("實現 Counting Bits 的解決方案 - 請先理解題目和測試案例")
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
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例
        assert_eq!(Solution::count_bits(0), vec![0]);
        assert_eq!(Solution::count_bits(1), vec![0, 1]);
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例 - 測試更大的數值和特殊二進制模式
        assert_eq!(Solution::count_bits(3), vec![0, 1, 1, 2]);
        assert_eq!(Solution::count_bits(4), vec![0, 1, 1, 2, 1]);
        assert_eq!(Solution::count_bits(7), vec![0, 1, 1, 2, 1, 2, 2, 3]);
        assert_eq!(Solution::count_bits(8), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]);

        // 測試 2 的冪次，這些數字在二進制中只有一個 1
        assert_eq!(
            Solution::count_bits(15),
            vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4]
        );
    }
}
