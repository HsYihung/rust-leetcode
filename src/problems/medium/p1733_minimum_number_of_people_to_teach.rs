/// 1733. Minimum Number of People to Teach
///
/// 題目描述：
/// 在一個社交網路中，有 m 個用戶，用戶需要有共同語言才能互相溝通。
///
/// 給定：
/// - n：總語言數（編號從 1 到 n）
/// - languages[i]：用戶 i 已掌握的語言列表（用戶在陣列中是 0-indexed，但在友誼關係中是 1-indexed）
/// - friendships[i] = [ui, vi]：友誼關係對的陣列
///
/// 找出需要教授一種語言給最少數量的用戶，使得所有朋友對都能溝通。
///
/// 關鍵點：
/// - 兩個用戶如果有至少一種共同語言就能溝通
/// - 只能選擇一種語言來教授給選定的用戶
/// - 目標是確保每個友誼關係對都有至少一種共同語言
/// - 友誼關係不具傳遞性（x 和 y 是朋友，y 和 z 是朋友，不表示 x 和 z 是朋友）
///
/// 示例 1：
/// 輸入：n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
/// 輸出：1
/// 解釋：用戶 1 掌握 [1]，用戶 2 掌握 [2]，用戶 3 掌握 [1,2]。
///       用戶 1 和用戶 2 沒有共同語言，需要教授語言。
///       最佳方案是教授語言 1 給用戶 2，或教授語言 2 給用戶 1。
///
/// 示例 2：
/// 輸入：n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
/// 輸出：2
/// 解釋：用戶 1 掌握 [2]，用戶 2 掌握 [1,3]，用戶 3 掌握 [1,2]，用戶 4 掌握 [3]。
///       需要找出無法溝通的朋友對，然後找出最佳語言來教授。
///
/// 限制條件：
/// - 2 <= n <= 500
/// - languages.length == m
/// - 1 <= m <= 500
/// - 1 <= languages[i].length <= n
/// - 1 <= languages[i][j] <= n
/// - 1 <= friendships.length <= 500
/// - friendships[i].length == 2
/// - 1 <= ui, vi <= languages.length
/// - ui != vi
/// - 所有 languages[i] 中的值都是唯一的
/// - 所有友誼關係對都是唯一的
#[allow(dead_code)]
impl Solution {
    pub fn minimum_teachings(
        _n: i32,
        _languages: Vec<Vec<i32>>,
        _friendships: Vec<Vec<i32>>,
    ) -> i32 {
        todo!("實現 Minimum Number of People to Teach 的解決方案 - 請先理解題目和測試案例")
    }
}

#[allow(dead_code)]
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 示例 1：n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
        assert_eq!(
            Solution::minimum_teachings(
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]]
            ),
            1
        );

        // 示例 2：n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
        assert_eq!(
            Solution::minimum_teachings(
                3,
                vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
                vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]]
            ),
            2
        );
    }

    #[test]
    fn test_edge_cases() {
        // 邊界案例：所有朋友都已經有共同語言
        assert_eq!(
            Solution::minimum_teachings(2, vec![vec![1], vec![1]], vec![vec![1, 2]]),
            0
        );

        // 邊界案例：只有一對朋友，完全沒有共同語言
        assert_eq!(
            Solution::minimum_teachings(2, vec![vec![1], vec![2]], vec![vec![1, 2]]),
            1
        );

        // 邊界案例：多種語言但用戶掌握不同語言
        assert_eq!(
            Solution::minimum_teachings(
                3,
                vec![vec![1], vec![2], vec![3]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]]
            ),
            2
        );
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況：複雜的友誼網絡，但大部分已經能溝通
        assert_eq!(
            Solution::minimum_teachings(
                3,
                vec![vec![1, 2], vec![1, 2], vec![3], vec![3]],
                vec![vec![1, 2], vec![3, 4]]
            ),
            0
        );

        // 特殊情況：所有用戶掌握多種語言但仍有無法溝通的對
        assert_eq!(
            Solution::minimum_teachings(
                4,
                vec![vec![1, 2], vec![3, 4], vec![1, 3], vec![2, 4]],
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4]
                ]
            ),
            0
        );

        // 特殊情況：需要教授給很多用戶的情況
        assert_eq!(
            Solution::minimum_teachings(
                2,
                vec![vec![1], vec![1], vec![2], vec![2]],
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]]
            ),
            2
        );
    }
}
