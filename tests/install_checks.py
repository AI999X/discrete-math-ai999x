"""discrete_math checks run by tools/clean_install.py inside a fresh environment (written by tools/new_domain.py from the certified procedures' documented questions)."""

SAMPLE = ('How many cut vertices does the graph with edges 1-2, 2-3, 3-4 have?', 2.0)
LATENCY = ["Find the articulation points of the graph with edges A-B, B-C, C-A, C-D, D-E.", "What is the 5th Bell number?", "What is the Bernoulli number B_10?"]
QUESTIONS = [['Find the articulation points of the graph with edges A-B, B-C, C-A, C-D, D-E.', {'points': ['C', 'D']}], ['What is the 5th Bell number?', {'bell': 52}], ['What is the Bernoulli number B_10?', {'numerator': 5, 'denominator': 66}], ['Give the breadth-first search order from A in the graph with edges A-B, A-C, B-D, C-D, D-E.', {'order': ['A', 'B', 'C', 'D', 'E']}], ['What is the coefficient of x^3 in (2x + 3)^5?', {'coefficient': 720}], ['Is the graph with edges A-B, B-C, C-D, D-A bipartite?', {'is_bipartite': True}], ['How many bracelets with 6 beads can be made using 2 colours?', {'bracelets': 13}], ['Find the bridges of the graph with edges A-B, B-C, C-A, C-D, D-E.', {'bridges': [['C', 'D'], ['D', 'E']]}], ['Compute the Carmichael function of 15.', {'lambda': 4}], ['What is the 10th Catalan number?', {'catalan': 16796}], ['Find the continued fraction of 415/93.', {'terms': [4, 2, 6, 7]}], ['Find the continued fraction of sqrt(23).', {'period': [1, 3, 1, 8]}], ['What is the chromatic number of the graph with edges A-B, B-C, C-A, C-D?', {'chromatic_number': 3}], ['What is the clique number of the graph with edges A-B, B-C, C-A, C-D, B-D, A-D, D-E?', {'clique_number': 4}], ['How many ways are there to choose 3 books from 10?', {'combinations': 120}], ['How many connected components does the graph with edges A-B, C-D, D-E have?', {'count': 2}], ['Solve x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7).', {'x': 23, 'modulus': 105}], ['What is the cyclomatic number of the graph with edges A-B, B-C, C-A, D-E?', {'cyclomatic_number': 1}], ['How many derangements of 5 objects are there?', {'derangements': 44}], ['Give the depth-first search order from A in the graph with edges A-B, A-C, B-D, C-D, D-E.', {'order': ['A', 'B', 'D', 'C', 'E']}], ['Diffie-Hellman with p = 23, g = 5, a = 6, b = 15: find the shared secret.', {'shared': 2}], ['Does the directed graph with edges A->B, B->C, C->A have a cycle?', {'has_cycle': True}], ['Find the smallest x with 3^x ≡ 13 (mod 17).', {'x': 4}], ['How many divisors does 360 have?', {'count': 24}], ['ElGamal: encrypt m = 10 with p = 23, g = 5, x = 6, k = 3.', {'c1': 10, 'c2': 14}], ["What is Euler's totient of 36?", {'phi': 12}], ['Does the graph with edges A-B, B-C, C-A, C-D, D-E, E-C have an Euler circuit?', {'has_circuit': True}], ['Find the Bezout coefficients of 240 and 46.', {'gcd': 2, 'x': -9, 'y': 47}], ['What is the 30th Fibonacci number?', {'fibonacci': 832040}], ['What is the gcd of 84 and 36?', {'gcd': 12, 'lcm': 252}], ['What is the girth of the graph with edges A-B, B-C, C-D, D-A, A-E, E-F, F-A?', {'girth': 3}], ['Find the degree sequence of the graph with edges A-B, A-C, B-C, C-D.', {'degree_sequence': [3, 2, 2, 1]}], ['What is the diameter of the graph with edges A-B, B-C, C-D, D-E, B-F?', {'diameter': 4}], ['How many triangles does the graph with edges A-B, B-C, C-A, C-D, D-A have?', {'triangles': 2}], ['Is the sequence 3, 3, 2, 2, 2 graphical?', {'is_graphical': True}], ['Does the graph with edges A-B, B-C, C-D, D-A, A-C have a Hamiltonian cycle?', {'exists': True}], ['How many integers from 1 to 1000 are divisible by 3, 5 or 7?', {'divisible_by_any': 543}], ['What is the independence number of the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1?', {'independence_number': 2}], ['How many partitions of 10 are there?', {'partitions': 42}], ['What is the integer square root of 1000?', {'isqrt': 31}]]


def _close(a, b):
    if isinstance(b, list):
        return isinstance(a, (list, tuple)) and len(a) == len(b) and all(_close(x, y) for x, y in zip(a, b))
    try:
        return abs(complex(a) - complex(b)) <= 1e-6 * max(1.0, abs(complex(b)))
    except (TypeError, ValueError):
        return a == b


def checks(P) -> dict:
    out = {"certified_procedures": len(P.pack().procedures) == 99,
           "refuses_nonsense": not P.ask("What is it?").solved}
    for i, (q, want) in enumerate(QUESTIONS):
        a = P.ask(q)
        out[f"question_{i}"] = a.solved and all(_close(a.values.get(k), v) for k, v in want.items())
    return out
