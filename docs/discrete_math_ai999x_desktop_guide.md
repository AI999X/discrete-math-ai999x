# Discrete Math: the desktop app, user guide

**Discrete Math**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Discrete Math covers number theory, modular arithmetic, combinatorics, graph theory and network algorithms, logic and the mathematics of cryptography. Its desktop app holds all 103 calculations in 9 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-discrete-math-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-discrete-math-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-discrete-math-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-discrete-math.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-discrete-math-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-discrete-math-1.6.5-linux-x86_64.tar.gz
./ai999x-discrete-math-1.6.5-linux-x86_64/ai999x-discrete-math
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Discrete Math app, opened on Modular multiplicative inverse with its worked example.](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/hero.webp)

*The Discrete Math app, opened on Modular multiplicative inverse with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Modular arithmetic › Modular multiplicative inverse (or type part of its name in the search box). It opens with a worked example filled in: “Find the inverse of 3 modulo 11.” The answer: inverse = 4.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Modular multiplicative inverse: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-1.webp)

*Modular multiplicative inverse: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the inverse of 3 modulo 11.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Modular multiplicative inverse
inverse = 4
  by: Modular multiplicative inverse  (extended Euclidean algorithm)
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Derangements](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-2.webp)

*Derangements*

![Chromatic number of a small graph](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-3.webp)

*Chromatic number of a small graph*

![Tautology test](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-4.webp)

*Tautology test*

![Affine cipher](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-5.webp)

*Affine cipher*

![Base (radix) conversion](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-6.webp)

*Base (radix) conversion*

![Venn diagram with three sets (inclusion-exclusion)](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-7.webp)

*Venn diagram with three sets (inclusion-exclusion)*

![Apportionment: Hamilton, Jefferson, Webster, Adams and Huntington-Hill](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/scene-8.webp)

*Apportionment: Hamilton, Jefferson, Webster, Adams and Huntington-Hill*

![Modular multiplicative inverse: a chart, enlarged.](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/chart-1.webp)

*Modular multiplicative inverse: a chart, enlarged.*

![Derangements: a chart, enlarged.](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/chart-2.webp)

*Derangements: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Discrete Math app in the dark theme.](https://www.ai999x.com/?ax_shot=discrete-math-ai999x/hero-dark.webp)

*The Discrete Math app in the dark theme.*

## 7. Every calculation

103 calculations in 9 topics; each has its own entry on the model's page (https://www.ai999x.com/models/discrete-math-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Combinatorics | Bell numbers, Bernoulli numbers, Bracelets (rotations and reflections identified), Catalan numbers, Coefficient in a binomial expansion, Combinations C(n, k), Combinations with repetition (multisets), Counting multiples by inclusion-exclusion, Derangements, Distinct arrangements of the letters of a word, Fibonacci numbers, Integer partitions p(n), Integer solutions of x1 + ... + xk = n (stars and bars), Lattice paths in a grid, Lucas numbers, Multinomial coefficient, Necklaces (rotations identified), Partitions into distinct parts, Partitions into exactly k parts, Permutations P(n, k), Pigeonhole principle: guaranteed load, Pigeonhole principle: objects needed for a guarantee, Sequences with repetition (n^k), Stirling numbers of the first kind, Stirling numbers of the second kind |
| Cryptography | Affine cipher, Caesar (shift) cipher, Diffie-Hellman key exchange, ElGamal encryption, RSA decryption, RSA decryption from p, q and e (CRT), RSA encryption, RSA key from p, q and e, Vigenère cipher |
| Graph theory | Articulation points (cut vertices), Bipartiteness test, Breadth-first search order, Bridges (cut edges), Chromatic number of a small graph, Clique number, Connected components, Cycle detection in a directed graph, Cycles in an undirected graph (cyclomatic number), Depth-first search order, Diameter, radius and center, Eulerian circuits and paths, Girth (shortest cycle length), Graphical degree sequence, Hamiltonian cycle of a small graph, Independence number, Number of spanning trees (Kirchhoff), Number of triangles, Number of walks of a given length, Prüfer code of a labelled tree, Strongly connected components, Topological sort, Tree from a Prüfer code, Tree test, Vertex degrees and degree sequence |
| Logic | Contradiction test, Evaluate a propositional formula, Logical equivalence of two formulas, Minimal product of sums (CNF), Minimal sum of products (DNF), Minimal sum of products from minterms (with don't-cares), Satisfiability of a propositional formula, Tautology test, Truth table and minterms |
| Modular arithmetic | Chinese remainder theorem, Discrete logarithm, Linear congruence ax ≡ b (mod m), Modular exponentiation, Modular multiplicative inverse, Multiplicative order, Remainder modulo m (Euclidean division), Smallest primitive root modulo a prime |
| Number representations | Base (radix) conversion, Two's complement representation |
| Number theory | Carmichael function, Continued fraction of a rational number, Continued fraction of a square root, Divisors, divisor count and divisor sum, Euler's totient function, Extended Euclidean algorithm (Bezout coefficients), Greatest common divisor and least common multiple, Integer square root and perfect squares, Jacobi symbol, Legendre symbol, Linear Diophantine equation ax + by = c, Möbius function, Next prime, Pell's equation (fundamental solution), Previous prime, Primality test, Prime factorization, Prime-counting function, Square roots modulo a prime, Sums of two squares, The n-th prime |
| Sets | Venn diagram with three sets (inclusion-exclusion), Venn diagram with two sets (inclusion-exclusion) |
| Voting and apportionment | Apportionment: Hamilton, Jefferson, Webster, Adams and Huntington-Hill, Voting methods: plurality, Borda count, instant runoff, pairwise comparisons |

## 8. Other ways to use it

The same engine runs on the command line (`discrete-math`, its own guide), in Python (`pip install discrete-math-ai999x`, `import discrete_math_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/discrete-math-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/discrete-math-ai999x/
