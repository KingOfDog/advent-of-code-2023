<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2023/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2023/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2023/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2023/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2023/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2023/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2023/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2023/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2023/day/12) | ⭐ | ⭐ |
| [Day 13](https://adventofcode.com/2023/day/13) | ⭐ | ⭐ |
| [Day 14](https://adventofcode.com/2023/day/14) | ⭐ | ⭐ |
| [Day 15](https://adventofcode.com/2023/day/15) | ⭐ | ⭐ |
| [Day 16](https://adventofcode.com/2023/day/16) | ⭐ | ⭐ |
| [Day 17](https://adventofcode.com/2023/day/17) | ⭐ | ⭐ |
| [Day 18](https://adventofcode.com/2023/day/18) | ⭐ | ⭐ |
| [Day 19](https://adventofcode.com/2023/day/19) | ⭐ | ⭐ |
| [Day 20](https://adventofcode.com/2023/day/20) | ⭐ | ⭐ |
| [Day 21](https://adventofcode.com/2023/day/21) | ⭐ | ⭐ |
| [Day 22](https://adventofcode.com/2023/day/22) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
|          Solution           | Comment                                                             |      Time Part 1 |          Time Part 2 |
| :-------------------------: | ------------------------------------------------------------------- | ---------------: | -------------------: |
| [day01.rs](./src/bin/01.rs) | PTSD                                                                |           82.8μs |              481.6μs |
| [day02.rs](./src/bin/02.rs) | not beautiful, but it works                                         |           58.1μs |               56.9μs |
| [day03.rs](./src/bin/03.rs) | two dimensions are too much for me                                  |          515.4μs |              372.6μs |
| [day04.rs](./src/bin/04.rs) | parsing fun                                                         |          127.9μs |              126.7μs |
| [day05.rs](./src/bin/05.rs) | much code. too much code? never!                                    |          531.5μs | 12 700 000μs (12.7s) |
| [day06.rs](./src/bin/06.rs) | that was surprisingly painless                                      |  0.285μs (285ns) |      2 500μs (2.5ms) |
| [day07.rs](./src/bin/07.rs) | just copy paste everything for part 2 #cleancode                    |          335.3μs |              324.1μs |
| [day08.rs](./src/bin/08.rs) | nothing works without a bit of math                                 |   1100μs (1.1ms) |       4700μs (4.7ms) |
| [day09.rs](./src/bin/09.rs) | much faster than I expected                                         |          390.4μs |              390.8μs |
| [day10.rs](./src/bin/10.rs) |                                                                     |                  |                      |
| [day11.rs](./src/bin/11.rs) | that went surprisingly smoothly                                     | 28400μs (28.4ms) |     28700μs (28.7ms) |
| [day12.rs](./src/bin/12.rs) | bruteforce -> doesn't work -> think -> doesn't work -> u32 overflow |          796.4μs |       1500μs (1.5ms) |
| [day13.rs](./src/bin/13.rs) | relatively straightforward cmopared to the last few days            |          287.6μs |              288.5μs |
| [day14.rs](./src/bin/14.rs) | meh                                                                 |           56.8μs |     21000μs (21.0ms) |
| [day15.rs](./src/bin/15.rs) | that was surprisingly elegant                                       |           60.1μs |              231.0μs |
| [day16.rs](./src/bin/16.rs) | not the most beautiful code, but it works! #parallelization         |   1700μs (1.7ms) |     55200μs (55.2ms) |
| [day17.rs](./src/bin/17.rs) | maybe I should stop using μs as the default unit                    |         125300μs |             396800μs |
| [day18.rs](./src/bin/18.rs) | it might have helped to search for a simple formular early on...    |           69.8μs |               52.0μs |
| [day19.rs](./src/bin/19.rs) | worked pretty well                                                  | 70300μs (70.3ms) |     69400μs (69.4ms) |
| [day20.rs](./src/bin/20.rs) | part 2 relies on the structure of the input always being the same   | 12700μs (12.7ms) |     26600μs (26.6ms) |
| [day21.rs](./src/bin/21.rs) |                                                                     |                  |                      |
| [day22.rs](./src/bin/22.rs) |                                                                     |                  |                      |
| [day23.rs](./src/bin/23.rs) |                                                                     |                  |                      |
| [day24.rs](./src/bin/24.rs) |                                                                     |                  |                      |
| [day25.rs](./src/bin/25.rs) |                                                                     |                  |                      |

---

## Template setup

This template supports all major OS (macOS, Linux, Windows).

### Create your repository 📝

1.  Open [the template repository](https://github.com/fspoettel/advent-of-code-rust) on Github.
2.  Click [Use this template](https://github.com/fspoettel/advent-of-code-rust/generate) and create your repository.
3.  Clone your repository to your computer.
4.  If you are solving a previous year's advent of code, change the `AOC_YEAR` variable in `.cargo/config.toml` to reflect the year you are solving.

### Setup rust 💻

1.  Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2.  (recommended) Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3.  (optional) Install a native debugger. If you are using VS Code, [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a good option.

---

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like, you can configure [some optional features](#optional-template-features).

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solutions against the example input.

Tip: when editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input & description for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

By default, `solve` executes your code once and shows the execution time. If you append the `--time` flag to the command, the runner will run your code between `10` and `10.000` times (depending on execution time of first execution) and print the average execution time.

For example, running a benchmarked, optimized execution of day 1 would look like `cargo solve 1 --release --time`. Displayed _timings_ show the raw execution time of your solution without overhead like file reads.

#### Submitting solutions

> **Note**  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

In order to submit part of a solution for checking, append the `--submit <part>` option to the `solve` command.

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, `--release` controls whether real inputs will be used.

#### Update readme benchmarks

The template can output a table with solution times to your readme. Please note that these are not "scientific" benchmarks, understand them as a fun approximation. 😉

In order to generate a benchmarking table, run `cargo all --release --time`. If everything goes well, the command will output "_Successfully updated README with benchmarks._" after the execution finishes.

### Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

### Read puzzle description in terminal

> **Note**  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

## Optional template features

### Configure aoc-cli integration

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.12.0`
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]

Once installed, you can use the [download command](#download-input--description-for-a-day) and automatically submit solutions via the [`--submit` flag](#submitting-solutions).

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`.
-   `AOC_YEAR`: the year you want to track. Example: `2021`.
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

Go to the _Variables_ tab in your repository settings and create the following variable:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

### Check code formatting / clippy lints in CI

Uncomment the respective sections in the `ci.yml` workflow.

### Use VS Code to debug your code

1.  Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2.  Set breakpoints in your code. [^3]
3.  Click _Debug_ next to the unit test or the _main_ function. [^4]
4.  The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessred.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Common pitfalls

-   **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
[^3]:
    <img src="https://user-images.githubusercontent.com/1682504/198838369-453dc22c-c645-4803-afe0-fc50d5a3f00c.png" alt="Set a breakpoint" width="450" />

[^4]:
    <img alt="Run debugger" src="https://user-images.githubusercontent.com/1682504/198838372-c89369f6-0d05-462e-a4c7-8cd97b0912e6.png" width="450" />

[^5]:
    <img alt="Inspect debugger state" src="https://user-images.githubusercontent.com/1682504/198838373-36df6996-23bf-4757-9335-0bc4c1db0276.png" width="450" />
