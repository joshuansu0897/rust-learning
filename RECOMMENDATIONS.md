# Recommendations (branch `recomendations`)

Review of the Week 1 code (Chapters 1–3). Every change has a `// WHY:` comment
in the code explaining the reason. Your original code stays untouched on `master`.

## How to study this branch

```powershell
git log --oneline master..recomendations   # one commit per project
git diff master recomendations -- fibonacci # compare one project
git switch recomendations                   # try the code
git switch master                           # back to your version
```

Suggested way to learn from it: read the diff for one project, close it, and
rewrite that improvement yourself on `master`. Don't just merge it.

## What changed, most important first

| Project | Change | Lesson |
| --- | --- | --- |
| guessing_game | Handles end of input (`read_line` returns 0) | **Bug fix.** Before, Ctrl+Z / Ctrl+D looped forever. Always ask "what if the input ends or is weird?" |
| guessing_game | Rejects numbers outside 1–100; counts attempts; `MIN`/`MAX` constants | Treat input as untrusted; no magic numbers |
| guessing_game | `rand::random_range` instead of `use rand::prelude::*` | Import only what you use |
| christmas_carol_lyrics | Generated from two arrays + nested loop, all 12 days | The real goal of the exercise: data in arrays, pattern in code |
| fibonacci | `n: u32 -> u64`, `for` loop, tuple assignment, no `return` | Let types reject bad input; i32 overflowed at fib(47), u64 reaches fib(93) |
| fah_to_cel | `f64`, `9.0 / 5.0`, a small table, tests with tolerance | Never compare floats with `==` |
| variables | `let x = 5;` instead of `let x; x = 5;`; shadowing vs `mut` type demo; a `const` | Clippy's "unneeded late initialization" |
| control_flow, functions | Comments and `cargo fmt` only | Already idiomatic |

## Habits that make code look professional

Run these three before every commit (inside each project folder):

```powershell
cargo fmt        # formats the code the standard way
cargo clippy     # the Rust linter: catches non-idiomatic code
cargo test       # runs the tests (Chapter 11)
```

Clippy found 2 issues in your `master` code (late initialization in
`variables`, needless `return` in `fibonacci`). After `cargo fmt`, the diffs
on this branch are only real changes.

## Repo hygiene

- **Line endings:** `primitive_types3.rs` shows as modified on `master` only
  because an editor changed LF to CRLF. Add a `.gitattributes` with
  `* text=auto eol=lf` to stop this.
- **Commit messages:** describe what you did. "Refactor variable exercises..."
  was really "rustlings: complete variables 1–6".
- **Later (optional):** a Cargo workspace (one root `Cargo.toml` listing the
  projects) lets you run `cargo clippy --workspace` once for everything.

## Not changed on purpose

- `hello_world`, `hello_cargo`: nothing to improve.
- Rustlings: compare your answers with the official ones in
  `rslings/rustlings/solutions/` after you finish each exercise.
