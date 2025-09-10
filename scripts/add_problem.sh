#!/bin/bash

# LeetCode Problem Generator Script
# Usage: ./scripts/add_problem.sh <number> <title> <difficulty>
# Example: ./scripts/add_problem.sh 1 "Two Sum" easy

set -e

if [ $# -lt 3 ]; then
    echo "Usage: $0 <number> <title> <difficulty>"
    echo "Example: $0 1 \"Two Sum\" easy"
    echo "Difficulty options: easy, medium, hard"
    exit 1
fi

PROBLEM_NUM=$1
PROBLEM_TITLE=$2
DIFFICULTY=$3

# Validate difficulty
if [[ ! "$DIFFICULTY" =~ ^(easy|medium|hard)$ ]]; then
    echo "Error: Difficulty must be 'easy', 'medium', or 'hard'"
    exit 1
fi

# Convert title to snake_case
SNAKE_CASE=$(echo "$PROBLEM_TITLE" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/_/g' | sed 's/__*/_/g' | sed 's/^_\|_$//g')
FILE_NAME="p${PROBLEM_NUM}_${SNAKE_CASE}.rs"

# Create file path
PROBLEM_DIR="src/problems/${DIFFICULTY}"
PROBLEM_FILE="${PROBLEM_DIR}/${FILE_NAME}"

# Check if file already exists
if [ -f "$PROBLEM_FILE" ]; then
    echo "Error: Problem file $PROBLEM_FILE already exists"
    exit 1
fi

# Create problem file content
cat > "$PROBLEM_FILE" << EOF
/// ${PROBLEM_NUM}. ${PROBLEM_TITLE}
///
/// 題目描述：
/// [請填入完整的中文題目描述，包括問題陳述、要求和注意事項]
///
/// 示例 1：
/// 輸入：[輸入範例]
/// 輸出：[輸出範例]
/// 解釋：[詳細解釋解決過程]
///
/// 示例 2：
/// 輸入：[輸入範例]
/// 輸出：[輸出範例]  
/// 解釋：[詳細解釋解決過程]
///
/// 限制條件：
/// - [限制條件 1]
/// - [限制條件 2]
/// - [其他相關限制]
#[allow(dead_code)]
impl Solution {
    pub fn solution() {
        todo!("實現 ${PROBLEM_TITLE} 的解決方案 - 請先理解題目和測試案例")
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
        // todo: 根據題目示例添加測試，使用 Solution::function_name() 格式
        // 範例: assert_eq!(Solution::function_name(input), expected_output);
    }
    
    #[test]
    fn test_edge_cases() {
        // 邊界測試案例
        // todo: 添加邊界條件測試（空輸入、單元素等），使用 Solution::function_name() 格式
    }
    
    #[test]
    fn test_corner_cases() {
        // 特殊情況測試案例  
        // todo: 添加針對此題目的特殊情況測試，使用 Solution::function_name() 格式
    }
}
EOF

# Update mod.rs to include the new problem
MOD_FILE="${PROBLEM_DIR}/mod.rs"
if grep -q "pub mod $FILE_NAME" "$MOD_FILE" 2>/dev/null; then
    echo "Module already declared in $MOD_FILE"
else
    # Add module declaration (remove .rs extension)
    MODULE_NAME=${FILE_NAME%.rs}
    echo "pub mod $MODULE_NAME;" >> "$MOD_FILE"
    echo "Added module declaration to $MOD_FILE"
fi

echo "✅ Created problem file: $PROBLEM_FILE"
echo ""
echo "📋 Next steps:"
echo "1. 📝 Fill in complete Chinese problem description"
echo "2. 📖 Add detailed examples with explanations"
echo "3. 🧪 Create comprehensive test cases (basic/edge/corner cases)"
echo "4. ❌ DO NOT implement solution yet - use todo!() placeholder"
echo "5. 💭 Focus on understanding the problem thoroughly first"
echo ""
echo "💡 Remember: Problem description and tests first, implementation later!"