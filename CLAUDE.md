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

## Problem Solving Commands
Use these keywords to get specific help:
- **解題參考** - Get algorithmic hints, approaches, and problem-solving strategies
- **解答** - Get complete solution implementation with explanation
- **Big O** - Analyze time and space complexity of current problem or solution

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

### Command Behaviors:
- **解題參考**: Provide problem-solving approach, algorithm hints, data structure suggestions, and step-by-step thinking process WITHOUT giving away the complete solution
- **解答**: Provide complete, well-commented solution implementation with detailed explanation of the approach
- **Big O**: Analyze and explain time/space complexity of the problem or current solution, including best/average/worst case scenarios

## Project Structure
- `src/` - Source code directory
- `Cargo.toml` - Project dependencies and metadata

## Notes
- Follow Rust conventions and best practices
- Include unit tests for solutions when appropriate