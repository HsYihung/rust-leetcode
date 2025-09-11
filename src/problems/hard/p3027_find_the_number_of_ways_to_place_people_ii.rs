/// 3027. Find the Number of Ways to Place People II
///
/// 題目描述：
/// 給定一個大小為 n × 2 的二維陣列 points，表示二維平面上的整數座標點，
/// 其中 points[i] = [xi, yi]。
///
/// 題目定義的方向如下：
/// - 右：正 x 軸方向（x 座標遞增）
/// - 左：負 x 軸方向（x 座標遞減）
/// - 上：正 y 軸方向（y 座標遞增）
/// - 下：負 y 軸方向（y 座標遞減）
///
/// 你需要在這些點上放置 n 個人（包括 Alice 和 Bob），使得每個點上恰好有一個人。
///
/// Alice 想要與 Bob 獨處，所以她將建造一個矩形圍欄：
/// - Alice 的位置作為左上角
/// - Bob 的位置作為右下角
///
/// 為了讓 Alice 在左上角，Bob 在右下角：
/// - Alice 的 x 座標必須 ≤ Bob 的 x 座標
/// - Alice 的 y 座標必須 ≥ Bob 的 y 座標
///
/// 圍欄可能不包含任何面積（可以是一條線）。如果除了 Alice 和 Bob 之外的任何人
/// 位於圍欄內部或圍欄邊界上，Alice 就會感到難過。
///
/// 你的任務是返回可以放置 Alice 和 Bob 的點對數量，使得 Alice 在建造圍欄時
/// 不會難過。換句話說，計算有效的 (Alice, Bob) 位置對的數量，其中沒有其他人
/// 落在這兩個位置形成的矩形圍欄內或圍欄上。
///
/// 注意：Alice 只能以她的位置作為左上角，Bob 的位置作為右下角來建造圍欄。
/// 位置不能顛倒。
///
/// 示例 1：
/// 輸入：points = [[1,1],[2,2],[3,3]]
/// 輸出：0
/// 解釋：無論如何選擇 Alice 和 Bob 的位置，總會有第三個人在圍欄內或邊界上，
///      所以沒有有效的配對。
///
/// 示例 2：
/// 輸入：points = [[6,2],[4,4],[2,6]]
/// 輸出：2
/// 解釋：有兩個有效的 (Alice, Bob) 配對：
///      - Alice 在 (2,6)，Bob 在 (4,4)：圍欄不包含其他點
///      - Alice 在 (4,4)，Bob 在 (6,2)：圍欄不包含其他點
///
/// 示例 3：
/// 輸入：points = [[3,1],[1,3],[1,1]]
/// 輸出：2
/// 解釋：有兩個有效的 (Alice, Bob) 配對：
///      - Alice 在 (1,3)，Bob 在 (1,1)：圍欄是一條垂直線，不包含其他點
///      - Alice 在 (1,3)，Bob 在 (3,1)：圍欄不包含其他點
///
/// 限制條件：
/// - 2 ≤ points.length ≤ 50
/// - points[i].length == 2
/// - 0 ≤ xi, yi ≤ 50
/// - 所有的點都是唯一的
#[allow(dead_code)]
impl Solution {
    pub fn number_of_points_in_the_rectangle(_points: Vec<Vec<i32>>) -> i32 {
        todo!("實現 Find the Number of Ways to Place People II 的解決方案 - 請先理解題目和測試案例")
    }
}

#[allow(dead_code)]
pub(crate) struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 基本測試案例 - 來自題目示例

        // 示例 1：points = [[1,1],[2,2],[3,3]]，期望輸出：0
        // 無論如何選擇都會有第三個人在圍欄內
        let points1 = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::number_of_points_in_the_rectangle(points1), 0);

        // 示例 2：points = [[6,2],[4,4],[2,6]]，期望輸出：2
        // 有效配對：(2,6)-(4,4) 和 (4,4)-(6,2)
        let points2 = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
        assert_eq!(Solution::number_of_points_in_the_rectangle(points2), 2);

        // 示例 3：points = [[3,1],[1,3],[1,1]]，期望輸出：2
        // 有效配對：(1,3)-(1,1) 和 (1,3)-(3,1)
        let points3 = vec![vec![3, 1], vec![1, 3], vec![1, 1]];
        assert_eq!(Solution::number_of_points_in_the_rectangle(points3), 2);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 最小情況：只有兩個點
        let points_min = vec![vec![0, 0], vec![1, 1]];
        assert_eq!(Solution::number_of_points_in_the_rectangle(points_min), 0);

        // 兩個點形成有效配對
        let points_valid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::number_of_points_in_the_rectangle(points_valid), 1);

        // 所有點在同一條水平線上
        // 修正：水平線上有有效配對 (1,5)-(2,5) 和 (1,5)-(3,5) 和 (2,5)-(3,5)
        let points_horizontal = vec![vec![1, 5], vec![2, 5], vec![3, 5]];
        assert_eq!(
            Solution::number_of_points_in_the_rectangle(points_horizontal),
            2 // 修正：應該是 2，因為有 (1,5)-(2,5) 和 (2,5)-(3,5) 兩個有效配對
        );

        // 所有點在同一條垂直線上
        let points_vertical = vec![vec![5, 1], vec![5, 2], vec![5, 3]];
        assert_eq!(
            Solution::number_of_points_in_the_rectangle(points_vertical),
            2
        );
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 四個點形成正方形
        // 修正：實際上有更多有效配對，需要重新計算
        let points_square = vec![vec![0, 0], vec![0, 2], vec![2, 0], vec![2, 2]];
        assert_eq!(
            Solution::number_of_points_in_the_rectangle(points_square),
            4 // 修正：(0,2)-(0,0), (0,2)-(2,0), (0,2)-(2,2), (2,2)-(2,0)
        );

        // L 形排列的點
        let points_l_shape = vec![vec![0, 2], vec![0, 1], vec![0, 0], vec![1, 0], vec![2, 0]];
        assert_eq!(
            Solution::number_of_points_in_the_rectangle(points_l_shape),
            4
        );

        // 邊界值測試：座標為最大值
        let points_max_coords = vec![vec![50, 50], vec![49, 49], vec![48, 48]];
        assert_eq!(
            Solution::number_of_points_in_the_rectangle(points_max_coords),
            0
        );

        // 多個點但有多個有效配對
        let points_one_valid = vec![
            vec![1, 4],
            vec![2, 3],
            vec![3, 2],
            vec![4, 1],
            vec![0, 5],
            vec![5, 0],
        ];
        assert_eq!(
            Solution::number_of_points_in_the_rectangle(points_one_valid),
            5 // 修正：實際上有 5 個有效配對
        );
    }
}
