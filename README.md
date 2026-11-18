# Discrete Math (discrete-math-ai999x)

[![tests](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/discrete-math-ai999x)](https://pypi.org/project/discrete-math-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/discrete-math-ai999x)

Discrete Math covers number theory, modular arithmetic, combinatorics, graph theory and network algorithms, logic and the mathematics of cryptography: 103 calculations in 9 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Discrete Math desktop app: Modular multiplicative inverse, with its worked example and live scene](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/hero.webp)](https://www.ai999x.com/models/discrete-math-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-discrete-math-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-discrete-math-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `discrete-math` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
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

## Command line: 15 examples

Each command below was run with `discrete-math` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ discrete-math solve bell n=5
bell = 52

$ discrete-math solve affine_cipher a=5 b=8 "message=AFFINE CIPHER" mode=encrypt
result = IHHWVCSWFRCP

$ discrete-math solve articulation_points "edges=A-B, B-C, C-A, C-D, D-E"
count = 2
points = ["C","D"]

$ discrete-math solve logic_contradiction "formula=p & ~p"
contradiction = true
true_rows = 0
witness = null

$ discrete-math solve crt "residues=[2, 3, 2]" "moduli=[3, 5, 7]"
x = 23
modulus = 105

$ discrete-math solve base_convert digits=1011 from_base=2 to_base=10
result = 11
decimal = 11

$ discrete-math solve carmichael_lambda n=15
lambda = 4

$ discrete-math solve venn_three_sets a=30 b=25 c=20 ab=10 ac=8 bc=5 abc=3 universe=60
union = 55
only_a = 15
only_b = 13
only_c = 10
only_a_and_b = 7
only_a_and_c = 5

$ discrete-math solve apportionment method=jefferson seats=10 "populations=[2400, 3300, 4300]"
seats = [2,3,5]
standard_divisor = 1000
quotas = [2.4,3.3,4.3]
total_seats = 10

$ discrete-math solve bernoulli n=10
numerator = 5
denominator = 66
value = 0.07575757576

$ discrete-math solve caesar_cipher "message=HELLO WORLD" mode=encrypt shift=3
result = KHOORZRUOG

$ discrete-math solve bipartite "edges=A-B, B-C, C-D, D-A"
is_bipartite = true

$ discrete-math solve logic_evaluate "formula=(p -> q) & ~r" "assignment=p = true, q = false, r = false"
value = false

$ discrete-math solve linear_congruence a=3 b=4 m=7
smallest = 6
count = 1
step = 7

$ discrete-math solve twos_complement n=-5 bits=8
bits_text = 11111011
unsigned = 251
```

## Rust library: 15 examples

The crate `discrete-math-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `discrete_math_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/discrete-math/examples/readme.rs`: `cd rust && cargo run --release --example readme -p discrete-math-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&discrete_math_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "bell", "inputs": {"n": "5"}}));
// SOLVED
// bell = 52

show(json!({"key": "affine_cipher", "inputs": {"a": "5", "b": "8", "message": "AFFINE CIPHER", "mode": "encrypt"}}));
// SOLVED
// result = "IHHWVCSWFRCP"

show(json!({"key": "articulation_points", "inputs": {"edges": "A-B, B-C, C-A, C-D, D-E"}}));
// SOLVED
// count = 2
// points = ["C","D"]

show(json!({"key": "logic_contradiction", "inputs": {"formula": "p & ~p"}}));
// SOLVED
// contradiction = true
// true_rows = 0
// witness = null

show(json!({"key": "crt", "inputs": {"residues": [2, 3, 2], "moduli": [3, 5, 7]}}));
// SOLVED
// x = 23
// modulus = 105

show(json!({"key": "base_convert", "inputs": {"digits": "1011", "from_base": "2", "to_base": "10"}}));
// SOLVED
// result = "11"
// decimal = 11

show(json!({"key": "carmichael_lambda", "inputs": {"n": "15"}}));
// SOLVED
// lambda = 4

show(json!({"key": "venn_three_sets", "inputs": {"a": "30", "b": "25", "c": "20", "ab": "10", "ac": "8", "bc": "5", "abc": "3", "universe": "60"}}));
// SOLVED
// union = 55
// only_a = 15
// only_b = 13
// only_c = 10
// only_a_and_b = 7
// only_a_and_c = 5

show(json!({"key": "apportionment", "inputs": {"method": "jefferson", "seats": "10", "populations": [2400, 3300, 4300]}}));
// SOLVED
// seats = [2,3,5]
// standard_divisor = 1000.0
// quotas = [2.4,3.3,4.3]
// total_seats = 10

show(json!({"key": "bernoulli", "inputs": {"n": "10"}}));
// SOLVED
// numerator = 5
// denominator = 66
// value = 0.07575757575757576

show(json!({"key": "caesar_cipher", "inputs": {"message": "HELLO WORLD", "mode": "encrypt", "shift": "3"}}));
// SOLVED
// result = "KHOORZRUOG"

show(json!({"key": "bipartite", "inputs": {"edges": "A-B, B-C, C-D, D-A"}}));
// SOLVED
// is_bipartite = true

show(json!({"key": "logic_evaluate", "inputs": {"formula": "(p -> q) & ~r", "assignment": "p = true, q = false, r = false"}}));
// SOLVED
// value = false

show(json!({"key": "linear_congruence", "inputs": {"a": "3", "b": "4", "m": "7"}}));
// SOLVED
// smallest = 6
// count = 1
// step = 7

show(json!({"key": "twos_complement", "inputs": {"n": "-5", "bits": "8"}}));
// SOLVED
// bits_text = "11111011"
// unsigned = 251
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Discrete Math on ai999x.com](https://www.ai999x.com/models/discrete-math-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/discrete-math-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/discrete-math-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `discrete-math` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/discrete-math`) and its data (`rust/models/discrete_math/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p discrete-math-ai999x
./target/release/discrete-math --help
cargo test -p discrete-math-ai999x
```
