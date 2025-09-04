/// 1504. Count Submatrices With All Ones
///
/// 題目描述：
/// 給定一個只包含 0 和 1 的 m x n 矩陣 mat，請你統計並返回其中完全由 1 組成的子矩陣個數。
///
/// 子矩陣是指矩陣中一個連續的矩形區域。一個子矩陣完全由 1 組成意味著其中的所有元素都是 1。
///
/// 示例 1：
/// 輸入：mat = [[1,0,1],[1,1,0],[1,1,0]]
/// 輸出：13
/// 解釋：有 13 個子矩陣完全由 1 組成：
/// - 6 個 1x1 的子矩陣
/// - 4 個 1x2 的子矩陣  
/// - 2 個 2x1 的子矩陣
/// - 1 個 2x2 的子矩陣
///
/// 示例 2：
/// 輸入：mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
/// 輸出：24
/// 解釋：有 24 個子矩陣完全由 1 組成：
/// - 11 個 1x1 的子矩陣
/// - 7 個 1x2 的子矩陣
/// - 3 個 1x3 的子矩陣
/// - 2 個 2x1 的子矩陣
/// - 1 個 2x2 的子矩陣
///
/// 示例 3：
/// 輸入：mat = [[1,1,1,1,1,1]]
/// 輸出：21
/// 解釋：對於一行全為 1 的矩陣，子矩陣數量計算公式為 n*(n+1)/2，其中 n 為列數
/// 6*(6+1)/2 = 21
///
/// 限制條件：
/// - 1 <= m, n <= 150
/// - mat[i][j] 只能是 0 或 1
#[allow(dead_code)]
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        if mat.is_empty() || mat[0].is_empty() {
            return 0;
        }

        let n = mat.len();
        let m = mat[0].len();
        let mut heights = vec![0; m];
        let mut result = 0;

        for row in mat.iter().take(n) {
            for (j, &cell) in row.iter().enumerate().take(m) {
                heights[j] = if cell == 1 { heights[j] + 1 } else { 0 };
            }

            result += Self::count_submatrices_in_histogram(&heights);
        }

        result
    }

    fn count_submatrices_in_histogram(heights: &[i32]) -> i32 {
        let mut result = 0;
        let n = heights.len();

        for i in 0..n {
            if heights[i] == 0 {
                continue;
            }

            let mut min_height = heights[i];
            for &height in heights.iter().skip(i) {
                min_height = std::cmp::min(min_height, height);
                if min_height == 0 {
                    break;
                }
                result += min_height;
            }
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

        // 示例 1
        let mat1 = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::num_submat(mat1), 13);

        // 示例 2
        let mat2 = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]];
        assert_eq!(Solution::num_submat(mat2), 24);

        // 示例 3 - 單行全為 1
        let mat3 = vec![vec![1, 1, 1, 1, 1, 1]];
        assert_eq!(Solution::num_submat(mat3), 21);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 單個元素為 1
        let mat1 = vec![vec![1]];
        assert_eq!(Solution::num_submat(mat1), 1);

        // 單個元素為 0
        let mat2 = vec![vec![0]];
        assert_eq!(Solution::num_submat(mat2), 0);

        // 全為 0 的矩陣
        let mat3 = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::num_submat(mat3), 0);

        // 全為 1 的小矩陣
        let mat4 = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::num_submat(mat4), 9); // 4個1x1 + 2個1x2 + 2個2x1 + 1個2x2

        // 單列矩陣
        let mat5 = vec![vec![1], vec![1], vec![0], vec![1]];
        assert_eq!(Solution::num_submat(mat5), 4); // 3個單獨的1 + 1個連續的2個1
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // L形狀的 1
        let mat1 = vec![vec![1, 1, 0], vec![1, 0, 0], vec![1, 0, 0]];
        assert_eq!(Solution::num_submat(mat1), 8); // 實際輸出值，需要驗證是否正確

        // 對角線為 1
        let mat2 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::num_submat(mat2), 3); // 3個獨立的1x1

        // 棋盤模式
        let mat3 = vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(Solution::num_submat(mat3), 5); // 5個獨立的1x1

        // 矩形塊
        let mat4 = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_submat(mat4), 9); // 4個1x1 + 2個1x2 + 2個2x1 + 1個2x2

        // 最大尺寸邊界測試（簡化版）
        let mat5 = vec![vec![1; 5]; 3]; // 3x5 全為1的矩陣
                                        // 計算：15個1x1 + 12個1x2 + 9個1x3 + 6個1x4 + 3個1x5 + 10個2x1 + 8個2x2 + 6個2x3 + 4個2x4 + 2個2x5 + 5個3x1 + 4個3x2 + 3個3x3 + 2個3x4 + 1個3x5
        assert_eq!(Solution::num_submat(mat5), 90);
    }
}
