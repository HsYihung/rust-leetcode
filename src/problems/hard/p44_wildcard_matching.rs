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
    pub fn is_match(s: String, p: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let s_len = s_chars.len();
        let p_len = p_chars.len();

        // Step 1: 創建 DP 表格
        // dp[i][j] 表示 s[0..i] 與 p[0..j] 是否匹配
        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];

        // Step 2: 初始化邊界條件
        // 空字符串與空模式匹配
        dp[0][0] = true;

        // 處理模式開頭的 '*' 字符
        // '*' 可以匹配空字符串，所以需要設置相應的狀態
        for j in 1..=p_len {
            if p_chars[j - 1] == '*' {
                dp[0][j] = dp[0][j - 1];
            }
        }

        // Step 3: 動態規劃狀態轉移
        for i in 1..=s_len {
            for j in 1..=p_len {
                let s_char = s_chars[i - 1];
                let p_char = p_chars[j - 1];

                match p_char {
                    // 情況 1: 模式字符是普通字符，必須完全匹配
                    c if c != '?' && c != '*' => {
                        dp[i][j] = s_char == p_char && dp[i - 1][j - 1];
                    }

                    // 情況 2: 模式字符是 '?'，可以匹配任何單個字符
                    '?' => {
                        dp[i][j] = dp[i - 1][j - 1];
                    }

                    // 情況 3: 模式字符是 '*'，最複雜的情況
                    '*' => {
                        // '*' 有三種可能的匹配方式：
                        // 1. 匹配 0 個字符：dp[i][j-1]
                        //    (忽略當前 '*'，看前面的模式能否匹配)
                        // 2. 匹配 1 個字符：dp[i-1][j-1]
                        //    (用 '*' 匹配當前字符，然後看剩餘部分)
                        // 3. 匹配多個字符：dp[i-1][j]
                        //    (用 '*' 匹配當前字符，'*' 還能繼續匹配更多字符)
                        dp[i][j] = dp[i][j - 1] || dp[i - 1][j - 1] || dp[i - 1][j];
                    }

                    _ => unreachable!(),
                }
            }
        }

        // Step 4: 返回最終結果
        // dp[s_len][p_len] 表示完整的字符串是否與完整的模式匹配
        dp[s_len][p_len]
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
            true
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
