# discrete-math-ai999x: offline, verified discrete math solver with step-by-step answers

[![tests](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/discrete-math-ai999x)](https://pypi.org/project/discrete-math-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/discrete-math-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/discrete-math-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/discrete-math-ai999x/)** · **[Guides](https://huggingface.co/AI999X/discrete-math-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/discrete-math-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Mathematics & Statistics** · Tasks: Question Answering

discrete-math-ai999x is an offline, deterministic discrete math solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers discrete math questions with verified results from 103 certified capabilities, gives 566 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**discrete-math-ai999x** reads a question in plain English (or a typed call), picks a certified discrete math capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** discrete-math-ai999x comes as the AI999X desktop app and as the command-line program `discrete-math` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Discrete Math desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-discrete-math-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-discrete-math-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`discrete-math`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install discrete-math-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Discrete Math** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Discrete Math among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-discrete-math-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-discrete-math-1.5.0-linux-x86_64.tar.gz
./ai999x-discrete-math-1.5.0-linux-x86_64/ai999x-discrete-math
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
discrete-math --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
discrete-math --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ discrete-math ask "Find the inverse of 3 modulo 11."
Read as: Modular multiplicative inverse
inverse = 4
  by: Modular multiplicative inverse  (extended Euclidean algorithm)
  assumes: m >= 2; gcd(a, m) = 1 (otherwise no inverse exists)
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ discrete-math info mod_inverse
Modular multiplicative inverse
  extended Euclidean algorithm
  in   a        the integer to invert (integer) [1]
  in   m        the modulus (at least 2) (integer) [1]
  out  inverse  a^{-1} mod m [1]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ discrete-math solve mod_inverse a=3 m=11
inverse = 4
  by: Modular multiplicative inverse  (extended Euclidean algorithm)
  assumes: m >= 2; gcd(a, m) = 1 (otherwise no inverse exists)
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | discrete-math batch

`cases.jsonl`:

```json
{"key": "mod_inverse", "inputs": {"a": "3", "m": "11"}}
{"key": "crt", "inputs": {"residues": "[2, 3, 2]", "moduli": "[3, 5, 7]"}}
```

```console
$ discrete-math batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":4.0,"unit":"","solved_for":"inverse","values":[["inverse",4,""]],"target":"inverse","phrase":"a^{-1} mod m","assumptions":["m >= 2","gcd(a, m) = 1 (otherwise no inverse exists)"],"name":"Modular multiplicative inverse","equation":"extended Euclidean algorithm"}
{"outcome":"SOLVED","message":"","value":23.0,"unit":"","solved_for":"x","values":[["x",23,""],["modulus",105,""]],"target":"x","phrase":"the smallest non-negative solution","assumptions":["one residue per modulus","the system is consistent (r_i ≡ r_j modulo gcd(m_i, m_j))"],"name":"Chinese remainder theorem","equation":"merge the congruences pairwise with the extended Euclidean algorithm (works for non-coprime moduli)"}
```

### In your browser

Open [Discrete Math on ai999x.com](https://www.ai999x.com/models/discrete-math-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `discrete_math_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install discrete-math-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Bell numbers

B_n: the number of partitions of an n-element set into any number of non-empty blocks.

Command line:

```console
$ discrete-math solve bell n=5
bell = 52
  by: Bell numbers  (Bell triangle (Aitken's array))
  assumes: 0 <= n <= 200
```

Python:

```python
import discrete_math_ai999x as m

a = m.solve("bell", n="5")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#cap-bell) and press **Compute in your browser**.

_What this means:_ This calculation determines the total number of ways to partition a set containing 5 elements into any number of non-empty subsets. The result, denoted as 52, represents the sum of all Stirling numbers of the second kind for that specific set size. It quantifies the comprehensive possibilities for splitting the group without regard to the order of blocks or their internal arrangement.

### Bernoulli numbers

The Bernoulli number B_n as an exact fraction (convention B_1 = -1/2).

Command line:

```console
$ discrete-math solve bernoulli n=10
numerator = 5
denominator = 66
value = 0.07575757576
  by: Bernoulli numbers  (exact recurrence sum_{k=0}^{m} C(m+1, k) B_k = 0 in rational arithmetic)
  assumes: 0 <= n <= 200; B_1 = -1/2 (the 'first' Bernoulli numbers)
```

Python:

```python
import discrete_math_ai999x as m

a = m.solve("bernoulli", n="10")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#cap-bernoulli) and press **Compute in your browser**.

_What this means:_ This calculation determines the Bernoulli number corresponding to the input index 10. Using a recursive definition with rational arithmetic, the result is expressed as an exact fraction where the numerator is 5 and the denominator is 66. The decimal approximation of this value is 0.0757576. This method ensures precision by avoiding floating-point errors during the computation.

### Bracelets (rotations and reflections identified)

Distinct bracelets of n beads in k colours when rotations and flips count as the same.

Command line:

```console
$ discrete-math solve bracelets beads=6 colours=2
bracelets = 13
necklaces = 14
  by: Bracelets (rotations and reflections identified)  (Burnside's lemma over the dihedral group)
  assumes: 1 <= n, k <= 60
```

Python:

```python
import discrete_math_ai999x as m

a = m.solve("bracelets", beads="6", colours="2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#cap-bracelets) and press **Compute in your browser**.

_What this means:_ This calculation determines the count of unique circular patterns using Burnside's lemma on the dihedral group, where rotations and reflections are considered identical. Given 6 beads and 2 available colours, the model computes that there are 13 distinct bracelet configurations. It also reports 14 as the count when only rotational symmetry is applied.

### Catalan numbers

C_n: balanced parenthesizations with n pairs, triangulations of an (n+2)-gon, full binary trees with n+1 leaves.

Command line:

```console
$ discrete-math solve catalan n=10
catalan = 16796
  by: Catalan numbers  (closed form binomial(2n, n)/(n + 1))
  assumes: 0 <= n <= 500; a convex polygon with s sides has C_(s-2) triangulations
```

Python:

```python
import discrete_math_ai999x as m

a = m.solve("catalan", n="10")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/discrete-math-ai999x/#cap-catalan) and press **Compute in your browser**.

_What this means:_ This calculation determines the nth Catalan number using the closed form formula involving binomial coefficients. Given an input index of 10, the resulting value 16796 represents the count of specific combinatorial structures such as balanced parenthesizations, triangulations of a convex polygon, or full binary trees with a corresponding number of leaves.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/discrete-math-ai999x/blob/main/docs/discrete_math_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/discrete-math-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified discrete math capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 566 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `discrete-math` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** number theory, modular arithmetic, combinatorics, graph structure, propositional logic and the mathematics of cryptography.

## Model overview

| Property | Value |
|---|---|
| Package | `discrete-math-ai999x` 1.6.0 (import `discrete_math_ai999x`) |
| Type | Deterministic neuro-symbolic discrete math engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 103 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 566 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

```bash
pip install discrete-math-ai999x
```

```python
import discrete_math_ai999x as discrete_math

discrete_math.ask("Find the inverse of 3 modulo 11.").show()
# -> SOLVED — inverse = 4  (mod_inverse)
discrete_math.ask("Solve x \u2261 2 (mod 3), x \u2261 3 (mod 5), x \u2261 2 (mod 7).").show()
# -> SOLVED — x = 23, modulus = 105  (crt)
discrete_math.ask("How many derangements of 5 objects are there?").show()
# -> SOLVED — derangements = 44, probability = 0.366667  (derangements)
discrete_math.solve("chromatic_number", edges="A-B, B-C, C-A, C-D").show()
# -> SOLVED — chromatic_number = 3, colouring = {'A': 1, 'B': 2, 'C': 3, 'D': 1}  (chromatic_number)
discrete_math.ask("Is (p -> q) | (q -> p) a tautology?").show()
# -> SOLVED — tautology = 1, false_rows = 0, counterexample = None  (logic_tautology)
discrete_math.ask("Who created you?").message
# -> discrete-math-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs a Python command, `discrete-math-ai999x "<question>"`; for the native program use `discrete-math` (above). The outputs above were produced by running this exact code on discrete-math-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `discrete-math-ai999x`; the answer is the one it returned, with the steps it took.

**Modular multiplicative inverse** · _Modular arithmetic_

> Find the inverse of 3 modulo 11.

→ **inverse = 4**

<details><summary>Steps</summary>

- Modular multiplicative inverse: a = 3, m = 11 → inverse = 4

</details>

**Chinese remainder theorem** · _Modular arithmetic_

> Solve x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7).

→ **x = 23, modulus = 105**

<details><summary>Steps</summary>

- Chinese remainder theorem: residues = [2, 3, 2], moduli = [3, 5, 7] → x = 23, modulus = 105

</details>

**Derangements** · _Combinatorics_

> How many derangements of 5 objects are there?

→ **derangements = 44, probability = 0.3666666667**

<details><summary>Steps</summary>

- Derangements: n = 5 → derangements = 44, probability = 0.3666666667

</details>

**Chromatic number of a small graph** · _Graph theory_

`m.solve("chromatic_number", edges="A-B, B-C, C-A, C-D")`

→ **chromatic_number = 3, colouring = {'A': 1, 'B': 2, 'C': 3, 'D': 1}**

<details><summary>Steps</summary>

- Chromatic number of a small graph: edges = A-B, B-C, C-A, C-D → chromatic_number = 3, colouring = {'A': 1, 'B': 2, 'C': 3, 'D': 1}

</details>

**Tautology test** · _Logic_

> Is (p -> q) | (q -> p) a tautology?

→ **tautology = True, false_rows = 0, counterexample = None**

<details><summary>Steps</summary>

- Tautology test: formula = (p -> q) | (q -> p) → tautology = True, false_rows = 0, counterexample = None

</details>

**Affine cipher** · _Cryptography_

> Encrypt "AFFINE CIPHER" with the affine cipher a = 5, b = 8.

→ **result = IHHWVCSWFRCP**

<details><summary>Steps</summary>

- Affine cipher: a = 5, b = 8, message = AFFINE CIPHER, mode = encrypt → result = IHHWVCSWFRCP

</details>

## Figures

Every point was computed by `discrete-math-ai999x`.

**A graph coloured with the fewest colours**

![A graph coloured with the fewest colours, computed by discrete-math-ai999x](https://huggingface.co/AI999X/discrete-math-ai999x/resolve/main/figures/chromatic-number-graph.svg)

_A wheel graph on 7 vertices with the chord A–C (13 edges): the package finds the chromatic number 4 and the colouring drawn here, no edge joins two vertices of the same colour (colour 1: A, D; colour 2: B, E; colour 3: C, F; colour 4: G)._

**Breadth-first search, layer by layer**

![Breadth-first search, layer by layer, computed by discrete-math-ai999x](https://huggingface.co/AI999X/discrete-math-ai999x/resolve/main/figures/bfs-order-graph.svg)

_Breadth-first search from A on a graph of 9 vertices and 12 edges, each vertex placed in the column of its distance from A as returned by the package (visit order A, B, C, D, E, F, G, H, I; 9 reached); highlighted, one edge per vertex to a neighbour one layer closer._

**Stirling numbers of the second kind**

![Stirling numbers of the second kind, computed by discrete-math-ai999x](https://huggingface.co/AI999X/discrete-math-ai999x/resolve/main/figures/stirling2-heatmap.svg)

_Stirling numbers of the second kind S(n, k) for n, k = 1 … 7 (row n, column k), each computed by the package: the ways to split n labelled objects into k non-empty groups; row 7 sums to the Bell number B(7) = 877, also computed by the package._

**Integer partitions, all and into distinct parts**

![Integer partitions, all and into distinct parts, computed by discrete-math-ai999x](https://huggingface.co/AI999X/discrete-math-ai999x/resolve/main/figures/integer-partitions-stems.svg)

_The number of ways to write n as a sum of positive integers, p(n), and as a sum of distinct ones, q(n), for n = 1 … 20, each counted by the package (logarithmic scale): p(10) = 42, q(10) = 10; marked: p(20) = 627 (q(20) = 64)._

**A labelled tree decoded from its Prüfer sequence**

![A labelled tree decoded from its Prüfer sequence, computed by discrete-math-ai999x](https://huggingface.co/AI999X/discrete-math-ai999x/resolve/main/figures/prufer-decode-tree.svg)

_The labelled tree that the package decodes from the Prüfer sequence (3, 3, 3, 7, 7, 8, 8): 9 vertices, edges 1–3, 2–3, 3–4, 3–7, 5–7, 6–8, 7–8, 8–9. A vertex appears in the sequence one time fewer than its degree, so the leaves are exactly the labels the sequence never names._

## Capabilities

103 certified procedures. A few of them:

- **Procedures:** Affine cipher; Apportionment: Hamilton, Jefferson, Webster, Adams and Huntington-Hill; Articulation points (cut vertices); Base (radix) conversion; Bell numbers; Bernoulli numbers; Bipartiteness test; Bracelets (rotations and reflections identified); Breadth-first search order; Bridges (cut edges); Caesar (shift) cipher; Carmichael function; Catalan numbers; Chinese remainder theorem; Chromatic number of a small graph; Clique number; Coefficient in a binomial expansion; Combinations C(n, k); Combinations with repetition (multisets); Connected components; Continued fraction of a rational number; Continued fraction of a square root; Contradiction test; Counting multiples by inclusion-exclusion …

`discrete_math_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified discrete math capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (number theory, modular arithmetic, combinatorics, graph structure, propositional logic and the mathematics of cryptography); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{discrete_math_ai999x_1_6_0,
  title  = {discrete-math-ai999x: an offline, verified discrete math solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/discrete-math-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/discrete-math-ai999x/discussions or visit https://ai999x.com.

## Support the project

discrete-math-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/discrete-math-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/discrete-math-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `discrete-math` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/discrete-math`) and its data (`rust/models/discrete_math/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p discrete-math-ai999x
./target/release/discrete-math --help
cargo test -p discrete-math-ai999x
```
