"""discrete-math-ai999x — offline discrete math answers with their method, steps and verification.

    import discrete_math_ai999x as m
    m.ask("Find the inverse of 3 modulo 11.").show()                 # inverse = 4
    m.ask("How many derangements of 5 objects are there?").show()    # derangements = 44
    m.solve("chromatic_number", edges="A-B, B-C, C-A, C-D").show()   # chromatic_number = 3
    m.capabilities()                   # every certified capability

Number theory (gcd and Bezout, primality, factorization, totient, divisors, continued fractions, Pell's equation,
sums of two squares), modular arithmetic (powers, inverses, linear congruences, Chinese remainder theorem, discrete
logarithms, Legendre/Jacobi symbols, square roots, orders, primitive roots), combinatorics (exact counting, Stirling,
Bell, Catalan, partitions, stars and bars, inclusion-exclusion, pigeonhole, necklaces, Bernoulli numbers), graph
structure (degrees, components, BFS/DFS orders, diameter, topological sort, cycles, bipartiteness, Euler and Hamilton,
colouring, spanning-tree counts, cut vertices and bridges, cliques, Prüfer codes), propositional logic (evaluation,
truth tables, satisfiability, tautologies, equivalence, minimal DNF/CNF) and the mathematics of cryptography (RSA,
Diffie-Hellman, ElGamal, Caesar, affine and Vigenère ciphers). Exact integer arithmetic; orders and paths follow the
tie rules stated in each capability. Built by the AI999X model machine from the same engine as every domain; a
question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.3"
_P = Product("discrete_math", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
