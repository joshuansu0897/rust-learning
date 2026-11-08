# Learning Rust

My path through [The Rust Programming Language](https://doc.rust-lang.org/book/)
("the Book") and [Rustlings](https://github.com/rust-lang/rustlings), started on
October 31, 2026. I type every example by hand; AI is only my tutor.

## Progress

**The Book:** 4 of 21 chapters done

```text
Book       [████░░░░░░░░░░░░░░░░]  4 / 21 chapters
Rustlings  [██████░░░░░░░░░░░░░░] 30 / 94 exercises
```

| Ch | Chapter | Status |
| --- | --- | --- |
| 1 | [Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) | ✅ |
| 2 | [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) | ✅ |
| 3 | [Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) | ✅ |
| 4 | [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) | ✅ |
| 5 | [Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html) | ⏭️ next |
| 6 | [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) | |
| 7 | [Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) | |
| 8 | [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html) | |
| 9 | [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) | |
| 10 | [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) | |
| 11 | [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) | |
| 12 | [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) | |
| 13 | [Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html) | |
| 14 | [More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) | |
| 15 | [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) | |
| 16 | [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | |
| 17 | [Async and Await](https://doc.rust-lang.org/book/ch17-00-async-await.html) | |
| 18 | [Object Oriented Programming Features](https://doc.rust-lang.org/book/ch18-00-oop.html) | |
| 19 | [Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html) | |
| 20 | [Advanced Features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | |
| 21 | [Final Project: A Multithreaded Web Server](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) | |

## Projects

Folders are named `NN-chXX-name`: `NN` is the order I created them in and
`chXX` is the Book chapter they come from.

| # | Project | Book section | Created | What I practiced |
| --- | --- | --- | --- | --- |
| 01 | [hello_world](01-ch01-hello_world) | [1.2 Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html) | 2026-10-31 | Compiling with `rustc` by hand, `fn main`, the `println!` macro |
| 02 | [hello_cargo](02-ch01-hello_cargo) | [1.3 Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) | 2026-10-31 | `cargo new`, `build`, `run`, `check`, `--release`, `Cargo.toml` |
| 03 | [guessing_game](03-ch02-guessing_game) | [2 Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) | 2026-10-31 | Reading input, `String`, `Result` and `expect`, `match`, `loop`, the `rand` crate |
| 04 | [variables](04-ch03-variables) | [3.1 Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) | 2026-11-03 | `let` vs `let mut`, shadowing, constants |
| 05 | [functions](05-ch03-functions) | [3.3 Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) | 2026-11-04 | Parameters, statements vs expressions, return values |
| 06 | [control_flow](06-ch03-control_flow) | [3.5 Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) | 2026-11-05 | `if` as an expression, `loop` / `while` / `for`, loop labels, ranges |
| 07 | [fah_to_cel](07-ch03-fah_to_cel) | [Ch 3 exercises](https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary) | 2026-11-05 | Converting temperatures between Fahrenheit and Celsius |
| 08 | [fibonacci](08-ch03-fibonacci) | [Ch 3 exercises](https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary) | 2026-11-05 | Generating the nth Fibonacci number |
| 09 | [christmas_carol_lyrics](09-ch03-christmas_carol_lyrics) | [Ch 3 exercises](https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary) | 2026-11-05 | Printing "The Twelve Days of Christmas" with loops |
| 10 | [ownership](10-ch04-ownership) | [4.1 What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [4.2 References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) | 2026-11-06 | Stack vs heap, move vs `Copy`, `clone`, `drop`, `&` and `&mut`, the borrowing rules, dangling references |
| 11 | [slices](11-ch04-slices) | [4.3 The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html) | 2026-11-08 | String slices, range shortcuts, `&str` as a parameter, why a slice beats a `usize` index |
| 12 | [word_stats](12-ch04-word_stats) | Ch 4 practice project (mine, not from the Book) | 2026-11-08 | Reads phrases and prints the first word, the longest word and the word count, all as slices of the input |

## Rustlings

[`rustlings/`](rustlings) holds my Rustlings exercises. Done so far: intro,
variables, functions, if, quiz1, primitive_types, vecs and move_semantics.
Next up: structs.

## Running a project

```powershell
cd 03-ch02-guessing_game
cargo run

# 01-ch01-hello_world has no Cargo project:
cd 01-ch01-hello_world
rustc main.rs
.\main.exe
```

Before each commit: `cargo fmt`, `cargo clippy`, `cargo test`.

## Branches

- `master`: my code, exactly as I wrote it.
- `recomendations`: a reviewed version of some projects (Week 1 and
  `word_stats`), with comments explaining each change. See its `RECOMMENDATIONS.md`.
