---
allowed-tools: WebSearch, WebFetch, Bash, Read, Edit, TodoWrite, MultiEdit
argument-hint: --problem <problem_number>
description: Automatically fetch LeetCode problem and create template with Chinese description
---

幫我執行完整的 LeetCode 問題 $2 自動化設置流程：

**步驟一** 🌐 使用 `scripts/lynx-slice.sh <$2>` 腳本建立 ./problem-src/$2.md 題目文檔

**步驟二** 🔍 從 ./problem-src/$2.md 分析題目內容，提取完整信息（標題、難度、描述、示例、限制條件）

**步驟三** 📝 使用 `./scripts/add_problem.sh <number> <title> <difficulty>` 生成樣板題目

**步驟四** 🈚 將英文內容翻譯為完整的中文描述，包括：
   - 詳細的問題陳述
   - 清晰的示例說明和解釋
   - 完整的限制條件
   - 完善的測試案例（基本、邊界、特殊情況）

**步驟五** 🔧 將解答函式中未使用到的參數加上 `_` 標記來通過 `cargo clippy`

**步驟六** 📚 更新 README.md，添加新問題到問題列表

**步驟七** 💾 使用 `git commit` 提交所有變更

請立即開始處理 LeetCode 問題 $2 的完整設置。