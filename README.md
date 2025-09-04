# LeetCode Solutions in Rust

這個專案收集了 LeetCode 題目的 Rust 實現。

## 專案結構

```
src/
├── main.rs
├── utils/
│   └── mod.rs                                  # 工具函數，包含性能測試框架
└── problems/
    ├── easy/
    │   ├── p0001_two_sum.rs
    │   ├── p392_is_subsequence.rs
    │   ├── p509_fibonacci_number.rs
    │   ├── p1800_maximum_ascending_subarray_sum.rs
    │   ├── p3174_clear_digits.rs
    │   └── p3516_find_closest_person.rs
    ├── medium/
    │   ├── p8_string_to_integer_atoi_.rs
    │   ├── p62_unique_paths.rs
    │   ├── p63_unique_paths_ii.rs
    │   ├── p73_set_matrix_zeroes.rs
    │   ├── p498_diagonal_traverse.rs
    │   ├── p1504_count_submatrices_with_all_ones.rs
    │   └── p3195_find_the_minimum_area_to_cover_all_ones_i.rs
    └── hard/
        └── p679_24_game.rs
```

## 題目列表

### Easy

| #    | 題目 | 解法 | 備註 |
|------|------|------|------|
| 1    | [Two Sum](src/problems/easy/p0001_two_sum.rs) | HashMap | O(n) 時間, O(n) 空間 |
| 392  | [Is Subsequence](src/problems/easy/p392_is_subsequence.rs) | 雙指針 | O(n+m) 時間, O(1) 空間 |
| 509  | [Fibonacci Number](src/problems/easy/p509_fibonacci_number.rs) | 矩陣快速冪 | O(log n) 時間, O(1) 空間 |
| 1800 | [Maximum Ascending Subarray Sum](src/problems/easy/p1800_maximum_ascending_subarray_sum.rs) | 一次遍歷 | O(n) 時間, O(1) 空間 |
| 3174 | [Clear Digits](src/problems/easy/p3174_clear_digits.rs) | 一次遍歷 | O(n) 時間, O(1) 空間 |
| 3516 | [Find Closest Person](src/problems/easy/p3516_find_closest_person.rs) | 距離比較 | O(1) 時間, O(1) 空間 |

### Medium

| #   | 題目 | 解法 | 備註 |
|-----|------|------|------|
| 8   | [String to Integer (atoi)](src/problems/medium/p8_string_to_integer_atoi_.rs) | 字符串解析 + 溢出檢查 | O(n) 時間, O(1) 空間
| 62  | [Unique Paths](src/problems/medium/p62_unique_paths.rs) | 組合數學 | O(min(m,n)) 時間, O(1) 空間 |
| 63  | [Unique Paths II](src/problems/medium/p63_unique_paths_ii.rs) | 動態規劃 | O(mn) 時間, O(mn) 空間 |
| 73  | [Set Matrix Zeroes](src/problems/medium/p73_set_matrix_zeroes.rs) | 原地標記法 | O(m+n) 空間，O(1) 額外空間優化
| 498 | [Diagonal Traverse](src/problems/medium/p498_diagonal_traverse.rs) | 模擬對角線遍歷 | O(mn) 時間, O(mn) 空間
| 1504 | [Count Submatrices With All Ones](src/problems/medium/p1504_count_submatrices_with_all_ones.rs) | 高度數組 + 直方圖遍歷 | O(nm²) 時間, O(m) 空間
| 3195 | [Find the Minimum Area to Cover All Ones I](src/problems/medium/p3195_find_the_minimum_area_to_cover_all_ones_i.rs) | 邊界框算法 | O(mn) 時間, O(1) 空間

### Hard

| #   | 題目 | 解法 | 備註 |
|-----|------|------|------|
| 679 | [24 Game](src/problems/hard/p679_24_game.rs) | 回溯算法 | O(4^4 * 4!) 時間複雜度 |

## 統計

* **總題目數**: 14
* **Easy**: 6 題
* **Medium**: 7 題  
* **Hard**: 1 題
* **已完成**: 14 題
* **待實現**: 0 題

## 執行測試

運行所有一般測試（不包含性能測試）：

```bash
cargo test  # 運行所有一般測試
cargo test p0001  # 運行 Two Sum 的一般測試
```

運行性能測試：

```bash
# 運行所有被標記為 ignore 的測試
cargo test -- --ignored

# 運行特定的性能測試
cargo test p0001_test_performance -- --ignored --nocapture

# 運行所有測試（包含一般測試和性能測試）
cargo test -- --include-ignored
```

## 新增題目

使用腳本快速生成題目模板：

```bash
./scripts/add_problem.sh <number> "<title>" <difficulty>

# 範例
./scripts/add_problem.sh 1 "Two Sum" easy
./scripts/add_problem.sh 42 "Trapping Rain Water" hard
```

## 性能測試框架

專案包含了一個簡單的性能測試框架（在 `src/utils/mod.rs` ），可以測量：
* 執行時間（精確到納秒）
* 內存使用
* 不同輸入大小的擴展性

使用方式：

```rust
use crate::utils::measure_time_and_space;

let (result, metrics) = measure_time_and_space(|| {
    // 你的函數調用
});
println!("Execution: {:?}", metrics.execution_time);
println!("Memory: {} bytes", metrics.memory_size);
```

## Solution Techniques

### Two Sum

* HashMap 解法：O(n) 時間複雜度，使用額外空間來換取時間效率

### Maximum Ascending Subarray Sum

* 一次遍歷：O(n) 時間複雜度，不需要額外空間

### Set Matrix Zeroes (p73)

* 原地標記法：利用第一行與第一列作為標記區域，先標記需歸零的行與列，最後再處理第一行與第一列，實現 O(1) 額外空間。

### String to Integer (atoi) (p8)

* 字符串解析：依序處理前導空格、符號、數字字符
* 溢出檢查：在累加每一位前檢查是否會超出 i32 範圍，提前返回邊界值
* 處理極端情況：支持 200+ 位數字字符的正確處理
* 時間複雜度：O(n)，空間複雜度：O(1)

### 24 Game (p679)

* 回溯算法：每次選擇兩個數字進行四則運算，將結果放回繼續遞歸，直到只剩一個數字檢查是否等於24
* 處理浮點精度問題使用 `EPS = 1e-6` 進行比較
* 除法運算前檢查分母不為0，避免除零錯誤
* 時間複雜度：O(4^4 * 4!)，空間複雜度：O(4)（遞歸深度）

### Diagonal Traverse (p498)

* 模擬對角線遍歷：使用狀態機模擬對角線方向的切換
* 邊界處理：正確處理到達矩陣邊界時的方向變換和位置移動
* 支持矩形矩陣：適用於 m×n 任意大小的矩陣，包括正方形和矩形
* 時間複雜度：O(mn)（必須訪問每個元素），空間複雜度：O(mn)（輸出陣列）

### Find the Minimum Area to Cover All Ones I (p3195)

* 邊界框算法：遍歷矩陣找到所有 1 的最小/最大行列索引，計算包圍矩形面積
* 使用四個變量追踪邊界：min_row, max_row, min_col, max_col
* 面積計算：(max_row - min_row + 1) × (max_col - min_col + 1)
* 時間複雜度：O(mn)（必須遍歷整個矩陣），空間複雜度：O(1)

### Find Closest Person (p3516)

* 距離比較算法：計算兩個人到目標位置的絕對距離，比較大小決定誰更接近
* 使用自定義絕對值函數處理距離計算：`absolute(z - x)` 和 `absolute(z - y)`
* 三種返回情況：第一個人更接近返回1，第二個人更接近返回2，距離相等返回0
* 時間複雜度：O(1)（固定的比較和計算操作），空間複雜度：O(1)（只使用常數額外變量）

### Fibonacci Number (p509)

* 矩陣快速冪算法：利用斐波那契數列的矩陣表示 `M^n = [[F(n+1), F(n)], [F(n), F(n-1)]]`
* 使用快速冪技術將矩陣乘法優化到對數級別，基於二進制分解：`M^n = M^(2^k1) × M^(2^k2) × ...`
* 核心矩陣：`[[1,1],[1,0]]`，通過矩陣乘法規則計算任意次冪
* 時間複雜度：O(log n)（快速冪需要log₂(n)次矩陣乘法），空間複雜度：O(1)（固定大小矩陣操作）

### Is Subsequence (p392)

* 雙指針算法：使用兩個指針分別追蹤 s 和 t 字符串的位置，當字符匹配時同時移動指針
* 提前終止優化：添加邊界檢查，空字符串直接返回 true，s 比 t 長直接返回 false
* 字符訪問：將字符串轉換為字符向量，支持高效的隨機訪問和比較
* 時間複雜度：O(n+m)（n 為 s 長度，m 為 t 長度），空間複雜度：O(n+m)（字符向量存儲）

### Unique Paths (p62)

* 組合數學算法：將路徑問題轉化為組合數計算 C(m+n-2, m-1)
* 數學優化：使用連續相乘除法避免階乘溢出：result = result × (m+n-2-i) / (i+1)
* 對稱性優化：確保 m ≤ n，減少迴圈次數從 O(max(m,n)) 到 O(min(m,n))
* 溢出處理：使用 u64 進行中間計算，避免 i32 溢出問題
* 邊界條件：當 m 或 n 為 1 時直接返回 1
* 時間複雜度：O(min(m,n))（組合數計算），空間複雜度：O(1)（常數空間）

### Unique Paths II (p63)

* 動態規劃算法：使用二維 DP 表格存儲每個位置的路徑數量，處理障礙物約束
* 障礙物處理：當格子為障礙物 (1) 時，路徑數設為 0；否則為上方和左方路徑數之和
* 邊界條件處理：起始位置為障礙物時直接返回 0，邊界位置正確處理 if-else 表達式
* 狀態轉移方程：`paths[i][j] = top + left`（其中 top 和 left 考慮邊界情況）
* 初始化策略：創建與輸入相同大小的 DP 表格，起始位置 (0,0) 設為 1
* 時間複雜度：O(m×n)（遍歷所有格子），空間複雜度：O(m×n)（DP 表格存儲）
