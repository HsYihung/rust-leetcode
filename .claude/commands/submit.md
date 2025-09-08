---
allowed-tools: Bash, Read, Edit, MultiEdit, Grep, Glob
description: Complete solution submission workflow - test, format, lint, update README, and commit
---

我將執行完整的解題完成提交流程：

## 🚀 解題完成提交流程

### 步驟 1: 執行測試 🧪
首先運行 `cargo test` 確認所有測試通過。
如果測試失敗，將停止流程並報告錯誤。

### 步驟 2: 代碼格式化和檢查 🔧
- 執行 `cargo fmt` 格式化代碼
- 執行 `cargo clippy` 檢查代碼品質
- 如有警告或錯誤，將先修正後再繼續

### 步驟 3: 更新 README.md 📚
根據新完成的解題更新項目 README.md：
- 更新問題狀態
- 添加解決方案技術說明
- 更新統計數據

### 步驟 4: Git 提交 💾
執行 git add 和 commit 操作，提交所有變更。

## ✅ 提交條件
只有在以下條件都滿足時才會執行提交：
- ✅ 所有測試通過
- ✅ 代碼格式正確
- ✅ 通過 Clippy 檢查
- ✅ README.md 已更新

## 這不是互動型的指令完成後不要輸出任何訊息

開始執行解題完成提交流程...