// 題目描述:
// 給定一個字符串 s。
// 你的任務是通過重複執行以下操作來刪除所有數字：
// 刪除第一個數字及其左側最近的非數字字符。
// 返回刪除所有數字後的結果字符串。
//
// 示例 1:
// 輸入: s = "abc"
// 輸出: "abc"
// 解釋:
// 字符串中沒有數字。
//
// 示例 2:
// 輸入: s = "cb34"
// 輸出: ""
// 解釋:
// 首先，我們對 s[2] 應用操作，s 變為 "c4"。
// 然後我們對 s[1] 應用操作，s 變為 ""。
//
// 約束條件:
// 1 <= s.length <= 100
// s 僅由小寫英文字母和數字組成。
// 輸入生成的情況下，可以刪除所有數字。
#[allow(dead_code)]
impl Solution {
    pub fn clear_digits(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let mut chars: Vec<char> = s.chars().collect();
        let mut i: usize = 0;

        while i < chars.len() {
            if chars[i].is_ascii_digit() {
                if i > 0 {
                    // 回退一个字符
                    i -= 1;
                    // 删除当前位置的字符
                    chars.remove(i);
                    // 如果当前位置仍有字符，则删除下一个字符
                    if i < chars.len() {
                        chars.remove(i);
                    }
                } else {
                    // 如果 i == 0，直接删除
                    chars.remove(i);
                }
            } else {
                i += 1;
            }
        }

        chars.into_iter().collect()
    }
}

#[allow(dead_code)]
struct Solution;

// 測試函數
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p3174_test_clear_digits() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc".to_string());
        assert_eq!(Solution::clear_digits("cb34".to_string()), "".to_string());
    }
}
