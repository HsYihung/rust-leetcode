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
    pub fn minimum_teachings(_n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet};
        let mut problem_users: HashSet<usize> = HashSet::new();

        for friendship in friendships {
            if friendship.len() < 2 {
                continue;
            }
            let u_idx = (friendship[0] - 1) as usize;
            let v_idx = (friendship[1] - 1) as usize;
            let user_u_language = &languages[u_idx];
            let user_v_language = &languages[v_idx];
            let mut is_problem_user = true;

            for language in user_u_language {
                if user_v_language.contains(language) {
                    is_problem_user = false;
                    break;
                }
            }

            if is_problem_user {
                problem_users.insert(u_idx);
                problem_users.insert(v_idx);
            }
        }

        let mut language_count: HashMap<i32, i32> = HashMap::new();

        for user in &problem_users {
            let user_languages = &languages[*user];

            for language in user_languages {
                *language_count.entry(*language).or_insert(0) += 1;
            }
        }

        let mut max_count = 0;

        for (_k, v) in language_count {
            if v > max_count {
                max_count = v;
            }
        }

        problem_users.len() as i32 - max_count
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
        // 用戶1[1,2] 和 用戶2[1,2] 有共同語言1,2，用戶3[3] 和 用戶4[3] 有共同語言3
        // 所有朋友對都能溝通，不需要教授任何人
        assert_eq!(
            Solution::minimum_teachings(
                3,
                vec![vec![1, 2], vec![1, 2], vec![3], vec![3]],
                vec![vec![1, 2], vec![3, 4]]
            ),
            0
        );

        // 特殊情況：所有用戶掌握多種語言但有無法溝通的對
        // 用戶1[1,2] 用戶2[3,4] 用戶3[1,3] 用戶4[2,4]
        // 分析所有友誼對：
        // 1-2: [1,2] ∩ [3,4] = ∅ -> 問題用戶
        // 1-3: [1,2] ∩ [1,3] = {1} -> 能溝通
        // 1-4: [1,2] ∩ [2,4] = {2} -> 能溝通
        // 2-3: [3,4] ∩ [1,3] = {3} -> 能溝通
        // 2-4: [3,4] ∩ [2,4] = {4} -> 能溝通
        // 3-4: [1,3] ∩ [2,4] = ∅ -> 問題用戶
        // 問題用戶集合: {0,1,2,3} (所有4個用戶)
        // 語言統計: 1->2人(用戶1,3), 2->2人(用戶1,4), 3->2人(用戶2,3), 4->2人(用戶2,4)
        // 最大2人，答案: 4-2=2
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
            2
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
