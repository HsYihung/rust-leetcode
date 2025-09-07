# Claude Code Configuration

This file helps Claude understand your project and work more effectively.

## Project Overview
This is a Rust project for LeetCode problem solutions.

## Commands
- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Adding New Problems
Use the script to add new LeetCode problems:
```bash
./scripts/add_problem.sh <number> "<title>" <difficulty>
```

Example:
```bash
./scripts/add_problem.sh 1 "Two Sum" easy
./scripts/add_problem.sh 42 "Trapping Rain Water" hard
```

The script will:
- Create a new problem file in the appropriate difficulty folder
- Add module declaration to mod.rs
- Generate template with problem structure

## Problem Creation Rules
When creating new LeetCode problems:
- **DO NOT** implement the solution directly
- **DO** provide comprehensive Chinese problem description
- **DO** include detailed examples with explanations
- **DO** create comprehensive test cases covering:
  - Basic examples from problem statement
  - Edge cases (empty input, single element, etc.)
  - Boundary conditions (min/max values)
  - Corner cases specific to the problem
- Use `todo!()` macro as placeholder for solution
- Focus on understanding the problem first, then implement separately

## Problem Solving Assistance Rules
When user is working on implementing solutions:
- **DO** provide hints and algorithmic approaches when asked
- **DO** suggest relevant data structures and algorithms
- **DO** explain time and space complexity considerations
- **DO** help debug implementation issues
- **DO** provide step-by-step guidance for complex algorithms
- **DO** offer alternative solution approaches
- **DO** help optimize existing implementations
- Only provide direct code when explicitly requested or when user is stuck

## Custom Commands
Use these custom Claude Code commands for problem solving:

### Problem Setup Command
- **/leetcode --problem \<number\>**: Automatically fetch LeetCode problem info and create template with Chinese description

### Problem Solving Commands  
- **/hints**: Provide problem-solving approach, algorithm hints, data structure suggestions, and step-by-step thinking process WITHOUT giving away the complete solution
- **/solution**: Provide complete, well-commented solution implementation with detailed explanation of the approach
- **/big-o**: Analyze and explain time/space complexity of the problem or current solution, including best/average/worst case scenarios

### Test Verification Command
- **/test-verify**: Check if test cases are correct and comprehensive, including boundary conditions coverage and test quality analysis

### Solution Submission Command
- **/submit**: Complete solution submission workflow - runs tests, formats code, checks with clippy, updates README.md, and commits changes

## Commit Rules
**IMPORTANT**: Before every commit:
- **MUST** update README.md to reflect current project status
- **MUST** include newly added problems in the README problem list
- **MUST** update problem counts and statistics
- **MUST** maintain consistent formatting in the README
- The README should always be current and comprehensive

### README Solution Techniques Section Rules:
- **If algorithm is completed**: Include detailed solution explanation and approach
- **If algorithm is incomplete/uses todo!()**: Mark as "待實現" and DO NOT include solution details
- Keep the section clean and only document actually implemented solutions

## Project Structure
- `src/` - Source code directory
- `Cargo.toml` - Project dependencies and metadata

## Notes
- Follow Rust conventions and best practices
- Include unit tests for solutions when appropriate