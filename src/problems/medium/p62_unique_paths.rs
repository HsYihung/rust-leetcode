/// 62. Unique Paths
///
/// 題目描述：
/// 一個機器人位於一個 m x n 網格的左上角（起始點在下圖中標記為 "Start" ）。
/// 機器人每次只能向下或者向右移動一步。機器人試圖到達網格的右下角（在下圖中標記為 "Finish" ）。
/// 現在考慮網格中從左上角到右下角的路徑，每一步只能向右或向下。
/// 問總共有多少條不同的路徑？
///
/// 示例 1：
/// 輸入：m = 3, n = 7
/// 輸出：28
/// 解釋：3x7 網格中，從左上角 (0,0) 到右下角 (2,6) 共有 28 條不同路徑
///
/// 示例 2：
/// 輸入：m = 3, n = 2
/// 輸出：3
/// 解釋：從左上角出發，到達右下角路徑總共有 3 條：
///      1. 向右 -> 向下 -> 向下
///      2. 向下 -> 向下 -> 向右
///      3. 向下 -> 向右 -> 向下
///
/// 示例 3：
/// 輸入：m = 7, n = 3
/// 輸出：28
/// 解釋：7x3 網格等價於 3x7 網格的轉置，路徑數量相同
///
/// 限制條件：
/// - 1 <= m, n <= 100
/// - 題目數據保證答案將小於等於 2 * 10^9
#[allow(dead_code)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }

        let mut grid = vec![vec![1; n as usize]; m as usize];

        for i in 1..m as usize {
            for j in 1..n as usize {
                grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
            }
        }

        grid[m as usize - 1][n as usize - 1]
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
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);

        // 其他基本案例
        assert_eq!(Solution::unique_paths(2, 2), 2);
        assert_eq!(Solution::unique_paths(4, 4), 20);
        assert_eq!(Solution::unique_paths(2, 3), 3);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 最小網格情況
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(1, 2), 1);
        assert_eq!(Solution::unique_paths(2, 1), 1);

        // 單行或單列網格
        assert_eq!(Solution::unique_paths(1, 10), 1);
        assert_eq!(Solution::unique_paths(10, 1), 1);
        assert_eq!(Solution::unique_paths(1, 100), 1);
        assert_eq!(Solution::unique_paths(100, 1), 1);

        // 對稱性測試 - m 和 n 交換結果應該相同
        assert_eq!(Solution::unique_paths(5, 3), Solution::unique_paths(3, 5));
        assert_eq!(Solution::unique_paths(6, 4), Solution::unique_paths(4, 6));
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 較大的網格（接近限制）
        assert_eq!(Solution::unique_paths(10, 10), 48620);
        assert_eq!(Solution::unique_paths(15, 15), 40116600);

        // 極端長寬比
        assert_eq!(Solution::unique_paths(2, 50), 50);
        assert_eq!(Solution::unique_paths(50, 2), 50);

        // 中等大小的常見案例
        assert_eq!(Solution::unique_paths(5, 5), 70);
        assert_eq!(Solution::unique_paths(6, 6), 252);
        assert_eq!(Solution::unique_paths(3, 4), 10);
        assert_eq!(Solution::unique_paths(4, 3), 10);

        // 驗證組合數學公式的正確性
        // 對於 3x3 網格，需要向右 2 步，向下 2 步，總共 4 步
        // 路徑數 = C(4,2) = 6
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
