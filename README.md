# Discrete Math (discrete-math-ai999x)

[![tests](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/discrete-math-ai999x)](https://pypi.org/project/discrete-math-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/discrete-math-ai999x)

Discrete Math covers number theory, modular arithmetic, combinatorics, graph theory and network algorithms, logic and the mathematics of cryptography: 103 calculations in 9 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Discrete Math desktop app: Bell numbers, with its worked example and live scene](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/hero.webp)](https://www.ai999x.com/models/discrete-math-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `discrete-math` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#try) |
| **Python** 3.10+ | `pip install discrete-math-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import discrete_math_ai999x as discrete_math
a = discrete_math.ask("Find the inverse of 3 modulo 11.")
print(a.outcome, a.value)
```

Output:

```text
SOLVED 4
```

Every calculation, with its formula, a worked example and its figures: **[Discrete Math on ai999x.com](https://www.ai999x.com/models/discrete-math-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/discrete-math-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/discrete-math-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `discrete-math` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/discrete-math`) and its data (`rust/models/discrete_math/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p discrete-math-ai999x
./target/release/discrete-math --help
cargo test -p discrete-math-ai999x
```
