/// 3227. Vowels Game in a String
///
/// 題目描述：
/// Alice 和 Bob 在一個字符串 s 上進行回合制遊戲。遊戲規則如下：
///
/// 遊戲規則：
/// - Alice 總是先手
/// - 在 Alice 的回合：她必須移除任何包含奇數個母音的非空子字符串
/// - 在 Bob 的回合：他必須移除任何包含偶數個母音的非空子字符串
/// - 第一個無法進行有效移動的玩家輸掉遊戲
/// - 雙方都會採用最優策略（做出最好的可能移動）
///
/// 母音：英語母音包括 a, e, i, o, u。
///
/// 目標：判斷 Alice 是否會贏得遊戲。如果 Alice 贏，返回 true；如果 Bob 贏，返回 false。
///
/// 關鍵洞察：這實際上是一個腦筋急轉彎。讓我們根據字符串中母音總數 k 進行分析：
/// 1. 如果 k = 0（沒有母音）：Alice 無法移除任何子字符串（因為她需要奇數個母音），所以 Bob 立即獲勝。
/// 2. 如果 k 是奇數：Alice 可以一次移除整個字符串（因為它包含奇數個母音），立即獲勝。
/// 3. 如果 k 是偶數且大於 0：Alice 可以策略性地移除包含 k-1 個母音的子字符串（這是奇數），
///    剩下恰好 1 個母音。然後 Bob 無法進行移動（1 是奇數，不是偶數），所以 Alice 獲勝。
///
/// 結論：Alice 獲勝當且僅當字符串包含至少一個母音。
///
/// 示例 1：
/// 輸入：s = "leetcoder"
/// 輸出：true
/// 解釋：字符串包含母音 "e", "e", "o", "e"，總共 4 個（偶數）。
///      Alice 可以移除包含 3 個母音的子字符串，剩下 1 個母音，Bob 無法移動。
///
/// 示例 2：
/// 輸入：s = "bbcd"
/// 輸出：false
/// 解釋：字符串不包含母音，Alice 無法進行任何移動，Bob 獲勝。
///
/// 示例 3：
/// 輸入：s = "aeiou"
/// 輸出：true
/// 解釋：字符串包含 5 個母音（奇數），Alice 可以一次移除整個字符串獲勝。
///
/// 限制條件：
/// - 1 ≤ s.length ≤ 10^5
/// - s 只包含小寫英文字母
#[allow(dead_code)]
impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        for c in s.chars() {
            if VOWELS[c as usize] {
                return true;
            }
        }
        false
    }
}
const VOWELS: [bool; 256] = {
    let mut v = [false; 256];
    v[b'a' as usize] = true;
    v[b'e' as usize] = true;
    v[b'i' as usize] = true;
    v[b'o' as usize] = true;
    v[b'u' as usize] = true;
    v
};

#[allow(dead_code)]
pub(crate) struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 基本測試案例 - 來自題目示例

        // 示例 1：包含偶數個母音，Alice 獲勝
        assert_eq!(Solution::does_alice_win("leetcoder".to_string()), true);

        // 示例 2：沒有母音，Bob 獲勝
        assert_eq!(Solution::does_alice_win("bbcd".to_string()), false);

        // 示例 3：包含奇數個母音，Alice 獲勝
        assert_eq!(Solution::does_alice_win("aeiou".to_string()), true);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 單個母音
        assert_eq!(Solution::does_alice_win("a".to_string()), true);

        // 單個非母音
        assert_eq!(Solution::does_alice_win("b".to_string()), false);

        // 只有母音的字符串
        assert_eq!(Solution::does_alice_win("aeio".to_string()), true);

        // 只有非母音的字符串
        assert_eq!(Solution::does_alice_win("bcdfg".to_string()), false);

        // 最長字符串（理論上）
        let long_vowels = "a".repeat(100000);
        assert_eq!(Solution::does_alice_win(long_vowels), true);

        let long_consonants = "b".repeat(100000);
        assert_eq!(Solution::does_alice_win(long_consonants), false);
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // 交替的母音和非母音
        assert_eq!(Solution::does_alice_win("abab".to_string()), true);
        assert_eq!(Solution::does_alice_win("baba".to_string()), true);

        // 多個相同母音
        assert_eq!(Solution::does_alice_win("aaaa".to_string()), true);
        assert_eq!(Solution::does_alice_win("eeee".to_string()), true);

        // 所有母音各一個
        assert_eq!(Solution::does_alice_win("aeiou".to_string()), true);

        // 母音在字符串開頭
        assert_eq!(Solution::does_alice_win("abcdef".to_string()), true);

        // 母音在字符串結尾
        assert_eq!(Solution::does_alice_win("bcdefg a".to_string()), true);

        // 母音在字符串中間
        assert_eq!(Solution::does_alice_win("bcd e fgh".to_string()), true);

        // 大量非母音加一個母音
        assert_eq!(
            Solution::does_alice_win("bcdfghjklmnpqrstvwxyza".to_string()),
            true
        );

        // 包含所有小寫字母
        assert_eq!(
            Solution::does_alice_win("abcdefghijklmnopqrstuvwxyz".to_string()),
            true
        );

        // 複雜的混合情況
        assert_eq!(Solution::does_alice_win("programming".to_string()), true);
        assert_eq!(Solution::does_alice_win("rhythm".to_string()), false);
    }
}
