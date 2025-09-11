/// 2785. Sort Vowels in a String
///
/// 題目描述：
/// 給定一個 0-indexed 的字符串 s，重新排列 s 來獲得一個新的字符串 t，使得：
/// 1. 所有輔音字母保持在原來的位置。更正式地說，如果存在一個索引 i，
///    其中 0 <= i < s.length 且 s[i] 是輔音字母，那麼 t[i] = s[i]。
/// 2. 母音字母必須按照其 ASCII 值的非遞減順序排序。更正式地說，
///    對於索引對 i, j，其中 0 <= i < j < s.length 且 s[i] 和 s[j] 都是母音字母，
///    那麼 t[i] 的 ASCII 值不能大於 t[j]。
///
/// 返回結果字符串。母音字母是 'a', 'e', 'i', 'o', 'u'，它們可以是小寫或大寫。
///
/// 示例 1：
/// 輸入：s = "lEetcOde"
/// 輸出："lEOtcede"
/// 解釋：'E', 'O', 和 'e' 是 s 中的母音字母；'l', 't', 'c', 和 'd' 都是輔音字母。
///        母音字母按照它們的 ASCII 值排序，輔音字母保持在相同的位置。
///
/// 示例 2：
/// 輸入：s = "lYmpH"
/// 輸出："lYmpH"
/// 解釋：s 中沒有母音字母（s 中的所有字符都是輔音字母），所以返回 "lYmpH"。
///
/// 限制條件：
/// - 1 <= s.length <= 10^5
/// - s 只包含字母（大寫和小寫）
#[allow(dead_code)]
impl Solution {
    /// 主要解法：函數式編程版本 - O(n + v log v) 時間，O(v) 空間，代碼簡潔優雅
    pub fn sort_vowels(s: String) -> String {
        Self::sort_vowels_function(s)
    }

    #[allow(dead_code)]
    fn sort_vowels_function(s: String) -> String {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
        }

        let mut vowels: Vec<char> = s.chars().filter(|&c| is_vowel(c)).collect();
        vowels.sort_unstable(); // sort by ASCII order

        let mut iter = vowels.into_iter();
        let result: String = s
            .chars()
            .map(|c| if is_vowel(c) { iter.next().unwrap() } else { c })
            .collect();

        result
    }

    #[allow(dead_code)]
    fn sort_vowels_hashmap(s: String) -> String {
        use std::collections::HashMap;
        let ascii_sort = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

        let mut vowel_count: HashMap<char, i32> = HashMap::new();
        let mut vowel_indexs: Vec<usize> = vec![];

        for (i, char) in s.chars().enumerate() {
            if ascii_sort.contains(&char) {
                *vowel_count.entry(char).or_insert(0) += 1;
                vowel_indexs.push(i);
            }
        }

        let mut result: Vec<char> = s.chars().collect();
        let mut vowel_idx = 0;

        for k in ascii_sort.iter() {
            let k_count = *vowel_count.get(k).unwrap_or(&0);

            for _ in 0..k_count {
                result[vowel_indexs[vowel_idx]] = *k;
                vowel_idx += 1;
            }
        }

        result.into_iter().collect()
    }
}

#[allow(dead_code)]
pub(crate) struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 示例 1：s = "lEetcOde"
        assert_eq!(
            Solution::sort_vowels("lEetcOde".to_string()),
            "lEOtcede".to_string()
        );

        // 示例 2：s = "lYmpH"
        assert_eq!(
            Solution::sort_vowels("lYmpH".to_string()),
            "lYmpH".to_string()
        );
    }

    #[test]
    fn test_edge_cases() {
        // 邊界案例：單個字符
        assert_eq!(Solution::sort_vowels("a".to_string()), "a".to_string());

        // 邊界案例：只有輔音字母
        assert_eq!(
            Solution::sort_vowels("bcdfg".to_string()),
            "bcdfg".to_string()
        );

        // 邊界案例：只有母音字母
        assert_eq!(
            Solution::sort_vowels("aeiou".to_string()),
            "aeiou".to_string()
        );

        // 邊界案例：大小寫混合的母音字母
        assert_eq!(
            Solution::sort_vowels("AeIoU".to_string()),
            "AIUeo".to_string()
        );
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況：需要重新排序的母音字母
        assert_eq!(
            Solution::sort_vowels("uOiEa".to_string()),
            "EOaiu".to_string()
        );

        // 特殊情況：複雜的母音和輔音混合
        assert_eq!(
            Solution::sort_vowels("RusTy".to_string()),
            "RusTy".to_string()
        );

        // 特殊情況：包含重複母音字母
        assert_eq!(
            Solution::sort_vowels("Programming".to_string()),
            "Pragrimmong".to_string()
        );

        // 特殊情況：全大寫
        assert_eq!(
            Solution::sort_vowels("HELLO".to_string()),
            "HELLO".to_string()
        );
    }
}
