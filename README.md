# Discrete Math (discrete-math-ai999x)

[![tests](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/discrete-math-ai999x)](https://pypi.org/project/discrete-math-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/discrete-math-ai999x)

Discrete Math covers number theory, modular arithmetic, combinatorics, graph theory and network algorithms, logic and the mathematics of cryptography: 103 calculations in 9 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Discrete Math desktop app: Modular multiplicative inverse, with its worked example and live scene](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/hero.webp)](https://www.ai999x.com/models/discrete-math-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-discrete-math-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-discrete-math-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `discrete-math` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install discrete-math-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Discrete Math desktop app: every scene and chart is drawn from the computed answer.

[![Discrete Math desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/clip.mp4)*

| | |
|---|---|
| [![Modular multiplicative inverse: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-1.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-mod_inverse)<br>Modular multiplicative inverse | [![Derangements: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-2.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-derangements)<br>Derangements |
| [![Chromatic number of a small graph: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-3.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-chromatic_number)<br>Chromatic number of a small graph | [![Tautology test: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-4.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-logic_tautology)<br>Tautology test |
| [![Affine cipher: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-5.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-affine_cipher)<br>Affine cipher | [![Base (radix) conversion: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-6.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-base_convert)<br>Base (radix) conversion |
| [![Venn diagram with three sets (inclusion-exclusion): the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-7.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-venn_three_sets)<br>Venn diagram with three sets (inclusion-exclusion) | [![Apportionment: Hamilton, Jefferson, Webster, Adams and Huntington-Hill: the live scene in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-8.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-apportionment)<br>Apportionment: Hamilton, Jefferson, Webster, Adams and Huntington-Hill |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Modular multiplicative inverse: a chart in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/chart-1.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-mod_inverse)<br>Modular multiplicative inverse | [![Derangements: a chart in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/chart-2.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-derangements)<br>Derangements | [![Chromatic number of a small graph: a chart in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/chart-3.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-chromatic_number)<br>Chromatic number of a small graph | [![Tautology test: a chart in the Discrete Math desktop app](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/chart-4.webp)](https://www.ai999x.com/models/discrete-math-ai999x/#cap-logic_tautology)<br>Tautology test |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import discrete_math_ai999x as discrete_math

a = discrete_math.ask("What is the 5th Bell number?")
print(a.outcome, a.values)
# SOLVED {'bell': 52}

a = discrete_math.ask(
    "Encrypt \"AFFINE CIPHER\" with the affine cipher a = 5, b = 8."
)
print(a.outcome, a.values)
# SOLVED {'result': 'IHHWVCSWFRCP'}

a = discrete_math.ask("Is p & ~p a contradiction?")
print(a.outcome, a.values)
# SOLVED {'contradiction': True, 'true_rows': 0, 'witness': None}

a = discrete_math.ask(
    "Solve x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7)."
)
print(a.outcome, a.values)
# SOLVED {'x': 23, 'modulus': 105}

a = discrete_math.ask("Convert 1011 from base 2 to base 10.")
print(a.outcome, a.values)
# SOLVED {'result': '11', 'decimal': 11}

a = discrete_math.ask("Compute the Carmichael function of 15.")
print(a.outcome, a.values)
# SOLVED {'lambda': 4}

a = discrete_math.ask("What is the Bernoulli number B_10?")
print(a.outcome, a.values)
# SOLVED {'numerator': 5, 'denominator': 66, 'value': 0.07575757575757576}

a = discrete_math.ask(
    "Encrypt \"HELLO WORLD\" with a Caesar shift of 3."
)
print(a.outcome, a.values)
# SOLVED {'result': 'KHOORZRUOG'}

a = discrete_math.ask(
    "Is the graph with edges A-B, B-C, C-D, D-A bipartite?"
)
print(a.outcome, a.values)
# SOLVED {'is_bipartite': True}

a = discrete_math.ask(
    "Evaluate (p -> q) & ~r when p = true, q = false, r = false."
)
print(a.outcome, a.values)
# SOLVED {'value': False}

a = discrete_math.ask("Find the smallest x with 3^x ≡ 13 (mod 17).")
print(a.outcome, a.values)
# SOLVED {'x': 4}

a = discrete_math.ask("What is the 8-bit two's complement of -5?")
print(a.outcome, a.values)
# SOLVED {'bits_text': '11111011', 'unsigned': 251}

a = discrete_math.ask("Find the continued fraction of 415/93.")
print(a.outcome, a.values)
# SOLVED {'terms': [4, 2, 6, 7], 'length': 4}

a = discrete_math.ask(
    "Venn diagram: n(A) = 45, n(B) = 60, n(A and B) = 20, n(U) = 100. "
    "Find the count of every region."
)
print(a.outcome, a.values)
# SOLVED {'union': 85, 'only_a': 25, 'only_b': 40, 'neither': 15}

a = discrete_math.ask(
    "How many bracelets with 6 beads can be made using 2 colours?"
)
print(a.outcome, a.values)
# SOLVED {'bracelets': 13, 'necklaces': 14}
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Discrete Math on ai999x.com](https://www.ai999x.com/models/discrete-math-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/discrete-math-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/discrete-math-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `discrete-math` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/discrete-math`) and its data (`rust/models/discrete_math/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p discrete-math-ai999x
./target/release/discrete-math --help
cargo test -p discrete-math-ai999x
```
