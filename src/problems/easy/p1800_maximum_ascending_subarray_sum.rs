/// 1800. 最大升序子數組和
/// 給你一個正整數組成的數組 nums ，請你返回在 nums 中可以組成的最大升序子數組和。
/// 
/// 子數組是數組中的一個連續數字序列。
/// 如果對於子數組 [numsl, numsl+1, ..., numsr-1, numsr] 中每個相鄰元素都滿足 numsi < numsi+1，
/// 則稱這個子數組為升序子數組。注意，只包含一個元素的子數組也是升序子數組。
/// 
/// 示例 1：
/// 輸入：nums = [10,20,30,5,10,50]
/// 輸出：65
/// 解釋：[5,10,50] 是一個升序子數組，其和為 65。
/// 
/// 示例 2：
/// 輸入：nums = [10,20,30,40,50]
/// 輸出：150
/// 解釋：[10,20,30,40,50] 是一個升序子數組，其和為 150。
/// 
/// 示例 3：
/// 輸入：nums = [12,17,15,13,10,11,12]
/// 輸出：33
/// 解釋：[10,11,12] 是一個升序子數組，其和為 33。

#[allow(dead_code)]
pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {    
    if nums.is_empty() {
        return 0;
    }
    
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            current_sum += nums[i];
        } else {
            current_sum = nums[i];
        }
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_ascending_sum() {
        assert_eq!(max_ascending_sum(vec![10,20,30,5,10,50]), 65);
        assert_eq!(max_ascending_sum(vec![10,20,30,40,50]), 150);
        assert_eq!(max_ascending_sum(vec![12,17,15,13,10,11,12]), 33);
        assert_eq!(max_ascending_sum(vec![3]), 3);
        assert_eq!(max_ascending_sum(vec![]), 0);
    }
}
