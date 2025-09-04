/// 63. Unique Paths II
///
/// 題目描述：
/// 一個機器人位於一個 m x n 網格的左上角（起始點在下圖中標記為 "Start" ）。
/// 機器人每次只能向下或者向右移動一步。機器人試圖到達網格的右下角（在下圖中標記為 "Finish" ）。
/// 現在考慮網格中從左上角到右下角的路徑，每一步只能向右或向下。
/// 但是現在有障礙物！網格中的障礙物和空位置分別用 1 和 0 來表示。
/// 機器人的路徑不能包含任何有障礙物的方格。
/// 問總共有多少條不同的路徑？
///
/// 示例 1：
/// 輸入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
/// 輸出：2
/// 解釋：3x3 網格的正中間有一個障礙物。
///      從左上角到右下角一共有 2 條不同的路徑：
///      1. 向右 -> 向右 -> 向下 -> 向下
///      2. 向下 -> 向下 -> 向右 -> 向右
///
/// 示例 2：
/// 輸入：obstacleGrid = [[0,1],[0,0]]
/// 輸出：1
/// 解釋：2x2 網格中，右上角有一個障礙物。
///      只有一條路徑：向下 -> 向右
///
/// 限制條件：
/// - 1 <= m, n <= 100
/// - obstacleGrid[i][j] 為 0 或 1
/// - 題目數據保證答案將小於等於 2 * 10^9
#[allow(dead_code)]
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut paths = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];

        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        for i in 0..paths.len() {
            for j in 0..paths[i].len() {
                if i == 0 && j == 0 {
                    paths[i][j] = 1;
                    continue;
                }

                if obstacle_grid[i][j] == 1 {
                    paths[i][j] = 0;
                    continue;
                }

                let top = if i == 0 { 0 } else { paths[i - 1][j] };
                let left = if j == 0 { 0 } else { paths[i][j - 1] };
                paths[i][j] = top + left;
            }
        }

        paths[paths.len() - 1][paths[0].len() - 1]
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
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );

        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );

        // 其他基本案例
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 0]]),
            2
        );

        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 0]
            ]),
            4
        );
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 單元素網格
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);

        // 起始位置有障礙物
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![1, 0], vec![0, 0]]),
            0
        );

        // 終點位置有障礙物
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]]),
            0
        );

        // 單行網格
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0]]),
            0
        );

        // 單列網格
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0], vec![0], vec![0]]),
            1
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0], vec![1], vec![0]]),
            0
        );
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 完全被障礙物阻隔的情況
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![1, 1, 1],
                vec![0, 0, 0]
            ]),
            0
        );

        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 1, 0],
                vec![0, 1, 0],
                vec![0, 1, 0]
            ]),
            0
        );

        // 複雜障礙物配置
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            4
        );

        // 邊界全是障礙物
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![1, 1, 1],
                vec![1, 0, 1],
                vec![1, 1, 1]
            ]),
            0
        );

        // 只有對角線路徑可行
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 1, 1],
                vec![1, 0, 1],
                vec![1, 1, 0]
            ]),
            0
        );

        // 較大的網格測試
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0],
                vec![0, 0, 0, 0, 0]
            ]),
            7
        );
    }
}
