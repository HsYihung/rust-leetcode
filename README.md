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
    │   ├── p1800_maximum_ascending_subarray_sum.rs
    │   └── p3174_clear_digits.rs
    ├── medium/
    └── hard/
```

## 題目列表

### Easy
| #    | 題目 | 解法 | 備註 |
|------|------|------|------|
| 1    | [Two Sum](src/problems/easy/p0001_two_sum.rs) | HashMap | O(n) 時間, O(n) 空間 |
| 1800 | [Maximum Ascending Subarray Sum](src/problems/easy/p1800_maximum_ascending_subarray_sum.rs) | 一次遍歷 | O(n) 時間, O(1) 空間 |
| 3174 | [Clear Digits](src/problems/easy/p3174_clear_digits.rs) | 一次遍歷 | O(n) 時間, O(1) 空間 |

### Medium
*待新增*

### Hard
*待新增*

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

## 性能測試框架

專案包含了一個簡單的性能測試框架（在 `src/utils/mod.rs`），可以測量：
- 執行時間（精確到納秒）
- 內存使用
- 不同輸入大小的擴展性

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
- HashMap 解法：O(n) 時間複雜度，使用額外空間來換取時間效率

### Maximum Ascending Subarray Sum
- 一次遍歷：O(n) 時間複雜度，不需要額外空間
