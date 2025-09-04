/// 3516. Find Closest Person
///
/// 題目描述：
/// 給定三個整數 x、y 和 z，分別代表數線上三個人的位置：
/// - x 是第一個人的位置
/// - y 是第二個人的位置  
/// - z 是第三個人的位置，第三個人不會移動
///
/// 第一個人和第二個人都以相同的速度朝向第三個人移動。
///
/// 請判斷哪個人會先到達第三個人的位置：
/// - 如果第一個人先到達，返回 1
/// - 如果第二個人先到達，返回 2
/// - 如果兩個人同時到達，返回 0
///
/// 示例 1：
/// 輸入：x = 1, y = 2, z = 3
/// 輸出：2
/// 解釋：第一個人距離第三個人的距離是 |1 - 3| = 2
///      第二個人距離第三個人的距離是 |2 - 3| = 1
///      第二個人距離更近，所以第二個人先到達，返回 2
///
/// 示例 2：
/// 輸入：x = 1, y = 5, z = 3
/// 輸出：0
/// 解釋：第一個人距離第三個人的距離是 |1 - 3| = 2
///      第二個人距離第三個人的距離是 |5 - 3| = 2
///      兩個人的距離相等，所以會同時到達，返回 0
///
/// 示例 3：
/// 輸入：x = 5, y = 2, z = 3  
/// 輸出：2
/// 解釋：第一個人距離第三個人的距離是 |5 - 3| = 2
///      第二個人距離第三個人的距離是 |2 - 3| = 1
///      第二個人距離更近，所以第二個人先到達，返回 2
///
/// 限制條件：
/// - 1 <= x, y, z <= 100
#[allow(dead_code)]
pub fn find_closest_person(x: i32, y: i32, z: i32) -> i32 {
    let n1 = absolute(z - x);
    let n2 = absolute(z - y);

    if n1 < n2 {
        1
    } else if n2 < n1 {
        2
    } else {
        0
    }
}

pub fn absolute(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 基本測試案例 - 來自題目示例
        assert_eq!(find_closest_person(1, 2, 3), 2); // 示例 1：第二個人更接近
        assert_eq!(find_closest_person(1, 5, 3), 0); // 示例 2：兩人距離相等
        assert_eq!(find_closest_person(5, 2, 3), 2); // 示例 3：第二個人更接近
        assert_eq!(find_closest_person(1, 4, 3), 2); // 第二個人更接近 (距離1 vs 2)
    }

    #[test]
    fn test_boundary_cases() {
        // 邊界測試案例 - 最小最大值
        assert_eq!(find_closest_person(1, 1, 1), 0); // 所有人在同一位置
        assert_eq!(find_closest_person(1, 100, 50), 1); // 最大範圍，第一個人更接近
        assert_eq!(find_closest_person(100, 1, 50), 2); // 最大範圍，第二個人更接近
        assert_eq!(find_closest_person(25, 75, 50), 0); // 中間位置，兩人等距
    }

    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例
        assert_eq!(find_closest_person(1, 3, 2), 0); // z 在 x 和 y 之間，等距
        assert_eq!(find_closest_person(3, 1, 2), 0); // z 在 x 和 y 之間，等距
        assert_eq!(find_closest_person(10, 20, 5), 1); // z 在 x 左側
        assert_eq!(find_closest_person(5, 10, 15), 2); // z 在 y 右側
        assert_eq!(find_closest_person(50, 50, 75), 0); // x 和 y 相同位置
        assert_eq!(find_closest_person(1, 1, 10), 0); // x 和 y 相同位置，距離 z 較遠
    }
}
