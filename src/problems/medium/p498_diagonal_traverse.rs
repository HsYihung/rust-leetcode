/// 498. Diagonal Traverse
///
/// 題目描述：
/// 給定一個 m x n 的矩陣，以對角線遍歷的方式返回矩陣中所有元素的一維陣列。
///
/// 對角線遍歷的規則如下：
/// 1. 從左上角 (0, 0) 開始
/// 2. 沿著對角線向右上方移動
/// 3. 當到達邊界時，轉換方向到下一條對角線
/// 4. 奇數編號的對角線：從左下到右上
/// 5. 偶數編號的對角線：從右上到左下
///
/// 示例 1：
/// 輸入：mat = [[1,2,3],[4,5,6],[7,8,9]]
/// 輸出：[1,2,4,7,5,3,6,8,9]
/// 解釋：
/// 對角線 0: [1] (向右上)
/// 對角線 1: [2,4] (向左下)
/// 對角線 2: [3,5,7] (向右上)
/// 對角線 3: [6,8] (向左下)
/// 對角線 4: [9] (向右上)
///
/// 示例 2：
/// 輸入：mat = [[1,2],[3,4]]
/// 輸出：[1,2,3,4]
/// 解釋：
/// 對角線 0: [1] (向右上)
/// 對角線 1: [2,3] (向左下)
/// 對角線 2: [4] (向右上)
///
/// 限制條件：
/// - m == mat.length
/// - n == mat[i].length
/// - 1 <= m, n <= 10^4
/// - 1 <= m * n <= 10^4
/// - -10^5 <= mat[i][j] <= 10^5
#[allow(dead_code)]
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if mat.is_empty() || mat[0].is_empty() {
            return vec![];
        }

        let rows = mat.len();
        let cols = mat[0].len();
        let mut result = Vec::with_capacity(rows * cols);

        let mut row = 0;
        let mut col = 0;
        let mut going_up = true;

        for _ in 0..rows * cols {
            result.push(mat[row][col]);

            if going_up {
                if row == 0 {
                    if col == cols - 1 {
                        row += 1;
                    } else {
                        col += 1;
                    }
                    going_up = false;
                } else if col == cols - 1 {
                    row += 1;
                    going_up = false;
                } else {
                    row -= 1;
                    col += 1;
                }
            } else if col == 0 {
                if row == rows - 1 {
                    col += 1;
                } else {
                    row += 1;
                }
                going_up = true;
            } else if row == rows - 1 {
                col += 1;
                going_up = true;
            } else {
                row += 1;
                col -= 1;
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
        // 示例 1: 3x3 矩陣
        let mat1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected1 = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
        assert_eq!(Solution::find_diagonal_order(mat1), expected1);

        // 示例 2: 2x2 矩陣
        let mat2 = vec![vec![1, 2], vec![3, 4]];
        let expected2 = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_diagonal_order(mat2), expected2);
    }

    #[test]
    fn test_edge_cases() {
        // 單一元素矩陣
        let mat1 = vec![vec![1]];
        let expected1 = vec![1];
        assert_eq!(Solution::find_diagonal_order(mat1), expected1);

        // 單行矩陣
        let mat2 = vec![vec![1, 2, 3, 4]];
        let expected2 = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_diagonal_order(mat2), expected2);

        // 單列矩陣
        let mat3 = vec![vec![1], vec![2], vec![3]];
        let expected3 = vec![1, 2, 3];
        assert_eq!(Solution::find_diagonal_order(mat3), expected3);
    }

    #[test]
    fn test_corner_cases() {
        // 矩形矩陣 - 行數多於列數
        let mat1 = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected1 = vec![1, 2, 3, 5, 4, 6];
        assert_eq!(Solution::find_diagonal_order(mat1), expected1);

        // 矩形矩陣 - 列數多於行數
        let mat2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        // 正確的對角線遍歷順序：
        // (0,0)=1 -> (0,1)=2 -> (1,0)=4 -> (1,1)=5 -> (0,2)=3 -> (1,2)=6
        let expected2 = vec![1, 2, 4, 5, 3, 6];
        assert_eq!(Solution::find_diagonal_order(mat2), expected2);

        // 包含負數的矩陣
        let mat3 = vec![vec![-1, 2], vec![3, -4]];
        let expected3 = vec![-1, 2, 3, -4];
        assert_eq!(Solution::find_diagonal_order(mat3), expected3);

        // 較大的矩陣測試
        let mat4 = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let expected4 = vec![1, 2, 5, 9, 6, 3, 4, 7, 10, 11, 8, 12];
        assert_eq!(Solution::find_diagonal_order(mat4), expected4);
    }
}
