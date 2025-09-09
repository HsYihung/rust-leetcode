/// 2327. Number of People Aware of a Secret
///
/// 題目描述：
/// 模擬秘密在人群中的傳播和遺忘過程。
///
/// 初始設置：
/// 第1天時，恰好有一個人發現了秘密。
///
/// 關鍵規則：
/// 1. 傳播規則：一個人發現秘密後，需要等待 delay 天才能開始分享秘密。
///    一旦可以分享，他們每天會向恰好一個新人分享秘密。
/// 2. 遺忘規則：一個人在發現秘密後的第 forget 天會忘記秘密。
///    忘記秘密後，他們就不能再分享秘密了。
/// 3. 時間約束：一個人不能在忘記秘密的同一天分享秘密。
///
/// 一個人的時間線範例：
/// - 第1天：發現秘密
/// - 第2天到第delay天：知道秘密但還不能分享
/// - 第(delay+1)天到第(forget-1)天：每天向一個新人分享秘密
/// - 第forget天：忘記秘密，停止分享
///
/// 目標：給定整數 n，計算第 n 天結束時有多少人知道秘密。
/// 由於答案可能很大，返回結果模 10^9 + 7。
///
/// 示例 1：
/// 輸入：n = 6, delay = 2, forget = 4
/// 輸出：5
/// 解釋：
/// - 第1天：1個人知道秘密
/// - 第2天：1個人知道秘密（還在delay期間）
/// - 第3天：2個人知道秘密（原來的人開始分享）
/// - 第4天：3個人知道秘密（原來的人繼續分享）
/// - 第5天：3個人知道秘密（原來的人忘記了，但其他人開始分享）
/// - 第6天：5個人知道秘密
///
/// 示例 2：
/// 輸入：n = 4, delay = 1, forget = 3
/// 輸出：6
/// 解釋：
/// - 第1天：1個人知道
/// - 第2天：2個人知道（第1天的人開始分享）
/// - 第3天：4個人知道（第1天的人繼續分享，第2天的人開始分享）
/// - 第4天：6個人知道（第1天的人忘記，第2、3天的人分享）
///
/// 限制條件：
/// - 2 <= n <= 1000
/// - 1 <= delay < forget <= n
#[allow(dead_code)]
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut share = 0i64;
        let mut dp = vec![0i64; n as usize];
        dp[0] = 1;

        for i in 1..n as usize {
            if i >= delay as usize {
                share = (share + dp[i - delay as usize]) % MOD;
            }
            if i >= forget as usize {
                share = (share - dp[i - forget as usize] + MOD) % MOD;
            }
            dp[i] = share;
        }

        let start = (n as usize).saturating_sub(forget as usize);

        let result = dp
            .iter()
            .skip(start)
            .take(n as usize - start)
            .fold(0i64, |acc, &val| (acc + val) % MOD);

        result as i32
    }
}

#[allow(dead_code)]
pub(crate) struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 基本測試案例 - 來自題目示例
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    }

    #[test]
    fn test_edge_cases() {
        // 邊界測試案例

        // 最小參數：n=2, delay=1, forget=2
        assert_eq!(Solution::people_aware_of_secret(2, 1, 2), 2);

        // delay和forget的邊界情況
        assert_eq!(Solution::people_aware_of_secret(3, 1, 2), 2);
        assert_eq!(Solution::people_aware_of_secret(5, 1, 5), 16);

        // 較大的n值
        assert_eq!(Solution::people_aware_of_secret(100, 5, 10), 108223204);

        // delay接近forget的情況
        assert_eq!(Solution::people_aware_of_secret(10, 4, 5), 1);
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例

        // delay=1的情況（立即可以分享）
        assert_eq!(Solution::people_aware_of_secret(3, 1, 3), 4);
        assert_eq!(Solution::people_aware_of_secret(5, 1, 4), 14);

        // forget很大的情況（很久才忘記）
        assert_eq!(Solution::people_aware_of_secret(10, 2, 10), 55);

        // 中等規模測試
        assert_eq!(Solution::people_aware_of_secret(20, 3, 8), 520);
        assert_eq!(Solution::people_aware_of_secret(50, 5, 15), 352858);

        // 測試模數運算
        assert_eq!(Solution::people_aware_of_secret(1000, 1, 1000), 344211605);

        // delay和forget差距較小的情況
        assert_eq!(Solution::people_aware_of_secret(15, 5, 7), 4);
        assert_eq!(Solution::people_aware_of_secret(8, 2, 4), 8);

        // 各種組合測試
        assert_eq!(Solution::people_aware_of_secret(12, 3, 6), 19);
        assert_eq!(Solution::people_aware_of_secret(7, 1, 4), 48);
    }
}
