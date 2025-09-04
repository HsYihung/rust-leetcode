/// 509. Fibonacci Number
///
/// 題目描述：
/// 斐波那契數列的定義如下：
/// - F(0) = 0, F(1) = 1
/// - F(n) = F(n - 1) + F(n - 2)，對於 n > 1
///
/// 給定整數 n，計算 F(n)。斐波那契數列是一個經典的數學序列，
/// 其中每個數字都是前兩個數字的和。這個問題可以用多種方法解決，
/// 包括遞歸、動態規劃和迭代方法。
///
/// 示例 1：
/// 輸入：n = 2
/// 輸出：1
/// 解釋：F(2) = F(1) + F(0) = 1 + 0 = 1
///
/// 示例 2：
/// 輸入：n = 3
/// 輸出：2
/// 解釋：F(3) = F(2) + F(1) = 1 + 1 = 2
///
/// 示例 3：
/// 輸入：n = 4
/// 輸出：3
/// 解釋：F(4) = F(3) + F(2) = 2 + 1 = 3
///
/// 示例 4：
/// 輸入：n = 0
/// 輸出：0
/// 解釋：根據定義，F(0) = 0
///
/// 示例 5：
/// 輸入：n = 1
/// 輸出：1
/// 解釋：根據定義，F(1) = 1
///
/// 限制條件：
/// - 0 <= n <= 30
#[allow(dead_code)]
impl Solution {
    pub fn fib(_n: i32) -> i32 {
        todo!("實現 Fibonacci Number 的解決方案 - 請先理解題目和測試案例")
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
        assert_eq!(Solution::fib(2), 1); // F(2) = F(1) + F(0) = 1 + 0 = 1
        assert_eq!(Solution::fib(3), 2); // F(3) = F(2) + F(1) = 1 + 1 = 2
        assert_eq!(Solution::fib(4), 3); // F(4) = F(3) + F(2) = 2 + 1 = 3
        assert_eq!(Solution::fib(5), 5); // F(5) = F(4) + F(3) = 3 + 2 = 5
        assert_eq!(Solution::fib(6), 8); // F(6) = F(5) + F(4) = 5 + 3 = 8
    }

    #[test]
    fn test_boundary_cases() {
        // 邊界測試案例 - 基本情況和最小值
        assert_eq!(Solution::fib(0), 0); // 基本情況：F(0) = 0
        assert_eq!(Solution::fib(1), 1); // 基本情況：F(1) = 1
        assert_eq!(Solution::fib(30), 832040); // 最大約束條件：n = 30
    }

    #[test]
    fn test_medium_values() {
        // 中等值測試案例
        assert_eq!(Solution::fib(7), 13); // F(7) = 13
        assert_eq!(Solution::fib(8), 21); // F(8) = 21
        assert_eq!(Solution::fib(10), 55); // F(10) = 55
        assert_eq!(Solution::fib(15), 610); // F(15) = 610
        assert_eq!(Solution::fib(20), 6765); // F(20) = 6765
        assert_eq!(Solution::fib(25), 75025); // F(25) = 75025
    }
}
