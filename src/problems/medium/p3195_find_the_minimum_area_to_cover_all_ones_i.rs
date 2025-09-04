/// 3195. Find the Minimum Area to Cover All Ones I
///
/// 題目描述：
/// 給定一個 2D 二進制數組 grid，找到一個具有水平和垂直邊的矩形，該矩形具有最小面積，
/// 使得 grid 中的所有 1 都位於此矩形內。返回矩形的最小可能面積。
///
/// 注意：輸入保證 grid 中至少存在一個 1。
///
/// 示例 1：
/// 輸入：grid = [[0,1,0],[1,0,1]]
/// 輸出：6
/// 解釋：包含所有 1 的最小矩形的左上角是 (0, 0)，右下角是 (1, 2)。
/// 矩形的高度為 2，寬度為 3，所以面積為 2 * 3 = 6。
///
/// 示例 2：
/// 輸入：grid = [[1,0],[0,0]]
/// 輸出：1
/// 解釋：包含所有 1 的最小矩形的左上角和右下角都是 (0, 0)。
/// 矩形的高度和寬度都為 1，所以面積為 1 * 1 = 1。
///
/// 示例 3：
/// 輸入：grid = [[0,0,0],[0,1,0],[0,0,0]]
/// 輸出：1
/// 解釋：包含所有 1 的最小矩形就是 (1, 1) 這一個格子，面積為 1。
///
/// 限制條件：
/// - 1 <= grid.length, grid[i].length <= 1000
/// - grid[i][j] 只能是 0 或 1
/// - 輸入保證 grid 中至少有一個 1
#[allow(dead_code)]
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut min_row = m;
        let mut max_row = 0;
        let mut min_col = n;
        let mut max_col = 0;

        // 優化版本：使用 enumerate 和 iterator
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    min_row = min_row.min(i);
                    max_row = max_row.max(i);
                    min_col = min_col.min(j);
                    max_col = max_col.max(j);
                }
            }
        }

        let height = max_row - min_row + 1;
        let width = max_col - min_col + 1;

        (height * width) as i32
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

        // 示例 1: [[0,1,0],[1,0,1]]
        let grid1 = vec![vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(Solution::minimum_area(grid1), 6);

        // 示例 2: [[1,0],[0,0]]
        let grid2 = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(Solution::minimum_area(grid2), 1);

        // 示例 3: [[0,0,0],[0,1,0],[0,0,0]]
        let grid3 = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::minimum_area(grid3), 1);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 單個 1 在左上角
        let grid1 = vec![vec![1]];
        assert_eq!(Solution::minimum_area(grid1), 1);

        // 全為 1 的矩形
        let grid2 = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::minimum_area(grid2), 4);

        // 只有一行有 1
        let grid3 = vec![vec![1, 0, 1, 0, 1]];
        assert_eq!(Solution::minimum_area(grid3), 5);

        // 只有一列有 1
        let grid4 = vec![vec![1], vec![0], vec![1], vec![0], vec![1]];
        assert_eq!(Solution::minimum_area(grid4), 5);
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 四角都有 1
        let grid1 = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(Solution::minimum_area(grid1), 9);

        // L 形狀的 1
        let grid2 = vec![vec![1, 1, 0], vec![1, 0, 0], vec![1, 0, 0]];
        assert_eq!(Solution::minimum_area(grid2), 6);

        // 對角線上的 1
        let grid3 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::minimum_area(grid3), 9);

        // 最大矩形 (邊界條件)
        let grid4 = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1],
        ];
        assert_eq!(Solution::minimum_area(grid4), 25);
    }
}
