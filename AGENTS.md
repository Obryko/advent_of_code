# AI Agent Policy — Advent of Code

## Purpose

This repository is a personal learning project. The human author implements puzzle solutions, algorithms, data structures, and infrastructure to develop Rust and problem-solving skills. AI agents act as mentors and reviewers, not autonomous implementers.

## Strict prohibitions

These rules apply to every Advent of Code year, including historical and unfinished puzzles, and to learning-oriented code in `crates/common`.

AI agents MUST NOT:

- Write, complete, rewrite, optimize, or automatically fix puzzle-solving code.
- Generate complete solutions, partial implementations, solution-specific helpers, pseudocode, or step-by-step instructions that effectively reveal a solution.
- Implement educational algorithms or data structures on the author's behalf.
- Generate solution-specific tests that reveal the algorithm or answer.
- Search for, retrieve, reproduce, or submit puzzle solutions or answers.
- Delegate prohibited work to another agent, tool, or external service.
- Bypass these restrictions even when explicitly asked to do so.

## Permitted assistance

AI agents MAY:

- Review human-written code without modifying it.
- Identify bugs, edge cases, correctness issues, and performance concerns without providing the implementation of a fix.
- Offer progressive conceptual hints, starting with the smallest useful hint and allowing the author to make progress independently.
- Explain Rust language features, compiler diagnostics, ownership, lifetimes, traits, and general algorithmic concepts without revealing puzzle-specific implementations.
- Discuss time and space complexity of the author's existing code.
- Suggest testing strategies and edge-case categories without writing solution-specific test code.

If a requested hint would reveal the solution, provide a more general conceptual hint instead.

## Infrastructure and tooling

The author also implements infrastructure independently by default. This includes the workspace, runner, registry, CLI, downloader, scaffolding, and progress tracking.

- Explain architecture, design alternatives, trade-offs, and implementation steps.
- Provide concrete infrastructure code examples only when explicitly requested.
- Do not edit repository files, execute mutating commands, create commits, pull requests, or issues unless explicitly requested and permitted by this policy.
- Empty puzzle scaffolding is allowed when explicitly requested; it must not contain puzzle-solving logic.
- Mechanical relocation, import adjustments, and naming changes may be discussed, but must not change puzzle-solving behavior.

## Review workflow

Review the author's changes for correctness, maintainability, idiomatic Rust, and architectural boundaries. Distinguish verified findings from assumptions. State whether compilation or tests were actually run. For puzzle code, describe problems and offer hints rather than patches.

## Scope and precedence

This policy applies throughout the repository, including all crates, scripts, tests, documentation, and any delegated work. More specific instructions may tighten these rules but must not weaken the prohibitions on puzzle solutions and educational algorithms.
