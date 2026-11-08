# Recommendations (branch `recomendations`)

Review of the Week 1 code (Chapters 1–3) and the Week 2 practice project (Chapter 4). Every change has a `// WHY:` comment
in the code explaining the reason. Your original code stays untouched on `master`.

## How to study this branch

```powershell
git log --oneline master..recomendations   # one commit per project
git diff master recomendations -- 08-ch03-fibonacci # compare one project
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

## Week 2: `12-ch04-word_stats`

Your version works (after the three bugs you fixed). This version keeps the
same idea, byte loops and slices with no `split_whitespace`, but changes:

| Change | Lesson |
| --- | --- |
| Checks `read_line` returning 0 and `break`s | **Bug fix:** Ctrl+Z / Ctrl+D looped forever on "TYPE A PHRASE!" |
| `longest_word` remembers where each word *starts* and keeps the best word as a slice | No `acc` / `local_acc` / `index - local_acc` math, so the off-by-one can't happen |
| Loops `0..=bytes.len()`, treating the end as one more separator | The last word is handled inside the loop, so there's no duplicated check after it |
| `word_count` counts word *starts* | No `+ 1` at the end; an empty string gives 0 |
| `is_ascii_whitespace()` instead of `== b' '` | Tabs separate words too |
| One `String` buffer reused with `clear()` | Fewer allocations, and a real example of the 4.2 borrowing rules |
| `///` doc comments, no comments that repeat the code | Comments say what a function promises, not what each line does |
| `#[cfg(test)]` with a test per bug | `cargo test` catches the bugs you found by hand |

Compare: `git diff master recomendations -- 12-ch04-word_stats`

## Habits that make code look professional

Run these three before every commit (inside each project folder):

```powershell
cargo fmt        # formats the code the standard way
cargo clippy     # the Rust linter: catches non-idiomatic code
cargo test       # runs the tests (Chapter 11)
```

Clippy found 2 issues in your `master` code (late initialization in
`variables`, needless `return` in `fibonacci`). You've since fixed both. After `cargo fmt`, the diffs
on this branch are only real changes.

## Repo hygiene

- **Line endings (optional):** your Windows Git converts LF <-> CRLF
  automatically (`core.autocrlf`), so `git status` is clean on your machine.
  Other tools (Linux, CI, a WSL shell) see the CRLF files as modified. A
  `.gitattributes` with `* text=auto eol=lf` makes it consistent everywhere.
- **Commit messages:** describe what you did. "Refactor variable exercises..."
  was really "rustlings: complete variables 1–6".
- **Later (optional):** a Cargo workspace (one root `Cargo.toml` listing the
  projects) lets you run `cargo clippy --workspace` once for everything.

## Not changed on purpose

- `hello_world`, `hello_cargo`: nothing to improve.
- Rustlings: compare your answers with the official ones in
  `rustlings/solutions/` after you finish each exercise.
