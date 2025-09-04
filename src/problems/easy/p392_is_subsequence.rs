/// 392. Is Subsequence
///
/// 題目描述：
/// 給定兩個字符串 s 和 t，判斷 s 是否為 t 的子序列。
///
/// 字符串的子序列是指從原字符串中刪除某些（可以不刪除）字符，
/// 而不改變其餘字符的相對位置所形成的新字符串。
/// 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是。
///
/// 示例 1：
/// 輸入：s = "abc", t = "ahbgdc"
/// 輸出：true
/// 解釋：s 中的字符可以在 t 中按順序找到：
///      a -> a (位置 0)
///      b -> b (位置 2)
///      c -> c (位置 5)
///      因此 "abc" 是 "ahbgdc" 的子序列
///
/// 示例 2：
/// 輸入：s = "axc", t = "ahbgdc"
/// 輸出：false
/// 解釋：雖然 t 中包含 a, c 字符，但 x 不存在於 t 中，
///      因此 "axc" 不是 "ahbgdc" 的子序列
///
/// 限制條件：
/// - 0 <= s.length <= 100
/// - 0 <= t.length <= 10^4
/// - s 和 t 都只包含小寫英文字母
///
/// 進階：如果有大量輸入的 S，稱作 S1, S2, ... , Sk 其中 k >= 10億，
/// 你需要依次檢查它們是否為 T 的子序列。在這種情況下，你會怎樣改變代碼？
#[allow(dead_code)]
impl Solution {
    pub fn is_subsequence(_s: String, _t: String) -> bool {
        todo!("實現 Is Subsequence 的解決方案 - 請先理解題目和測試案例")
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
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );

        // 其他基本案例
        assert_eq!(
            Solution::is_subsequence("ace".to_string(), "abcde".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("aec".to_string(), "abcde".to_string()),
            false
        );
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 空字符串情況
        assert_eq!(
            Solution::is_subsequence("".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("".to_string(), "".to_string()),
            true
        );

        // 單字符情況
        assert_eq!(
            Solution::is_subsequence("a".to_string(), "a".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("b".to_string(), "abc".to_string()),
            true
        );

        // s 和 t 相等的情況
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "abc".to_string()),
            true
        );
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // s 比 t 長的情況
        assert_eq!(
            Solution::is_subsequence("abcdef".to_string(), "abc".to_string()),
            false
        );

        // 重複字符的情況
        assert_eq!(
            Solution::is_subsequence("aaa".to_string(), "aaaa".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("aaaa".to_string(), "aaa".to_string()),
            false
        );

        // 字符在 t 中不按順序出現
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "cba".to_string()),
            false
        );

        // 最後一個字符匹配
        assert_eq!(
            Solution::is_subsequence("z".to_string(), "abcdefghijklmnopqrstuvwxyz".to_string()),
            true
        );

        // 第一個字符匹配
        assert_eq!(
            Solution::is_subsequence("a".to_string(), "abcdefghijklmnopqrstuvwxyz".to_string()),
            true
        );

        // 所有字符都在 t 中但順序錯誤
        assert_eq!(
            Solution::is_subsequence("dcba".to_string(), "abcd".to_string()),
            false
        );
    }
}
