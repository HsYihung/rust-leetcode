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
    pub fn fib(n: i32) -> i32 {
        // 基本情況：F(0) = 0, F(1) = 1
        if n <= 1 {
            return n;
        }

        // 定義基本矩陣 [[1, 1], [1, 0]]
        // 這個矩陣有特性：M^n = [[F(n+1), F(n)], [F(n), F(n-1)]]
        let base_matrix = [[1, 1], [1, 0]];

        // 使用快速冪計算 M^n
        let result_matrix = Self::matrix_power(base_matrix, n as u32);

        // F(n) 位於結果矩陣的 [0][1] 位置
        result_matrix[0][1]
    }

    /// 矩陣快速冪算法
    /// 時間複雜度：O(log n)
    /// 空間複雜度：O(1)
    fn matrix_power(mut base: [[i32; 2]; 2], mut exp: u32) -> [[i32; 2]; 2] {
        // 初始化結果為單位矩陣 [[1, 0], [0, 1]]
        let mut result = [[1, 0], [0, 1]];

        // 快速冪核心邏輯：利用二進制分解
        while exp > 0 {
            // 如果當前位為1，將當前的 base^(2^k) 累乘到結果中
            if exp & 1 == 1 {
                result = Self::matrix_multiply(result, base);
            }
            // base 自乘，準備下一個 2^k 次冪
            base = Self::matrix_multiply(base, base);
            // 右移一位，處理下一個二進制位
            exp >>= 1;
        }

        result
    }

    /// 2x2 矩陣乘法
    /// 計算 A × B，其中 A 和 B 都是 2x2 矩陣
    fn matrix_multiply(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
        [
            // 第一行：[a00*b00 + a01*b10, a00*b01 + a01*b11]
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            // 第二行：[a10*b00 + a11*b10, a10*b01 + a11*b11]
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ]
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
