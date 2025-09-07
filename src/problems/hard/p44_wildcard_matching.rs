/// 44. Wildcard Matching
///
/// 題目描述：
/// 給定一個輸入字符串 s 和一個模式字符串 p，實現支援 '?' 和 '*' 的萬用字符匹配功能：
/// - '?' 可以匹配任何單一字符
/// - '*' 可以匹配任意字符序列（包括空序列）
///
/// 匹配應該覆蓋整個輸入字符串（而不是部分匹配）。
///
/// 這是一個經典的動態規劃問題，需要考慮多種情況的狀態轉移。
///
/// 示例 1：
/// 輸入：s = "aa", p = "a"
/// 輸出：false
/// 解釋：模式 "a" 無法匹配整個字符串 "aa"
///
/// 示例 2：
/// 輸入：s = "aa", p = "*"
/// 輸出：true
/// 解釋：'*' 可以匹配任意字符序列，因此可以匹配 "aa"
///
/// 示例 3：
/// 輸入：s = "cb", p = "?a"
/// 輸出：false
/// 解釋：'?' 匹配 'c'，但第二個字符是 'a'，無法匹配 'b'
///
/// 示例 4：
/// 輸入：s = "adceb", p = "*a*b"
/// 輸出：true
/// 解釋：第一個 '*' 匹配空序列，第二個 '*' 匹配子字符串 "dce"
///
/// 示例 5：
/// 輸入：s = "acdcb", p = "a*c?b"
/// 輸出：false
/// 解釋：雖然 'a' 和 'c' 可以匹配，'*' 可以匹配 "d"，'?' 可以匹配 'c'，但最後的 'b' 無法找到匹配
///
/// 限制條件：
/// - s 可能為空，只包含小寫字母 a-z
/// - p 可能為空，只包含小寫字母 a-z 和字符 '?' 或 '*'
/// - 0 <= s.length, p.length <= 2000
#[allow(dead_code)]
impl Solution {
    pub fn is_match(_s: String, _p: String) -> bool {
        todo!("實現 Wildcard Matching 的解決方案 - 請先理解題目和測試案例")
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
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
        assert_eq!(
            Solution::is_match("cb".to_string(), "?a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("acdcb".to_string(), "a*c?b".to_string()),
            false
        );
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), "*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), "?".to_string()), false);
        assert_eq!(Solution::is_match("a".to_string(), "?".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "a".to_string()), true);
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例
        // 多個連續的 '*'
        assert_eq!(
            Solution::is_match("abc".to_string(), "***a*b*c***".to_string()),
            true
        );

        // 複雜的混合模式
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "m*ss*ss*pi".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("abefcdgiescdfimde".to_string(), "ab*cd?i*de".to_string()),
            true
        );

        // 只有 '*' 和 '?'
        assert_eq!(
            Solution::is_match("abcd".to_string(), "*?*?".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("ab".to_string(), "*?*?*".to_string()),
            true
        );

        // 長字符串測試
        let long_s = "a".repeat(1000);
        let long_p = "*a".repeat(500) + "*";
        assert_eq!(Solution::is_match(long_s, long_p), true);

        // '*' 在開頭和結尾
        assert_eq!(
            Solution::is_match("hello".to_string(), "*lo".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("hello".to_string(), "he*".to_string()),
            true
        );
    }
}
