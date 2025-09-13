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
    │   ├── p338_counting_bits.rs
    │   ├── p392_is_subsequence.rs
    │   ├── p509_fibonacci_number.rs
    │   ├── p1304_find_n_unique_integers_sum_up_to_zero.rs
    │   ├── p1800_maximum_ascending_subarray_sum.rs
    │   ├── p1317_convert_integer_to_the_sum_of_two_no_zero_integers.rs
    │   ├── p3174_clear_digits.rs
    │   ├── p3516_find_closest_person.rs
    │   └── p3541_find_most_frequent_vowel_and_consonant.rs
    ├── medium/
    │   ├── p8_string_to_integer_atoi_.rs
    │   ├── p16_3sum_closest.rs
    │   ├── p62_unique_paths.rs
    │   ├── p63_unique_paths_ii.rs
    │   ├── p2327_number_of_people_aware_of_a_secret.rs
    │   ├── p73_set_matrix_zeroes.rs
    │   ├── p498_diagonal_traverse.rs
    │   ├── p1504_count_submatrices_with_all_ones.rs
    │   ├── p1733_minimum_number_of_people_to_teach.rs
    │   ├── p2749_minimum_operations_to_make_the_integer_zero.rs
    │   ├── p3025_find_the_number_of_ways_to_place_people_i.rs
    │   ├── p3195_find_the_minimum_area_to_cover_all_ones_i.rs
    │   ├── p3227_vowels_game_in_a_string.rs
    │   └── p3362_zero_array_transformation_iii.rs
    └── hard/
        ├── p44_wildcard_matching.rs
        ├── p679_24_game.rs
        └── p3027_find_the_number_of_ways_to_place_people_ii.rs
```

## 題目列表

### Easy

| #    | 題目 | 解法 | 備註 |
|------|------|------|------|
| 1    | [Two Sum](src/problems/easy/p0001_two_sum.rs) | HashMap | O(n) 時間, O(n) 空間 |
| 392  | [Is Subsequence](src/problems/easy/p392_is_subsequence.rs) | 雙指針 | O(n+m) 時間, O(1) 空間 |
| 509  | [Fibonacci Number](src/problems/easy/p509_fibonacci_number.rs) | 矩陣快速冪 | O(log n) 時間, O(1) 空間 |
| 338  | [Counting Bits](src/problems/easy/p338_counting_bits.rs) | 動態規劃 + 位操作 | O(n) 時間, O(1) 空間 |
| 1304 | [Find N Unique Integers Sum up to Zero](src/problems/easy/p1304_find_n_unique_integers_sum_up_to_zero.rs) | 數學構造法 | O(n) 時間, O(n) 空間 |
| 1317 | [Convert Integer to the Sum of Two No-Zero Integers](src/problems/easy/p1317_convert_integer_to_the_sum_of_two_no_zero_integers.rs) | 順序遍歷 + 隨機解法 | O(n) 時間, O(1) 空間 |
| 1800 | [Maximum Ascending Subarray Sum](src/problems/easy/p1800_maximum_ascending_subarray_sum.rs) | 一次遍歷 | O(n) 時間, O(1) 空間 |
| 3174 | [Clear Digits](src/problems/easy/p3174_clear_digits.rs) | 一次遍歷 | O(n) 時間, O(1) 空間 |
| 3516 | [Find Closest Person](src/problems/easy/p3516_find_closest_person.rs) | 距離比較 | O(1) 時間, O(1) 空間 |
| 3541 | [Find Most Frequent Vowel and Consonant](src/problems/easy/p3541_find_most_frequent_vowel_and_consonant.rs) | 陣列計數法 | O(n) 時間, O(1) 空間 |

### Medium

| #   | 題目 | 解法 | 備註 |
|-----|------|------|------|
| 8   | [String to Integer (atoi)](src/problems/medium/p8_string_to_integer_atoi_.rs) | 字符串解析 + 溢出檢查 | O(n) 時間, O(1) 空間
| 16  | [3Sum Closest](src/problems/medium/p16_3sum_closest.rs) | 雙指針 + 二分搜索優化 | O(n²) 時間, O(1) 空間 |
| 62  | [Unique Paths](src/problems/medium/p62_unique_paths.rs) | 組合數學 | O(min(m,n)) 時間, O(1) 空間 |
| 2327 | [Number of People Aware of a Secret](src/problems/medium/p2327_number_of_people_aware_of_a_secret.rs) | 滑動窗口 + 動態規劃 | O(n) 時間, O(n) 空間 |
| 63  | [Unique Paths II](src/problems/medium/p63_unique_paths_ii.rs) | 動態規劃 | O(mn) 時間, O(mn) 空間 |
| 73  | [Set Matrix Zeroes](src/problems/medium/p73_set_matrix_zeroes.rs) | 原地標記法 | O(m+n) 空間，O(1) 額外空間優化
| 498 | [Diagonal Traverse](src/problems/medium/p498_diagonal_traverse.rs) | 模擬對角線遍歷 | O(mn) 時間, O(mn) 空間
| 1504 | [Count Submatrices With All Ones](src/problems/medium/p1504_count_submatrices_with_all_ones.rs) | 高度數組 + 直方圖遍歷 | O(nm²) 時間, O(m) 空間
| 1733 | [Minimum Number of People to Teach](src/problems/medium/p1733_minimum_number_of_people_to_teach.rs) | 貪心算法 + 集合操作 | O(L + F×n + n×m) 時間, O(L + m) 空間 |
| 2749 | [Minimum Operations to Make the Integer Zero](src/problems/medium/p2749_minimum_operations_to_make_the_integer_zero.rs) | 位操作 + 數學分析 | O(60) 時間, O(1) 空間
| 2785 | [Sort Vowels in a String](src/problems/medium/p2785_sort_vowels_in_a_string.rs) | 函數式編程 + 迭代器 | O(n + v log v) 時間, O(v) 空間 |
| 3025 | [Find the Number of Ways to Place People I](src/problems/medium/p3025_find_the_number_of_ways_to_place_people_i.rs) | 排序 + 掃描線優化 | O(n²) 時間, O(1) 空間 |
| 3195 | [Find the Minimum Area to Cover All Ones I](src/problems/medium/p3195_find_the_minimum_area_to_cover_all_ones_i.rs) | 邊界框算法 | O(mn) 時間, O(1) 空間 |
| 3227 | [Vowels Game in a String](src/problems/medium/p3227_vowels_game_in_a_string.rs) | 查表法 + 提前終止 | O(n) 時間, O(1) 空間 |
| 3362 | [Zero Array Transformation III](src/problems/medium/p3362_zero_array_transformation_iii.rs) | 貪心 + 優先隊列 | O(n log n) 時間, O(n) 空間

### Hard

| #    | 題目 | 解法 | 備註 |
|------|------|------|------|
| 44   | [Wildcard Matching](src/problems/hard/p44_wildcard_matching.rs) | 動態規劃 | O(mn) 時間, O(mn) 空間 |
| 679  | [24 Game](src/problems/hard/p679_24_game.rs) | 回溯算法 | O(4^4 * 4!) 時間複雜度 |
| 3027 | [Find the Number of Ways to Place People II](src/problems/hard/p3027_find_the_number_of_ways_to_place_people_ii.rs) | 待實現 | 幾何 + 枚舉 |

## 統計

* **總題目數**: 28
* **Easy**: 10 題
* **Medium**: 15 題  
* **Hard**: 3 題
* **已完成**: 27 題
* **待實現**: 1 題

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

### Minimum Operations to Make the Integer Zero (p2749)

* 位操作 + 數學分析：基於二進制表示的組合問題，每次操作相當於選擇一個 2 的幂次
* 核心思路：經過 k 次操作後，target = num1 - k*num2 必須能表示為 k 個 2 的幂次之和
* 三個必要條件檢查：target > 0（正數），target ≥ k（最小值），popcount(target) ≤ k（二進制 1 的個數）
* Popcount 計算：使用位運算逐位檢查，計算二進制表示中 1 的個數
* 優化策略：限制操作次數上限為 60（2^60 超過 i32 範圍），提前終止不可能的情況
* 時間複雜度：O(60)（固定迴圈上限），空間複雜度：O(1)（常數空間）

### Zero Array Transformation III (p3362)

* 貪心 + 優先隊列：使用貪心策略最大化移除的查詢數量，同時確保剩餘操作足以清零陣列
* 差分陣列技術：使用 delta_array 追蹤區間操作的累積效果，高效處理範圍更新
* 優先隊列管理：按查詢結束位置的最大堆，優先選擇能覆蓋更遠位置的操作
* 動態決策：遍歷每個位置時，根據當前需求動態決定是否使用可用的查詢操作
* 可行性檢查：如果某位置無法通過剩餘操作清零，直接返回 -1
* 時間複雜度：O(n log n)（排序 + 堆操作），空間複雜度：O(n)（差分陣列 + 優先隊列）

### Counting Bits (p338)

* 動態規劃 + 位操作技巧：利用 `i & (i-1)` 移除最右邊的1，基於已計算結果構建新結果
* 核心狀態轉移：`dp[i] = dp[i & (i-1)] + 1`，其中 `i & (i-1)` 得到移除最右邊1後的數字
* 位元操作原理：當計算 `i-1` 時，最右邊的1變為0，該位右邊所有0變為1，AND運算後移除最右邊的1
* 最優化解法：避免對每個數字重新計算位元數，直接利用子問題的結果加1
* 線性時間算法：滿足題目進階要求，每個數字只需O(1)時間計算，避免O(n log n)暴力解法
* 時間複雜度：O(n)（每個數字計算一次），空間複雜度：O(1)（除答案陣列外常數額外空間）

### Find N Unique Integers Sum up to Zero (p1304)

* 對稱配對算法：使用正負數配對策略，每次迭代同時設置一對數字
* 數學構造法：構造 1,2,...,mid 和對應的 -1,-2,...,-mid，確保和為 0
* 優化迭代策略：只需 n/2 次迴圈，每次設置 `result[i] = i+1` 和 `result[n-1-i] = -(i+1)`
* 奇數處理機制：當 n 為奇數時，中間位置設為 0，保持總和為 0
* 特殊情況優化：n = 1 時直接返回預初始化的 [0] 陣列
* 效率提升：相比逐一計算，減少 50% 的迴圈操作和條件判斷
* 時間複雜度：O(n)（陣列初始化 + n/2 次迭代），空間複雜度：O(n)（結果陣列，無法優化）

### Wildcard Matching (p44)

* 動態規劃算法：使用二維 DP 表格處理字符串與萬用字符模式的匹配問題
* 狀態定義：`dp[i][j]` 表示字符串 s[0..i] 與模式 p[0..j] 是否匹配
* 萬用字符處理：'?' 匹配任意單個字符，'*' 匹配任意字符序列（包括空序列）
* 狀態轉移：普通字符需完全匹配，'?' 直接轉移，'*' 考慮三種情況（匹配0個、1個、多個字符）
* 邊界初始化：空字符串與空模式匹配，處理模式開頭的連續 '*' 字符
* 時間複雜度：O(m×n)（m 為字符串長度，n 為模式長度），空間複雜度：O(m×n)（DP 表格）

### 3Sum Closest (p16)

* 雙指針 + 二分搜索優化：結合排序、雙指針技術和二分搜索預優化，達到接近最優的實際性能
* 三層結構：外層固定第一個元素，中層使用二分搜索找到最佳雙指針起始位置，內層執行標準雙指針搜索
* 二分搜索優化：根據當前三數和與目標的關係，智能定位雙指針的最佳起始範圍，減少無效移動
* 提前終止機制：找到完全匹配的三數和時立即返回，避免不必要的後續搜索
* 邊界處理：正確處理索引重複、陣列邊界，以及各種極端輸入情況
* 時間複雜度：O(n²)（理論最優），空間複雜度：O(1)（原地排序，常數額外空間）

### Convert Integer to the Sum of Two No-Zero Integers (p1317)

* 雙解法實現：提供順序遍歷和隨機生成兩種不同的解決方案，展示不同的算法思路
* 順序遍歷法：從 1 到 n-1 依序檢查每個可能的 a 值，計算 b = n - a，檢查兩數是否都不含 0
* 隨機生成法：使用隨機數生成器產生候選解，重複直到找到有效的無零整數對
* 無零檢查：使用字符串轉換和 contains('0') 方法高效檢查數字是否包含 0
* 算法保證：題目保證解存在，順序遍歷必定找到解，隨機法期望很快收斂
* 時間複雜度：O(n) 順序遍歷最壞情況，O(1) 隨機法期望，空間複雜度：O(1)（常數額外空間）

### Number of People Aware of a Secret (p2327)

* 滑動窗口優化動態規劃：使用滑動窗口技術維護當前可分享秘密的人數總和，避免重複計算
* 核心思路：每天計算新知道秘密的人數，基於延遲分享和遺忘規則動態更新分享窗口
* 狀態轉移：`share` 變量追蹤當前分享者總數，`dp[i]` 記錄第i天新發現秘密的人數
* 分享窗口管理：delay天後加入新分享者，forget天後移除忘記的分享者
* 結果計算：統計最後forget天內發現秘密的所有人數（未忘記的人群）
* 模數運算：使用 1e9+7 模數避免大數溢出，處理負數情況確保結果正確性
* 時間複雜度：O(n)（線性掃描 + 常數窗口操作），空間複雜度：O(n)（DP陣列存儲）

### Minimum Number of People to Teach (p1733)

* 貪心算法 + 集合操作：識別無法溝通的朋友對，收集問題用戶，選擇最優語言教授策略
* 核心策略：將語言數據轉為 HashSet 提升查找效率，使用集合交集判斷是否有共同語言
* 問題用戶收集：遍歷友誼關係，找出沒有共同語言的用戶對，將其加入問題用戶集合
* 最優語言選擇：統計每種語言在問題用戶中的掌握人數，選擇已掌握人數最多的語言
* 結果計算：問題用戶總數減去最多人已掌握的語言人數，即為需要教授的最少用戶數
* 優化技巧：HashSet 提供 O(1) 查找，intersection() 高效計算集合交集，提前終止無問題用戶的情況
* 時間複雜度：O(L + F×n + n×m)（L為語言條目數，F為友誼數，在題目限制下接近常數），空間複雜度：O(L + m)

### Find the Number of Ways to Place People I (p3025)

* 排序 + 掃描線優化：將點按 x 升序、x 相等時 y 降序排列，利用排序特性避免 O(n³) 的矩形內點檢查
* 核心優化技巧：對每個左上角點，維護 max_y 變量記錄已找到的有效右下角點的最大 y 坐標
* 智能剪枝：新的右下角點只有在 y 坐標大於 max_y 時才是有效的，避免被之前的點阻擋
* 位置關係檢查：確保左上角點滿足 A.x ≤ B.x 且 A.y ≥ B.y 的約束條件（計算機座標系）
* 算法正確性：通過排序保證 x 坐標關係，通過 max_y 隱式檢查矩形內無阻擋點
* 時間複雜度：O(n²)（排序 O(n log n) + 雙重循環 O(n²)），空間複雜度：O(1)（原地排序，常數額外空間）

### Sort Vowels in a String (p2785)

* 函數式編程 + 迭代器狀態管理：使用三步驟策略（提取、排序、重建）優雅處理母音字母排序問題
* 核心技術選擇：使用 matches! 宏進行高效母音檢測，sort_unstable() 實現快速排序，迭代器自動管理狀態
* 算法步驟：先用 filter() 提取所有母音字母，按 ASCII 值排序，再用 map() 重建字符串
* 迭代器優化：利用 into_iter() 創建消耗型迭代器，每次遇到母音位置時自動取出下一個排序後的母音
* 內存效率：只存儲必要的母音字母而非整個字符串，空間使用最優化
* 時間複雜度：O(n + v log v)（字符串遍歷 + 母音排序），空間複雜度：O(v)（僅存儲母音字母）

### Vowels Game in a String (p3227)

* 查表法 + 博弈論分析：通過預計算查找表實現 O(1) 母音檢測，結合博弈論分析得出核心規律
* 核心洞察：Alice 獲勝當且僅當字符串包含至少一個母音，這是博弈論分析的關鍵結果
* 查表優化：使用編譯時常數數組 VOWELS[256] 預存所有字符的母音標記，避免運行時比較操作
* 提前終止機制：一旦找到第一個母音立即返回 true，平均情況下只需檢查一半字符
* 博弈分析：無母音時 Alice 無法移動；有母音時 Alice 總能通過策略性移除獲勝
* 時間複雜度：O(n) 最壞情況，O(1) 最佳情況，空間複雜度：O(1)（常數查找表）

### Find Most Frequent Vowel and Consonant (p3541)

* 陣列計數法 + 字節處理優化：使用固定大小陣列進行字符頻率統計，避免 HashMap 的動態分配開銷
* 核心策略：分離統計母音和子音頻率，各自維護獨立的 26 元素陣列進行計數
* 字節級處理：使用 `s.bytes()` 直接處理字節，避免 UTF-8 字符解碼的額外開銷
* 索引映射優化：通過 `(char - b'a') as usize` 將字符直接映射到陣列索引，實現 O(1) 訪問
* 最大值查找：利用 `into_iter().max().unwrap_or(0)` 高效找出各類別的最大頻率
* 緩存友好：連續的陣列訪問模式提供更好的緩存局部性，優於 HashMap 的散列訪問
* 時間複雜度：O(n)（字符串遍歷），空間複雜度：O(1)（固定大小陣列）
