/// 338. Counting Bits (計數二進位中的 1)
///
/// 題目描述：
/// 給你一個整數 n，你需要建立一個長度為 n + 1 的陣列 ans，其中 ans[i] 是 i 的二進位表示中 1 的個數。
///
/// 對於從 0 到 n（包含）的每個索引 i，ans[i] 的值應該是 i 的二進位表示中 1 位元的數量。
///
/// 這是一個位元操作和動態規劃的經典問題。可以使用多種方法求解：
/// 1. 暴力解法：對每個數字直接計算其二進位表示中 1 的個數
/// 2. 動態規劃：利用已計算過的結果來計算新的結果
/// 3. 位元操作技巧：i & (i-1) 會清除 i 的最低位的 1
///
/// 示例 1：
/// 輸入：n = 2
/// 輸出：[0,1,1]
/// 解釋：
/// 0 --> 0 (二進位: 0) --> 0 個 1
/// 1 --> 1 (二進位: 1) --> 1 個 1  
/// 2 --> 10 (二進位: 10) --> 1 個 1
///
/// 示例 2：
/// 輸入：n = 5
/// 輸出：[0,1,1,2,1,2]
/// 解釋：
/// 0 --> 0 (二進位: 0) --> 0 個 1
/// 1 --> 1 (二進位: 1) --> 1 個 1
/// 2 --> 10 (二進位: 10) --> 1 個 1
/// 3 --> 11 (二進位: 11) --> 2 個 1
/// 4 --> 100 (二進位: 100) --> 1 個 1
/// 5 --> 101 (二進位: 101) --> 2 個 1
///
/// 限制條件：
/// - 0 <= n <= 10^5
///
/// 進階要求：
/// - 很容易想到時間複雜度 O(n log n) 的解法，你可以在線性時間內完成嗎？
/// - 要求不使用任何內建函數（如 __builtin_popcount）來計算漢明重量
/// - 空間複雜度除了答案陣列之外，你的解法空間複雜度應該是 O(1)
#[allow(dead_code)]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; (n + 1) as usize];

        for i in 1..=n {
            result[i as usize] = result[(i & (i - 1)) as usize] + 1;
        }

        result
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
        assert_eq!(Solution::count_bits(0), vec![0]); // 最小值
        assert_eq!(Solution::count_bits(1), vec![0, 1]); // 只有 0 和 1
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例
        assert_eq!(Solution::count_bits(8), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]); // 2^3
        assert_eq!(
            Solution::count_bits(15),
            vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4]
        ); // 2^4 - 1
        assert_eq!(Solution::count_bits(3), vec![0, 1, 1, 2]); // 小範圍完整測試

        // 測試 2 的冪次
        assert_eq!(Solution::count_bits(4), vec![0, 1, 1, 2, 1]); // 包含 2^2
        assert_eq!(Solution::count_bits(7), vec![0, 1, 1, 2, 1, 2, 2, 3]); // 包含多個位元模式
    }
}
