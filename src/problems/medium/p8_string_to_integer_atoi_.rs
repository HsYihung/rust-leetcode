/// 8. String to Integer (atoi)
///
/// 題目描述：
/// 請實現一個 atoi（ASCII to Integer）函數，將字符串轉換為 32 位有符號整數。
///
/// atoi 函數的算法如下：
/// 1. 忽略字符串開頭的空白字符（' '、'\t'、'\n'、'\r'、'\f'、'\v'）
/// 2. 檢查下一個字符是否為正負號（'+' 或 '-'），決定最終結果的符號
/// 3. 讀入後續的數字字符，直到遇到非數字字符或字符串結尾
/// 4. 將這些數字轉換為整數（例如："123" -> 123）
/// 5. 如果沒有讀入數字，則返回 0
/// 6. 如果整數超出 32 位有符號整數範圍 [-2^31, 2^31 - 1]，則將整數夾到該範圍內
///
/// 示例 1：
/// 輸入：s = "42"
/// 輸出：42
/// 解釋：粗體字符為已經讀入的字符，插入符號是目前讀入的字符。
///      第 1 步："42"（當前沒有讀入字符，因為沒有前導空格）
///      第 2 步："42"（當前沒有讀入字符，因為這裡不存在 '-' 或者 '+'）
///      第 3 步："42"（讀入 "42"）
///      解析得到整數 42。由於 "42" 在範圍 [-2^31, 2^31 - 1] 內，最終結果為 42。
///
/// 示例 2：
/// 輸入：s = "   -42"
/// 輸出：-42
/// 解釋：第 1 步："   -42"（讀入前導空格，但忽略掉）
///      第 2 步："   -42"（讀入 '-' 字符，所以結果應該是負數）
///      第 3 步："   -42"（讀入 "42"）
///      解析得到整數 -42。由於 "-42" 在範圍 [-2^31, 2^31 - 1] 內，最終結果為 -42。
///
/// 示例 3：
/// 輸入：s = "4193 with words"
/// 輸出：4193
/// 解釋：第 1 步："4193 with words"（當前沒有讀入字符，因為沒有前導空格）
///      第 2 步："4193 with words"（當前沒有讀入字符，因為這裡不存在 '-' 或者 '+'）
///      第 3 步："4193 with words"（讀入 "4193"；由於下一個字符不是一個數字，所以讀入停止）
///      解析得到整數 4193。由於 "4193" 在範圍 [-2^31, 2^31 - 1] 內，最終結果為 4193。
///
/// 限制條件：
/// - 0 <= s.length <= 200
/// - s 由英文字母（大小寫）、數字（0-9）、' '、'+'、'-' 和 '.' 組成

#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }

    let mut chars = s.chars();
    let mut result = 0i64;
    let sign = match chars.next().unwrap() {
        '+' => 1,
        '-' => -1,
        c if c.is_ascii_digit() => {
            result = (c as i64) - ('0' as i64);
            1
        }
        _ => return 0,
    };

    for ch in chars {
        if !ch.is_ascii_digit() {
            break;
        }
        let digit = (ch as i64) - ('0' as i64);

        if sign == 1 {
            if result > (i32::MAX as i64 - digit) / 10 {
                return i32::MAX;
            }
        } else if result > (2147483648 - digit) / 10 {
            return i32::MIN;
        }

        result = result * 10 + digit;
    }

    (result * sign) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 基本測試案例 - 來自題目示例
        assert_eq!(my_atoi("42".to_string()), 42);
        assert_eq!(my_atoi("   -42".to_string()), -42);
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例
        assert_eq!(my_atoi("".to_string()), 0); // 空字符串
        assert_eq!(my_atoi("   ".to_string()), 0); // 只有空格
        assert_eq!(my_atoi("0".to_string()), 0); // 單個零
        assert_eq!(my_atoi("1".to_string()), 1); // 單個數字
        assert_eq!(my_atoi("+1".to_string()), 1); // 正號
        assert_eq!(my_atoi("-1".to_string()), -1); // 負號
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例
        assert_eq!(my_atoi("2147483647".to_string()), 2147483647); // 最大 32 位整數
        assert_eq!(my_atoi("-2147483648".to_string()), -2147483648); // 最小 32 位整數
        assert_eq!(my_atoi("2147483648".to_string()), 2147483647); // 溢出正數
        assert_eq!(my_atoi("-2147483649".to_string()), -2147483648); // 溢出負數
        assert_eq!(my_atoi("words and 987".to_string()), 0); // 開頭是字母
        assert_eq!(my_atoi("+-12".to_string()), 0); // 多個符號
        assert_eq!(my_atoi("00000-42a1234".to_string()), 0); // 複雜情況
        assert_eq!(my_atoi("   +0 123".to_string()), 0); // 數字後有空格和字符
        assert_eq!(my_atoi("3.14159".to_string()), 3); // 小數點
        assert_eq!(my_atoi("  0000000000012345678".to_string()), 12345678); // 前導零
    }

    #[test]
    fn test_extreme_cases() {
        // 極端情況測試案例
        let long_positive = "1".repeat(200);
        assert_eq!(my_atoi(long_positive), i32::MAX); // 200個1

        let long_negative = format!("-{}", "9".repeat(200));
        assert_eq!(my_atoi(long_negative), i32::MIN); // -後接200個9

        let mixed_long = format!("   +{}", "8".repeat(100));
        assert_eq!(my_atoi(mixed_long), i32::MAX); // 空格+正號+100個8
    }
}
