# discrete-math-ai999x — offline, verified discrete math solver with step-by-step answers

[![tests](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/discrete-math-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/discrete-math-ai999x)](https://pypi.org/project/discrete-math-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/discrete-math-ai999x)

**[Install](https://pypi.org/project/discrete-math-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/discrete-math-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/discrete-math-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

discrete-math-ai999x is an offline, deterministic Python solver that answers discrete math questions with verified results from 99 certified capabilities, gives 538 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**discrete-math-ai999x** is a compact, deterministic discrete math engine for Python. It reads a question in plain English (or a typed call), picks a certified discrete math capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified discrete math capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 538 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** number theory, modular arithmetic, combinatorics, graph structure, propositional logic and the mathematics of cryptography.

## Model overview

| Property | Value |
|---|---|
| Package | `discrete-math-ai999x` 1.0.0 (import `discrete_math_ai999x`) |
| Type | Deterministic neuro-symbolic discrete math engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 99 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 538 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

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

Command line: `discrete-math-ai999x "<question>"`, `discrete-math-ai999x --capabilities`. The outputs above were produced by running this exact code on discrete-math-ai999x 1.0.0.

## Find the right capability, and draw it

`discrete_math.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "discrete-math-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import discrete_math_ai999x as discrete_math

discrete_math.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

99 certified procedures. A few of them:

- **Procedures:** Affine cipher; Articulation points (cut vertices); Base (radix) conversion; Bell numbers; Bernoulli numbers; Bipartiteness test; Bracelets (rotations and reflections identified); Breadth-first search order; Bridges (cut edges); Caesar (shift) cipher; Carmichael function; Catalan numbers; Chinese remainder theorem; Chromatic number of a small graph; Clique number; Coefficient in a binomial expansion; Combinations C(n, k); Combinations with repetition (multisets); Connected components; Continued fraction of a rational number; Continued fraction of a square root; Contradiction test; Counting multiples by inclusion-exclusion; Cycle detection in a directed graph …

`discrete_math_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified discrete math capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (number theory, modular arithmetic, combinatorics, graph structure, propositional logic and the mathematics of cryptography); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{discrete_math_ai999x_1_0_0,
  title  = {discrete-math-ai999x: an offline, verified discrete math solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/discrete-math-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/discrete-math-ai999x/discussions or visit https://ai999x.com.

## Support the project

discrete-math-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/discrete-math-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/discrete-math-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/discrete-math-ai999x/tree/main/onnx).
