// 3362. 零陣列轉換 III
// https://leetcode.com/problems/zero-array-transformation-iii/
//
// 給你一個長度為 n 的整數陣列 nums，以及一個二維陣列 queries，其中 queries[i] = [li, ri]。
//
// 每個 queries[i] 表示對 nums 進行以下操作：
//   將 nums 中區間 [li, ri] 內的每個元素，各自最多減去 1（每個元素可以減的數量可以不同，也可以為 0）。
//
// 「零陣列」是指所有元素都為 0 的陣列。
//
// 請你回傳：最多可以從 queries 中移除多少個查詢，使得剩下的查詢仍然可以將 nums 轉換為零陣列。如果無法將 nums 轉換為零陣列，請回傳 -1。
//
// 範例 1：
// 輸入：nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]
// 輸出：1
// 解釋：移除 queries[2] 後，剩下的查詢仍可將 nums 變成零陣列。
// - 使用 queries[0]，將 nums[0] 和 nums[2] 各減 1，nums[1] 減 0。
// - 使用 queries[1]，再將 nums[0] 和 nums[2] 各減 1，nums[1] 減 0。
//
// 範例 2：
// 輸入：nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]
// 輸出：2
// 解釋：可以移除 queries[2] 和 queries[3]。
//
// 範例 3：
// 輸入：nums = [1,2,3,4], queries = [[0,3]]
// 輸出：-1
// 解釋：即使用上所有查詢，也無法將 nums 變成零陣列。
//
// 限制條件：
// 1 <= nums.length <= 10^5
// 0 <= nums[i] <= 10^5
// 1 <= queries.length <= 10^5
// queries[i].length == 2
// 0 <= li <= ri < nums.length
#[allow(dead_code)]
impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        queries.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut heap = BinaryHeap::new();
        let mut delta_array = vec![0; nums.len() + 1];
        let mut operations = 0;
        let mut j = 0;

        for i in 0..nums.len() {
            operations += delta_array[i];
            while j < queries.len() && queries[j][0] == i as i32 {
                heap.push(queries[j][1]);
                j += 1;
            }
            while operations < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
                operations += 1;
                let end = heap.pop().unwrap() as usize;
                delta_array[end + 1] -= 1;
            }
            if operations < nums[i] {
                return -1;
            }
        }
        heap.len() as i32
    }
}

#[allow(dead_code)]
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    // 大型測資
    #[test]
    fn custom_case_large() {
        let nums = vec![2, 2, 4, 4, 1, 0, 5, 0, 5];
        let queries = vec![
            vec![7, 8],
            vec![7, 8],
            vec![1, 8],
            vec![1, 5],
            vec![2, 7],
            vec![3, 6],
            vec![4, 5],
            vec![1, 3],
            vec![5, 7],
            vec![0, 0],
            vec![2, 5],
            vec![2, 6],
            vec![2, 2],
            vec![1, 2],
            vec![3, 7],
            vec![4, 4],
            vec![2, 6],
            vec![6, 6],
            vec![4, 5],
            vec![5, 8],
            vec![3, 5],
            vec![0, 5],
            vec![5, 8],
            vec![2, 7],
            vec![0, 8],
            vec![3, 4],
            vec![2, 4],
        ];
        assert_eq!(Solution::max_removal(nums, queries), 20);
    }

    // 題目自訂測資2
    #[test]
    fn custom_case_2() {
        let nums = vec![1, 2];
        let queries = vec![
            vec![1, 1],
            vec![0, 0],
            vec![1, 1],
            vec![1, 1],
            vec![0, 1],
            vec![0, 0],
        ];
        assert_eq!(Solution::max_removal(nums, queries), 4);
    }

    // 題目範例 1
    #[test]
    fn example1() {
        let nums = vec![2, 0, 2];
        let queries = vec![vec![0, 2], vec![0, 2], vec![1, 1]];
        assert_eq!(Solution::max_removal(nums, queries), 1);
    }

    // 題目自訂測資
    #[test]
    fn custom_case_1() {
        let nums = vec![0, 3];
        let queries = vec![vec![0, 1], vec![0, 0], vec![0, 1], vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::max_removal(nums, queries), 2);
    }

    // 題目範例 2
    #[test]
    fn example2() {
        let nums = vec![1, 1, 1, 1];
        let queries = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]];
        assert_eq!(Solution::max_removal(nums, queries), 2);
    }

    // 題目範例 3
    #[test]
    fn example3() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![0, 3]];
        assert_eq!(Solution::max_removal(nums, queries), -1);
    }
}
