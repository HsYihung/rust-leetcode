// LeetCode 73. Set Matrix Zeroes (Medium)
// 題目描述（中文）：
// 給定一個 m x n 的整數矩陣 matrix，若其中一個元素為 0，則將其所在的整行與整列都設為 0。
// 請你原地修改矩陣。
//
// 條件限制：
// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -2^31 <= matrix[i][j] <= 2^31 - 1
//
// 進階：
// 你能想出一個只用 O(mn) 額外空間的簡單做法嗎？
// 有沒有辦法優化到 O(m + n) 空間？
// 你可以只用常數空間（O(1)）完成這個問題嗎？

#[allow(dead_code)]
pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }
    let m = matrix.len();
    let n = matrix[0].len();
    let mut row_zero = false;
    let mut col_zero = false;

    for j in 0..n {
        if matrix[0][j] == 0 {
            row_zero = true;
            break;
        }
    }
    for row in matrix.iter().take(m) {
        if row[0] == 0 {
            col_zero = true;
            break;
        }
    }
    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }
    for i in 1..m {
        for j in 1..n {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }
    if row_zero {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }
    if col_zero {
        for row in matrix.iter_mut().take(m) {
            row[0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        // 範例 1
        // 輸入: [[1,1,1],[1,0,1],[1,1,1]]
        // 輸出: [[1,0,1],[0,0,0],[1,0,1]]
        let mut mat = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut mat);
        assert_eq!(mat, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        // 範例 2
        // 輸入: [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
        // 輸出: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
        let mut mat2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut mat2);
        assert_eq!(
            mat2,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
