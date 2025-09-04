/// 1. Two Sum (兩數之和)
///
/// 給定一個整數數組 nums 和一個整數目標值 target，請你在該數組中找出和為目標值 target 的那兩個整數，
/// 並返回它們的數組下標。
///
/// 你可以假設每種輸入只會對應一個答案。但是，數組中同一個元素在答案裡不能重複出現。
/// 你可以按任意順序返回答案。
///
/// 示例 1：
/// 輸入：nums = [2,7,11,15], target = 9
/// 輸出：[0,1]
/// 解釋：因為 nums[0] + nums[1] == 9 ，返回 [0, 1]。
///
/// 示例 2：
/// 輸入：nums = [3,2,4], target = 6
/// 輸出：[1,2]
///
/// 示例 3：
/// 輸入：nums = [3,3], target = 6
/// 輸出：[0,1]
///
/// 提示：
/// - 2 <= nums.length <= 104
/// - -109 <= nums[i] <= 109
/// - -109 <= target <= 109
/// - 只會存在一個有效答案
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        nums.iter()
            .enumerate()
            .find_map(|(i, &num)| {
                map.get(&(target - num))
                    .map(|&j| vec![j, i as i32])
                    .or_else(|| {
                        map.insert(num, i as i32);
                        None
                    })
            })
            .unwrap()
    }
}

#[allow(dead_code)]
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{generate_test_data, measure_time_and_space};

    #[test]
    fn p0001_test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    #[ignore]
    fn p0001_test_performance() {
        let sizes = [100, 1000, 10000];

        for size in sizes {
            let nums = generate_test_data(size);
            let target = size as i32 - 1; // 確保答案在數組的最後

            let (result, metrics) =
                measure_time_and_space(|| Solution::two_sum(nums.clone(), target));

            println!("Input size: {}", size);
            println!("Result: {:?}", result);
            println!("Execution time: {:?}", metrics.execution_time);
            println!("Memory size: {} bytes", metrics.memory_size);
            println!("---");
        }
    }
}
