"""Discrete mathematics procedures: number theory, modular arithmetic, combinatorics, graph algorithms, propositional
logic and the mathematics of cryptography (domain code, loaded by the pack; enabled only once
tools/certify_procedures.py has certified them).

Every procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
worked examples, an INDEPENDENT reference computation (a checking library, imported inside the reference and used
only by certification) and a deliberately wrong variant (certification shows the checks reject it). The shipped code
is plain Python: exact integer arithmetic, no third-party library.

Conventions that make orders and paths deterministic (stated in each procedure's conditions):
  * node names are ordered numerically when they are integers ("2" < "10"), otherwise alphabetically, integers first;
  * breadth-first and depth-first searches visit neighbours in that ascending order;
  * a topological order is the lexicographically smallest one (Kahn's algorithm with a min-priority queue);
  * truth tables list variables in alphabetical order, the first variable being the most significant bit;
  * Bernoulli numbers use B_1 = -1/2; Fibonacci F_0 = 0, F_1 = 1; Lucas L_0 = 2, L_1 = 1.
"""
from __future__ import annotations

import heapq
import itertools
import math
import re
from fractions import Fraction

PROCEDURES: dict = {}

MAXI = 10 ** 15            # integer inputs are read through floating point by the runtime: exact up to 2**53


# ==================================================================================================================
# shared helpers
# ==================================================================================================================
def _iv(x) -> bool:
    return isinstance(x, int) and not isinstance(x, bool)


def _safe(fn):
    """A verifier: True/False (or a residual) from a check that may meet malformed outputs; any error means reject."""
    def verify(inputs, outputs):
        try:
            r = fn(inputs, outputs)
        except Exception:
            return 1.0
        if r is True:
            return 0.0
        if r is False or r is None:
            return 1.0
        return float(r)
    return verify


def _whole(v, name, lo=None, hi=MAXI):
    if isinstance(v, bool):
        raise ValueError(f"{name} must be a whole number")
    if isinstance(v, float):
        if not math.isfinite(v) or v != int(v):
            raise ValueError(f"{name} must be a whole number")
        v = int(v)
    elif not isinstance(v, int):
        try:
            v = int(str(v).strip())
        except ValueError:
            raise ValueError(f"{name} must be a whole number") from None
    if lo is not None and v < lo:
        raise ValueError(f"{name} must be at least {lo}")
    if hi is not None and abs(v) > hi:
        raise ValueError(f"{name} must be at most {hi} in size")
    return v


def _wlist(xs, name, lo=None, hi=MAXI, n_min=1, n_max=None):
    if isinstance(xs, str):
        xs = [t for t in re.split(r"[,\s;]+", xs.strip("[] ")) if t]
    out = [_whole(x, name, lo, hi) for x in xs]
    if len(out) < n_min:
        raise ValueError(f"{name} needs at least {n_min} entries")
    if n_max is not None and len(out) > n_max:
        raise ValueError(f"{name} allows at most {n_max} entries")
    return out


def _I(sym, meaning, lo=None, hi=MAXI, required=True):
    d = {"symbol": sym, "kind": "number", "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
        if lo is None:
            d["min"] = -hi
    return d


def _V(sym, meaning, n_min=1, n_max=None, required=True):
    d = {"symbol": sym, "kind": "number", "type": "vector", "meaning": meaning, "required": required, "min_length": n_min}
    if n_max:
        d["max_length"] = n_max
    return d


def _T(sym, meaning, required=True, n_max=None):
    d = {"symbol": sym, "kind": "text", "type": "text", "meaning": meaning, "required": required}
    if n_max:
        d["max_length"] = n_max
    return d


def _E(sym="edges", meaning="the graph as a list of edges u-v (an optional third entry is the weight)"):
    return {"symbol": sym, "kind": "graph", "type": "edges", "meaning": meaning, "required": True, "min_length": 1}


def _N(sym, meaning, required=True):
    return {"symbol": sym, "kind": "text", "type": "text", "meaning": meaning, "required": required}


def _B(sym, meaning):
    return {"symbol": sym, "kind": "number", "type": "boolean", "meaning": meaning, "required": False}


def _O(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    for need in ("name", "topic", "purpose", "inputs", "outputs", "primary", "method", "latex", "run", "verify", "parse",
                 "reference", "wrong", "random_inputs", "templates", "examples", "questions"):
        if need not in spec:
            raise KeyError(f"{key}: missing {need}")
    spec["verify"] = _safe(spec["verify"])
    PROCEDURES[key] = spec


def _euclid(a, b):
    while b:
        a, b = b, a % b
    return abs(a)


_SMALL = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
_MR_BASES = (2, 325, 9375, 28178, 450775, 9780504, 1795265022)      # deterministic for n < 2**64


def _is_prime(n: int) -> bool:
    """Deterministic Miller-Rabin for n < 2**64 (Jaeschke/Sinclair bases)."""
    if n < 2:
        return False
    for p in _SMALL:
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in _MR_BASES:
        a %= n
        if a == 0:
            continue
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def _is_prime_alt(n: int) -> bool:
    """A second primality test (verifiers): trial division by primes below 50, then Miller-Rabin to the first twelve
    prime bases (deterministic below 3.3e24)."""
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47):
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while not d & 1:
        d >>= 1
        s += 1
    for a in _SMALL:
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = pow(x, 2, n)
            if x == n - 1:
                break
        else:
            return False
    return True


def _rho(n: int) -> int:
    if n % 2 == 0:
        return 2
    c = 1
    while True:
        x = y = 2
        d = 1
        while d == 1:
            x = (x * x + c) % n
            y = (y * y + c) % n
            y = (y * y + c) % n
            d = math.gcd(abs(x - y), n)
        if d != n:
            return d
        c += 1


def _factor(n: int) -> dict:
    """Prime factorization {p: e} (Pollard rho with Miller-Rabin), primes ascending."""
    out = {}
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47):
        while n % p == 0:
            out[p] = out.get(p, 0) + 1
            n //= p
    stack = [n] if n > 1 else []
    while stack:
        m = stack.pop()
        if m == 1:
            continue
        if _is_prime(m):
            out[m] = out.get(m, 0) + 1
            continue
        r = math.isqrt(m)
        if r * r == m:
            stack += [r, r]
            continue
        d = _rho(m)
        stack += [d, m // d]
    return dict(sorted(out.items()))


def _trial_factor(n: int) -> dict:
    out, p = {}, 2
    while p * p <= n:
        while n % p == 0:
            out[p] = out.get(p, 0) + 1
            n //= p
        p += 1 if p == 2 else 2
    if n > 1:
        out[n] = out.get(n, 0) + 1
    return out


def _ints(text: str) -> list:
    return [int(x) for x in re.findall(r"(?<![\w.])-?\d+(?!\w|\.\d)", text)]


def _sieve(n: int) -> bytearray:
    s = bytearray([1]) * (n + 1)
    s[0:2] = b"\x00\x00"[: min(2, n + 1)]
    for i in range(2, math.isqrt(n) + 1):
        if s[i]:
            s[i * i::i] = bytearray(len(range(i * i, n + 1, i)))
    return s


def _ext(a, b):
    """Extended Euclid: (g, x, y) with a*x + b*y = g."""
    old_r, r, old_s, s, old_t, t = a, b, 1, 0, 0, 1
    while r:
        q = old_r // r
        old_r, r = r, old_r - q * r
        old_s, s = s, old_s - q * s
        old_t, t = t, old_t - q * t
    return old_r, old_s, old_t


def _inv(a, m):
    g, x, _ = _ext(a % m, m)
    if g != 1:
        raise ValueError(f"{a} has no inverse modulo {m} (gcd is {g})")
    return x % m


def _rand_prime(rng, lo, hi):
    while True:
        n = rng.randint(lo, hi)
        if _is_prime(n):
            return n


_MOD = r"(?:\(\s*mod(?:ulo)?\s*(\d+)\s*\)|mod(?:ulo)?\s+(\d+))"
_EQ = r"(?:≡|==|=|is congruent to)"


def _first(*vals):
    return next((v for v in vals if v is not None), None)


# ==================================================================================================================
# number theory
# ==================================================================================================================
def gcd_lcm(numbers):
    xs = _wlist(numbers, "numbers", lo=1, n_min=2, n_max=20)
    g, l = 0, 1
    for x in xs:
        g = _euclid(g, x)
        l = l // _euclid(l, x) * x
    return {"gcd": g, "lcm": l}


def _v_gcd(i, o):
    xs = [int(v) for v in i["numbers"]]
    g, l = o["gcd"], o["lcm"]
    if not (_iv(g) and _iv(l) and g >= 1 and l >= 1):
        return False
    from functools import reduce
    return (all(x % g == 0 for x in xs) and reduce(math.gcd, [x // g for x in xs]) == 1
            and all(l % x == 0 for x in xs) and reduce(math.gcd, [l // x for x in xs]) == 1)


def _ref_gcd(numbers):
    import sympy
    xs = [int(v) for v in numbers]
    return {"gcd": int(sympy.igcd(*xs)), "lcm": int(sympy.ilcm(*xs))}


def parse_gcd(q0):
    q = q0.lower()
    if not re.search(r"\b(gcd|hcf|lcm|greatest common (divisor|factor)|highest common factor|least common multiple|"
                     r"lowest common multiple)\b", q):
        return None
    if re.search(r"extended|b[eé]zout|coefficient|\bx\b|\by\b|\bmod\b|modulo|polynomial|\^", q):
        return None
    nums = _ints(q)
    return {"numbers": nums} if nums else {}


_add("gcd_lcm", name="Greatest common divisor and least common multiple", topic="number theory / divisibility",
     purpose="The gcd and lcm of two or more positive integers (Euclid's algorithm).",
     inputs=[_V("numbers", "the positive integers (two or more)", 2, 20)],
     outputs=[_O("gcd", "greatest common divisor"), _O("lcm", "least common multiple")],
     primary="gcd", conditions=["two or more whole numbers, each at least 1"],
     method="Euclid's algorithm gcd(a, b) = gcd(b, a mod b), folded over the list; lcm(a, b) = a / gcd(a, b) * b",
     latex=r"\gcd(a,b)=\gcd(b,\,a \bmod b),\qquad \operatorname{lcm}(a,b)=\frac{ab}{\gcd(a,b)}",
     run=gcd_lcm, verify=_v_gcd, parse=parse_gcd, reference=_ref_gcd,
     wrong=lambda numbers: {"gcd": gcd_lcm(numbers)["gcd"], "lcm": math.prod(int(v) for v in numbers)},
     random_inputs=lambda rng: (lambda g: {"numbers": [float(g * rng.randint(1, 400)) for _ in range(rng.randint(2, 4))]})(
         rng.randint(2, 60)),
     templates=["What is the gcd of 84 and 36?", "Find the LCM of 4, 6 and 10.",
                "Find the greatest common divisor of 252 and 105."],
     examples=[{"inputs": {"numbers": "84, 36"}, "outputs": {"gcd": 12, "lcm": 252}},
               {"inputs": {"numbers": "4, 6, 10"}, "outputs": {"gcd": 2, "lcm": 60}},
               {"inputs": {"numbers": [17, 5]}, "outputs": {"gcd": 1, "lcm": 85}}],
     questions=[{"q": "What is the gcd of 84 and 36?", "outputs": {"gcd": 12, "lcm": 252}},
                {"q": "Find the greatest common divisor of 252 and 105.", "outputs": {"gcd": 21}},
                {"q": "Find the LCM of 4, 6 and 10.", "outputs": {"lcm": 60}},
                {"q": "What is the gcd of 0 and 12?", "outcome": "INVALID_INPUT"},
                {"q": "Find the least common multiple of the numbers.", "outcome": "NEEDS_INPUT"}])


def extended_gcd(a, b):
    a, b = _whole(a, "a", 1), _whole(b, "b", 1)
    g, x, y = _ext(a, b)
    step = b // g
    x0 = x % step
    return {"gcd": g, "x": x, "y": y, "x_min": x0, "y_min": (g - a * x0) // b}


def _v_ext(i, o):
    a, b = int(i["a"]), int(i["b"])
    g = o["gcd"]
    if not all(_iv(o[k]) for k in ("gcd", "x", "y", "x_min", "y_min")):
        return False
    return (g == math.gcd(a, b) and a * o["x"] + b * o["y"] == g and a * o["x_min"] + b * o["y_min"] == g
            and 0 <= o["x_min"] < b // g)


def _ref_ext(a, b):
    import gmpy2
    g, s, _t = gmpy2.gcdext(int(a), int(b))
    g, s = int(g), int(s)
    step = int(b) // g
    x0 = s % step
    return {"gcd": g, "x_min": x0, "y_min": (g - int(a) * x0) // int(b)}


def parse_ext(q0):
    q = q0.lower()
    if not re.search(r"extended euclid|b[eé]zout", q):
        return None
    nums = [n for n in _ints(q)]
    if len(nums) != 2:
        return {} if not nums else None
    return {"a": nums[0], "b": nums[1]}


_add("extended_gcd", name="Extended Euclidean algorithm (Bezout coefficients)", topic="number theory / divisibility",
     purpose="gcd(a, b) and integers x, y with a*x + b*y = gcd(a, b).",
     inputs=[_I("a", "first positive integer", 1), _I("b", "second positive integer", 1)],
     outputs=[_O("gcd", "gcd(a, b)"), _O("x", "coefficient of a from the extended Euclidean algorithm"),
              _O("y", "coefficient of b from the extended Euclidean algorithm"),
              _O("x_min", "the coefficient of a with 0 <= x_min < b/gcd"), _O("y_min", "the matching coefficient of b")],
     primary="gcd", conditions=["a, b whole numbers >= 1",
                                "(x, y) are the coefficients the standard extended Euclidean algorithm produces; "
                                "(x_min, y_min) is the solution with the smallest non-negative x"],
     method="extended Euclidean algorithm (back-substitution kept as running coefficients)",
     latex=r"a x + b y = \gcd(a, b)",
     run=extended_gcd, verify=_v_ext, parse=parse_ext, reference=_ref_ext,
     wrong=lambda a, b: (lambda r: dict(r, x_min=r["y_min"], y_min=r["x_min"]))(extended_gcd(a, b)),
     random_inputs=lambda rng: {"a": rng.randint(2, 10 ** rng.randint(2, 9)), "b": rng.randint(2, 10 ** rng.randint(2, 9))},
     templates=["Find the Bezout coefficients of 240 and 46.", "Apply the extended Euclidean algorithm to 240 and 46."],
     examples=[{"inputs": {"a": 240, "b": 46}, "outputs": {"gcd": 2, "x": -9, "y": 47, "x_min": 14, "y_min": -73}},
               {"inputs": {"a": 3, "b": 7}, "outputs": {"gcd": 1, "x_min": 5, "y_min": -2}},
               {"inputs": {"a": 12, "b": 18}, "outputs": {"gcd": 6, "x_min": 2, "y_min": -1}}],
     questions=[{"q": "Find the Bezout coefficients of 240 and 46.", "outputs": {"gcd": 2, "x": -9, "y": 47}},
                {"q": "Apply the extended Euclidean algorithm to 3 and 7.", "outputs": {"gcd": 1, "x_min": 5}},
                {"q": "Apply the extended Euclidean algorithm to -3 and 7.", "outcome": "INVALID_INPUT"}])


def linear_diophantine(a, b, c):
    a, b, c = _whole(a, "a"), _whole(b, "b"), _whole(c, "c")
    if a == 0 or b == 0:
        raise ValueError("a and b must both be non-zero")
    g, s, t = _ext(abs(a), abs(b))
    step = abs(b) // g
    if c % g:
        return {"solvable": False, "x0": None, "y0": None, "x_step": b // g, "y_step": -(a // g)}
    x = s * (1 if a > 0 else -1) * (c // g)
    x0 = x % step
    return {"solvable": True, "x0": x0, "y0": (c - a * x0) // b, "x_step": b // g, "y_step": -(a // g)}


def _v_dioph(i, o):
    a, b, c = int(i["a"]), int(i["b"]), int(i["c"])
    g = math.gcd(a, b)
    if o["solvable"] is not (c % g == 0) or o["x_step"] != b // g or o["y_step"] != -(a // g):
        return False
    if not o["solvable"]:
        return o["x0"] is None and o["y0"] is None
    return _iv(o["x0"]) and _iv(o["y0"]) and a * o["x0"] + b * o["y0"] == c and 0 <= o["x0"] < abs(b) // g


def _ref_dioph(a, b, c):
    """Brute force: the first x in [0, |b|) with b | c - a x (solutions repeat with period |b|/g <= |b|)."""
    import sympy
    g = int(sympy.igcd(a, b))
    for x in range(abs(b)):
        if (c - a * x) % b == 0:
            return {"solvable": True, "x0": x, "y0": (c - a * x) // b, "x_step": b // g, "y_step": -(a // g)}
    return {"solvable": False, "x0": None, "y0": None}


def _wrong_dioph(a, b, c):
    a, b, c = int(a), int(b), int(c)
    g, s, t = _ext(abs(a), abs(b))
    if c % g:
        return {"solvable": False, "x0": None, "y0": None, "x_step": b // g, "y_step": -(a // g)}
    step = abs(b) // g
    x0 = (s * (1 if a > 0 else -1) * c) % step                  # forgot to divide c by g
    return {"solvable": True, "x0": x0, "y0": (c - a * x0) // b, "x_step": b // g, "y_step": -(a // g)}


def _rand_dioph(rng):
    g = rng.choice([1, 2, 3, 4, 6, 12, 15])
    a, b = g * rng.choice([-1, 1]) * rng.randint(1, 60), g * rng.choice([-1, 1]) * rng.randint(1, 60)
    c = rng.randint(-500, 500) * (g if rng.random() < 0.8 else 1)
    return {"a": a, "b": b, "c": c}


def parse_dioph(q0):
    q = q0.lower().replace("−", "-")
    if not re.search(r"\binteger|diophantine", q):
        return None
    m = re.search(r"(-?\s*\d*)\s*\*?\s*x\s*([+-])\s*(\d*)\s*\*?\s*y\s*=\s*(-?\s*\d+)", q)
    if not m:
        return None
    a = m.group(1).replace(" ", "")
    a = -1 if a == "-" else (1 if a == "" else int(a))
    b = (int(m.group(3)) if m.group(3) else 1) * (-1 if m.group(2) == "-" else 1)
    return {"a": a, "b": b, "c": int(m.group(4).replace(" ", ""))}


_add("linear_diophantine", name="Linear Diophantine equation ax + by = c", topic="number theory / Diophantine equations",
     purpose="Whether a*x + b*y = c has integer solutions, and the general solution x = x0 + (b/g) t, y = y0 - (a/g) t.",
     inputs=[_I("a", "coefficient of x (non-zero)"), _I("b", "coefficient of y (non-zero)"), _I("c", "right-hand side")],
     outputs=[_O("solvable", "true when gcd(a, b) divides c", "boolean"), _O("x0", "the solution with the smallest non-negative x"),
              _O("y0", "the matching y"), _O("x_step", "b/g: the step of x between solutions"),
              _O("y_step", "-a/g: the step of y between solutions")],
     primary="solvable", conditions=["a, b non-zero integers", "solutions exist exactly when gcd(a, b) divides c"],
     method="extended Euclidean algorithm, scaled by c/g, reduced to the smallest non-negative x",
     latex=r"ax+by=c \text{ solvable} \iff \gcd(a,b)\mid c;\quad x=x_0+\tfrac{b}{g}t,\; y=y_0-\tfrac{a}{g}t",
     run=linear_diophantine, verify=_v_dioph, parse=parse_dioph, reference=_ref_dioph, wrong=_wrong_dioph,
     random_inputs=_rand_dioph,
     templates=["Find all integer solutions of 12x + 18y = 30.", "Solve the Diophantine equation 3x - 5y = 7."],
     examples=[{"inputs": {"a": 12, "b": 18, "c": 30}, "outputs": {"solvable": True, "x0": 1, "y0": 1, "x_step": 3, "y_step": -2}},
               {"inputs": {"a": 3, "b": -5, "c": 7}, "outputs": {"solvable": True, "x0": 4, "y0": 1}},
               {"inputs": {"a": 6, "b": 9, "c": 4}, "outputs": {"solvable": False}}],
     questions=[{"q": "Find all integer solutions of 12x + 18y = 30.", "outputs": {"solvable": True, "x0": 1, "y0": 1}},
                {"q": "Solve the Diophantine equation 3x - 5y = 7.", "outputs": {"x0": 4, "y0": 1}},
                {"q": "Does 6x + 9y = 4 have integer solutions?", "outputs": {"solvable": False}}])


def linear_congruence(a, b, m):
    a, b, m = _whole(a, "a"), _whole(b, "b"), _whole(m, "m", 1)
    g = math.gcd(a % m, m) if a % m else m
    if b % g:
        return {"smallest": None, "count": 0, "step": None}
    mp = m // g
    x0 = (b // g) % mp * _inv((a // g) % mp, mp) % mp if mp > 1 else 0
    return {"smallest": x0, "count": g, "step": mp}


def _v_lincong(i, o):
    a, b, m = int(i["a"]), int(i["b"]), int(i["m"])
    g = math.gcd(a, m)
    if o["count"] != (g if b % g == 0 else 0):
        return False
    if o["count"] == 0:
        return o["smallest"] is None and o["step"] is None
    return _iv(o["smallest"]) and o["step"] == m // g and 0 <= o["smallest"] < o["step"] and (a * o["smallest"] - b) % m == 0


def _ref_lincong(a, b, m):
    sols = [x for x in range(m) if (a * x - b) % m == 0]
    if not sols:
        return {"smallest": None, "count": 0, "step": None}
    return {"smallest": sols[0], "count": len(sols), "step": (sols[1] - sols[0]) if len(sols) > 1 else m}


def _rand_lincong(rng):
    g = rng.choice([2, 3, 4, 6])
    m = g * rng.randint(2, 500)
    a = g * rng.randint(1, m)
    b = g * rng.randint(0, m) if rng.random() < 0.85 else rng.randint(0, m)
    return {"a": a, "b": b, "m": m}


_CONG = r"(?<![\w^])(-?\d*)\s*\*?\s*x\s*" + _EQ + r"\s*(-?\d+)\s*" + _MOD


def parse_lincong(q0):
    q = q0.lower()
    found = re.findall(_CONG, q)
    if len(found) != 1 or re.search(r"x\s*(\^|\*\*)|(\^|\*\*)\s*x|\by\b", q):
        return None
    a, b, m1, m2 = found[0]
    a = -1 if a == "-" else (1 if a == "" else int(a))
    return {"a": a, "b": int(b), "m": int(m1 or m2)}


_add("linear_congruence", name="Linear congruence ax ≡ b (mod m)", topic="modular arithmetic / congruences",
     purpose="All solutions of a*x ≡ b (mod m): how many there are modulo m, the smallest, and their spacing.",
     inputs=[_I("a", "coefficient of x"), _I("b", "right-hand side"), _I("m", "modulus (at least 1)", 1)],
     outputs=[_O("smallest", "smallest non-negative solution (none when there is no solution)"),
              _O("count", "number of solutions modulo m (gcd(a, m) when it divides b, else 0)"),
              _O("step", "m / gcd(a, m): solutions are smallest + k*step")],
     primary="smallest", conditions=["m >= 1", "solvable exactly when gcd(a, m) divides b"],
     method="divide by g = gcd(a, m), then multiply by the inverse of a/g modulo m/g",
     latex=r"ax\equiv b \pmod m \iff x \equiv \tfrac{b}{g}\left(\tfrac{a}{g}\right)^{-1} \pmod{\tfrac{m}{g}},\ g=\gcd(a,m)\mid b",
     run=linear_congruence, verify=_v_lincong, parse=parse_lincong, reference=_ref_lincong,
     wrong=lambda a, b, m: (lambda r: dict(r, count=1 if r["count"] else 0))(linear_congruence(a, b, m)),
     random_inputs=_rand_lincong,
     templates=["Solve 3x ≡ 4 (mod 7).", "Solve 6x ≡ 4 (mod 10)."],
     examples=[{"inputs": {"a": 3, "b": 4, "m": 7}, "outputs": {"smallest": 6, "count": 1, "step": 7}},
               {"inputs": {"a": 6, "b": 4, "m": 10}, "outputs": {"smallest": 4, "count": 2, "step": 5}},
               {"inputs": {"a": 4, "b": 3, "m": 6}, "outputs": {"smallest": None, "count": 0}}],
     questions=[{"q": "Solve 3x ≡ 4 (mod 7).", "outputs": {"smallest": 6, "count": 1}},
                {"q": "Solve 6x ≡ 4 (mod 10).", "outputs": {"smallest": 4, "count": 2, "step": 5}},
                {"q": "Solve 4x = 3 mod 6.", "outputs": {"count": 0}},
                {"q": "Solve 3x ≡ 4 (mod 0).", "outcome": "INVALID_INPUT"}])


def is_prime(n):
    n = _whole(n, "n", 0)
    if n >= 2 ** 64:
        raise ValueError("n must be below 2**64")
    p = _is_prime(n)
    spf = None if n < 2 else (n if p else min(_factor(n)))
    return {"is_prime": p, "smallest_prime_factor": spf}


def _v_isprime(i, o):
    n = int(i["n"])
    p = o["is_prime"]
    if not isinstance(p, bool) or p != _is_prime_alt(n):
        return False
    s = o["smallest_prime_factor"]
    if n < 2:
        return s is None
    return _iv(s) and n % s == 0 and _is_prime_alt(s) and ((s == n) == p)


def _ref_isprime(n):
    import sympy
    return {"is_prime": bool(sympy.isprime(n)), "smallest_prime_factor": (None if n < 2 else int(min(sympy.factorint(n))))}


def _wrong_isprime(n):
    n = int(n)
    p = n == 2 or (n > 1 and n % 2 == 1 and all(n % q for q in (3, 5, 7) if q != n))     # small trial divisors only
    return {"is_prime": p, "smallest_prime_factor": n if p else (None if n < 2 else min(_factor(n)))}


def _rand_isprime(rng):
    if rng.random() < 0.35:
        return {"n": _rand_prime(rng, 2, 10 ** rng.randint(2, 15))}
    p = _rand_prime(rng, 11, 10 ** rng.randint(2, 7))
    q = _rand_prime(rng, 11, 10 ** rng.randint(2, 7))
    return {"n": p * q}


def parse_isprime(q0):
    q = q0.lower()
    if re.search(r"factor|next|previous|greater|less|between|how many|\d(st|nd|rd|th) prime|relatively|coprime|primitive", q):
        return None
    m = re.search(r"\bis\s+(-?\d+)\s+(?:a\s+)?(?:prime|composite)(?:\s+number)?\b", q) or \
        re.search(r"(?:primality of|whether)\s+(-?\d+)\s+(?:is\s+)?(?:a\s+)?(?:prime|composite)?", q)
    return {"n": int(m.group(1))} if m else None


_add("is_prime", name="Primality test", topic="number theory / primes",
     purpose="Whether n is prime (deterministic Miller-Rabin, exact for every n below 2^64), with its smallest prime factor.",
     inputs=[_I("n", "the whole number to test", 0)],
     outputs=[_O("is_prime", "true when n is prime", "boolean"),
              _O("smallest_prime_factor", "smallest prime dividing n (n itself when prime; none for n < 2)")],
     primary="is_prime", conditions=["0 <= n < 2^64 (questions accept n up to 10^15)"],
     method="trial division by primes up to 37, then Miller-Rabin to the bases 2, 325, 9375, 28178, 450775, 9780504, "
            "1795265022 (deterministic below 2^64); the smallest factor by Pollard's rho",
     latex=r"a^{d}\equiv 1 \ \text{or}\ a^{2^r d}\equiv -1 \pmod n \text{ for every base } a",
     run=is_prime, verify=_v_isprime, parse=parse_isprime, reference=_ref_isprime, wrong=_wrong_isprime,
     random_inputs=_rand_isprime,
     templates=["Is 97 prime?", "Is 1000003 a prime number?", "Test whether 561 is prime."],
     examples=[{"inputs": {"n": 97}, "outputs": {"is_prime": True, "smallest_prime_factor": 97}},
               {"inputs": {"n": 561}, "outputs": {"is_prime": False, "smallest_prime_factor": 3}},
               {"inputs": {"n": 1}, "outputs": {"is_prime": False, "smallest_prime_factor": None}},
               {"inputs": {"n": 999999999989}, "outputs": {"is_prime": True}}],
     questions=[{"q": "Is 97 prime?", "outputs": {"is_prime": True}},
                {"q": "Is 1000003 a prime number?", "outputs": {"is_prime": True}},
                {"q": "Test whether 561 is prime.", "outputs": {"is_prime": False, "smallest_prime_factor": 3}},
                {"q": "Is 91 composite?", "outputs": {"is_prime": False, "smallest_prime_factor": 7}},
                {"q": "Is -7 prime?", "outcome": "INVALID_INPUT"}])


def prime_factorization(n):
    n = _whole(n, "n", 2)
    f = _factor(n)
    return {"factorization": [[p, e] for p, e in f.items()], "distinct_primes": len(f),
            "prime_factors_with_multiplicity": sum(f.values()), "largest_prime_factor": max(f),
            "smallest_prime_factor": min(f), "text": " × ".join(f"{p}^{e}" if e > 1 else str(p) for p, e in f.items())}


def _v_factor(i, o):
    n = int(i["n"])
    fs = o["factorization"]
    ps = [p for p, _e in fs]
    if ps != sorted(set(ps)) or not all(_iv(p) and _iv(e) and e >= 1 and _is_prime_alt(p) for p, e in fs):
        return False
    return (math.prod(p ** e for p, e in fs) == n and o["largest_prime_factor"] == ps[-1] and _iv(o["largest_prime_factor"])
            and o["smallest_prime_factor"] == ps[0] and o["distinct_primes"] == len(ps)
            and o["prime_factors_with_multiplicity"] == sum(e for _p, e in fs))


def _ref_factor(n):
    import sympy
    f = sympy.factorint(n)
    return {"factorization": [[int(p), int(e)] for p, e in sorted(f.items())], "distinct_primes": len(f),
            "prime_factors_with_multiplicity": int(sum(f.values())), "largest_prime_factor": int(max(f)),
            "smallest_prime_factor": int(min(f))}


def _rand_factor(rng):
    n = 1
    for _ in range(rng.randint(1, 5)):
        n *= _rand_prime(rng, 2, rng.choice([20, 200, 10 ** 4])) ** rng.randint(1, 3)
    if rng.random() < 0.4:
        n *= _rand_prime(rng, 10 ** 4, 10 ** 6)
    return {"n": n if 2 <= n <= MAXI else 360}


def parse_factor(q0):
    q = q0.lower()
    if not re.search(r"factori[sz]|prime factors?|prime decomposition|factor (\d+) into primes", q):
        return None
    if re.search(r"\bnumber of divisors|polynomial|\bx\b", q):
        return None
    nums = _ints(q)
    if len(nums) != 1:
        return {} if not nums else None
    return {"n": nums[0]}


_add("prime_factorization", name="Prime factorization", topic="number theory / primes",
     purpose="The prime factorization of n, its largest and smallest prime factors and the number of prime factors.",
     inputs=[_I("n", "the whole number to factor (at least 2)", 2)],
     outputs=[_O("factorization", "list of [prime, exponent] pairs, primes ascending", "list"),
              _O("distinct_primes", "number of distinct prime factors"),
              _O("prime_factors_with_multiplicity", "number of prime factors counted with multiplicity"),
              _O("largest_prime_factor", "largest prime factor"), _O("smallest_prime_factor", "smallest prime factor"),
              _O("text", "the factorization written as p^e × ...", "text")],
     primary="largest_prime_factor", conditions=["n a whole number, 2 <= n <= 10^15"],
     method="trial division by small primes, then Pollard's rho with deterministic Miller-Rabin for the cofactors",
     latex=r"n = p_1^{e_1} p_2^{e_2} \cdots p_k^{e_k}",
     run=prime_factorization, verify=_v_factor, parse=parse_factor, reference=_ref_factor,
     wrong=lambda n: (lambda r: dict(r, factorization=[[p, 1] for p, _e in r["factorization"]],
                                     prime_factors_with_multiplicity=r["distinct_primes"]))(prime_factorization(n)),
     random_inputs=_rand_factor,
     templates=["Find the prime factorization of 360.", "Factorize 1001.", "What is the largest prime factor of 13195?"],
     examples=[{"inputs": {"n": 360}, "outputs": {"factorization": [[2, 3], [3, 2], [5, 1]], "largest_prime_factor": 5,
                                                  "text": "2^3 × 3^2 × 5"}},
               {"inputs": {"n": 13195}, "outputs": {"largest_prime_factor": 29, "distinct_primes": 4}},
               {"inputs": {"n": 600851475143}, "outputs": {"largest_prime_factor": 6857}}],
     questions=[{"q": "Find the prime factorization of 360.", "outputs": {"factorization": [[2, 3], [3, 2], [5, 1]]}},
                {"q": "Factorize 1001.", "outputs": {"factorization": [[7, 1], [11, 1], [13, 1]]}},
                {"q": "What is the largest prime factor of 13195?", "outputs": {"largest_prime_factor": 29}},
                {"q": "Find the prime factorization of 1.", "outcome": "INVALID_INPUT"}])


def euler_phi(n):
    n = _whole(n, "n", 1)
    phi = n
    for p in _factor(n):
        phi = phi // p * (p - 1)
    return {"phi": phi}


def _v_phi(i, o):
    n = int(i["n"])
    if not _iv(o["phi"]):
        return False
    if n <= 3000:
        return o["phi"] == sum(1 for k in range(1, n + 1) if math.gcd(k, n) == 1)
    return o["phi"] == math.prod(p ** (e - 1) * (p - 1) for p, e in _factor(n).items())


def _rand_composite(rng, hi=10 ** 12):
    n = 1
    while n < 4:
        n = 1
        for _ in range(rng.randint(2, 5)):
            n *= _rand_prime(rng, 2, rng.choice([30, 1000, 10 ** 5])) ** rng.randint(1, 3)
    return n if n <= hi else rng.randint(4, 10 ** 6) * 2


def parse_phi(q0):
    q = q0.lower()
    if re.search(r"reduced|carmichael", q):
        return None
    m = re.search(r"(?:totient|euler'?s phi|euler phi|phi function)\D*?(\d+)", q) or \
        re.search(r"(?:\bphi|φ)\s*\(\s*(\d+)\s*\)", q)
    if m:
        return {"n": int(m.group(1))}
    m = re.search(r"how many (?:positive )?integers (?:less than or equal to|less than|up to|not exceeding|from 1 to|"
                  r"between 1 and) (\d+) are (?:coprime|relatively prime) to (\d+)", q)
    if m and m.group(1) == m.group(2):
        return {"n": int(m.group(1))}
    return None


_add("euler_phi", name="Euler's totient function", topic="number theory / arithmetic functions",
     purpose="phi(n): how many integers in 1..n are coprime to n.",
     inputs=[_I("n", "a positive integer", 1)],
     outputs=[_O("phi", "Euler's totient phi(n)")],
     primary="phi", conditions=["n a whole number, 1 <= n <= 10^15"],
     method="product formula over the distinct prime factors of n",
     latex=r"\varphi(n) = n \prod_{p \mid n}\left(1 - \frac{1}{p}\right)",
     run=euler_phi, verify=_v_phi, parse=parse_phi,
     reference=lambda n: {"phi": int(__import__("sympy").totient(n))},
     wrong=lambda n: {"phi": int(n) - 1},
     random_inputs=lambda rng: {"n": _rand_composite(rng) if rng.random() < 0.85 else rng.randint(4, 3000)},
     templates=["What is Euler's totient of 36?", "Compute phi(100).",
                "How many positive integers less than 36 are coprime to 36?"],
     examples=[{"inputs": {"n": 36}, "outputs": {"phi": 12}}, {"inputs": {"n": 97}, "outputs": {"phi": 96}},
               {"inputs": {"n": 1}, "outputs": {"phi": 1}}, {"inputs": {"n": 1000000}, "outputs": {"phi": 400000}}],
     questions=[{"q": "What is Euler's totient of 36?", "outputs": {"phi": 12}},
                {"q": "Compute phi(100).", "outputs": {"phi": 40}},
                {"q": "How many positive integers less than 36 are coprime to 36?", "outputs": {"phi": 12}},
                {"q": "Compute phi(0).", "outcome": "INVALID_INPUT"}])


def _divisors_from(f: dict) -> list:
    ds = [1]
    for p, e in f.items():
        ds = [d * p ** k for d in ds for k in range(e + 1)]
    return sorted(ds)


def divisor_functions(n):
    n = _whole(n, "n", 1, 10 ** 12)
    ds = _divisors_from(_factor(n))
    s = sum(ds)
    proper = s - n
    cls = "perfect" if proper == n else ("abundant" if proper > n else "deficient")
    return {"count": len(ds), "sum": s, "proper_sum": proper, "classification": cls, "divisors": ds}


def _v_div(i, o):
    n = int(i["n"])
    ds = o["divisors"]
    tau = math.prod(e + 1 for e in _trial_factor(n).values()) if n <= 10 ** 10 else \
        math.prod(e + 1 for e in _factor(n).values())
    proper = o["sum"] - n
    cls = "perfect" if proper == n else ("abundant" if proper > n else "deficient")
    return (_iv(o["count"]) and o["count"] == tau == len(ds) and ds == sorted(set(ds)) and all(n % d == 0 for d in ds)
            and o["sum"] == sum(ds) and o["proper_sum"] == proper and o["classification"] == cls)


def _ref_div(n):
    import sympy
    return {"count": int(sympy.divisor_count(n)), "sum": int(sympy.divisor_sigma(n, 1)),
            "proper_sum": int(sympy.divisor_sigma(n, 1)) - n, "divisors": [int(d) for d in sympy.divisors(n)]}


def parse_div(q0):
    q = q0.lower()
    if not re.search(r"\bdivisors?\b|perfect number|abundant|deficient|aliquot|\bsigma\s*\(|\btau\s*\(", q):
        return None
    if re.search(r"common divisor|greatest|\bgcd\b|polynomial", q):
        return None
    nums = _ints(q)
    if len(nums) != 1:
        return {} if not nums else None
    return {"n": nums[0]}


_add("divisor_functions", name="Divisors, divisor count and divisor sum", topic="number theory / arithmetic functions",
     purpose="All positive divisors of n, their number tau(n), their sum sigma(n), and whether n is perfect, abundant or deficient.",
     inputs=[_I("n", "a positive integer", 1, 10 ** 12)],
     outputs=[_O("count", "number of divisors tau(n)"), _O("sum", "sum of divisors sigma(n)"),
              _O("proper_sum", "sum of the proper divisors sigma(n) - n"),
              _O("classification", "perfect, abundant or deficient", "text"), _O("divisors", "every divisor, ascending", "list")],
     primary="count", conditions=["n a whole number, 1 <= n <= 10^12"],
     method="divisors generated from the prime factorization; tau = prod(e + 1), sigma = sum of the divisors",
     latex=r"\tau(n)=\prod (e_i+1),\qquad \sigma(n)=\prod \frac{p_i^{e_i+1}-1}{p_i-1}",
     run=divisor_functions, verify=_v_div, parse=parse_div, reference=_ref_div,
     wrong=lambda n: (lambda r: dict(r, count=r["count"] - 1, sum=r["proper_sum"]))(divisor_functions(n)),
     random_inputs=lambda rng: {"n": rng.choice([rng.randint(1, 1000), rng.randint(1, 10 ** 6), _rand_composite(rng, 10 ** 12)])},
     templates=["How many divisors does 360 have?", "Find the sum of the divisors of 28.", "Is 12 a perfect number?"],
     examples=[{"inputs": {"n": 360}, "outputs": {"count": 24, "sum": 1170}},
               {"inputs": {"n": 28}, "outputs": {"count": 6, "sum": 56, "classification": "perfect", "divisors": [1, 2, 4, 7, 14, 28]}},
               {"inputs": {"n": 12}, "outputs": {"classification": "abundant", "proper_sum": 16}},
               {"inputs": {"n": 1}, "outputs": {"count": 1, "sum": 1, "classification": "deficient"}}],
     questions=[{"q": "How many divisors does 360 have?", "outputs": {"count": 24}},
                {"q": "Find the sum of the divisors of 28.", "outputs": {"sum": 56}},
                {"q": "Is 12 a perfect number?", "outputs": {"classification": "abundant"}},
                {"q": "List the divisors of 0.", "outcome": "INVALID_INPUT"}])


def mod_reduce(a, m):
    a, m = _whole(a, "a"), _whole(m, "m", 1)
    q, r = a // m, a % m
    return {"remainder": r, "quotient": q}


def _v_modred(i, o):
    a, m = int(i["a"]), int(i["m"])
    return _iv(o["remainder"]) and _iv(o["quotient"]) and 0 <= o["remainder"] < m and o["quotient"] * m + o["remainder"] == a


def _ref_modred(a, m):
    import gmpy2
    q, r = gmpy2.f_divmod(a, m)
    return {"remainder": int(r), "quotient": int(q)}


def parse_modred(q0):
    q = q0.lower().replace("−", "-")
    if re.search(r"\^|\*\*|inverse|≡|congruen|\d\s*x\b|\bx\b|power|order|log|root|square|totient", q):
        return None
    m = re.search(r"(?<![\w.])(-?\d+)\s*(?:mod|modulo|%)\s*(\d+)\b", q) or \
        re.search(r"remainder (?:when|of) (-?\d+) (?:is )?divided by (\d+)", q)
    return {"a": int(m.group(1)), "m": int(m.group(2))} if m else None


_add("mod_reduce", name="Remainder modulo m (Euclidean division)", topic="modular arithmetic / residues",
     purpose="a mod m as the least non-negative residue, with the floor quotient: a = q*m + r, 0 <= r < m.",
     inputs=[_I("a", "the integer to reduce"), _I("m", "the modulus (at least 1)", 1)],
     outputs=[_O("remainder", "r with 0 <= r < m"), _O("quotient", "q = floor(a/m)")],
     primary="remainder", conditions=["m >= 1", "the remainder is never negative, also for negative a"],
     method="floor division", latex=r"a = q m + r,\quad 0 \le r < m",
     run=mod_reduce, verify=_v_modred, parse=parse_modred, reference=_ref_modred,
     wrong=lambda a, m: {"remainder": int(math.fmod(int(a), int(m))), "quotient": int(int(a) / int(m))},
     random_inputs=lambda rng: {"a": rng.randint(-10 ** 9, 10 ** 8), "m": rng.randint(2, 10 ** 4)},
     templates=["What is 17 mod 5?", "Find the remainder when -17 is divided by 5."],
     examples=[{"inputs": {"a": 17, "m": 5}, "outputs": {"remainder": 2, "quotient": 3}},
               {"inputs": {"a": -17, "m": 5}, "outputs": {"remainder": 3, "quotient": -4}}],
     questions=[{"q": "What is 17 mod 5?", "outputs": {"remainder": 2}},
                {"q": "Find the remainder when -17 is divided by 5.", "outputs": {"remainder": 3, "quotient": -4}},
                {"q": "What is 17 mod 0?", "outcome": "INVALID_INPUT"}])


def mod_pow(base, exponent, modulus):
    a, e, m = _whole(base, "base"), _whole(exponent, "exponent", 0), _whole(modulus, "modulus", 1)
    result, a = 1 % m, a % m
    while e:
        if e & 1:
            result = result * a % m
        a = a * a % m
        e >>= 1
    return {"result": result}


def _ref_modpow(base, exponent, modulus):
    import gmpy2
    return {"result": int(gmpy2.powmod(base, exponent, modulus))}


def parse_modpow(q0):
    q = q0.lower().replace("−", "-")
    if re.search(r"\bx\b|inverse|order|discrete", q):
        return None
    m = re.search(r"last (\d+) digits? of (-?\d+)\s*(?:\^|\*\*)\s*(\d+)", q)
    if m:
        k = int(m.group(1))
        return {"base": int(m.group(2)), "exponent": int(m.group(3)), "modulus": 10 ** k} if 1 <= k <= 15 else None
    m = re.search(r"(-?\d+)\s*(?:\^|\*\*)\s*(\d+)\s*(?:\(\s*mod(?:ulo)?\s*(\d+)\s*\)|mod(?:ulo)?\s+(\d+)|is divided by\s+(\d+))", q)
    if m:
        return {"base": int(m.group(1)), "exponent": int(m.group(2)), "modulus": int(_first(m.group(3), m.group(4), m.group(5)))}
    return None


_add("mod_pow", name="Modular exponentiation", topic="modular arithmetic / powers",
     purpose="a^e mod m by repeated squaring (also the last k digits of a power).",
     inputs=[_I("base", "the base a"), _I("exponent", "the exponent e (at least 0)", 0), _I("modulus", "the modulus m (at least 1)", 1)],
     outputs=[_O("result", "a^e mod m, between 0 and m - 1")],
     primary="result", conditions=["e >= 0", "m >= 1"],
     method="binary (square-and-multiply) exponentiation, reducing modulo m at every step",
     latex=r"a^{e} \bmod m,\quad a^{2k} = (a^k)^2,\ a^{2k+1} = a\,(a^k)^2",
     run=mod_pow, verify=lambda i, o: _iv(o["result"]) and o["result"] == pow(int(i["base"]), int(i["exponent"]), int(i["modulus"])),
     parse=parse_modpow, reference=_ref_modpow,
     wrong=lambda base, exponent, modulus: {"result": pow(int(base), int(exponent) % int(modulus), int(modulus))},
     random_inputs=lambda rng: {"base": rng.randint(-10 ** 6, 10 ** 9), "exponent": rng.randint(10 ** 7, 10 ** 12),
                                "modulus": rng.randint(2, 10 ** rng.randint(1, 6))},
     templates=["What is 3^200 mod 13?", "Compute 7^222 (mod 1000).", "Find the last 3 digits of 7^222.",
                "What is the remainder when 2^100 is divided by 7?"],
     examples=[{"inputs": {"base": 3, "exponent": 200, "modulus": 13}, "outputs": {"result": 9}},
               {"inputs": {"base": 7, "exponent": 222, "modulus": 1000}, "outputs": {"result": 49}},
               {"inputs": {"base": 2, "exponent": 100, "modulus": 7}, "outputs": {"result": 2}},
               {"inputs": {"base": 5, "exponent": 0, "modulus": 1}, "outputs": {"result": 0}}],
     questions=[{"q": "What is 3^200 mod 13?", "outputs": {"result": 9}},
                {"q": "Find the last 3 digits of 7^222.", "outputs": {"result": 49}},
                {"q": "What is the remainder when 2^100 is divided by 7?", "outputs": {"result": 2}},
                {"q": "What is 3^200 mod 0?", "outcome": "INVALID_INPUT"}])


def mod_inverse(a, m):
    a, m = _whole(a, "a"), _whole(m, "m", 2)
    return {"inverse": _inv(a, m)}


def _ref_modinv(a, m):
    import sympy
    return {"inverse": int(sympy.mod_inverse(a, m))}


def _rand_modinv(rng):
    m = rng.choice([_rand_composite(rng, 10 ** 9), rng.randint(2, 10 ** 9)])
    while True:
        a = rng.randint(-10 ** 6, 10 ** 9)
        if math.gcd(a, m) == 1:
            return {"a": a, "m": m}


def parse_modinv(q0):
    q = q0.lower().replace("−", "-")
    m = re.search(r"inverse of (-?\d+)\s*(?:\(\s*)?(?:mod|modulo)\s*(\d+)", q) or \
        re.search(r"(-?\d+)\s*\^\s*\(?\s*-\s*1\s*\)?\s*(?:\(\s*)?(?:mod|modulo)\s*(\d+)", q)
    return {"a": int(m.group(1)), "m": int(m.group(2))} if m else None


_add("mod_inverse", name="Modular multiplicative inverse", topic="modular arithmetic / inverses",
     purpose="The x in 0..m-1 with a*x ≡ 1 (mod m); it exists exactly when gcd(a, m) = 1.",
     inputs=[_I("a", "the integer to invert"), _I("m", "the modulus (at least 2)", 2)],
     outputs=[_O("inverse", "a^{-1} mod m")],
     primary="inverse", conditions=["m >= 2", "gcd(a, m) = 1 (otherwise no inverse exists)"],
     method="extended Euclidean algorithm", latex=r"a x \equiv 1 \pmod m",
     run=mod_inverse, verify=lambda i, o: _iv(o["inverse"]) and 0 <= o["inverse"] < int(i["m"]) and (int(i["a"]) * o["inverse"]) % int(i["m"]) == 1,
     parse=parse_modinv, reference=_ref_modinv,
     wrong=lambda a, m: {"inverse": pow(int(a), int(m) - 2, int(m))},
     random_inputs=_rand_modinv,
     templates=["Find the inverse of 3 modulo 11.", "What is 17^-1 mod 3120?"],
     examples=[{"inputs": {"a": 3, "m": 11}, "outputs": {"inverse": 4}}, {"inputs": {"a": 17, "m": 3120}, "outputs": {"inverse": 2753}},
               {"inputs": {"a": -3, "m": 7}, "outputs": {"inverse": 2}}],
     questions=[{"q": "Find the inverse of 3 modulo 11.", "outputs": {"inverse": 4}},
                {"q": "What is 17^-1 mod 3120?", "outputs": {"inverse": 2753}},
                {"q": "Find the inverse of 4 modulo 10.", "outcome": "INVALID_INPUT"}])


def crt(residues, moduli):
    rs = _wlist(residues, "residues", n_min=2, n_max=12)
    ms = _wlist(moduli, "moduli", lo=1, n_min=2, n_max=12)
    if len(rs) != len(ms):
        raise ValueError("give one residue for each modulus")
    x, M = 0, 1
    for r, m in zip(rs, ms):
        g = math.gcd(M, m)
        if (r - x) % g:
            raise ValueError("the congruences are inconsistent: no integer satisfies all of them")
        mg = m // g
        t = ((r - x) // g) * (_inv((M // g) % mg, mg) if mg > 1 else 0) % (mg if mg > 1 else 1)
        x = x + M * t
        M = M * mg
        x %= M
    return {"x": x, "modulus": M}


def _v_crt(i, o):
    rs, ms = [int(v) for v in i["residues"]], [int(v) for v in i["moduli"]]
    x, M = o["x"], o["modulus"]
    return _iv(x) and _iv(M) and M == math.lcm(*ms) and 0 <= x < M and all((x - r) % m == 0 for r, m in zip(rs, ms))


def _ref_crt(residues, moduli):
    from sympy.ntheory.modular import solve_congruence
    r = solve_congruence(*[(int(a), int(m)) for a, m in zip(residues, moduli)])
    return {"x": int(r[0]), "modulus": int(r[1])}


def _wrong_crt(residues, moduli):
    rs, ms = [int(v) for v in residues], [int(v) for v in moduli]
    M = math.lcm(*ms)
    return {"x": sum(r * (M // m) for r, m in zip(rs, ms)) % M, "modulus": M}      # forgot the inverses


def _rand_crt(rng):
    k = rng.randint(2, 4)
    if rng.random() < 0.6:
        ms = []
        while len(ms) < k:
            m = rng.randint(2, 60)
            if all(math.gcd(m, o) == 1 for o in ms):
                ms.append(m)
    else:
        ms = [rng.randint(2, 40) for _ in range(k)]
    x = rng.randint(0, 10 ** 6)
    return {"residues": [float(x % m + m * rng.randint(0, 2)) for m in ms], "moduli": [float(m) for m in ms]}


def parse_crt(q0):
    q = q0.lower().replace("−", "-")
    if re.search(r"x\s*(\^|\*\*)|(\^|\*\*)\s*x", q):
        return None
    found = re.findall(_CONG, q)
    if len(found) >= 2:
        if any(a not in ("", "1") for a, *_ in found):
            return {"_out_of_scope": "a system with coefficients on x (each congruence must read x ≡ r (mod m))"}
        return {"residues": [int(b) for _a, b, *_ in found], "moduli": [int(m1 or m2) for _a, _b, m1, m2 in found]}
    if "chinese remainder" in q:
        m = re.search(r"residues?\s*:?\s*([-\d,\s]+?)\s*(?:and|,|;)?\s*moduli\s*:?\s*([\d,\s]+)", q)
        if m:
            return {"residues": _ints(m.group(1)), "moduli": _ints(m.group(2))}
        return {}
    return None


_add("crt", name="Chinese remainder theorem", topic="modular arithmetic / congruences",
     purpose="The x with x ≡ r_i (mod m_i) for every i, unique modulo lcm(m_i); moduli need not be coprime.",
     inputs=[_V("residues", "the residues r_i", 2, 12), _V("moduli", "the moduli m_i (each at least 1)", 2, 12)],
     outputs=[_O("x", "the smallest non-negative solution"), _O("modulus", "lcm of the moduli: the solution is unique modulo it")],
     primary="x", conditions=["one residue per modulus", "the system is consistent (r_i ≡ r_j modulo gcd(m_i, m_j))"],
     method="merge the congruences pairwise with the extended Euclidean algorithm (works for non-coprime moduli)",
     latex=r"x \equiv r_i \pmod{m_i},\ i=1..k \;\Rightarrow\; x \bmod \operatorname{lcm}(m_1,\dots,m_k)",
     run=crt, verify=_v_crt, parse=parse_crt, reference=_ref_crt, wrong=_wrong_crt, random_inputs=_rand_crt,
     templates=["Solve x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7).",
                "Use the Chinese remainder theorem with residues 1, 2 and moduli 4, 9."],
     examples=[{"inputs": {"residues": "2, 3, 2", "moduli": "3, 5, 7"}, "outputs": {"x": 23, "modulus": 105}},
               {"inputs": {"residues": "1, 2", "moduli": "4, 9"}, "outputs": {"x": 29, "modulus": 36}},
               {"inputs": {"residues": "3, 5", "moduli": "4, 6"}, "outputs": {"x": 11, "modulus": 12}}],
     questions=[{"q": "Solve x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7).", "outputs": {"x": 23, "modulus": 105}},
                {"q": "Use the Chinese remainder theorem with residues 1, 2 and moduli 4, 9.", "outputs": {"x": 29}},
                {"q": "Solve x ≡ 1 (mod 4), x ≡ 2 (mod 6).", "outcome": "INVALID_INPUT"},
                {"q": "Solve 2x ≡ 1 (mod 3), x ≡ 2 (mod 5).", "outcome": "OUT_OF_SCOPE"}])


def discrete_log(base, target, modulus):
    b, t, m = _whole(base, "base"), _whole(target, "target"), _whole(modulus, "modulus", 2, 10 ** 12)
    if math.gcd(b, m) != 1:
        raise ValueError("base and modulus must be coprime")
    b, t = b % m, t % m
    s = math.isqrt(m) + 1
    table, cur = {}, 1
    for j in range(s):
        table.setdefault(cur, j)
        cur = cur * b % m
    factor = pow(_inv(b, m), s, m)
    gamma = t
    for i in range(s + 1):
        if gamma in table:
            return {"x": i * s + table[gamma]}
        gamma = gamma * factor % m
    raise ValueError(f"no exponent x satisfies {base}^x ≡ {target} (mod {modulus})")


def _v_dlog(i, o):
    b, t, m = int(i["base"]) % int(i["modulus"]), int(i["target"]) % int(i["modulus"]), int(i["modulus"])
    x = o["x"]
    if not (_iv(x) and x >= 0 and pow(b, x, m) == t):
        return False
    cur = 1 % m
    for _ in range(x):                         # no smaller exponent works
        if cur == t:
            return False
        cur = cur * b % m
    return True


def _ref_dlog(base, target, modulus):
    from sympy.ntheory import discrete_log as dl, n_order
    return {"x": int(dl(modulus, target % modulus, base % modulus)) % int(n_order(base % modulus, modulus))}


def _rand_dlog(rng):
    m = rng.randint(3, 100000)
    while True:
        b = rng.randint(2, m - 1) if m > 3 else 2
        if math.gcd(b, m) == 1:
            break
    x = rng.randint(0, m)
    return {"base": b, "target": pow(b, x, m), "modulus": m}


def parse_dlog(q0):
    q = q0.lower()
    m = re.search(r"(\d+)\s*(?:\^|\*\*)\s*x\s*" + _EQ + r"\s*(\d+)\s*" + _MOD, q)
    if m:
        return {"base": int(m.group(1)), "target": int(m.group(2)), "modulus": int(m.group(3) or m.group(4))}
    m = re.search(r"discrete log(?:arithm)? of (\d+) (?:to|in|with) (?:the )?base (\d+)\s*(?:\(\s*)?(?:mod|modulo)\s*(\d+)", q)
    if m:
        return {"base": int(m.group(2)), "target": int(m.group(1)), "modulus": int(m.group(3))}
    return None


_add("discrete_log", name="Discrete logarithm", topic="modular arithmetic / discrete logarithms",
     purpose="The smallest x >= 0 with base^x ≡ target (mod m).",
     inputs=[_I("base", "the base g"), _I("target", "the value h"), _I("modulus", "the modulus m (2..10^12)", 2, 10 ** 12)],
     outputs=[_O("x", "the smallest non-negative exponent with g^x ≡ h (mod m)")],
     primary="x", conditions=["gcd(base, m) = 1", "a solution exists"],
     method="baby-step giant-step (keeping the smallest baby step for each value, so the first match is the smallest x)",
     latex=r"g^{x} \equiv h \pmod m,\quad x = i\lceil\sqrt m\rceil + j",
     run=discrete_log, verify=_v_dlog, parse=parse_dlog, reference=_ref_dlog,
     wrong=lambda base, target, modulus: {"x": discrete_log(base, target, modulus)["x"] + 1},
     random_inputs=_rand_dlog,
     templates=["Find the smallest x with 3^x ≡ 13 (mod 17).", "Find the discrete logarithm of 13 to the base 3 modulo 17."],
     examples=[{"inputs": {"base": 3, "target": 13, "modulus": 17}, "outputs": {"x": 4}},
               {"inputs": {"base": 2, "target": 1, "modulus": 7}, "outputs": {"x": 0}},
               {"inputs": {"base": 5, "target": 8, "modulus": 23}, "outputs": {"x": 6}}],
     questions=[{"q": "Find the smallest x with 3^x ≡ 13 (mod 17).", "outputs": {"x": 4}},
                {"q": "Find the discrete logarithm of 13 to the base 3 modulo 17.", "outputs": {"x": 4}},
                {"q": "Find the smallest x with 2^x ≡ 3 (mod 7).", "outcome": "INVALID_INPUT"}])


def _jacobi(a, n):
    a %= n
    result = 1
    while a:
        while a % 2 == 0:
            a //= 2
            if n % 8 in (3, 5):
                result = -result
        a, n = n, a
        if a % 4 == 3 and n % 4 == 3:
            result = -result
        a %= n
    return result if n == 1 else 0


def _jacobi_wrong(a, n):
    a %= n
    result = 1
    while a:
        while a % 2 == 0:
            a //= 2                                   # forgot the (2/n) rule
        a, n = n, a
        if a % 4 == 3 and n % 4 == 3:
            result = -result
        a %= n
    return result if n == 1 else 0


def legendre_symbol(a, p):
    a, p = _whole(a, "a"), _whole(p, "p", 3)
    if p % 2 == 0 or not _is_prime(p):
        raise ValueError("p must be an odd prime")
    return {"symbol": _jacobi(a, p)}


def _v_legendre(i, o):
    a, p = int(i["a"]), int(i["p"])
    e = pow(a % p, (p - 1) // 2, p)
    return _iv(o["symbol"]) and o["symbol"] == {0: 0, 1: 1, p - 1: -1}.get(e, 99)


def parse_legendre(q0):
    q = q0.lower().replace("−", "-")
    m = re.search(r"legendre symbol\s*\(?\s*(-?\d+)\s*(?:/|\||over)\s*(\d+)\s*\)?", q)
    return {"a": int(m.group(1)), "p": int(m.group(2))} if m else None


_add("legendre_symbol", name="Legendre symbol", topic="number theory / quadratic residues",
     purpose="(a/p) for an odd prime p: 1 if a is a non-zero quadratic residue mod p, -1 if not, 0 if p divides a.",
     inputs=[_I("a", "the integer a"), _I("p", "an odd prime", 3)],
     outputs=[_O("symbol", "the Legendre symbol (a/p): -1, 0 or 1")],
     primary="symbol", conditions=["p an odd prime"],
     method="quadratic reciprocity with the supplementary law for 2 (no factoring needed)",
     latex=r"\left(\frac{a}{p}\right) \equiv a^{(p-1)/2} \pmod p",
     run=legendre_symbol, verify=_v_legendre, parse=parse_legendre,
     reference=lambda a, p: {"symbol": int(__import__("sympy").legendre_symbol(a % p, p))},
     wrong=lambda a, p: (lambda e: {"symbol": {1: 1, int(p) - 1: -1}.get(e, 0)})(pow(int(a) % int(p), (int(p) + 1) // 2, int(p))),
     random_inputs=lambda rng: (lambda p: {"a": 2 ** rng.randint(1, 3) * rng.randint(1, 10 ** 6), "p": p})(
         _rand_prime(rng, 3, 10 ** rng.randint(2, 12))),
     templates=["Find the Legendre symbol (5/11).", "Compute the Legendre symbol (-1/13)."],
     examples=[{"inputs": {"a": 5, "p": 11}, "outputs": {"symbol": 1}}, {"inputs": {"a": 2, "p": 7}, "outputs": {"symbol": 1}},
               {"inputs": {"a": 3, "p": 7}, "outputs": {"symbol": -1}}, {"inputs": {"a": 14, "p": 7}, "outputs": {"symbol": 0}}],
     questions=[{"q": "Find the Legendre symbol (5/11).", "outputs": {"symbol": 1}},
                {"q": "Compute the Legendre symbol (-1/13).", "outputs": {"symbol": 1}},
                {"q": "Compute the Legendre symbol (2/15).", "outcome": "INVALID_INPUT"}])


def jacobi_symbol(a, n):
    a, n = _whole(a, "a"), _whole(n, "n", 1)
    if n % 2 == 0:
        raise ValueError("n must be odd")
    return {"symbol": _jacobi(a, n)}


def _v_jacobi(i, o):
    a, n = int(i["a"]), int(i["n"])
    val = 1
    for p, e in _factor(n).items():
        r = pow(a % p, (p - 1) // 2, p)
        val *= {0: 0, 1: 1, p - 1: -1}[r] ** e
    return _iv(o["symbol"]) and o["symbol"] == val


def parse_jacobi(q0):
    q = q0.lower().replace("−", "-")
    m = re.search(r"jacobi symbol\s*\(?\s*(-?\d+)\s*(?:/|\||over)\s*(\d+)\s*\)?", q)
    return {"a": int(m.group(1)), "n": int(m.group(2))} if m else None


_add("jacobi_symbol", name="Jacobi symbol", topic="number theory / quadratic residues",
     purpose="(a/n) for an odd positive n, the product of Legendre symbols over the prime factors of n.",
     inputs=[_I("a", "the integer a"), _I("n", "an odd positive integer", 1)],
     outputs=[_O("symbol", "the Jacobi symbol (a/n): -1, 0 or 1")],
     primary="symbol", conditions=["n odd and positive"],
     method="quadratic reciprocity with the supplementary law for 2 (no factoring needed)",
     latex=r"\left(\frac{a}{n}\right) = \prod_i \left(\frac{a}{p_i}\right)^{e_i},\quad n = \prod p_i^{e_i}",
     run=jacobi_symbol, verify=_v_jacobi, parse=parse_jacobi,
     reference=lambda a, n: {"symbol": int(__import__("sympy").jacobi_symbol(a % n, n))},
     wrong=lambda a, n: (lambda e: {"symbol": {1: 1, int(n) - 1: -1}.get(e, 0)})(pow(int(a) % int(n), (int(n) - 1) // 2, int(n))),
     random_inputs=lambda rng: {"a": 2 ** rng.randint(1, 3) * rng.randint(1, 10 ** 6), "n": 2 * rng.randint(1, 10 ** rng.randint(2, 11)) + 1},
     templates=["Compute the Jacobi symbol (1001/9907).", "Find the Jacobi symbol (2/15)."],
     examples=[{"inputs": {"a": 1001, "n": 9907}, "outputs": {"symbol": -1}}, {"inputs": {"a": 2, "n": 15}, "outputs": {"symbol": 1}},
               {"inputs": {"a": 5, "n": 21}, "outputs": {"symbol": 1}}, {"inputs": {"a": 3, "n": 9}, "outputs": {"symbol": 0}}],
     questions=[{"q": "Compute the Jacobi symbol (1001/9907).", "outputs": {"symbol": -1}},
                {"q": "Find the Jacobi symbol (2/15).", "outputs": {"symbol": 1}},
                {"q": "Find the Jacobi symbol (3/10).", "outcome": "INVALID_INPUT"}])


def _tonelli(a, p):
    if p == 2:
        return a % 2
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def sqrt_mod(a, p):
    a, p = _whole(a, "a"), _whole(p, "p", 2)
    if not _is_prime(p):
        raise ValueError("p must be prime")
    a %= p
    if a == 0:
        roots = [0]
    elif p == 2:
        roots = [1]
    elif pow(a, (p - 1) // 2, p) != 1:
        roots = []
    else:
        r = _tonelli(a, p)
        roots = sorted({r, p - r})
    return {"roots": roots, "count": len(roots), "smallest": roots[0] if roots else None}


def _v_sqrtmod(i, o):
    a, p = int(i["a"]) % int(i["p"]), int(i["p"])
    rs = o["roots"]
    want = 1 if (a == 0 or p == 2) else (2 if pow(a, (p - 1) // 2, p) == 1 else 0)
    return (rs == sorted(set(rs)) and all(_iv(r) and 0 <= r < p and r * r % p == a for r in rs) and len(rs) == want
            and o["count"] == want and o["smallest"] == (rs[0] if rs else None))


def _ref_sqrtmod(a, p):
    from sympy.ntheory import sqrt_mod as sm
    rs = sorted(int(r) for r in (sm(a % p, p, all_roots=True) or []))
    return {"roots": rs, "count": len(rs), "smallest": rs[0] if rs else None}


def _rand_sqrtmod(rng):
    p = _rand_prime(rng, 2, 10 ** rng.randint(1, 12))
    x = rng.randint(1, p)
    return {"a": x * x % p if rng.random() < 0.75 else rng.randint(0, 10 ** 6), "p": p}


def parse_sqrtmod(q0):
    q = q0.lower()
    m = re.search(r"x\s*(?:\^|\*\*)\s*2\s*" + _EQ + r"\s*(-?\d+)\s*" + _MOD, q)
    if m:
        return {"a": int(m.group(1)), "p": int(m.group(2) or m.group(3))}
    m = re.search(r"square roots? of (-?\d+)\s*(?:\(\s*)?(?:mod|modulo)\s*(\d+)", q)
    return {"a": int(m.group(1)), "p": int(m.group(2))} if m else None


_add("sqrt_mod", name="Square roots modulo a prime", topic="number theory / quadratic residues",
     purpose="Every x in 0..p-1 with x^2 ≡ a (mod p), for a prime p.",
     inputs=[_I("a", "the integer a"), _I("p", "a prime modulus", 2)],
     outputs=[_O("roots", "all square roots of a modulo p, ascending", "list"), _O("count", "number of roots (0, 1 or 2)"),
              _O("smallest", "the smallest root (none when a is a non-residue)")],
     primary="smallest", conditions=["p prime"],
     method="Euler's criterion decides solvability; Tonelli-Shanks finds a root r, the other is p - r",
     latex=r"x^2 \equiv a \pmod p,\quad a^{(p-1)/2} \equiv 1",
     run=sqrt_mod, verify=_v_sqrtmod, parse=parse_sqrtmod, reference=_ref_sqrtmod,
     wrong=lambda a, p: (lambda r: dict(r, roots=r["roots"][-1:], count=min(1, r["count"]), smallest=(r["roots"][-1] if r["roots"] else None)))(sqrt_mod(a, p)),
     random_inputs=_rand_sqrtmod,
     templates=["Solve x^2 ≡ 10 (mod 13).", "Find the square roots of 2 modulo 7."],
     examples=[{"inputs": {"a": 10, "p": 13}, "outputs": {"roots": [6, 7], "count": 2, "smallest": 6}},
               {"inputs": {"a": 2, "p": 7}, "outputs": {"roots": [3, 4]}},
               {"inputs": {"a": 3, "p": 7}, "outputs": {"roots": [], "count": 0, "smallest": None}}],
     questions=[{"q": "Solve x^2 ≡ 10 (mod 13).", "outputs": {"roots": [6, 7]}},
                {"q": "Find the square roots of 2 modulo 7.", "outputs": {"smallest": 3}},
                {"q": "Solve x^2 ≡ 3 (mod 7).", "outputs": {"count": 0}},
                {"q": "Solve x^2 ≡ 4 (mod 15).", "outcome": "INVALID_INPUT"}])


def _carmichael_from(f: dict) -> int:
    lam = 1
    for p, e in f.items():
        l = (2 ** (e - 2) if e >= 3 else (1 if e == 1 else 2)) if p == 2 else p ** (e - 1) * (p - 1)
        lam = lam * l // math.gcd(lam, l)
    return lam


def _order(a, n):
    k = _carmichael_from(_factor(n)) if n > 1 else 1
    for p in _factor(k):
        while k % p == 0 and pow(a, k // p, n) == 1:
            k //= p
    return k


def multiplicative_order(a, n):
    a, n = _whole(a, "a"), _whole(n, "n", 2)
    if math.gcd(a, n) != 1:
        raise ValueError("a and n must be coprime")
    return {"order": _order(a % n, n)}


def _v_order(i, o):
    a, n, k = int(i["a"]) % int(i["n"]), int(i["n"]), o["order"]
    return _iv(k) and k >= 1 and pow(a, k, n) == 1 and all(pow(a, k // p, n) != 1 for p in _trial_factor(k))


def _rand_order(rng):
    n = rng.choice([rng.randint(3, 10 ** 6), _rand_prime(rng, 3, 10 ** 9), _rand_composite(rng, 10 ** 10)])
    while True:
        a = rng.randint(2, 10 ** 6)
        if math.gcd(a, n) == 1:
            return {"a": a, "n": n}


def parse_order(q0):
    q = q0.lower()
    m = re.search(r"order of (-?\d+)\s*(?:\(\s*)?(?:mod|modulo|in z_?)\s*(\d+)", q)
    return {"a": int(m.group(1)), "n": int(m.group(2))} if m else None


_add("multiplicative_order", name="Multiplicative order", topic="modular arithmetic / orders and primitive roots",
     purpose="The smallest k >= 1 with a^k ≡ 1 (mod n).",
     inputs=[_I("a", "the integer a (coprime to n)"), _I("n", "the modulus (at least 2)", 2)],
     outputs=[_O("order", "ord_n(a)")],
     primary="order", conditions=["gcd(a, n) = 1"],
     method="start from the Carmichael exponent lambda(n) and divide out prime factors while a^(k/p) ≡ 1",
     latex=r"\operatorname{ord}_n(a) = \min\{k \ge 1 : a^k \equiv 1 \pmod n\},\ \operatorname{ord}_n(a) \mid \lambda(n)",
     run=multiplicative_order, verify=_v_order, parse=parse_order,
     reference=lambda a, n: {"order": int(__import__("sympy").ntheory.n_order(a % n, n))},
     wrong=lambda a, n: {"order": euler_phi(n)["phi"]},
     random_inputs=_rand_order,
     templates=["What is the order of 2 modulo 7?", "Find the multiplicative order of 3 mod 100."],
     examples=[{"inputs": {"a": 2, "n": 7}, "outputs": {"order": 3}}, {"inputs": {"a": 3, "n": 100}, "outputs": {"order": 20}},
               {"inputs": {"a": 10, "n": 7}, "outputs": {"order": 6}}],
     questions=[{"q": "What is the order of 2 modulo 7?", "outputs": {"order": 3}},
                {"q": "Find the multiplicative order of 3 mod 100.", "outputs": {"order": 20}},
                {"q": "Find the multiplicative order of 4 mod 10.", "outcome": "INVALID_INPUT"}])


def primitive_root(p):
    p = _whole(p, "p", 2, 10 ** 12)
    if not _is_prime(p):
        raise ValueError("p must be prime")
    if p == 2:
        return {"root": 1}
    qs = list(_factor(p - 1))
    g = 2
    while any(pow(g, (p - 1) // q, p) == 1 for q in qs):
        g += 1
    return {"root": g}


def _v_proot(i, o):
    p, r = int(i["p"]), o["root"]
    if not _iv(r):
        return False
    if p == 2:
        return r == 1
    qs = list(_trial_factor(p - 1)) if p < 10 ** 10 else list(_factor(p - 1))
    prim = lambda g: all(pow(g, (p - 1) // q, p) != 1 for q in qs)
    return 2 <= r < p and prim(r) and not any(prim(g) for g in range(2, r))


def parse_proot(q0):
    q = q0.lower()
    m = re.search(r"primitive root\s*(?:of|mod|modulo)\s*(?:p\s*=\s*)?(\d+)", q)
    return {"p": int(m.group(1))} if m else None


_add("primitive_root", name="Smallest primitive root modulo a prime", topic="modular arithmetic / orders and primitive roots",
     purpose="The smallest g whose powers run through every non-zero residue modulo the prime p.",
     inputs=[_I("p", "a prime", 2, 10 ** 12)],
     outputs=[_O("root", "the smallest primitive root modulo p")],
     primary="root", conditions=["p prime (primitive roots also exist for 4, p^k and 2p^k, which this capability does not take)"],
     method="g is a primitive root iff g^((p-1)/q) ≢ 1 for every prime q dividing p - 1; test g = 2, 3, ...",
     latex=r"g^{(p-1)/q} \not\equiv 1 \pmod p \ \ \forall q \mid p-1",
     run=primitive_root, verify=_v_proot, parse=parse_proot,
     reference=lambda p: {"root": int(__import__("sympy").ntheory.primitive_root(p))},
     wrong=lambda p: {"root": 2 if int(p) > 2 else 1},
     random_inputs=lambda rng: {"p": _rand_prime(rng, 3, 10 ** rng.randint(2, 11))},
     templates=["Find the smallest primitive root modulo 23.", "What is the primitive root of 41?"],
     examples=[{"inputs": {"p": 23}, "outputs": {"root": 5}}, {"inputs": {"p": 7}, "outputs": {"root": 3}},
               {"inputs": {"p": 41}, "outputs": {"root": 6}}, {"inputs": {"p": 2}, "outputs": {"root": 1}}],
     questions=[{"q": "Find the smallest primitive root modulo 23.", "outputs": {"root": 5}},
                {"q": "What is the primitive root of 41?", "outputs": {"root": 6}},
                {"q": "Find the primitive root modulo 15.", "outcome": "INVALID_INPUT"}])


def prime_counting(x):
    x = _whole(x, "x", 0, 5 * 10 ** 6)
    return {"count": sum(_sieve(x)) if x >= 2 else 0}


def _odd_sieve_count(x):
    if x < 2:
        return 0
    size = (x - 1) // 2                          # index i stands for 2i + 1, i >= 1
    s = bytearray([1]) * (size + 1)
    s[0] = 0
    i = 1
    while (2 * i + 1) ** 2 <= x:
        if s[i]:
            p = 2 * i + 1
            start = (p * p - 1) // 2
            s[start::p] = bytearray(len(range(start, size + 1, p)))
        i += 1
    return 1 + sum(s)


def parse_pcount(q0):
    q = q0.lower()
    m = re.search(r"how many primes?(?: numbers)? (?:are there |are )?(less than or equal to|less than|below|under|up to|"
                  r"not exceeding|at most|smaller than|<=|≤|<|between 1 and)\s*(\d+)", q)
    if m:
        x = int(m.group(2))
        return {"x": x - 1 if m.group(1) in ("less than", "below", "under", "smaller than", "<") else x}
    m = re.search(r"(?:\bpi|π)\s*\(\s*(\d+)\s*\)", q)
    if m and re.search(r"prime", q):
        return {"x": int(m.group(1))}
    return None


_add("prime_counting", name="Prime-counting function", topic="number theory / primes",
     purpose="pi(x): how many primes are at most x.",
     inputs=[_I("x", "the bound (0..5,000,000)", 0, 5 * 10 ** 6)],
     outputs=[_O("count", "number of primes p <= x")],
     primary="count", conditions=["0 <= x <= 5,000,000", "'less than N' counts primes up to N - 1"],
     method="sieve of Eratosthenes", latex=r"\pi(x) = \#\{p \le x : p \text{ prime}\}",
     run=prime_counting, verify=lambda i, o: _iv(o["count"]) and o["count"] == _odd_sieve_count(int(i["x"])),
     parse=parse_pcount, reference=lambda x: {"count": int(__import__("sympy").primepi(x))},
     wrong=lambda x: {"count": round(int(x) / math.log(int(x))) if int(x) > 1 else 0},
     random_inputs=lambda rng: {"x": rng.randint(0, 10 ** rng.randint(1, 6))},
     templates=["How many primes are there less than 100?", "How many primes are at most 1000?", "Compute the prime counting function pi(10000)."],
     examples=[{"inputs": {"x": 100}, "outputs": {"count": 25}}, {"inputs": {"x": 1000}, "outputs": {"count": 168}},
               {"inputs": {"x": 1}, "outputs": {"count": 0}}, {"inputs": {"x": 1000000}, "outputs": {"count": 78498}}],
     questions=[{"q": "How many primes are there less than 100?", "outputs": {"count": 25}},
                {"q": "How many primes are at most 1000?", "outputs": {"count": 168}},
                {"q": "Compute the prime counting function pi(10000).", "outputs": {"count": 1229}},
                {"q": "How many primes are there below 10?", "outputs": {"count": 4}}])


def nth_prime(n):
    n = _whole(n, "n", 1, 300000)
    lim = 15 if n < 6 else int(n * (math.log(n) + math.log(math.log(n)))) + 3
    s = _sieve(lim)
    c = 0
    for i in range(lim + 1):
        if s[i]:
            c += 1
            if c == n:
                return {"prime": i}
    raise ValueError("internal bound too small")


def parse_nthprime(q0):
    q = q0.lower()
    if re.search(r"how many|factor|greater|less|after|before", q):
        return None
    m = re.search(r"\b(\d+)(?:st|nd|rd|th) prime\b", q)
    return {"n": int(m.group(1))} if m else None


_add("nth_prime", name="The n-th prime", topic="number theory / primes",
     purpose="The n-th prime number p_n (p_1 = 2).",
     inputs=[_I("n", "the index n (1..300,000)", 1, 300000)],
     outputs=[_O("prime", "the n-th prime")],
     primary="prime", conditions=["1 <= n <= 300,000", "p_1 = 2"],
     method="sieve of Eratosthenes up to the Rosser bound n (ln n + ln ln n)",
     latex=r"p_n < n(\ln n + \ln\ln n)\quad (n \ge 6)",
     run=nth_prime, verify=lambda i, o: _iv(o["prime"]) and _is_prime_alt(o["prime"]) and _odd_sieve_count(o["prime"]) == int(i["n"]),
     parse=parse_nthprime, reference=lambda n: {"prime": int(__import__("sympy").prime(n))},
     wrong=lambda n: {"prime": nth_prime(int(n) + 1)["prime"]},
     random_inputs=lambda rng: {"n": rng.randint(1, 10 ** rng.randint(1, 5))},
     templates=["What is the 100th prime?", "Find the 10001st prime number."],
     examples=[{"inputs": {"n": 1}, "outputs": {"prime": 2}}, {"inputs": {"n": 100}, "outputs": {"prime": 541}},
               {"inputs": {"n": 10001}, "outputs": {"prime": 104743}}],
     questions=[{"q": "What is the 100th prime?", "outputs": {"prime": 541}},
                {"q": "Find the 10001st prime number.", "outputs": {"prime": 104743}},
                {"q": "What is the 400000th prime?", "outcome": "INVALID_INPUT"}])


def next_prime(n):
    n = _whole(n, "n", 0)
    p = n + 1
    while not _is_prime(p):
        p += 1
    return {"prime": p}


def previous_prime(n):
    n = _whole(n, "n", 3)
    p = n - 1
    while not _is_prime(p):
        p -= 1
    return {"prime": p}


def _v_next(i, o):
    n, p = int(i["n"]), o["prime"]
    return _iv(p) and p > n and _is_prime_alt(p) and not any(_is_prime_alt(k) for k in range(n + 1, p))


def _v_prev(i, o):
    n, p = int(i["n"]), o["prime"]
    return _iv(p) and p < n and _is_prime_alt(p) and not any(_is_prime_alt(k) for k in range(p + 1, n))


def parse_next(q0):
    q = q0.lower()
    m = re.search(r"(?:smallest|least|first|next) prime(?: number)? (?:that is )?(?:greater than|larger than|bigger than|"
                  r"after|above|exceeding|>)\s*(\d+)", q)
    return {"n": int(m.group(1))} if m else None


def parse_prev(q0):
    q = q0.lower()
    m = re.search(r"(?:largest|greatest|biggest|previous|last) prime(?: number)? (?:that is )?(?:less than|smaller than|"
                  r"below|before|under|<)\s*(\d+)", q)
    return {"n": int(m.group(1))} if m else None


def _rand_np(rng):
    return {"n": _rand_prime(rng, 3, 10 ** rng.randint(1, 5)) if rng.random() < 0.75 else rng.randint(3, 10 ** rng.randint(1, 5))}


_add("next_prime", name="Next prime", topic="number theory / primes",
     purpose="The smallest prime strictly greater than n.",
     inputs=[_I("n", "the whole number n (at least 0)", 0)],
     outputs=[_O("prime", "the smallest prime p > n")],
     primary="prime", conditions=["n >= 0", "strictly greater: the next prime after a prime p is not p"],
     method="test n + 1, n + 2, ... with deterministic Miller-Rabin", latex=r"\min\{p > n : p \text{ prime}\}",
     run=next_prime, verify=_v_next, parse=parse_next, reference=lambda n: {"prime": int(__import__("sympy").nextprime(n))},
     wrong=lambda n: {"prime": int(n) if _is_prime(int(n)) else next_prime(n)["prime"]},
     random_inputs=_rand_np,
     templates=["What is the smallest prime greater than 100?", "Find the next prime after 1000."],
     examples=[{"inputs": {"n": 100}, "outputs": {"prime": 101}}, {"inputs": {"n": 13}, "outputs": {"prime": 17}},
               {"inputs": {"n": 0}, "outputs": {"prime": 2}}],
     questions=[{"q": "What is the smallest prime greater than 100?", "outputs": {"prime": 101}},
                {"q": "Find the next prime after 1000.", "outputs": {"prime": 1009}}])

_add("previous_prime", name="Previous prime", topic="number theory / primes",
     purpose="The largest prime strictly less than n.",
     inputs=[_I("n", "the whole number n (at least 3)", 3)],
     outputs=[_O("prime", "the largest prime p < n")],
     primary="prime", conditions=["n >= 3", "strictly less: the prime before a prime p is not p"],
     method="test n - 1, n - 2, ... with deterministic Miller-Rabin", latex=r"\max\{p < n : p \text{ prime}\}",
     run=previous_prime, verify=_v_prev, parse=parse_prev, reference=lambda n: {"prime": int(__import__("sympy").prevprime(n))},
     wrong=lambda n: {"prime": int(n) if _is_prime(int(n)) else previous_prime(n)["prime"]},
     random_inputs=_rand_np,
     templates=["What is the largest prime less than 100?", "Find the largest prime below 1000."],
     examples=[{"inputs": {"n": 100}, "outputs": {"prime": 97}}, {"inputs": {"n": 13}, "outputs": {"prime": 11}},
               {"inputs": {"n": 3}, "outputs": {"prime": 2}}],
     questions=[{"q": "What is the largest prime less than 100?", "outputs": {"prime": 97}},
                {"q": "Find the largest prime below 1000.", "outputs": {"prime": 997}},
                {"q": "What is the largest prime less than 2?", "outcome": "INVALID_INPUT"}])


def carmichael_lambda(n):
    n = _whole(n, "n", 1)
    return {"lambda": _carmichael_from(_factor(n)) if n > 1 else 1}


def _v_carm(i, o):
    n, lam = int(i["n"]), o["lambda"]
    if not _iv(lam) or lam < 1:
        return False
    f = _trial_factor(n) if n < 10 ** 10 else _factor(n)
    want = 1
    for p, e in f.items():                       # lambda(p^e) as the order of the group of units, via phi
        l = p ** (e - 1) * (p - 1)
        if p == 2 and e >= 3:
            l //= 2
        want = math.lcm(want, l)
    return lam == want and all(pow(a, lam, n) == 1 % n for a in range(2, 30) if math.gcd(a, n) == 1)


def parse_carm(q0):
    q = q0.lower()
    m = re.search(r"(?:carmichael (?:function|lambda)|reduced totient)\D*?(\d+)", q) or \
        re.search(r"(?:λ|\blambda)\s*\(\s*(\d+)\s*\)", q)
    return {"n": int(m.group(1))} if m else None


_add("carmichael_lambda", name="Carmichael function", topic="number theory / arithmetic functions",
     purpose="lambda(n): the smallest m with a^m ≡ 1 (mod n) for every a coprime to n (the exponent of the unit group).",
     inputs=[_I("n", "a positive integer", 1)],
     outputs=[_O("lambda", "Carmichael's lambda(n)")],
     primary="lambda", conditions=["n >= 1"],
     method="lcm over prime powers: lambda(2^e) = 2^(e-2) for e >= 3, else phi(p^e)",
     latex=r"\lambda(n) = \operatorname{lcm}\big(\lambda(p_1^{e_1}),\dots\big),\ \lambda(p^e)=p^{e-1}(p-1)",
     run=carmichael_lambda, verify=_v_carm, parse=parse_carm,
     reference=lambda n: {"lambda": int(__import__("sympy").reduced_totient(n))},
     wrong=lambda n: {"lambda": euler_phi(n)["phi"]},
     random_inputs=lambda rng: {"n": _rand_composite(rng, 10 ** 12)},
     templates=["Compute the Carmichael function of 15.", "What is the reduced totient of 561?", "Find λ(100)."],
     examples=[{"inputs": {"n": 15}, "outputs": {"lambda": 4}}, {"inputs": {"n": 561}, "outputs": {"lambda": 80}},
               {"inputs": {"n": 8}, "outputs": {"lambda": 2}}, {"inputs": {"n": 100}, "outputs": {"lambda": 20}}],
     questions=[{"q": "Compute the Carmichael function of 15.", "outputs": {"lambda": 4}},
                {"q": "What is the reduced totient of 561?", "outputs": {"lambda": 80}},
                {"q": "Find λ(100).", "outputs": {"lambda": 20}}])


def mobius(n):
    n = _whole(n, "n", 1)
    f = _factor(n)
    sqfree = all(e == 1 for e in f.values())
    return {"mu": ((-1) ** len(f)) if sqfree else 0, "squarefree": sqfree}


def _v_mobius(i, o):
    n = int(i["n"])
    f = _trial_factor(n) if n < 10 ** 10 else _factor(n)
    sq = all(e == 1 for e in f.values())
    return _iv(o["mu"]) and o["squarefree"] is sq and o["mu"] == (0 if not sq else (-1) ** len(f))


def parse_mobius(q0):
    q = q0.lower()
    m = re.search(r"m[öo]e?bius (?:function )?(?:of |for )?(?:μ\s*\(|mu\s*\()?\s*(\d+)", q) or \
        re.search(r"(?:μ|\bmu)\s*\(\s*(\d+)\s*\)", q)
    return {"n": int(m.group(1))} if m else None


_add("mobius", name="Möbius function", topic="number theory / arithmetic functions",
     purpose="mu(n): 0 if a square divides n, otherwise (-1)^(number of prime factors).",
     inputs=[_I("n", "a positive integer", 1)],
     outputs=[_O("mu", "the Möbius function mu(n)"), _O("squarefree", "true when no square > 1 divides n", "boolean")],
     primary="mu", conditions=["n >= 1"], method="prime factorization",
     latex=r"\mu(n) = \begin{cases} (-1)^k & n = p_1\cdots p_k \text{ squarefree} \\ 0 & p^2 \mid n \end{cases}",
     run=mobius, verify=_v_mobius, parse=parse_mobius,
     reference=lambda n: {"mu": int(__import__("sympy").mobius(n))},
     wrong=lambda n: (lambda f: {"mu": (-1) ** sum(f.values()), "squarefree": True})(_factor(int(n))),
     random_inputs=lambda rng: {"n": _rand_composite(rng, 10 ** 12) * rng.choice([4, 9, 25, 1, 1])},
     templates=["Compute the Möbius function of 30.", "What is mu(12)?"],
     examples=[{"inputs": {"n": 30}, "outputs": {"mu": -1, "squarefree": True}}, {"inputs": {"n": 12}, "outputs": {"mu": 0}},
               {"inputs": {"n": 1}, "outputs": {"mu": 1}}, {"inputs": {"n": 35}, "outputs": {"mu": 1}}],
     questions=[{"q": "Compute the Möbius function of 30.", "outputs": {"mu": -1}},
                {"q": "What is mu(12)?", "outputs": {"mu": 0}}])

def cf_rational(numerator, denominator):
    p, q = _whole(numerator, "numerator"), _whole(denominator, "denominator", 1)
    terms = []
    while q:
        a = p // q
        terms.append(a)
        p, q = q, p - a * q
    return {"terms": terms, "length": len(terms)}


def _v_cfr(i, o):
    t = o["terms"]
    if not t or not all(_iv(a) for a in t) or any(a < 1 for a in t[1:]) or (len(t) > 1 and t[-1] < 2) or o["length"] != len(t):
        return False
    v = Fraction(t[-1])
    for a in reversed(t[:-1]):
        v = a + 1 / v
    return v == Fraction(int(i["numerator"]), int(i["denominator"]))


def _ref_cfr(numerator, denominator):
    from sympy import Rational
    from sympy.ntheory.continued_fraction import continued_fraction
    t = [int(a) for a in continued_fraction(Rational(int(numerator), int(denominator)))]
    return {"terms": t, "length": len(t)}


def _wrong_cfr(numerator, denominator):
    t = cf_rational(numerator, denominator)["terms"]
    t = t[:-1] + [t[-1] - 1, 1] if len(t) > 1 or t[-1] > 1 else t + [1]      # the non-canonical ending [..., a - 1, 1]
    return {"terms": t, "length": len(t)}


def parse_cfr(q0):
    q = q0.lower().replace("−", "-")
    if "continued fraction" not in q or re.search(r"sqrt|√|root", q):
        return None
    m = re.search(r"(-?\d+)\s*/\s*(\d+)", q)
    return {"numerator": int(m.group(1)), "denominator": int(m.group(2))} if m else {}


_add("cf_rational", name="Continued fraction of a rational number", topic="number theory / continued fractions",
     purpose="The finite simple continued fraction [a0; a1, ..., an] of p/q (canonical form: last term >= 2).",
     inputs=[_I("numerator", "the numerator p"), _I("denominator", "the denominator q (at least 1)", 1)],
     outputs=[_O("terms", "the partial quotients [a0, a1, ..., an]", "list"), _O("length", "number of partial quotients")],
     primary="length", conditions=["q >= 1", "a0 = floor(p/q) (negative for negative p); canonical: the last term is at least 2"],
     method="Euclidean algorithm: a_k = floor(p/q), then (p, q) <- (q, p - a_k q)",
     latex=r"\frac{p}{q} = a_0 + \cfrac{1}{a_1 + \cfrac{1}{\ddots + \cfrac{1}{a_n}}}",
     run=cf_rational, verify=_v_cfr, parse=parse_cfr, reference=_ref_cfr, wrong=_wrong_cfr,
     random_inputs=lambda rng: {"numerator": rng.randint(-10 ** 6, 10 ** 9), "denominator": rng.randint(1, 10 ** 6)},
     templates=["Find the continued fraction of 415/93.", "What is the continued fraction expansion of 355/113?"],
     examples=[{"inputs": {"numerator": 415, "denominator": 93}, "outputs": {"terms": [4, 2, 6, 7], "length": 4}},
               {"inputs": {"numerator": 355, "denominator": 113}, "outputs": {"terms": [3, 7, 16]}},
               {"inputs": {"numerator": -7, "denominator": 3}, "outputs": {"terms": [-3, 1, 2]}}],
     questions=[{"q": "Find the continued fraction of 415/93.", "outputs": {"terms": [4, 2, 6, 7]}},
                {"q": "What is the continued fraction expansion of 355/113?", "outputs": {"terms": [3, 7, 16]}},
                {"q": "Find the continued fraction of 5/0.", "outcome": "INVALID_INPUT"}])


def cf_sqrt(d):
    d = _whole(d, "d", 2, 10 ** 12)
    a0 = math.isqrt(d)
    if a0 * a0 == d:
        raise ValueError("d must not be a perfect square (its square root is an integer)")
    m, den, a, period = 0, 1, a0, []
    while a != 2 * a0:
        m = den * a - m
        den = (d - m * m) // den
        a = (a0 + m) // den
        period.append(a)
    return {"a0": a0, "period": period, "period_length": len(period)}


def _convergent(terms):
    h0, h1, k0, k1 = 1, terms[0], 0, 1
    for a in terms[1:]:
        h0, h1 = h1, a * h1 + h0
        k0, k1 = k1, a * k1 + k0
    return h1, k1


def _v_cfs(i, o):
    d, a0, per = int(i["d"]), o["a0"], o["period"]
    if not (_iv(a0) and a0 == math.isqrt(d) and per and per[-1] == 2 * a0 and o["period_length"] == len(per)):
        return False
    if per[:-1] != per[:-1][::-1]:                      # the period before 2*a0 is a palindrome
        return False
    p, q = _convergent([a0] + per[:-1])
    return p * p - d * q * q == (-1) ** len(per)


def _ref_cfs(d):
    from sympy.ntheory.continued_fraction import continued_fraction_periodic
    cf = continued_fraction_periodic(0, 1, int(d))
    per = [int(a) for a in cf[1]]
    return {"a0": int(cf[0]), "period": per, "period_length": len(per)}


def parse_cfs(q0):
    q = q0.lower()
    if "continued fraction" not in q:
        return None
    m = re.search(r"(?:sqrt|√|square root of)\s*\(?\s*(\d+)\s*\)?", q)
    return {"d": int(m.group(1))} if m else None


_add("cf_sqrt", name="Continued fraction of a square root", topic="number theory / continued fractions",
     purpose="The periodic continued fraction sqrt(d) = [a0; (a1, ..., ak)] of a non-square d, and its period length.",
     inputs=[_I("d", "a positive non-square integer", 2, 10 ** 12)],
     outputs=[_O("a0", "integer part floor(sqrt(d))"), _O("period", "the repeating block (ends with 2*a0)", "list"),
              _O("period_length", "length of the period")],
     primary="period_length", conditions=["d not a perfect square"],
     method="the exact recurrence m' = den*a - m, den' = (d - m'^2)/den, a' = floor((a0 + m')/den') until a' = 2*a0",
     latex=r"\sqrt d = [a_0; \overline{a_1, \dots, a_{k-1}, 2a_0}]",
     run=cf_sqrt, verify=_v_cfs, parse=parse_cfs, reference=_ref_cfs,
     wrong=lambda d: (lambda r: dict(r, period=r["period"][:-1] or [0], period_length=max(1, r["period_length"] - 1)))(cf_sqrt(d)),
     random_inputs=lambda rng: {"d": rng.randint(2, 10 ** rng.randint(1, 6))},
     templates=["Find the continued fraction of sqrt(23).", "What is the continued fraction expansion of √14?"],
     examples=[{"inputs": {"d": 23}, "outputs": {"a0": 4, "period": [1, 3, 1, 8], "period_length": 4}},
               {"inputs": {"d": 2}, "outputs": {"a0": 1, "period": [2]}},
               {"inputs": {"d": 14}, "outputs": {"a0": 3, "period": [1, 2, 1, 6]}}],
     questions=[{"q": "Find the continued fraction of sqrt(23).", "outputs": {"period": [1, 3, 1, 8]}},
                {"q": "What is the continued fraction expansion of √14?", "outputs": {"a0": 3, "period_length": 4}},
                {"q": "Find the continued fraction of sqrt(16).", "outcome": "INVALID_INPUT"}])


def pell(d):
    r = cf_sqrt(_whole(d, "d", 2, 1000))
    terms = [r["a0"]] + r["period"][:-1]
    p, q = _convergent(terms)
    if r["period_length"] % 2:                    # odd period: the first solution solves x^2 - d y^2 = -1; square it
        p, q = p * p + int(d) * q * q, 2 * p * q
    return {"x": p, "y": q}


def _v_pell(i, o):
    d, x, y = int(i["d"]), o["x"], o["y"]
    if not (_iv(x) and _iv(y) and y >= 1 and x * x - d * y * y == 1):
        return False
    a0 = math.isqrt(d)                            # the first convergent solving x^2 - d y^2 = 1 is the fundamental one
    m, den, a = 0, 1, a0
    h0, h1, k0, k1 = 1, a0, 0, 1
    while h1 * h1 - d * k1 * k1 != 1:
        m = den * a - m
        den = (d - m * m) // den
        a = (a0 + m) // den
        h0, h1 = h1, a * h1 + h0
        k0, k1 = k1, a * k1 + k0
    return (x, y) == (h1, k1)


def _ref_pell(d):
    from sympy.solvers.diophantine.diophantine import diop_DN
    x, y = min((s for s in diop_DN(int(d), 1) if s[1] > 0), key=lambda s: s[1])
    return {"x": int(x), "y": int(y)}


def parse_pell(q0):
    q = q0.lower().replace("−", "-")
    m = re.search(r"x\s*(?:\^|\*\*|²)\s*2?\s*-\s*(\d+)\s*\*?\s*y\s*(?:\^|\*\*|²)\s*2?\s*=\s*1\b", q)
    if m:
        return {"d": int(m.group(1))}
    m = re.search(r"pell'?s? equation (?:with|for) d\s*=\s*(\d+)", q)
    return {"d": int(m.group(1))} if m else None


def _rand_pell(rng):
    while True:
        d = rng.randint(2, 1000)
        if math.isqrt(d) ** 2 != d:
            return {"d": d}


_add("pell_equation", name="Pell's equation (fundamental solution)", topic="number theory / Diophantine equations",
     purpose="The smallest positive solution (x, y) of x^2 - d y^2 = 1 for a non-square d.",
     inputs=[_I("d", "a non-square positive integer (2..1000)", 2, 1000)],
     outputs=[_O("x", "fundamental solution x"), _O("y", "fundamental solution y")],
     primary="x", conditions=["d not a perfect square", "2 <= d <= 1000"],
     method="convergent of the continued fraction of sqrt(d) at the end of the first period (squared when the period is odd)",
     latex=r"x^2 - d\,y^2 = 1,\quad \frac{x}{y} = [a_0; a_1, \dots, a_{k-1}]",
     run=pell, verify=_v_pell, parse=parse_pell, reference=_ref_pell,
     wrong=lambda d: (lambda r: {"x": r["x"] ** 2 + int(d) * r["y"] ** 2, "y": 2 * r["x"] * r["y"]})(pell(d)),
     random_inputs=_rand_pell,
     templates=["Find the fundamental solution of x^2 - 61y^2 = 1.", "Solve Pell's equation for d = 7."],
     examples=[{"inputs": {"d": 61}, "outputs": {"x": 1766319049, "y": 226153980}}, {"inputs": {"d": 2}, "outputs": {"x": 3, "y": 2}},
               {"inputs": {"d": 7}, "outputs": {"x": 8, "y": 3}}, {"inputs": {"d": 13}, "outputs": {"x": 649, "y": 180}}],
     questions=[{"q": "Find the fundamental solution of x^2 - 61y^2 = 1.", "outputs": {"x": 1766319049, "y": 226153980}},
                {"q": "Solve Pell's equation for d = 7.", "outputs": {"x": 8, "y": 3}},
                {"q": "Find the fundamental solution of x^2 - 9y^2 = 1.", "outcome": "INVALID_INPUT"}])


def two_squares(n):
    n = _whole(n, "n", 0, 10 ** 12)
    reps = []
    x = 0
    while 2 * x * x <= n:
        y2 = n - x * x
        y = math.isqrt(y2)
        if y * y == y2:
            reps.append([x, y])
        x += 1
    if n == 0:
        r2 = 1
    else:
        d1 = d3 = 0
        for d in _divisors_from({p: e for p, e in _factor(n).items() if p != 2}):
            if d % 4 == 1:
                d1 += 1
            else:
                d3 += 1
        r2 = 4 * (d1 - d3)
    return {"representations": reps, "count": len(reps), "r2": r2}


def _mult(x, y):
    if x == 0 and y == 0:
        return 1
    if x == 0 or x == y:
        return 4
    return 8


def _v_twosq(i, o):
    n, reps = int(i["n"]), o["representations"]
    ok = all(_iv(x) and _iv(y) and 0 <= x <= y and x * x + y * y == n for x, y in reps) and reps == sorted(reps)
    return ok and o["count"] == len(reps) and _iv(o["count"]) and o["r2"] == sum(_mult(x, y) for x, y in reps)


def _ref_twosq(n):
    import gmpy2
    reps = [[x, int(gmpy2.isqrt(n - x * x))] for x in range(int(gmpy2.isqrt(n // 2)) + 1) if gmpy2.is_square(n - x * x)]
    return {"representations": reps, "count": len(reps), "r2": sum(_mult(x, y) for x, y in reps)}


def parse_twosq(q0):
    q = q0.lower()
    if "sum of two squares" not in q and "sum of two square numbers" not in q:
        return None
    nums = _ints(q)
    if len(nums) != 1:
        return {} if not nums else None
    return {"n": nums[0]}


def _rand_twosq(rng):
    n = 1
    for _ in range(rng.randint(1, 4)):
        n *= rng.choice([2, 5, 13, 17, 29, 37, 41, 3, 7, 53, 61, 9])
    return {"n": n * rng.choice([1, 1, 4, 25]) if rng.random() < 0.8 else rng.randint(0, 10 ** 8)}


_add("two_squares", name="Sums of two squares", topic="number theory / sums of squares",
     purpose="Every way to write n = x^2 + y^2 with 0 <= x <= y, and r2(n), the number of ordered signed representations.",
     inputs=[_I("n", "a whole number (0..10^12)", 0, 10 ** 12)],
     outputs=[_O("representations", "pairs [x, y] with 0 <= x <= y and x^2 + y^2 = n", "list"),
              _O("count", "number of such pairs"), _O("r2", "r2(n): ordered representations with signs")],
     primary="count", conditions=["0 <= n <= 10^12"],
     method="search x up to sqrt(n/2) with an exact integer square root; r2(n) = 4 (d1(n) - d3(n)) from the divisors",
     latex=r"r_2(n) = 4\big(d_1(n) - d_3(n)\big)",
     run=two_squares, verify=_v_twosq, parse=parse_twosq, reference=_ref_twosq,
     wrong=lambda n: (lambda r: dict(r, r2=4 * sum(1 for d in _divisors_from(_factor(int(n))) if d % 4 == 1) if int(n) else 1))(two_squares(n)),
     random_inputs=_rand_twosq,
     templates=["Write 325 as a sum of two squares.", "In how many ways is 25 a sum of two squares?"],
     examples=[{"inputs": {"n": 325}, "outputs": {"representations": [[1, 18], [6, 17], [10, 15]], "count": 3, "r2": 24}},
               {"inputs": {"n": 25}, "outputs": {"representations": [[0, 5], [3, 4]], "r2": 12}},
               {"inputs": {"n": 3}, "outputs": {"count": 0, "r2": 0}}],
     questions=[{"q": "Write 325 as a sum of two squares.", "outputs": {"count": 3}},
                {"q": "In how many ways is 25 a sum of two squares?", "outputs": {"representations": [[0, 5], [3, 4]]}}])


def integer_sqrt(n):
    n = _whole(n, "n", 0)
    r = math.isqrt(n)
    return {"isqrt": r, "remainder": n - r * r, "is_square": r * r == n}


def _v_isqrt(i, o):
    n, r = int(i["n"]), o["isqrt"]
    return _iv(r) and r >= 0 and r * r <= n < (r + 1) ** 2 and o["remainder"] == n - r * r and o["is_square"] is (r * r == n)


def _ref_isqrt(n):
    import gmpy2
    r, rem = gmpy2.isqrt_rem(n)
    return {"isqrt": int(r), "remainder": int(rem), "is_square": bool(gmpy2.is_square(n))}


def parse_isqrt(q0):
    q = q0.lower()
    m = re.search(r"integer square root of (\d+)|isqrt\s*\(\s*(\d+)|is (\d+) a perfect square|floor of (?:the )?square root of (\d+)", q)
    return {"n": int(_first(*m.groups()))} if m else None


_add("integer_sqrt", name="Integer square root and perfect squares", topic="number theory / squares",
     purpose="floor(sqrt(n)) computed exactly, the remainder n - floor(sqrt(n))^2, and whether n is a perfect square.",
     inputs=[_I("n", "a whole number (at least 0)", 0)],
     outputs=[_O("isqrt", "floor(sqrt(n))"), _O("remainder", "n - isqrt^2"), _O("is_square", "true when n is a perfect square", "boolean")],
     primary="isqrt", conditions=["n >= 0"], method="exact integer Newton iteration (no floating point)",
     latex=r"r = \lfloor \sqrt n \rfloor \iff r^2 \le n < (r+1)^2",
     run=integer_sqrt, verify=_v_isqrt, parse=parse_isqrt, reference=_ref_isqrt,
     wrong=lambda n: (lambda r: dict(r, remainder=int(n) - r["isqrt"]))(integer_sqrt(n)),
     random_inputs=lambda rng: {"n": rng.choice([rng.randint(2, 10 ** 15), rng.randint(2, 10 ** 6) ** 2, rng.randint(2, 1000)])},
     templates=["What is the integer square root of 1000?", "Is 144 a perfect square?"],
     examples=[{"inputs": {"n": 1000}, "outputs": {"isqrt": 31, "remainder": 39, "is_square": False}},
               {"inputs": {"n": 144}, "outputs": {"isqrt": 12, "remainder": 0, "is_square": True}},
               {"inputs": {"n": 0}, "outputs": {"isqrt": 0, "is_square": True}}],
     questions=[{"q": "What is the integer square root of 1000?", "outputs": {"isqrt": 31}},
                {"q": "Is 144 a perfect square?", "outputs": {"is_square": True}},
                {"q": "Is 145 a perfect square?", "outputs": {"is_square": False}}])


# ==================================================================================================================
# number representations
# ==================================================================================================================
_DIG = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
_BASES = {"binary": 2, "octal": 8, "decimal": 10, "hexadecimal": 16, "hex": 16, "ternary": 3, "base ten": 10}


def _to_base(v: int, b: int) -> str:
    if v == 0:
        return "0"
    neg, v, out = v < 0, abs(v), []
    while v:
        v, r = divmod(v, b)
        out.append(_DIG[r])
    return ("-" if neg else "") + "".join(reversed(out))


def base_convert(digits, from_base, to_base):
    fb, tb = _whole(from_base, "from_base", 2, 36), _whole(to_base, "to_base", 2, 36)
    s = str(digits).strip().upper().replace("_", "").replace(" ", "")
    neg = s.startswith("-")
    s = s[1:] if neg else s
    if not s or len(s) > 200:
        raise ValueError("give the number's digits (at most 200)")
    v = 0
    for ch in s:
        d = _DIG.find(ch)
        if d < 0 or d >= fb:
            raise ValueError(f"'{ch}' is not a digit in base {fb}")
        v = v * fb + d
    v = -v if neg else v
    if abs(v) > 10 ** 300:
        raise ValueError("the number is too large")
    return {"result": _to_base(v, tb), "decimal": v}


def _v_base(i, o):
    s = str(i["digits"]).strip().upper().replace("_", "").replace(" ", "")
    fb, tb = int(i["from_base"]), int(i["to_base"])
    neg = s.startswith("-")
    val = sum(_DIG.index(c) * fb ** k for k, c in enumerate(reversed(s.lstrip("-"))))
    val = -val if neg else val
    r = o["result"]
    rn = r.startswith("-")
    back = sum(_DIG.index(c) * tb ** k for k, c in enumerate(reversed(r.lstrip("-"))))
    canon = (r == "0") or (r.lstrip("-")[0] != "0")
    return _iv(o["decimal"]) and o["decimal"] == val and (-back if rn else back) == val and canon and all(_DIG.index(c) < tb for c in r.lstrip("-"))


def _ref_base(digits, from_base, to_base):
    import gmpy2
    s = str(digits).strip().replace("_", "").replace(" ", "")
    v = gmpy2.mpz(s, int(from_base))
    out = gmpy2.digits(v, int(to_base)).upper()
    out = re.sub(r"^(-?)0[XBO]", r"\1", out)
    return {"result": out, "decimal": int(v)}


def _rand_base(rng):
    fb, tb = rng.randint(2, 36), rng.randint(2, 36)
    v = rng.randint(-10 ** 6, 10 ** rng.randint(1, 30))
    return {"digits": _to_base(v, fb), "from_base": fb, "to_base": tb}


def parse_base(q0):
    q = q0.lower().strip().rstrip("?.!")
    if not re.search(r"\bconvert|\bin base\b|\bto base\b|binary|hexadecimal|\bhex\b|octal|ternary", q):
        return None
    if re.search(r"complement|bit |bits", q):
        return None
    name = r"(binary|octal|decimal|hexadecimal|hex|ternary|base\s*(\d+))"

    def base_of(word, num):
        return int(num) if num else _BASES[word.strip()]
    m = re.search(r"convert (?:the )?(?:" + name + r" (?:number )?)?(-?[0-9a-z]+)(?:\s*\(?(?:in )?" + name + r"\)?)?\s+(?:from )?(?:" + name + r" )?to " + name + r"$", q)
    if not m:
        return None
    g = m.groups()          # (w1, n1, digits, w2, n2, w3, n3, w4, n4)
    fb = base_of(g[0], g[1]) if g[0] else (base_of(g[3], g[4]) if g[3] else (base_of(g[5], g[6]) if g[5] else 10))
    return {"digits": g[2], "from_base": fb, "to_base": base_of(g[7], g[8])}


_add("base_convert", name="Base (radix) conversion", topic="number representations / positional systems",
     purpose="Rewrite an integer from one base (2..36) to another, with its decimal value.",
     inputs=[_T("digits", "the number written in the source base (digits 0-9 then A-Z)", n_max=201),
             _I("from_base", "the source base (2..36)", 2, 36), _I("to_base", "the target base (2..36)", 2, 36)],
     outputs=[_O("result", "the number written in the target base (upper-case digits)", "text"), _O("decimal", "its value in base 10")],
     primary="decimal", conditions=["2 <= bases <= 36", "every digit is valid in the source base", "a leading minus sign is kept"],
     method="Horner evaluation in the source base, then repeated division by the target base",
     latex=r"(d_k \dots d_0)_b = \sum_i d_i\, b^i",
     run=base_convert, verify=_v_base, parse=parse_base, reference=_ref_base,
     wrong=lambda digits, from_base, to_base: (lambda r: dict(r, result=("-" if r["result"].startswith("-") else "") + r["result"].lstrip("-")[::-1]))(base_convert(digits, from_base, to_base)),
     random_inputs=_rand_base,
     templates=["Convert 1011 from base 2 to base 10.", "Convert 255 to hexadecimal.", "Convert FF from hexadecimal to binary.",
                "Convert 1234 from base 10 to base 7."],
     examples=[{"inputs": {"digits": "1011", "from_base": 2, "to_base": 10}, "outputs": {"result": "11", "decimal": 11}},
               {"inputs": {"digits": "255", "from_base": 10, "to_base": 16}, "outputs": {"result": "FF"}},
               {"inputs": {"digits": "ff", "from_base": 16, "to_base": 2}, "outputs": {"result": "11111111", "decimal": 255}},
               {"inputs": {"digits": "-100", "from_base": 10, "to_base": 7}, "outputs": {"result": "-202"}}],
     questions=[{"q": "Convert 1011 from base 2 to base 10.", "outputs": {"result": "11"}},
                {"q": "Convert 255 to hexadecimal.", "outputs": {"result": "FF"}},
                {"q": "Convert FF from hexadecimal to binary.", "outputs": {"result": "11111111"}},
                {"q": "Convert 1234 from base 10 to base 7.", "outputs": {"result": "3412"}},
                {"q": "Convert 129 from base 2 to base 10.", "outcome": "INVALID_INPUT"}])


def twos_complement(n, bits):
    n, b = _whole(n, "n"), _whole(bits, "bits", 2, 64)
    if not -(1 << (b - 1)) <= n < (1 << (b - 1)):
        raise ValueError(f"{n} does not fit in {b}-bit two's complement (range {-(1 << (b - 1))} to {(1 << (b - 1)) - 1})")
    if n >= 0:
        s = format(n, "b").zfill(b)
    else:
        mag = format(-n, "b").zfill(b)
        inv = "".join("1" if c == "0" else "0" for c in mag)
        s = format(int(inv, 2) + 1, "b").zfill(b)[-b:]
    return {"bits_text": s, "unsigned": int(s, 2)}


def _v_twos(i, o):
    n, b, s = int(i["n"]), int(i["bits"]), o["bits_text"]
    if not (_iv(o["unsigned"]) and len(s) == b and set(s) <= {"0", "1"}):
        return False
    u = int(s, 2)
    return u == o["unsigned"] and (u - (1 << b) if s[0] == "1" else u) == n


def _ref_twos(n, bits):
    import gmpy2
    u = int(gmpy2.f_mod(gmpy2.mpz(n), gmpy2.mpz(2) ** int(bits)))
    return {"bits_text": gmpy2.digits(u, 2).replace("0b", "").zfill(int(bits)), "unsigned": u}


def _wrong_twos(n, bits):
    n, b = int(n), int(bits)
    if n >= 0:
        return twos_complement(n, b)
    s = "".join("1" if c == "0" else "0" for c in format(-n, "b").zfill(b))      # one's complement: forgot the + 1
    return {"bits_text": s, "unsigned": int(s, 2)}


def parse_twos(q0):
    q = q0.lower().replace("−", "-")
    if "complement" not in q or "two" not in q and "2's" not in q:
        return None
    m = re.search(r"(\d+)[- ]bit", q)
    n = re.search(r"(?:of|represent|for|write|express|encode)\s+(-?\d+)", q)
    if not (m and n):
        return {} if not m and not n else ({"bits": int(m.group(1))} if m else {"n": int(n.group(1))})
    return {"n": int(n.group(1)), "bits": int(m.group(1))}


_add("twos_complement", name="Two's complement representation", topic="number representations / binary arithmetic",
     purpose="The b-bit two's complement bit pattern of a signed integer, and that pattern read as an unsigned number.",
     inputs=[_I("n", "the signed integer"), _I("bits", "the word length b (2..64)", 2, 64)],
     outputs=[_O("bits_text", "the b-bit pattern", "text"), _O("unsigned", "the pattern read as an unsigned integer (n mod 2^b)")],
     primary="unsigned", conditions=["-2^(b-1) <= n < 2^(b-1)"],
     method="non-negative: binary padded to b bits; negative: invert the bits of |n| and add 1",
     latex=r"\text{pattern}(n) = n \bmod 2^b",
     run=twos_complement, verify=_v_twos, parse=parse_twos, reference=_ref_twos, wrong=_wrong_twos,
     random_inputs=lambda rng: (lambda b: {"n": rng.randint(-(1 << (b - 1)), -1) if rng.random() < 0.8 else rng.randint(0, (1 << (b - 1)) - 1), "bits": b})(rng.randint(2, 48)),
     templates=["What is the 8-bit two's complement of -5?", "Write -128 in 8-bit two's complement."],
     examples=[{"inputs": {"n": -5, "bits": 8}, "outputs": {"bits_text": "11111011", "unsigned": 251}},
               {"inputs": {"n": -128, "bits": 8}, "outputs": {"bits_text": "10000000"}},
               {"inputs": {"n": 5, "bits": 4}, "outputs": {"bits_text": "0101", "unsigned": 5}}],
     questions=[{"q": "What is the 8-bit two's complement of -5?", "outputs": {"bits_text": "11111011"}},
                {"q": "Write -128 in 8-bit two's complement.", "outputs": {"unsigned": 128}},
                {"q": "What is the 8-bit two's complement of -200?", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# combinatorics (exact integers)
# ==================================================================================================================
_NCAP = 170          # results stay below 1e307 (170! ~ 7.3e306), so they compare as numbers everywhere


def combinations(n, k):
    n, k = _whole(n, "n", 0, 1000), _whole(k, "k", 0, 1000)
    if k > n:
        raise ValueError("k must not exceed n")
    c = 1
    for i in range(1, min(k, n - k) + 1):
        c = c * (n - min(k, n - k) + i) // i
    if c > 10 ** 300:
        raise ValueError("the result exceeds 10^300")
    return {"combinations": c}


def permutations(n, k):
    n, k = _whole(n, "n", 0, _NCAP), _whole(k, "k", 0, _NCAP)
    if k > n:
        raise ValueError("k must not exceed n")
    p = 1
    for i in range(n - k + 1, n + 1):
        p *= i
    return {"permutations": p}


def parse_comb(q0):
    q = q0.lower()
    if re.search(r"repetition|replacement|repeat|order matters|arrange|permutation|letters of|anagram|multiset|stirling|"
                 r"lattice|coefficient of", q):
        return None
    for pat, order in ((r"\bc\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)", "nk"), (r"\b(\d+)\s+choose\s+(\d+)", "nk"),
                       (r"\b(\d+)c(\d+)\b", "nk"), (r"binomial coefficient\D*?(\d+)\s*(?:,|and|over)\s*(\d+)", "nk"),
                       (r"(?:choose|select|pick|form a committee of)\s+(\d+)\s+(?:[a-z]+\s+){0,3}?(?:from|out of|of|among)\s+(\d+)", "kn"),
                       (r"combinations of (\d+) (?:[a-z]+ )?taken (\d+)", "nk")):
        m = re.search(pat, q)
        if m:
            a, b = int(m.group(1)), int(m.group(2))
            return {"n": a, "k": b} if order == "nk" else {"n": b, "k": a}
    return None


def parse_perm(q0):
    q = q0.lower()
    if re.search(r"repetition|replacement|repeat|letters of|anagram|word|cycle|derange|circular|round table|with exactly", q):
        return None
    for pat, order in ((r"\bp\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)", "nk"), (r"\b(\d+)p(\d+)\b", "nk"),
                       (r"permutations of (\d+) (?:[a-z]+ )?taken (\d+)", "nk"),
                       (r"(?:arrange|order|line up|rank)\s+(\d+)\s+(?:[a-z]+\s+){0,3}?(?:from|out of|of|among)\s+(\d+)", "kn"),
                       (r"(?:arrangements|permutations) of (\d+) (?:[a-z]+ )?(?:chosen |selected )?from (\d+)", "kn")):
        m = re.search(pat, q)
        if m:
            a, b = int(m.group(1)), int(m.group(2))
            return {"n": a, "k": b} if order == "nk" else {"n": b, "k": a}
    m = re.search(r"(?:permutations of|arrange|line up|order) (\d+) (?:distinct |different )?(?:objects|items|elements|people|books|"
                  r"letters|cards|students|things|digits)(?! from| out of| of)", q) or \
        re.search(r"(\d+) (?:distinct |different )?(?:objects|items|elements|people|books|cards|students|things) "
                  r"(?:be |can be )?(?:arranged|lined up|ordered|seated) in a (?:row|line)", q)
    if m:
        return {"n": int(m.group(1)), "k": int(m.group(1))}
    return None


_add("combinations", name="Combinations C(n, k)", topic="combinatorics / basic counting",
     purpose="The number of ways to choose k of n distinct items when order does not matter (exact integer).",
     inputs=[_I("n", "number of items to choose from (0..1000)", 0, 1000), _I("k", "number chosen (0..n)", 0, 1000)],
     outputs=[_O("combinations", "C(n, k) = n! / (k! (n-k)!)")],
     primary="combinations", conditions=["0 <= k <= n <= 1000", "result at most 10^300"],
     method="multiplicative formula with exact integer division at every step",
     latex=r"\binom{n}{k} = \frac{n!}{k!\,(n-k)!}",
     run=combinations, verify=lambda i, o: _iv(o["combinations"]) and o["combinations"] * math.factorial(int(i["k"])) * math.factorial(int(i["n"]) - int(i["k"])) == math.factorial(int(i["n"])),
     parse=parse_comb, reference=lambda n, k: {"combinations": int(__import__("sympy").binomial(n, k))},
     wrong=lambda n, k: {"combinations": math.comb(int(n), int(k) - 1)},
     random_inputs=lambda rng: (lambda n: {"n": n, "k": rng.randint(2, max(2, n - 1)) if n > 3 else rng.randint(0, n)})(rng.randint(4, 1000)),
     templates=["How many ways are there to choose 3 books from 10?", "Compute C(52, 5).", "What is 10 choose 3?"],
     examples=[{"inputs": {"n": 10, "k": 3}, "outputs": {"combinations": 120}}, {"inputs": {"n": 52, "k": 5}, "outputs": {"combinations": 2598960}},
               {"inputs": {"n": 0, "k": 0}, "outputs": {"combinations": 1}}],
     questions=[{"q": "How many ways are there to choose 3 books from 10?", "outputs": {"combinations": 120}},
                {"q": "Compute C(52, 5).", "outputs": {"combinations": 2598960}},
                {"q": "What is 10 choose 3?", "outputs": {"combinations": 120}},
                {"q": "Compute C(4, 7).", "outcome": "INVALID_INPUT"}])

_add("permutations", name="Permutations P(n, k)", topic="combinatorics / basic counting",
     purpose="The number of ordered arrangements of k of n distinct items (exact integer); P(n, n) = n!.",
     inputs=[_I("n", "number of distinct items (0..170)", 0, _NCAP), _I("k", "number arranged (0..n)", 0, _NCAP)],
     outputs=[_O("permutations", "P(n, k) = n! / (n-k)!")],
     primary="permutations", conditions=["0 <= k <= n <= 170"],
     method="falling factorial n (n-1) ... (n-k+1)", latex=r"P(n,k) = \frac{n!}{(n-k)!}",
     run=permutations, verify=lambda i, o: _iv(o["permutations"]) and o["permutations"] * math.factorial(int(i["n"]) - int(i["k"])) == math.factorial(int(i["n"])),
     parse=parse_perm, reference=lambda n, k: {"permutations": int(__import__("sympy").ff(n, k))},
     wrong=lambda n, k: {"permutations": math.comb(int(n), int(k))},
     random_inputs=lambda rng: (lambda n: {"n": n, "k": rng.randint(2, n)})(rng.randint(2, _NCAP)),
     templates=["Compute P(10, 3).", "In how many ways can we arrange 3 books out of 10?", "In how many ways can 5 people be arranged in a row?"],
     examples=[{"inputs": {"n": 10, "k": 3}, "outputs": {"permutations": 720}}, {"inputs": {"n": 5, "k": 5}, "outputs": {"permutations": 120}},
               {"inputs": {"n": 7, "k": 0}, "outputs": {"permutations": 1}}],
     questions=[{"q": "Compute P(10, 3).", "outputs": {"permutations": 720}},
                {"q": "In how many ways can we arrange 3 books out of 10?", "outputs": {"permutations": 720}},
                {"q": "In how many ways can 5 people be arranged in a row?", "outputs": {"permutations": 120}},
                {"q": "Compute P(3, 5).", "outcome": "INVALID_INPUT"}])


def sequences_with_repetition(n, k):
    n, k = _whole(n, "n", 1, 10 ** 6), _whole(k, "k", 0, 10 ** 4)
    if k * math.log10(n) > 300:
        raise ValueError("the result exceeds 10^300")
    return {"sequences": n ** k}


def _ref_seq(n, k):
    import sympy
    from more_itertools import ilen
    if n ** k <= 2 * 10 ** 5:
        return {"sequences": ilen(itertools.product(range(n), repeat=k))}
    return {"sequences": int(sympy.Integer(n) ** k)}


def multisets(n, k):
    n, k = _whole(n, "n", 1, 1000), _whole(k, "k", 0, 1000)
    c = math.comb(n + k - 1, k)
    if c > 10 ** 300:
        raise ValueError("the result exceeds 10^300")
    return {"multisets": c}


def _ref_mset(n, k):
    import sympy
    from more_itertools import ilen
    if math.comb(n + k - 1, k) <= 2 * 10 ** 5:
        return {"multisets": ilen(itertools.combinations_with_replacement(range(n), k))}
    return {"multisets": int(sympy.binomial(n + k - 1, k))}


def parse_seq(q0):
    q = q0.lower()
    if not re.search(r"repetition|replacement|repeat", q) or not re.search(r"sequence|string|ordered|code|word|pin|password|arrangement", q):
        return None
    if re.search(r"choose|select|pick|combination|multiset|unordered|scoop", q):
        return None
    k = re.search(r"(?:length|of size|long)\s+(\d+)|(\d+)[- ](?:letter|digit|character|symbol)", q)
    n = re.search(r"(?:from|of|over|using) (?:an alphabet of |a set of )?(\d+) (?:distinct |different )?(?:symbols|digits|letters|characters|elements|items|colou?rs)", q)
    if not (k and n):
        return {}
    return {"n": int(n.group(1)), "k": int(_first(k.group(1), k.group(2)))}


def parse_mset(q0):
    q = q0.lower()
    if not re.search(r"repetition|replacement|repeat|multiset", q) or re.search(r"sequence|string|ordered\b|code|word|pin|password|arrangement", q):
        return None
    m = re.search(r"(?:choose|select|pick)\s+(\d+)\s+(?:[a-z]+\s+){0,3}?(?:from|out of|of|among)\s+(\d+)", q)
    if m:
        return {"n": int(m.group(2)), "k": int(m.group(1))}
    m = re.search(r"multisets? of size (\d+) (?:from|of|over) (\d+)", q)
    return {"n": int(m.group(2)), "k": int(m.group(1))} if m else None


_add("sequences_with_repetition", name="Sequences with repetition (n^k)", topic="combinatorics / basic counting",
     purpose="The number of length-k sequences over n symbols when symbols may repeat.",
     inputs=[_I("n", "number of available symbols (at least 1)", 1, 10 ** 6), _I("k", "length of the sequence", 0, 10 ** 4)],
     outputs=[_O("sequences", "n^k")],
     primary="sequences", conditions=["n >= 1", "result at most 10^300"], method="multiplication principle",
     latex=r"n^k",
     run=sequences_with_repetition, verify=lambda i, o: _iv(o["sequences"]) and o["sequences"] == math.prod([int(i["n"])] * int(i["k"])),
     parse=parse_seq, reference=_ref_seq, wrong=lambda n, k: {"sequences": math.perm(int(n), int(k)) if int(k) <= int(n) else 0},
     random_inputs=lambda rng: {"n": rng.randint(2, 40), "k": rng.randint(2, 12)},
     templates=["How many sequences of length 4 can be formed from 10 digits with repetition allowed?",
                "How many 5-letter strings can be made from 26 letters with repetition?"],
     examples=[{"inputs": {"n": 10, "k": 4}, "outputs": {"sequences": 10000}}, {"inputs": {"n": 26, "k": 5}, "outputs": {"sequences": 11881376}},
               {"inputs": {"n": 2, "k": 10}, "outputs": {"sequences": 1024}}],
     questions=[{"q": "How many sequences of length 4 can be formed from 10 digits with repetition allowed?", "outputs": {"sequences": 10000}},
                {"q": "How many 5-letter strings can be made from 26 letters with repetition?", "outputs": {"sequences": 11881376}}])

_add("multisets", name="Combinations with repetition (multisets)", topic="combinatorics / basic counting",
     purpose="The number of ways to choose k items from n types when types may repeat and order does not matter.",
     inputs=[_I("n", "number of types (at least 1)", 1, 1000), _I("k", "number of items chosen", 0, 1000)],
     outputs=[_O("multisets", "C(n + k - 1, k)")],
     primary="multisets", conditions=["n >= 1", "result at most 10^300"], method="stars and bars",
     latex=r"\left(\!\binom{n}{k}\!\right) = \binom{n+k-1}{k}",
     run=multisets, verify=lambda i, o: _iv(o["multisets"]) and o["multisets"] * math.factorial(int(i["k"])) * math.factorial(int(i["n"]) - 1) == math.factorial(int(i["n"]) + int(i["k"]) - 1),
     parse=parse_mset, reference=_ref_mset, wrong=lambda n, k: {"multisets": math.comb(int(n) + int(k), int(k))},
     random_inputs=lambda rng: {"n": rng.randint(1, 300), "k": rng.randint(1, 300)},
     templates=["How many ways can we choose 3 scoops from 5 flavours with repetition allowed?",
                "How many multisets of size 4 from 6 types are there?"],
     examples=[{"inputs": {"n": 5, "k": 3}, "outputs": {"multisets": 35}}, {"inputs": {"n": 6, "k": 4}, "outputs": {"multisets": 126}},
               {"inputs": {"n": 1, "k": 7}, "outputs": {"multisets": 1}}],
     questions=[{"q": "How many ways can we choose 3 scoops from 5 flavours with repetition allowed?", "outputs": {"multisets": 35}},
                {"q": "How many multisets of size 4 from 6 types are there?", "outputs": {"multisets": 126}}])


def multinomial(counts, total=None):
    ks = _wlist(counts, "counts", lo=0, n_min=1, n_max=30)
    n = sum(ks)
    if total is not None and _whole(total, "total", 0) != n:
        raise ValueError(f"the group sizes add up to {n}, not to the stated total {total}")
    if n > _NCAP:
        raise ValueError(f"the counts may add up to at most {_NCAP}")
    r = math.factorial(n)
    for k in ks:
        r //= math.factorial(k)
    return {"coefficient": r, "total": n}


def _ref_multinomial(counts, total=None):
    import sympy
    ks = [int(c) for c in counts]
    r, run = 1, 0
    for k in ks:                       # choose each group in turn from what is left
        run += k
        r *= int(sympy.binomial(run, k))
    return {"coefficient": r, "total": sum(ks)}


def parse_multinomial(q0):
    q = q0.lower()
    m = re.search(r"multinomial coefficient(?: for| of)?\s*(?:\(\s*(\d+)\s*;\s*)?([\d,\s and]+)\)?", q)
    if m and _ints(m.group(2)):
        out = {"counts": _ints(m.group(2))}
        if m.group(1):
            out["total"] = int(m.group(1))
        return out
    m = re.search(r"(\d+) (?:people|students|objects|items|players|books) (?:be )?(?:split|divided|partitioned|separated) into "
                  r"(?:labell?ed |distinct |named )?groups of (?:sizes? )?([\d,\s and]+)", q)
    if m:
        return {"counts": _ints(m.group(2)), "total": int(m.group(1))}
    return None


_add("multinomial", name="Multinomial coefficient", topic="combinatorics / basic counting",
     purpose="n! / (k1! k2! ... km!): ways to split n distinct items into labelled groups of the given sizes.",
     inputs=[_V("counts", "the group sizes k1, ..., km (whole numbers)", 1, 30),
             _I("total", "the stated total n (optional; must equal the sum of the sizes)", 0, required=False)],
     outputs=[_O("coefficient", "the multinomial coefficient"), _O("total", "n = k1 + ... + km")],
     primary="coefficient", conditions=["sizes are whole numbers >= 0 adding up to at most 170", "a stated total must equal their sum"],
     method="n! divided by each k_i! (exact integers)", latex=r"\binom{n}{k_1,\dots,k_m} = \frac{n!}{k_1!\cdots k_m!}",
     run=multinomial, verify=lambda i, o: _iv(o["coefficient"]) and o["coefficient"] * math.prod(math.factorial(int(k)) for k in i["counts"]) == math.factorial(sum(int(k) for k in i["counts"])),
     parse=parse_multinomial, reference=_ref_multinomial,
     wrong=lambda counts, total=None: (lambda ks: {"coefficient": math.factorial(sum(ks)) // math.prod(math.factorial(k) for k in ks[:-1]), "total": sum(ks)})([int(c) for c in counts]),
     random_inputs=lambda rng: {"counts": [float(rng.randint(0, 12)) for _ in range(rng.randint(2, 5))] + [float(rng.randint(2, 9))]},
     templates=["Compute the multinomial coefficient for 2, 3, 5.",
                "In how many ways can 10 people be split into groups of sizes 2, 3 and 5?"],
     examples=[{"inputs": {"counts": "2, 3, 5"}, "outputs": {"coefficient": 2520, "total": 10}},
               {"inputs": {"counts": "1, 4, 4, 2"}, "outputs": {"coefficient": 34650}},
               {"inputs": {"counts": [3, 3], "total": 6}, "outputs": {"coefficient": 20}}],
     questions=[{"q": "Compute the multinomial coefficient for 2, 3, 5.", "outputs": {"coefficient": 2520}},
                {"q": "In how many ways can 10 people be split into groups of sizes 2, 3 and 5?", "outputs": {"coefficient": 2520}},
                {"q": "In how many ways can 12 people be split into groups of sizes 2, 3 and 5?", "outcome": "INVALID_INPUT"}])


def word_arrangements(word):
    w = str(word).strip().upper()
    if not w or not w.isalpha() or not w.isascii():
        raise ValueError("the word must consist of letters A-Z only")
    if len(w) > _NCAP:
        raise ValueError(f"at most {_NCAP} letters")
    counts = {}
    for ch in w:
        counts[ch] = counts.get(ch, 0) + 1
    r, used = 1, 0
    for c in counts.values():
        used += c
        r *= math.comb(used, c)
    return {"arrangements": r, "length": len(w), "letter_counts": dict(sorted(counts.items()))}


def _ref_word(word):
    import sympy
    from sympy.utilities.iterables import multiset_permutations
    w = str(word).strip().upper()
    if len(w) <= 8:
        n = sum(1 for _ in multiset_permutations(list(w)))
    else:
        n = int(sympy.factorial(len(w)) / sympy.prod([sympy.factorial(w.count(c)) for c in set(w)]))
    return {"arrangements": n, "length": len(w)}


def _v_word(i, o):
    w = str(i["word"]).strip().upper()
    cnt = {c: w.count(c) for c in sorted(set(w))}
    return _iv(o["arrangements"]) and o["letter_counts"] == cnt and o["length"] == len(w) and \
        o["arrangements"] * math.prod(math.factorial(c) for c in cnt.values()) == math.factorial(len(w))


def parse_word(q0):
    m = re.search(r"(?:arrangements|arrange|rearrange|permutations|anagrams|orderings|reorder)\b.*?\bletters? (?:of|in) "
                  r"(?:the word )?[\"'“‘]?([A-Za-z]+)[\"'”’]?", q0, re.I) or \
        re.search(r"anagrams? (?:of|does|can be made from) (?:the word )?[\"'“‘]?([A-Za-z]+)[\"'”’]?", q0, re.I) or \
        re.search(r"letters (?:of|in) (?:the word )?[\"'“‘]?([A-Za-z]+)[\"'”’]? (?:be |can be )?(?:arranged|rearranged|permuted|ordered)",
                  q0, re.I)
    if not m or m.group(1).lower() in ("the", "word", "a", "an"):
        return None
    return {"word": m.group(1)}


_add("word_arrangements", name="Distinct arrangements of the letters of a word", topic="combinatorics / basic counting",
     purpose="The number of distinct orderings of a word's letters, repeated letters counted once (multiset permutations).",
     inputs=[_T("word", "the word (letters A-Z; case is ignored)", n_max=_NCAP)],
     outputs=[_O("arrangements", "n! / (n1! n2! ...) over the letter multiplicities"), _O("length", "number of letters"),
              _O("letter_counts", "how often each letter occurs", "dict")],
     primary="arrangements", conditions=["letters A-Z only (upper and lower case are the same letter)", "at most 170 letters"],
     method="product of binomial coefficients, placing one letter type at a time",
     latex=r"\frac{n!}{n_1!\,n_2!\cdots n_r!}",
     run=word_arrangements, verify=_v_word, parse=parse_word, reference=_ref_word,
     wrong=lambda word: (lambda r: dict(r, arrangements=math.factorial(r["length"])))(word_arrangements(word)),
     random_inputs=lambda rng: {"word": "".join(rng.choice("AABBCDEEESST") for _ in range(rng.randint(3, 14)))},
     templates=["How many distinct arrangements of the letters of MISSISSIPPI are there?", "How many anagrams of BANANA are there?"],
     examples=[{"inputs": {"word": "MISSISSIPPI"}, "outputs": {"arrangements": 34650, "length": 11}},
               {"inputs": {"word": "banana"}, "outputs": {"arrangements": 60}}, {"inputs": {"word": "ABC"}, "outputs": {"arrangements": 6}}],
     questions=[{"q": "How many distinct arrangements of the letters of MISSISSIPPI are there?", "outputs": {"arrangements": 34650}},
                {"q": "How many anagrams of BANANA are there?", "outputs": {"arrangements": 60}},
                {"q": "In how many ways can the letters of the word STATISTICS be arranged?", "outputs": {"arrangements": 50400}}])


def stirling2(n, k):
    n, k = _whole(n, "n", 0, 100), _whole(k, "k", 0, 100)
    if k > n:
        return {"stirling2": 0, "surjections": 0}
    row = [1] + [0] * k                      # S(0, j)
    for i in range(1, n + 1):
        new = [0] * (k + 1)
        for j in range(1, min(i, k) + 1):
            new[j] = j * row[j] + row[j - 1]
        row = new
    return {"stirling2": row[k], "surjections": math.factorial(k) * row[k]}


def _v_s2(i, o):
    n, k = int(i["n"]), int(i["k"])
    sur = sum((-1) ** j * math.comb(k, j) * (k - j) ** n for j in range(k + 1))     # inclusion-exclusion: onto maps
    return _iv(o["stirling2"]) and o["surjections"] == sur and o["stirling2"] * math.factorial(k) == sur


def _ref_s2(n, k):
    from sympy.functions.combinatorial.numbers import stirling
    import sympy
    s = int(stirling(n, k))
    return {"stirling2": s, "surjections": int(sympy.factorial(k)) * s}


def parse_s2(q0):
    q = q0.lower()
    m = re.search(r"stirling numbers? of the second kind\D*?(\d+)\D+?(\d+)", q)
    if m:
        return {"n": int(m.group(1)), "k": int(m.group(2))}
    m = re.search(r"partition (?:a set of|the set of|a set with) (\d+) (?:elements|objects|items|people) into (?:exactly )?(\d+) "
                  r"(?:non-?empty |nonempty |unlabell?ed )?(?:subsets|blocks|groups|parts)", q)
    if m:
        return {"n": int(m.group(1)), "k": int(m.group(2))}
    m = re.search(r"(?:onto|surjective) (?:functions|maps|mappings) (?:[a-z]+ ){0,3}?from a (?:set of |set with )?(\d+)[- ]element set (?:to|onto) "
                  r"a (?:set of |set with )?(\d+)[- ]element set", q)
    return {"n": int(m.group(1)), "k": int(m.group(2))} if m else None


_add("stirling2", name="Stirling numbers of the second kind", topic="combinatorics / set partitions",
     purpose="S(n, k): ways to partition an n-element set into k non-empty unlabelled blocks; k! S(n, k) onto functions.",
     inputs=[_I("n", "size of the set (0..100)", 0, 100), _I("k", "number of blocks (0..100)", 0, 100)],
     outputs=[_O("stirling2", "S(n, k)"), _O("surjections", "k! S(n, k): onto functions from an n-set to a k-set")],
     primary="stirling2", conditions=["0 <= n, k <= 100"], method="recurrence S(n, k) = k S(n-1, k) + S(n-1, k-1)",
     latex=r"S(n,k) = k\,S(n-1,k) + S(n-1,k-1) = \frac{1}{k!}\sum_{j=0}^{k} (-1)^j \binom{k}{j} (k-j)^n",
     run=stirling2, verify=_v_s2, parse=parse_s2, reference=_ref_s2,
     wrong=lambda n, k: {"stirling2": math.comb(max(int(n) - 1, 0), max(int(k) - 1, 0)), "surjections": 0},
     random_inputs=lambda rng: (lambda n: {"n": n, "k": rng.randint(2, max(2, n - 1))})(rng.randint(4, 100)),
     templates=["Compute the Stirling number of the second kind S(7, 3).",
                "In how many ways can we partition a set of 7 elements into 3 non-empty subsets?",
                "How many onto functions are there from a 5-element set to a 3-element set?"],
     examples=[{"inputs": {"n": 7, "k": 3}, "outputs": {"stirling2": 301, "surjections": 1806}},
               {"inputs": {"n": 5, "k": 3}, "outputs": {"stirling2": 25, "surjections": 150}},
               {"inputs": {"n": 0, "k": 0}, "outputs": {"stirling2": 1}}, {"inputs": {"n": 3, "k": 5}, "outputs": {"stirling2": 0}}],
     questions=[{"q": "Compute the Stirling number of the second kind S(7, 3).", "outputs": {"stirling2": 301}},
                {"q": "In how many ways can we partition a set of 7 elements into 3 non-empty subsets?", "outputs": {"stirling2": 301}},
                {"q": "How many onto functions are there from a 5-element set to a 3-element set?", "outputs": {"surjections": 150}}])


def stirling1(n, k):
    n, k = _whole(n, "n", 0, 100), _whole(k, "k", 0, 100)
    if k > n:
        return {"stirling1": 0, "signed": 0}
    row = [1] + [0] * k
    for i in range(1, n + 1):
        new = [0] * (k + 1)
        for j in range(1, min(i, k) + 1):
            new[j] = (i - 1) * row[j] + row[j - 1]
        row = new
    return {"stirling1": row[k], "signed": (-1) ** (n - k) * row[k]}


def _v_s1(i, o):
    n, k = int(i["n"]), int(i["k"])
    poly = [1]                                   # coefficients of x (x+1) ... (x+n-1), lowest degree first
    for j in range(n):
        new = [0] * (len(poly) + 1)
        for d, c in enumerate(poly):
            new[d + 1] += c
            new[d] += j * c
        poly = new
    c = poly[k] if k < len(poly) else 0
    return _iv(o["stirling1"]) and o["stirling1"] == c and o["signed"] == (-1) ** (n - k) * c


def _ref_s1(n, k):
    from sympy.functions.combinatorial.numbers import stirling
    return {"stirling1": int(stirling(n, k, kind=1)), "signed": int(stirling(n, k, kind=1, signed=True))}


def parse_s1(q0):
    q = q0.lower()
    m = re.search(r"stirling numbers? of the first kind\D*?(\d+)\D+?(\d+)", q)
    if m:
        return {"n": int(m.group(1)), "k": int(m.group(2))}
    m = re.search(r"permutations of (\d+) (?:elements|objects|items|letters) (?:that have |have |with )(?:exactly )?(\d+) cycles", q)
    return {"n": int(m.group(1)), "k": int(m.group(2))} if m else None


_add("stirling1", name="Stirling numbers of the first kind", topic="combinatorics / permutations and cycles",
     purpose="c(n, k): permutations of n elements with exactly k cycles (unsigned), and the signed s(n, k).",
     inputs=[_I("n", "number of elements (0..100)", 0, 100), _I("k", "number of cycles (0..100)", 0, 100)],
     outputs=[_O("stirling1", "unsigned c(n, k)"), _O("signed", "signed s(n, k) = (-1)^(n-k) c(n, k)")],
     primary="stirling1", conditions=["0 <= n, k <= 100"], method="recurrence c(n, k) = (n-1) c(n-1, k) + c(n-1, k-1)",
     latex=r"c(n,k) = (n-1)\,c(n-1,k) + c(n-1,k-1),\quad x^{\overline n} = \sum_k c(n,k)\,x^k",
     run=stirling1, verify=_v_s1, parse=parse_s1, reference=_ref_s1,
     wrong=lambda n, k: {"stirling1": stirling2(n, k)["stirling2"], "signed": 0},
     random_inputs=lambda rng: (lambda n: {"n": n, "k": rng.randint(1, max(1, n - 2))})(rng.randint(4, 100)),
     templates=["Compute the Stirling number of the first kind c(5, 2).", "How many permutations of 6 elements have exactly 2 cycles?"],
     examples=[{"inputs": {"n": 5, "k": 2}, "outputs": {"stirling1": 50, "signed": -50}},
               {"inputs": {"n": 6, "k": 2}, "outputs": {"stirling1": 274}}, {"inputs": {"n": 4, "k": 4}, "outputs": {"stirling1": 1}}],
     questions=[{"q": "Compute the Stirling number of the first kind c(5, 2).", "outputs": {"stirling1": 50}},
                {"q": "How many permutations of 6 elements have exactly 2 cycles?", "outputs": {"stirling1": 274}}])


def bell(n):
    n = _whole(n, "n", 0, 200)
    row = [1]
    for _ in range(n):
        new = [row[-1]]
        for x in row:
            new.append(new[-1] + x)
        row = new
    return {"bell": row[0]}


def _v_bell(i, o):
    n = int(i["n"])
    b = [1]
    for m in range(n):
        b.append(sum(math.comb(m, j) * b[j] for j in range(m + 1)))
    return _iv(o["bell"]) and o["bell"] == b[n]


def parse_bell(q0):
    q = q0.lower()
    m = re.search(r"bell number\D*?(\d+)|(\d+)(?:st|nd|rd|th) bell number", q)
    if m:
        return {"n": int(_first(m.group(1), m.group(2)))}
    if re.search(r"into (?:exactly )?\d", q):
        return None
    m = re.search(r"(?:partitions of|partition|ways to partition) (?:a set of|the set of|a set with) (\d+) (?:elements|objects|items|people)", q)
    return {"n": int(m.group(1))} if m else None


_add("bell", name="Bell numbers", topic="combinatorics / set partitions",
     purpose="B_n: the number of partitions of an n-element set into any number of non-empty blocks.",
     inputs=[_I("n", "size of the set (0..200)", 0, 200)],
     outputs=[_O("bell", "the Bell number B_n")],
     primary="bell", conditions=["0 <= n <= 200"], method="Bell triangle (Aitken's array)",
     latex=r"B_{n+1} = \sum_{k=0}^{n} \binom{n}{k} B_k",
     run=bell, verify=_v_bell, parse=parse_bell, reference=lambda n: {"bell": int(__import__("sympy").bell(n))},
     wrong=lambda n: {"bell": 2 ** max(int(n) - 1, 0)},
     random_inputs=lambda rng: {"n": rng.randint(4, 200)},
     templates=["What is the 5th Bell number?", "In how many ways can we partition a set of 4 elements?"],
     examples=[{"inputs": {"n": 5}, "outputs": {"bell": 52}}, {"inputs": {"n": 0}, "outputs": {"bell": 1}},
               {"inputs": {"n": 10}, "outputs": {"bell": 115975}}],
     questions=[{"q": "What is the 5th Bell number?", "outputs": {"bell": 52}},
                {"q": "In how many ways can we partition a set of 4 elements?", "outputs": {"bell": 15}}])


def catalan(n):
    n = _whole(n, "n", 0, 500)
    return {"catalan": math.comb(2 * n, n) // (n + 1)}


def _v_catalan(i, o):
    c = 1
    for k in range(int(i["n"])):
        c = c * 2 * (2 * k + 1) // (k + 2)
    return _iv(o["catalan"]) and o["catalan"] == c


def parse_catalan(q0):
    q = q0.lower()
    m = re.search(r"catalan number\D*?(\d+)|(\d+)(?:st|nd|rd|th) catalan number", q)
    if m:
        return {"n": int(_first(m.group(1), m.group(2)))}
    m = re.search(r"(?:balanced|valid|well-formed) (?:strings of |sequences of |arrangements of )?(?:parentheses|brackets) "
                  r"(?:with|using|of) (\d+) pairs", q)
    if m:
        return {"n": int(m.group(1))}
    m = re.search(r"triangulat\w* (?:of )?a convex polygon with (\d+) (?:sides|vertices)|triangulat\w* (?:of )?a convex (\d+)-gon", q)
    if m:
        s = int(_first(m.group(1), m.group(2)))
        return {"n": s - 2} if s >= 3 else {"n": -1}
    return None


_add("catalan", name="Catalan numbers", topic="combinatorics / recursive counting",
     purpose="C_n: balanced parenthesizations with n pairs, triangulations of an (n+2)-gon, full binary trees with n+1 leaves.",
     inputs=[_I("n", "the index n (0..500)", 0, 500)],
     outputs=[_O("catalan", "the Catalan number C_n")],
     primary="catalan", conditions=["0 <= n <= 500", "a convex polygon with s sides has C_(s-2) triangulations"],
     method="closed form binomial(2n, n)/(n + 1)", latex=r"C_n = \frac{1}{n+1}\binom{2n}{n}",
     run=catalan, verify=_v_catalan, parse=parse_catalan, reference=lambda n: {"catalan": int(__import__("sympy").catalan(n))},
     wrong=lambda n: {"catalan": math.comb(2 * int(n), int(n))},
     random_inputs=lambda rng: {"n": rng.randint(1, 500)},
     templates=["What is the 10th Catalan number?", "How many balanced strings of parentheses with 4 pairs are there?",
                "How many triangulations of a convex polygon with 8 sides are there?"],
     examples=[{"inputs": {"n": 10}, "outputs": {"catalan": 16796}}, {"inputs": {"n": 4}, "outputs": {"catalan": 14}},
               {"inputs": {"n": 0}, "outputs": {"catalan": 1}}],
     questions=[{"q": "What is the 10th Catalan number?", "outputs": {"catalan": 16796}},
                {"q": "How many balanced strings of parentheses with 4 pairs are there?", "outputs": {"catalan": 14}},
                {"q": "How many triangulations of a convex polygon with 8 sides are there?", "outputs": {"catalan": 132}}])


def fibonacci(n):
    n = _whole(n, "n", 0, 1000)
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return {"fibonacci": a}


def _fib_pair(n):
    if n == 0:
        return 0, 1
    a, b = _fib_pair(n >> 1)
    c = a * (2 * b - a)
    d = a * a + b * b
    return (d, c + d) if n & 1 else (c, d)


def lucas(n):
    n = _whole(n, "n", 0, 1000)
    a, b = 2, 1
    for _ in range(n):
        a, b = b, a + b
    return {"lucas": a}


def parse_fib(q0):
    q = q0.lower()
    if "fibonacci" not in q:
        return None
    m = re.search(r"(\d+)(?:st|nd|rd|th) fibonacci|fibonacci number\s*(?:f_?|f\(|number )?\s*(\d+)|\bf_?\(?\s*(\d+)\s*\)?", q)
    return {"n": int(_first(*m.groups()))} if m else {}


def parse_lucas(q0):
    q = q0.lower()
    if "lucas" not in q:
        return None
    m = re.search(r"(\d+)(?:st|nd|rd|th) lucas|lucas number\s*(?:l_?|l\(|number )?\s*(\d+)|\bl_?\(?\s*(\d+)\s*\)?", q)
    return {"n": int(_first(*m.groups()))} if m else {}


_add("fibonacci", name="Fibonacci numbers", topic="combinatorics / recurrences",
     purpose="F_n with F_0 = 0, F_1 = 1, F_n = F_(n-1) + F_(n-2) (exact integer).",
     inputs=[_I("n", "the index n (0..1000)", 0, 1000)],
     outputs=[_O("fibonacci", "F_n")],
     primary="fibonacci", conditions=["0 <= n <= 1000", "F_0 = 0, F_1 = 1"], method="iteration of the recurrence",
     latex=r"F_n = F_{n-1} + F_{n-2},\ F_0 = 0,\ F_1 = 1",
     run=fibonacci, verify=lambda i, o: _iv(o["fibonacci"]) and o["fibonacci"] == _fib_pair(int(i["n"]))[0], parse=parse_fib,
     reference=lambda n: {"fibonacci": int(__import__("sympy").fibonacci(n))},
     wrong=lambda n: {"fibonacci": fibonacci(int(n) + 1)["fibonacci"]},
     random_inputs=lambda rng: {"n": rng.randint(3, 1000)},
     templates=["What is the 30th Fibonacci number?", "Compute the Fibonacci number F(100)."],
     examples=[{"inputs": {"n": 30}, "outputs": {"fibonacci": 832040}}, {"inputs": {"n": 0}, "outputs": {"fibonacci": 0}},
               {"inputs": {"n": 100}, "outputs": {"fibonacci": 354224848179261915075}}],
     questions=[{"q": "What is the 30th Fibonacci number?", "outputs": {"fibonacci": 832040}},
                {"q": "Compute the Fibonacci number F(100).", "outputs": {"fibonacci": 354224848179261915075}},
                {"q": "Compute the Fibonacci number F(5000).", "outcome": "INVALID_INPUT"}])

_add("lucas", name="Lucas numbers", topic="combinatorics / recurrences",
     purpose="L_n with L_0 = 2, L_1 = 1, L_n = L_(n-1) + L_(n-2) (exact integer).",
     inputs=[_I("n", "the index n (0..1000)", 0, 1000)],
     outputs=[_O("lucas", "L_n")],
     primary="lucas", conditions=["0 <= n <= 1000", "L_0 = 2, L_1 = 1"], method="iteration of the recurrence",
     latex=r"L_n = L_{n-1} + L_{n-2},\ L_0 = 2,\ L_1 = 1",
     run=lucas, verify=lambda i, o: _iv(o["lucas"]) and (lambda a, b: o["lucas"] == 2 * b - a)(*_fib_pair(int(i["n"]))),
     parse=parse_lucas, reference=lambda n: {"lucas": int(__import__("sympy").lucas(n))},
     wrong=lambda n: {"lucas": fibonacci(n)["fibonacci"]},
     random_inputs=lambda rng: {"n": rng.randint(1, 1000)},
     templates=["What is the 10th Lucas number?", "Compute the Lucas number L(20)."],
     examples=[{"inputs": {"n": 10}, "outputs": {"lucas": 123}}, {"inputs": {"n": 0}, "outputs": {"lucas": 2}},
               {"inputs": {"n": 20}, "outputs": {"lucas": 15127}}],
     questions=[{"q": "What is the 10th Lucas number?", "outputs": {"lucas": 123}},
                {"q": "Compute the Lucas number L(20).", "outputs": {"lucas": 15127}}])


def derangements(n):
    n = _whole(n, "n", 0, _NCAP)
    a, b = 1, 0                          # D(0), D(1)
    if n == 0:
        d = 1
    else:
        for i in range(2, n + 1):
            a, b = b, (i - 1) * (a + b)
        d = b
    return {"derangements": d, "probability": float(Fraction(d, math.factorial(n)))}


def _v_der(i, o):
    n = int(i["n"])
    d = sum((-1) ** k * (math.factorial(n) // math.factorial(k)) for k in range(n + 1))
    return _iv(o["derangements"]) and o["derangements"] == d and o["probability"] == float(Fraction(d, math.factorial(n)))


def _ref_der(n):
    import sympy
    d = int(sympy.subfactorial(n))
    return {"derangements": d, "probability": float(sympy.Rational(d, sympy.factorial(n)))}


def parse_der(q0):
    q = q0.lower()
    m = re.search(r"derangements? of (\d+)|subfactorial (?:of )?(\d+)|!\s*(\d+)\b", q)
    if m and re.search(r"derange|subfactorial", q):
        return {"n": int(_first(*m.groups()))}
    if re.search(r"(?:no|none of the|not a single) \w+ (?:is |are |gets? |ends? up |goes? |receives? )?(?:in|into|back|with) "
                 r"(?:its|their|his|her) (?:own|correct|original|right)", q):
        nums = set(_ints(q))
        return {"n": nums.pop()} if len(nums) == 1 else None
    return None


_add("derangements", name="Derangements", topic="combinatorics / inclusion-exclusion",
     purpose="!n: permutations of n items with no item in its original place, and the probability !n/n!.",
     inputs=[_I("n", "number of items (0..170)", 0, _NCAP)],
     outputs=[_O("derangements", "!n"), _O("probability", "!n / n! (tends to 1/e)")],
     primary="derangements", conditions=["0 <= n <= 170"], method="recurrence D(n) = (n-1)(D(n-1) + D(n-2))",
     latex=r"!n = n!\sum_{k=0}^{n}\frac{(-1)^k}{k!} = (n-1)\big(!(n-1) + !(n-2)\big)",
     run=derangements, verify=_v_der, parse=parse_der, reference=_ref_der,
     wrong=lambda n: (lambda d: {"derangements": d, "probability": float(Fraction(d, math.factorial(int(n))))})(
         sum(math.factorial(int(n)) // math.factorial(k) for k in range(int(n) + 1))),
     random_inputs=lambda rng: {"n": rng.randint(2, _NCAP)},
     templates=["How many derangements of 5 objects are there?",
                "In how many ways can 6 letters be put into envelopes so that no letter is in its own envelope?"],
     examples=[{"inputs": {"n": 5}, "outputs": {"derangements": 44}}, {"inputs": {"n": 0}, "outputs": {"derangements": 1}},
               {"inputs": {"n": 1}, "outputs": {"derangements": 0}}, {"inputs": {"n": 6}, "outputs": {"derangements": 265}}],
     questions=[{"q": "How many derangements of 5 objects are there?", "outputs": {"derangements": 44}},
                {"q": "In how many ways can 6 letters be put into envelopes so that no letter is in its own envelope?",
                 "outputs": {"derangements": 265}}])


def integer_partitions(n):
    n = _whole(n, "n", 0, 2000)
    p = [1] + [0] * n
    for m in range(1, n + 1):
        s, k = 0, 1
        while True:
            g1 = k * (3 * k - 1) // 2
            if g1 > m:
                break
            sign = 1 if k % 2 else -1
            s += sign * p[m - g1]
            g2 = k * (3 * k + 1) // 2
            if g2 <= m:
                s += sign * p[m - g2]
            k += 1
        p[m] = s
    return {"partitions": p[n]}


def _v_ipart(i, o):
    n = int(i["n"])
    ways = [1] + [0] * n
    for part in range(1, n + 1):
        for s in range(part, n + 1):
            ways[s] += ways[s - part]
    return _iv(o["partitions"]) and o["partitions"] == ways[n]


def _ref_ipart(n):
    try:
        from sympy.functions.combinatorial.numbers import partition
        return {"partitions": int(partition(n))}
    except ImportError:
        from sympy import npartitions
        return {"partitions": int(npartitions(n))}


def parse_ipart(q0):
    q = q0.lower()
    if re.search(r"\bset\b|distinct|unequal|different parts|exactly \d+ parts|into \d+ parts|odd parts", q):
        return None
    m = re.search(r"(?:number of )?(?:integer )?partitions of (?:the (?:integer|number) )?(\d+)|\bp\s*\(\s*(\d+)\s*\)", q)
    if m and (re.search(r"partition", q)):
        return {"n": int(_first(m.group(1), m.group(2)))}
    m = re.search(r"ways (?:can|to) (?:we )?(?:write|express) (\d+) as a sum of positive integers", q)
    return {"n": int(m.group(1))} if m else None


_add("integer_partitions", name="Integer partitions p(n)", topic="combinatorics / integer partitions",
     purpose="p(n): the number of ways to write n as a sum of positive integers, order ignored.",
     inputs=[_I("n", "the integer n (0..2000)", 0, 2000)],
     outputs=[_O("partitions", "p(n)")],
     primary="partitions", conditions=["0 <= n <= 2000"], method="Euler's pentagonal number recurrence",
     latex=r"p(n) = \sum_{k \ne 0} (-1)^{k+1}\, p\!\left(n - \tfrac{k(3k-1)}{2}\right)",
     run=integer_partitions, verify=_v_ipart, parse=parse_ipart, reference=_ref_ipart,
     wrong=lambda n: integer_partitions(int(n) - 1),
     random_inputs=lambda rng: {"n": rng.randint(4, 2000)},
     templates=["How many partitions of 10 are there?", "Compute the number of integer partitions of 100."],
     examples=[{"inputs": {"n": 10}, "outputs": {"partitions": 42}}, {"inputs": {"n": 100}, "outputs": {"partitions": 190569292}},
               {"inputs": {"n": 0}, "outputs": {"partitions": 1}}],
     questions=[{"q": "How many partitions of 10 are there?", "outputs": {"partitions": 42}},
                {"q": "Compute the number of integer partitions of 100.", "outputs": {"partitions": 190569292}},
                {"q": "In how many ways can we write 5 as a sum of positive integers?", "outputs": {"partitions": 7}}])


def partitions_k(n, k):
    n, k = _whole(n, "n", 0, 300), _whole(k, "k", 0, 300)
    t = [[0] * (k + 1) for _ in range(n + 1)]
    t[0][0] = 1
    for i in range(1, n + 1):
        for j in range(1, min(i, k) + 1):
            t[i][j] = t[i - 1][j - 1] + (t[i - j][j] if i - j >= j else 0)
    return {"partitions": t[n][k] if k <= n else 0}


def _v_pk(i, o):
    n, k = int(i["n"]), int(i["k"])

    def at_most(m):                      # partitions of n with every part at most m (coin change)
        w = [1] + [0] * n
        for part in range(1, m + 1):
            for s in range(part, n + 1):
                w[s] += w[s - part]
        return w[n]
    want = (1 if n == 0 else 0) if k == 0 else (at_most(k) - at_most(k - 1))      # largest part exactly k (conjugation)
    return _iv(o["partitions"]) and o["partitions"] == want


def _ref_pk(n, k):
    """Generating function: coefficient of x^(n-k) in prod_{i=1..k} 1/(1 - x^i), by sympy polynomial arithmetic."""
    import sympy
    x = sympy.Symbol("x")
    if k == 0:
        return {"partitions": 1 if n == 0 else 0}
    if k > n:
        return {"partitions": 0}
    deg = n - k
    poly = sympy.Poly(1, x)
    for i in range(1, k + 1):
        geo = sympy.Poly(sum(x ** (i * j) for j in range(deg // i + 1)), x)
        poly = poly * geo
        poly = sympy.Poly(sum(c * x ** e[0] for e, c in poly.terms() if e[0] <= deg), x)
    return {"partitions": int(poly.coeff_monomial(x ** deg))}


def parse_pk(q0):
    q = q0.lower()
    m = re.search(r"partitions? of (\d+) into (?:exactly )?(\d+) (?:positive )?parts|(?:write|express) (\d+) as a sum of (?:exactly )?(\d+) "
                  r"positive integers", q)
    if not m:
        return None
    a, b = (m.group(1), m.group(2)) if m.group(1) else (m.group(3), m.group(4))
    return {"n": int(a), "k": int(b)}


_add("partitions_k", name="Partitions into exactly k parts", topic="combinatorics / integer partitions",
     purpose="p(n, k): the number of ways to write n as a sum of exactly k positive integers, order ignored.",
     inputs=[_I("n", "the integer n (0..300)", 0, 300), _I("k", "the number of parts (0..300)", 0, 300)],
     outputs=[_O("partitions", "p(n, k)")],
     primary="partitions", conditions=["0 <= n, k <= 300"], method="recurrence p(n, k) = p(n-1, k-1) + p(n-k, k)",
     latex=r"p(n,k) = p(n-1,k-1) + p(n-k,k)",
     run=partitions_k, verify=_v_pk, parse=parse_pk, reference=_ref_pk,
     wrong=lambda n, k: {"partitions": sum(partitions_k(n, j)["partitions"] for j in range(int(k) + 1))},
     random_inputs=lambda rng: (lambda n: {"n": n, "k": rng.randint(2, max(2, n - 1))})(rng.randint(4, 120)),
     templates=["How many partitions of 10 into exactly 3 parts are there?", "In how many ways can we write 8 as a sum of exactly 4 positive integers?"],
     examples=[{"inputs": {"n": 10, "k": 3}, "outputs": {"partitions": 8}}, {"inputs": {"n": 8, "k": 4}, "outputs": {"partitions": 5}},
               {"inputs": {"n": 5, "k": 7}, "outputs": {"partitions": 0}}],
     questions=[{"q": "How many partitions of 10 into exactly 3 parts are there?", "outputs": {"partitions": 8}},
                {"q": "In how many ways can we write 8 as a sum of exactly 4 positive integers?", "outputs": {"partitions": 5}}])


def partitions_distinct(n):
    n = _whole(n, "n", 0, 1000)
    q = [1] + [0] * n
    for part in range(1, n + 1):
        for s in range(n, part - 1, -1):
            q[s] += q[s - part]
    return {"partitions": q[n]}


def _v_pd(i, o):
    n = int(i["n"])
    w = [1] + [0] * n                    # Euler: distinct parts = odd parts
    for part in range(1, n + 1, 2):
        for s in range(part, n + 1):
            w[s] += w[s - part]
    return _iv(o["partitions"]) and o["partitions"] == w[n]


def _ref_pd(n):
    import sympy
    x = sympy.Symbol("x")
    poly = sympy.Poly(1, x)
    for i in range(1, n + 1):
        poly = poly * sympy.Poly(1 + x ** i, x)
        poly = sympy.Poly(sum(c * x ** e[0] for e, c in poly.terms() if e[0] <= n), x)
    return {"partitions": int(poly.coeff_monomial(x ** n))}


def parse_pd(q0):
    q = q0.lower()
    m = re.search(r"partitions? of (\d+) into (?:distinct|unequal|different) parts|(?:write|express) (\d+) as a sum of (?:distinct|different) "
                  r"positive integers", q)
    return {"n": int(_first(m.group(1), m.group(2)))} if m else None


_add("partitions_distinct", name="Partitions into distinct parts", topic="combinatorics / integer partitions",
     purpose="q(n): the number of ways to write n as a sum of distinct positive integers (equal to partitions into odd parts).",
     inputs=[_I("n", "the integer n (0..1000)", 0, 1000)],
     outputs=[_O("partitions", "q(n)")],
     primary="partitions", conditions=["0 <= n <= 1000"], method="0/1 knapsack counting over the parts 1..n",
     latex=r"\sum_n q(n) x^n = \prod_{k \ge 1} (1 + x^k) = \prod_{k \ge 1} \frac{1}{1 - x^{2k-1}}",
     run=partitions_distinct, verify=_v_pd, parse=parse_pd, reference=_ref_pd,
     wrong=lambda n: integer_partitions(n),
     random_inputs=lambda rng: {"n": rng.randint(3, 60)},
     templates=["How many partitions of 10 into distinct parts are there?", "In how many ways can we write 15 as a sum of distinct positive integers?"],
     examples=[{"inputs": {"n": 10}, "outputs": {"partitions": 10}}, {"inputs": {"n": 15}, "outputs": {"partitions": 27}},
               {"inputs": {"n": 0}, "outputs": {"partitions": 1}}],
     questions=[{"q": "How many partitions of 10 into distinct parts are there?", "outputs": {"partitions": 10}},
                {"q": "In how many ways can we write 15 as a sum of distinct positive integers?", "outputs": {"partitions": 27}}])

def stars_and_bars(total, variables, lower=0, upper=None):
    n, k = _whole(total, "total", 0, 2000), _whole(variables, "variables", 1, 50)
    lo = _whole(lower, "lower", 0, 2000)
    rest = n - k * lo
    if upper is not None:
        hi = _whole(upper, "upper", 0, 2000)
        if hi < lo:
            raise ValueError("the upper bound must be at least the lower bound")
    if rest < 0:
        return {"solutions": 0}
    if upper is None:
        return {"solutions": math.comb(rest + k - 1, k - 1)}
    c = hi - lo + 1
    s = 0
    for j in range(k + 1):
        m = rest - j * c
        if m < 0:
            break
        s += (-1) ** j * math.comb(k, j) * math.comb(m + k - 1, k - 1)
    return {"solutions": s}


def _v_stars(i, o):
    n, k, lo = int(i["total"]), int(i["variables"]), int(i.get("lower") or 0)
    hi = int(i["upper"]) if i.get("upper") is not None else n
    ways = [1] + [0] * n                     # DP: add one variable at a time
    for _ in range(k):
        new = [0] * (n + 1)
        for s, w in enumerate(ways):
            if w:
                for v in range(lo, min(hi, n - s) + 1):
                    new[s + v] += w
        ways = new
    return _iv(o["solutions"]) and o["solutions"] == ways[n]


def _ref_stars(total, variables, lower=0, upper=None):
    import sympy
    x = sympy.Symbol("x")
    hi = total if upper is None else min(int(upper), total)
    factor = sympy.Poly(sum(x ** j for j in range(int(lower), hi + 1)) if hi >= int(lower) else 0, x)
    poly = sympy.Poly(1, x)
    for _ in range(int(variables)):
        poly = poly * factor
        poly = sympy.Poly(sum(c * x ** e[0] for e, c in poly.terms() if e[0] <= total) or 0, x)
    return {"solutions": int(poly.coeff_monomial(x ** total))}


def parse_stars(q0):
    q = q0.lower().replace("≤", "<=").replace("≥", ">=")
    out = None
    m = re.search(r"((?:[a-z](?:_?\d+)?\s*\+\s*)+[a-z](?:_?\d+)?)\s*=\s*(\d+)", q)
    if m and re.search(r"solutions", q):
        nvar = len(re.findall(r"[a-z](?:_?\d+)?", m.group(1)))
        if re.search(r"non-?negative", q):
            out = {"total": int(m.group(2)), "variables": nvar, "lower": 0}
        elif re.search(r"\bpositive", q):
            out = {"total": int(m.group(2)), "variables": nvar, "lower": 1}
        else:
            return None
    m2 = re.search(r"(\d+) identical (?:[a-z]+ )?(?:balls|objects|items|candies|coins|sweets|marbles|cookies|apples|books) "
                   r"(?:be |can be )?(?:placed |put |distributed |divided )?(?:into|among|between|in) (\d+) (?:distinct |different |labell?ed )?"
                   r"(?:boxes|bins|children|people|urns|kids|students)", q)
    if m2 and out is None:
        out = {"total": int(m2.group(1)), "variables": int(m2.group(2)), "lower": 0}
        if re.search(r"at least one|non-?empty|no (?:box|bin|child|urn|kid|student) (?:is )?(?:left )?empty", q):
            out["lower"] = 1
    if out is None:
        return None
    lo = re.search(r"(?:each|every|all)[a-z\s]*?(?:at least|>=|no fewer than)\s*(\d+)", q)
    hi = re.search(r"(?:each|every|all|no)[a-z\s]*?(?:at most|<=|no more than|not exceeding|more than)\s*(\d+)", q)
    if lo:
        out["lower"] = int(lo.group(1))
    if hi:
        out["upper"] = int(hi.group(1))
    return out


_add("stars_and_bars", name="Integer solutions of x1 + ... + xk = n (stars and bars)", topic="combinatorics / distributions",
     purpose="How many integer solutions x1 + ... + xk = n have every xi between a lower and an optional upper bound "
             "(identical objects into distinct boxes).",
     inputs=[_I("total", "the total n (0..2000)", 0, 2000), _I("variables", "the number of variables or boxes k (1..50)", 1, 50),
             _I("lower", "lower bound for every variable (0 = non-negative, 1 = positive)", 0, 2000, required=False),
             _I("upper", "optional upper bound for every variable", 0, 2000, required=False)],
     outputs=[_O("solutions", "number of solutions")],
     primary="solutions", conditions=["the bounds apply to every variable", "a question must say non-negative or positive"],
     method="stars and bars after shifting by the lower bound; inclusion-exclusion over variables exceeding the upper bound",
     latex=r"\#\{x_i \ge 0, \textstyle\sum x_i = n\} = \binom{n+k-1}{k-1},\quad \sum_j (-1)^j \binom{k}{j}\binom{n - j(c+1) + k-1}{k-1}",
     run=stars_and_bars, verify=_v_stars, parse=parse_stars, reference=_ref_stars,
     wrong=lambda total, variables, lower=0, upper=None: (lambda r: {"solutions": math.comb(max(r, 0) + int(variables), int(variables))})(int(total) - int(variables) * int(lower or 0)),
     random_inputs=lambda rng: dict({"total": rng.randint(0, 60), "variables": rng.randint(2, 8), "lower": rng.randint(0, 2)},
                                    **({"upper": rng.randint(3, 15)} if rng.random() < 0.5 else {})),
     templates=["How many non-negative integer solutions does x1 + x2 + x3 = 10 have?",
                "How many positive integer solutions does x + y + z + w = 12 have?",
                "In how many ways can 10 identical balls be placed into 4 distinct boxes?",
                "How many non-negative integer solutions does x1 + x2 + x3 + x4 = 12 have with each variable at most 5?"],
     examples=[{"inputs": {"total": 10, "variables": 3, "lower": 0}, "outputs": {"solutions": 66}},
               {"inputs": {"total": 12, "variables": 4, "lower": 1}, "outputs": {"solutions": 165}},
               {"inputs": {"total": 12, "variables": 4, "lower": 0, "upper": 5}, "outputs": {"solutions": 125}}],
     questions=[{"q": "How many non-negative integer solutions does x1 + x2 + x3 = 10 have?", "outputs": {"solutions": 66}},
                {"q": "How many positive integer solutions does x + y + z + w = 12 have?", "outputs": {"solutions": 165}},
                {"q": "In how many ways can 10 identical balls be placed into 4 distinct boxes?", "outputs": {"solutions": 286}},
                {"q": "In how many ways can 10 identical balls be placed into 4 distinct boxes so that each box gets at least one?",
                 "outputs": {"solutions": 84}},
                {"q": "How many non-negative integer solutions does x1 + x2 + x3 + x4 = 12 have with each variable at most 5?",
                 "outputs": {"solutions": 125}}])


def inclusion_exclusion(n, divisors):
    n = _whole(n, "n", 1)
    ds = sorted(set(_wlist(divisors, "divisors", lo=1, hi=10 ** 9, n_min=1, n_max=12)))
    total = 0

    def walk(start, l, size):
        nonlocal total
        for i in range(start, len(ds)):
            l2 = l // math.gcd(l, ds[i]) * ds[i]
            if l2 > n:
                continue
            total += (-1) ** size * (n // l2)
            walk(i + 1, l2, size + 1)
    walk(0, 1, 0)
    return {"divisible_by_any": total, "divisible_by_none": n - total}


def _v_incl(i, o):
    n = int(i["n"])
    ds = sorted({int(d) for d in i["divisors"]})
    if n <= 3 * 10 ** 5:
        want = sum(1 for x in range(1, n + 1) if any(x % d == 0 for d in ds))
    else:
        want = 0
        for r in range(1, len(ds) + 1):
            for c in itertools.combinations(ds, r):
                want += (-1) ** (r + 1) * (n // math.lcm(*c))
    return _iv(o["divisible_by_any"]) and o["divisible_by_any"] == want and o["divisible_by_none"] == n - want


def parse_incl(q0):
    q = q0.lower()
    if re.search(r"\bnot\b|neither|nor\b|coprime|relatively prime", q):
        return None
    m = re.search(r"how many (?:positive )?integers (?:from 1 to|between 1 and|in the range 1 to|1 through|up to|not exceeding|"
                  r"less than or equal to|from 1 through) (\d+)(?: inclusive)?,? are divisible by (?:at least one of |any of |either )?"
                  r"([\d,\s]+(?:,? ?(?:or|and) \d+)?)", q)
    if not m:
        return None
    return {"n": int(m.group(1)), "divisors": _ints(m.group(2))}


_add("inclusion_exclusion", name="Counting multiples by inclusion-exclusion", topic="combinatorics / inclusion-exclusion",
     purpose="How many integers in 1..n are divisible by at least one of the given numbers (and how many by none).",
     inputs=[_I("n", "the upper end of the range 1..n", 1), _V("divisors", "the divisors (whole numbers >= 1)", 1, 12)],
     outputs=[_O("divisible_by_any", "count divisible by at least one divisor"), _O("divisible_by_none", "count divisible by none")],
     primary="divisible_by_any", conditions=["n >= 1", "1 to 12 divisors, each >= 1"],
     method="inclusion-exclusion over subsets of the divisors, using floor(n / lcm(subset))",
     latex=r"\Big|\bigcup_i A_i\Big| = \sum_{\emptyset \ne S} (-1)^{|S|+1} \left\lfloor \frac{n}{\operatorname{lcm}(S)} \right\rfloor",
     run=inclusion_exclusion, verify=_v_incl, parse=parse_incl,
     reference=lambda n, divisors: (lambda ds: (lambda a: {"divisible_by_any": a, "divisible_by_none": n - a})(
         sum(1 for x in range(1, n + 1) if any(x % d == 0 for d in ds))))(sorted({int(d) for d in divisors})),
     wrong=lambda n, divisors: (lambda a: {"divisible_by_any": a, "divisible_by_none": int(n) - a})(sum(int(n) // d for d in {int(d) for d in divisors})),
     random_inputs=lambda rng: {"n": rng.randint(30, 20000), "divisors": [float(rng.randint(2, 15)) for _ in range(rng.randint(2, 4))]},
     templates=["How many integers from 1 to 1000 are divisible by 3, 5 or 7?", "How many integers from 1 to 100 are divisible by 2 or 3?"],
     examples=[{"inputs": {"n": 1000, "divisors": "3, 5, 7"}, "outputs": {"divisible_by_any": 543, "divisible_by_none": 457}},
               {"inputs": {"n": 100, "divisors": "2, 3"}, "outputs": {"divisible_by_any": 67}},
               {"inputs": {"n": 30, "divisors": "4, 6"}, "outputs": {"divisible_by_any": 10}}],
     questions=[{"q": "How many integers from 1 to 1000 are divisible by 3, 5 or 7?", "outputs": {"divisible_by_any": 543}},
                {"q": "How many integers from 1 to 100 are divisible by 2 or 3?", "outputs": {"divisible_by_any": 67}}])


def pigeonhole_max(items, boxes):
    n, k = _whole(items, "items", 1), _whole(boxes, "boxes", 1)
    return {"guaranteed": -(-n // k)}


def _ref_pmax(items, boxes):
    import z3
    xs = [z3.Int(f"x{i}") for i in range(boxes)]
    t = z3.Int("t")
    o = z3.Optimize()
    o.add(*[x >= 0 for x in xs], *[x <= t for x in xs], z3.Sum(xs) == items)
    h = o.minimize(t)
    o.check()
    return {"guaranteed": int(o.model()[t].as_long())}


def pigeonhole_needed(boxes, at_least):
    k, r = _whole(boxes, "boxes", 1), _whole(at_least, "at_least", 1)
    return {"needed": k * (r - 1) + 1}


def _ref_pneed(boxes, at_least):
    import z3
    xs = [z3.Int(f"x{i}") for i in range(boxes)]
    o = z3.Optimize()
    o.add(*[x >= 0 for x in xs], *[x <= at_least - 1 for x in xs])
    s = z3.Sum(xs)
    o.maximize(s)
    o.check()
    return {"needed": int(o.model().eval(s).as_long()) + 1}


def parse_pmax(q0):
    q = q0.lower()
    if re.search(r"identical|how many (?:[a-z]+ )*?(?:must|need to|should) (?:be |we )?(?:drawn|chosen|picked|selected|taken|draw|choose|pick|select|take)", q):
        return None
    if not re.search(r"at least|guarantee|must|some ", q):
        return None
    m = re.search(r"(\d+) (?:[a-z]+ ){1,2}(?:are |is )?(?:placed|put|distributed|assigned|sorted|dropped|placed) (?:in|into|among|between) (\d+)", q)
    return {"items": int(m.group(1)), "boxes": int(m.group(2))} if m else None


def parse_pneed(q0):
    q = q0.lower()
    if not re.search(r"how many (?:[a-z]+ )*?(?:must|need to|should|do you need to|do we need to) (?:be |we |you )?(?:drawn|chosen|picked|selected|"
                     r"taken|draw|choose|pick|select|take)", q):
        return None
    k = re.search(r"(\d+) (?:different |distinct )?(?:colou?rs|kinds|types|categories|boxes|holes|flavou?rs|suits|groups|sorts)", q)
    r = re.search(r"(?:at least )?(\d+) (?:[a-z]+ )?(?:of the same|in the same|that share|sharing|with the same|matching)", q)
    if not (k and r):
        return {}
    return {"boxes": int(k.group(1)), "at_least": int(r.group(1))}


_add("pigeonhole_max", name="Pigeonhole principle: guaranteed load", topic="combinatorics / pigeonhole principle",
     purpose="Placing n objects into k boxes, some box must hold at least ceil(n / k) objects (and this is the best guarantee).",
     inputs=[_I("items", "number of objects n", 1), _I("boxes", "number of boxes k", 1)],
     outputs=[_O("guaranteed", "ceil(n / k): the largest load every placement must reach in some box")],
     primary="guaranteed", conditions=["n, k >= 1"], method="generalized pigeonhole principle",
     latex=r"\max_i x_i \ge \left\lceil \frac{n}{k} \right\rceil",
     run=pigeonhole_max, verify=lambda i, o: _iv(o["guaranteed"]) and (o["guaranteed"] - 1) * int(i["boxes"]) < int(i["items"]) <= o["guaranteed"] * int(i["boxes"]),
     parse=parse_pmax, reference=_ref_pmax, wrong=lambda items, boxes: {"guaranteed": int(items) // int(boxes)},
     random_inputs=lambda rng: (lambda k: {"items": k * rng.randint(1, 50) + rng.randint(1, k - 1), "boxes": k})(rng.randint(2, 40)),
     templates=["If 25 pigeons are placed into 7 holes, at least how many pigeons must share a hole?",
                "100 letters are put into 9 mailboxes; what is the least number some mailbox must contain?"],
     examples=[{"inputs": {"items": 25, "boxes": 7}, "outputs": {"guaranteed": 4}}, {"inputs": {"items": 100, "boxes": 9}, "outputs": {"guaranteed": 12}},
               {"inputs": {"items": 12, "boxes": 4}, "outputs": {"guaranteed": 3}}],
     questions=[{"q": "If 25 pigeons are placed into 7 holes, at least how many pigeons must share a hole?", "outputs": {"guaranteed": 4}},
                {"q": "100 letters are put into 9 mailboxes; what is the least number some mailbox must contain?", "outputs": {"guaranteed": 12}}])

_add("pigeonhole_needed", name="Pigeonhole principle: objects needed for a guarantee", topic="combinatorics / pigeonhole principle",
     purpose="How many objects drawn from k categories guarantee at least r of one category: k (r - 1) + 1.",
     inputs=[_I("boxes", "number of categories k", 1), _I("at_least", "the guaranteed count r in one category", 1)],
     outputs=[_O("needed", "k (r - 1) + 1")],
     primary="needed", conditions=["k, r >= 1"], method="pigeonhole: k (r - 1) objects can avoid r in every category",
     latex=r"N = k(r-1) + 1",
     run=pigeonhole_needed, verify=lambda i, o: _iv(o["needed"]) and -(-(o["needed"]) // int(i["boxes"])) >= int(i["at_least"]) > -(-(o["needed"] - 1) // int(i["boxes"])),
     parse=parse_pneed, reference=_ref_pneed, wrong=lambda boxes, at_least: {"needed": int(boxes) * int(at_least)},
     random_inputs=lambda rng: {"boxes": rng.randint(2, 40), "at_least": rng.randint(2, 30)},
     templates=["How many socks must be drawn from a drawer with 4 colours to guarantee 3 of the same colour?",
                "How many cards must be chosen from 4 suits to guarantee that at least 5 of the same suit are chosen?"],
     examples=[{"inputs": {"boxes": 4, "at_least": 3}, "outputs": {"needed": 9}}, {"inputs": {"boxes": 4, "at_least": 5}, "outputs": {"needed": 17}},
               {"inputs": {"boxes": 12, "at_least": 2}, "outputs": {"needed": 13}}],
     questions=[{"q": "How many socks must be drawn from a drawer with 4 colours to guarantee 3 of the same colour?", "outputs": {"needed": 9}},
                {"q": "How many cards must be chosen from 4 suits to guarantee that at least 5 of the same suit are chosen?",
                 "outputs": {"needed": 17}}])


def _necklace_count(n, k):
    return sum(euler_phi(d)["phi"] * k ** (n // d) for d in _divisors_from(_factor(n)) if True) // n if n > 1 else k


def necklaces(beads, colours):
    n, k = _whole(beads, "beads", 1, 60), _whole(colours, "colours", 1, 60)
    N = _necklace_count(n, k)
    B = (N + k ** ((n + 1) // 2)) // 2 if n % 2 else (N + (k + 1) * k ** (n // 2) // 2) // 2
    return {"necklaces": N, "bracelets": B}


def _v_neck(i, o):
    n, k = int(i["beads"]), int(i["colours"])
    rot = sum(k ** math.gcd(j, n) for j in range(n))
    ref = n * k ** ((n + 1) // 2) if n % 2 else (n // 2) * (k ** (n // 2) + k ** (n // 2 + 1))
    return _iv(o["necklaces"]) and _iv(o["bracelets"]) and o["necklaces"] * n == rot and o["bracelets"] * 2 * n == rot + ref


def _ref_neck(beads, colours):
    from more_itertools import ilen
    from sympy.utilities.iterables import necklaces as gen
    return {"necklaces": ilen(gen(beads, colours)), "bracelets": ilen(gen(beads, colours, free=True))}


def parse_neck(q0, word):
    q = q0.lower()
    other = "bracelet" if word == "necklace" else "necklace"
    if word not in q or other in q:
        return None
    b = re.search(r"(\d+) (?:beads|stones|pearls|positions)", q)
    c = re.search(r"(\d+) (?:different |distinct |available )?colou?rs", q)
    if not (b and c):
        return {}
    return {"beads": int(b.group(1)), "colours": int(c.group(1))}


_neck_examples = [{"inputs": {"beads": 6, "colours": 2}, "outputs": {"necklaces": 14, "bracelets": 13}},
                  {"inputs": {"beads": 4, "colours": 3}, "outputs": {"necklaces": 24, "bracelets": 21}},
                  {"inputs": {"beads": 5, "colours": 3}, "outputs": {"necklaces": 51, "bracelets": 39}}]
_add("necklaces", name="Necklaces (rotations identified)", topic="combinatorics / counting under symmetry",
     purpose="Distinct necklaces of n beads in k colours when rotations count as the same (and bracelets, also flips).",
     inputs=[_I("beads", "number of beads n (1..60)", 1, 60), _I("colours", "number of colours k (1..60)", 1, 60)],
     outputs=[_O("necklaces", "classes under rotation"), _O("bracelets", "classes under rotation and reflection")],
     primary="necklaces", conditions=["1 <= n, k <= 60"], method="Burnside's lemma over the cyclic group (and the dihedral group)",
     latex=r"N(n,k) = \frac{1}{n}\sum_{d \mid n} \varphi(d)\, k^{n/d}",
     run=necklaces, verify=_v_neck, parse=lambda q: parse_neck(q, "necklace"), reference=_ref_neck,
     wrong=lambda beads, colours: {"necklaces": int(colours) ** int(beads) // int(beads), "bracelets": 0},
     random_inputs=lambda rng: (lambda n: {"beads": n, "colours": rng.randint(2, max(2, int(20000 ** (1 / n))))})(rng.randint(3, 12)),
     templates=["How many necklaces with 6 beads can be made using 2 colours?"],
     examples=_neck_examples,
     questions=[{"q": "How many necklaces with 6 beads can be made using 2 colours?", "outputs": {"necklaces": 14}},
                {"q": "How many distinct necklaces of 4 beads in 3 colours are there?", "outputs": {"necklaces": 24}}])

_add("bracelets", name="Bracelets (rotations and reflections identified)", topic="combinatorics / counting under symmetry",
     purpose="Distinct bracelets of n beads in k colours when rotations and flips count as the same.",
     inputs=[_I("beads", "number of beads n (1..60)", 1, 60), _I("colours", "number of colours k (1..60)", 1, 60)],
     outputs=[_O("bracelets", "classes under rotation and reflection"), _O("necklaces", "classes under rotation only")],
     primary="bracelets", conditions=["1 <= n, k <= 60"], method="Burnside's lemma over the dihedral group",
     latex=r"B(n,k) = \tfrac12 N(n,k) + \begin{cases} \tfrac12 k^{(n+1)/2} & n \text{ odd} \\ \tfrac14 (k+1) k^{n/2} & n \text{ even} \end{cases}",
     run=lambda beads, colours: (lambda r: {"bracelets": r["bracelets"], "necklaces": r["necklaces"]})(necklaces(beads, colours)),
     verify=_v_neck, parse=lambda q: parse_neck(q, "bracelet"), reference=_ref_neck,
     wrong=lambda beads, colours: (lambda r: {"bracelets": r["necklaces"], "necklaces": r["necklaces"]})(necklaces(beads, colours)),
     random_inputs=lambda rng: (lambda n: {"beads": n, "colours": rng.randint(2, max(2, int(20000 ** (1 / n))))})(rng.randint(3, 12)),
     templates=["How many bracelets with 6 beads can be made using 2 colours?"],
     examples=_neck_examples,
     questions=[{"q": "How many bracelets with 6 beads can be made using 2 colours?", "outputs": {"bracelets": 13}},
                {"q": "How many distinct bracelets of 5 beads in 3 colours are there?", "outputs": {"bracelets": 39}}])


def binomial_term(a, b, n, k):
    a, b = _whole(a, "a", hi=10 ** 6), _whole(b, "b", hi=10 ** 6)
    n, k = _whole(n, "n", 0, 1000), _whole(k, "k", 0, 1000)
    if k > n:
        return {"coefficient": 0}
    c = math.comb(n, k) * a ** k * b ** (n - k)
    if abs(c) > 10 ** 300:
        raise ValueError("the coefficient exceeds 10^300")
    return {"coefficient": c}


def _ref_binterm(a, b, n, k):
    import sympy
    x = sympy.Symbol("x")
    return {"coefficient": int(sympy.Poly((a * x + b) ** n, x).coeff_monomial(x ** k))}


def parse_binterm(q0):
    q = q0.lower().replace("−", "-").replace(" ", "")
    m = re.search(r"coefficientof([a-z])(?:(?:\^|\*\*)(\d+))?in(?:theexpansionof)?\(([-+]?\d*)\*?\1([+-]\d+)\)(?:\^|\*\*)(\d+)", q)
    if m:
        a = m.group(3)
        a = -1 if a == "-" else (1 if a in ("", "+") else int(a))
        return {"a": a, "b": int(m.group(4)), "n": int(m.group(5)), "k": int(m.group(2) or 1)}
    m = re.search(r"coefficientof([a-z])(?:(?:\^|\*\*)(\d+))?in(?:theexpansionof)?\(([-+]?\d+)([+-]\d*)\*?\1\)(?:\^|\*\*)(\d+)", q)
    if m:
        a = m.group(4)
        a = -1 if a == "-" else (1 if a == "+" else int(a))
        return {"a": a, "b": int(m.group(3)), "n": int(m.group(5)), "k": int(m.group(2) or 1)}
    return None


_add("binomial_term", name="Coefficient in a binomial expansion", topic="combinatorics / binomial theorem",
     purpose="The coefficient of x^k in (a x + b)^n by the binomial theorem (exact integer).",
     inputs=[_I("a", "coefficient of x inside the bracket", hi=10 ** 6), _I("b", "constant inside the bracket", hi=10 ** 6),
             _I("n", "the power n (0..1000)", 0, 1000), _I("k", "the power of x asked for", 0, 1000)],
     outputs=[_O("coefficient", "C(n, k) a^k b^(n-k)")],
     primary="coefficient", conditions=["integer a, b", "0 <= k <= n <= 1000"], method="binomial theorem",
     latex=r"(ax+b)^n = \sum_{k=0}^{n} \binom{n}{k} a^k b^{n-k} x^k",
     run=binomial_term, verify=lambda i, o: _iv(o["coefficient"]) and o["coefficient"] * math.factorial(int(i["k"])) * math.factorial(int(i["n"]) - int(i["k"])) == math.factorial(int(i["n"])) * int(i["a"]) ** int(i["k"]) * int(i["b"]) ** (int(i["n"]) - int(i["k"])),
     parse=parse_binterm, reference=_ref_binterm,
     wrong=lambda a, b, n, k: {"coefficient": math.comb(int(n), int(k)) * int(a) ** (int(n) - int(k)) * int(b) ** int(k)},
     random_inputs=lambda rng: (lambda n: {"a": rng.choice([-1, 1]) * rng.randint(2, 9), "b": rng.choice([-1, 1]) * rng.randint(2, 9),
                                          "n": n, "k": rng.choice([j for j in range(n + 1) if 2 * j != n])})(rng.randint(2, 40)),
     templates=["What is the coefficient of x^3 in (2x + 3)^5?", "Find the coefficient of x^2 in (1 - 2x)^6."],
     examples=[{"inputs": {"a": 2, "b": 3, "n": 5, "k": 3}, "outputs": {"coefficient": 720}},
               {"inputs": {"a": -2, "b": 1, "n": 6, "k": 2}, "outputs": {"coefficient": 60}},
               {"inputs": {"a": 1, "b": 1, "n": 10, "k": 5}, "outputs": {"coefficient": 252}}],
     questions=[{"q": "What is the coefficient of x^3 in (2x + 3)^5?", "outputs": {"coefficient": 720}},
                {"q": "Find the coefficient of x^2 in (1 - 2x)^6.", "outputs": {"coefficient": 60}},
                {"q": "Find the coefficient of x in (x - 4)^3.", "outputs": {"coefficient": 48}}])


def lattice_paths(right, up):
    m, n = _whole(right, "right", 0, 1000), _whole(up, "up", 0, 1000)
    if m + n > 1000:
        raise ValueError("right + up must be at most 1000")
    return {"paths": math.comb(m + n, m)}


def _ref_lattice(right, up):
    row = [1] * (up + 1)                          # dynamic programming over the grid
    for _ in range(right):
        for j in range(1, up + 1):
            row[j] += row[j - 1]
    return {"paths": row[up]}


def parse_lattice(q0):
    q = q0.lower().replace("−", "-")
    m = re.search(r"from \(\s*(-?\d+)\s*,\s*(-?\d+)\s*\) to \(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)", q)
    if m and re.search(r"path|route|walk", q) and re.search(r"lattice|right|up\b|grid|east|north", q):
        x1, y1, x2, y2 = map(int, m.groups())
        return {"right": x2 - x1, "up": y2 - y1}
    m = re.search(r"(\d+)\s*(?:x|×|by)\s*(\d+) grid", q)
    if m and re.search(r"path|route", q) and re.search(r"right|up\b|down|east|north|south|corner", q):
        return {"right": int(m.group(1)), "up": int(m.group(2))}
    return None


_add("lattice_paths", name="Lattice paths in a grid", topic="combinatorics / basic counting",
     purpose="Monotone lattice paths using unit steps right and up from (0, 0) to (m, n): C(m + n, m).",
     inputs=[_I("right", "number of steps right m", 0, 1000), _I("up", "number of steps up n", 0, 1000)],
     outputs=[_O("paths", "C(m + n, m)")],
     primary="paths", conditions=["m, n >= 0 (the end point is up and to the right)", "an a by b grid of squares has C(a + b, a) corner-to-corner paths"],
     method="choose which of the m + n steps go right", latex=r"\binom{m+n}{m}",
     run=lattice_paths, verify=lambda i, o: _iv(o["paths"]) and o["paths"] * math.factorial(int(i["right"])) * math.factorial(int(i["up"])) == math.factorial(int(i["right"]) + int(i["up"])),
     parse=parse_lattice, reference=_ref_lattice, wrong=lambda right, up: {"paths": math.comb(int(right) + int(up) - 1, int(right))},
     random_inputs=lambda rng: {"right": rng.randint(1, 60), "up": rng.randint(1, 60)},
     templates=["How many lattice paths go from (0, 0) to (5, 3) moving only right and up?",
                "How many shortest paths are there from one corner of a 4 by 4 grid to the opposite corner moving right or up?"],
     examples=[{"inputs": {"right": 5, "up": 3}, "outputs": {"paths": 56}}, {"inputs": {"right": 4, "up": 4}, "outputs": {"paths": 70}},
               {"inputs": {"right": 0, "up": 7}, "outputs": {"paths": 1}}],
     questions=[{"q": "How many lattice paths go from (0, 0) to (5, 3) moving only right and up?", "outputs": {"paths": 56}},
                {"q": "How many shortest paths are there from one corner of a 4 by 4 grid to the opposite corner moving right or up?",
                 "outputs": {"paths": 70}},
                {"q": "How many lattice paths go from (2, 1) to (7, 4) moving only right and up?", "outputs": {"paths": 56}},
                {"q": "How many lattice paths go from (5, 5) to (2, 7) moving only right and up?", "outcome": "INVALID_INPUT"}])


def bernoulli(n):
    n = _whole(n, "n", 0, 200)
    B = [Fraction(1)]
    for m in range(1, n + 1):
        B.append(-sum(math.comb(m + 1, k) * B[k] for k in range(m)) / (m + 1))
    b = B[n]
    return {"numerator": b.numerator, "denominator": b.denominator, "value": float(b)}


def _v_bern(i, o):
    n = int(i["n"])
    a = [Fraction(0)] * (n + 1)                   # Akiyama-Tanigawa (gives B_1 = +1/2)
    for m in range(n + 1):
        a[m] = Fraction(1, m + 1)
        for j in range(m, 0, -1):
            a[j - 1] = j * (a[j - 1] - a[j])
    b = -a[0] if n == 1 else a[0]
    return (_iv(o["numerator"]) and _iv(o["denominator"]) and Fraction(o["numerator"], o["denominator"]) == b
            and o["denominator"] > 0 and o["value"] == float(b))


def _ref_bern(n):
    import sympy
    b = sympy.Rational(-1, 2) if n == 1 else sympy.bernoulli(n)
    return {"numerator": int(b.p), "denominator": int(b.q), "value": float(b)}


def parse_bern(q0):
    q = q0.lower()
    if "bernoulli" not in q:
        return None
    m = re.search(r"bernoulli number\s*(?:b_?\s*|b\(\s*)?(\d+)|(\d+)(?:st|nd|rd|th) bernoulli number|\bb_?\(?\s*(\d+)\s*\)?", q)
    return {"n": int(_first(*m.groups()))} if m else {}


_add("bernoulli", name="Bernoulli numbers", topic="combinatorics / special numbers",
     purpose="The Bernoulli number B_n as an exact fraction (convention B_1 = -1/2).",
     inputs=[_I("n", "the index n (0..200)", 0, 200)],
     outputs=[_O("value", "B_n as a decimal"), _O("numerator", "numerator of B_n in lowest terms"),
              _O("denominator", "denominator of B_n (positive)")],
     primary="value", conditions=["0 <= n <= 200", "B_1 = -1/2 (the 'first' Bernoulli numbers)"],
     method="exact recurrence sum_{k=0}^{m} C(m+1, k) B_k = 0 in rational arithmetic",
     latex=r"\sum_{k=0}^{m} \binom{m+1}{k} B_k = 0,\quad B_0 = 1",
     run=bernoulli, verify=_v_bern, parse=parse_bern, reference=_ref_bern,
     wrong=lambda n: (lambda r: dict(r, numerator=-r["numerator"], value=-r["value"]))(bernoulli(n)),
     random_inputs=lambda rng: {"n": 2 * rng.randint(1, 100)},
     templates=["What is the Bernoulli number B_10?", "Compute the 12th Bernoulli number."],
     examples=[{"inputs": {"n": 10}, "outputs": {"numerator": 5, "denominator": 66}}, {"inputs": {"n": 1}, "outputs": {"numerator": -1, "denominator": 2}},
               {"inputs": {"n": 12}, "outputs": {"numerator": -691, "denominator": 2730}}, {"inputs": {"n": 3}, "outputs": {"numerator": 0}}],
     questions=[{"q": "What is the Bernoulli number B_10?", "outputs": {"numerator": 5, "denominator": 66}},
                {"q": "Compute the 12th Bernoulli number.", "outputs": {"numerator": -691}}])


# ==================================================================================================================
# graph structure (edges input; node names ordered numerically when integers, else alphabetically)
# ==================================================================================================================
def _nkey(x):
    s = str(x)
    return (0, int(s), "") if re.fullmatch(r"-?\d+", s) else (1, 0, s)


def _graph(edges, directed=False, weighted=False):
    if not weighted and any(len(e) == 3 for e in edges):
        raise ValueError("this capability takes an unweighted graph: give edges without weights")
    adj = {}
    for e in edges:
        u, v = str(e[0]).strip(), str(e[1]).strip()
        if u == v:
            raise ValueError(f"self-loop at {u}: simple graphs only")
        adj.setdefault(u, {})
        adj.setdefault(v, {})
        if v in adj[u] or (not directed and u in adj[v]):
            raise ValueError(f"the edge {u}-{v} is given twice")
        adj[u][v] = float(e[2]) if len(e) == 3 else 1.0
        if not directed:
            adj[v][u] = adj[u][v]
    if not adj:
        raise ValueError("the graph has no edges")
    if len(adj) > 400:
        raise ValueError("at most 400 vertices")
    return adj


def _sorted(nodes):
    return sorted(nodes, key=_nkey)


def _components(adj):
    seen, comps = set(), []
    for s in _sorted(adj):
        if s in seen:
            continue
        comp, stack = [], [s]
        seen.add(s)
        while stack:
            u = stack.pop()
            comp.append(u)
            for v in adj[u]:
                if v not in seen:
                    seen.add(v)
                    stack.append(v)
        comps.append(_sorted(comp))
    return comps


def _und(adj):
    """Undirected view of a directed adjacency."""
    out = {u: dict(nb) for u, nb in adj.items()}
    for u, nb in adj.items():
        for v in nb:
            out[v][u] = 1.0
    return out


def _nxg(edges, directed=False):
    import networkx as nx
    G = nx.DiGraph() if directed else nx.Graph()
    G.add_nodes_from(_sorted({str(e[0]) for e in edges} | {str(e[1]) for e in edges}))
    for e in edges:
        G.add_edge(str(e[0]), str(e[1]), weight=float(e[2]) if len(e) == 3 else 1.0)
    return G


def _names(rng, n):
    if rng.random() < 0.5:
        return [str(i) for i in rng.sample(range(1, 3 * n + 2), n)]
    return [chr(65 + i) for i in rng.sample(range(26), n)] if n <= 26 else [f"v{i}" for i in range(n)]


def _rgraph(rng, n, p, connected=False, directed=False, names=None):
    names = names or _names(rng, n)
    edges = set()
    if connected:
        order = names[:]
        rng.shuffle(order)
        for i in range(1, n):
            edges.add((order[rng.randrange(i)], order[i]))
    for i in range(n):
        for j in range(n):
            if i != j and (directed or i < j) and rng.random() < p:
                u, v = names[i], names[j]
                if (u, v) not in edges and (directed or (v, u) not in edges):
                    edges.add((u, v))
    if not edges:
        edges.add((names[0], names[1]))
    out = sorted(edges)
    rng.shuffle(out)
    return out


_EITEM = re.compile(r"\s*([A-Za-z0-9_]+)\s*(->|→|-)\s*([A-Za-z0-9_]+)(?:\s+(-?\d+(?:\.\d+)?))?")


def _qedges(q0):
    """The edge list written after the word 'edges' ('A-B, B-C' or 'A->B 3, B->C 1'): (edges, directed) or None."""
    found = list(re.finditer(r"\bedges?\b\s*(?:are|is|:|=)?\s*", q0, re.I))
    if not found:
        return None
    s, pos, items, seps = q0[found[-1].end():], 0, [], set()
    while True:
        it = _EITEM.match(s, pos)
        if not it:
            break
        u, sep, v, w = it.groups()
        seps.add("->" if sep in ("->", "→") else "-")
        items.append((u, v, float(w)) if w is not None else (u, v))
        pos = it.end()
        # ", and" ends a list as often as "and" or "," does: the last edge of "1-2, 2-3, and 3-1" was once dropped
        nxt = re.match(r"\s*(?:,\s*and\b|;\s*and\b|,|;|\band\b)\s*", s[pos:])
        if not nxt:
            break
        pos += nxt.end()
    if not items or len(seps) > 1 or len({len(e) for e in items}) > 1:
        return None
    directed = "->" in seps or bool(re.search(r"(?<!un)directed|digraph", q0, re.I))
    return items, directed


def _gparse(q0, pattern, exclude=None, want=None):
    q = q0.lower()
    if not re.search(pattern, q) or (exclude and re.search(exclude, q)):
        return None
    g = _qedges(q0)
    if g is None:                         # no edge list: not a question about a given graph
        return None
    edges, directed = g
    if want is not None and directed != want:     # the other kind of graph: a sibling capability reads it
        return None
    return {"edges": edges}


def _source(q0):
    m = re.search(r"(?:starting (?:at|from)|start(?:ing)? at|from|source)\s+(?:node |vertex )?([A-Za-z0-9_]+)", q0, re.I)
    return m.group(1) if m else None


def graph_degrees(edges):
    adj = _graph(edges)
    deg = {v: len(adj[v]) for v in _sorted(adj)}
    seq = sorted(deg.values(), reverse=True)
    return {"vertices": len(adj), "edges": sum(seq) // 2, "degrees": deg, "degree_sequence": seq,
            "max_degree": seq[0], "min_degree": seq[-1]}


def _v_deg(i, o):
    d = {}
    for e in i["edges"]:
        for x in (str(e[0]), str(e[1])):
            d[x] = d.get(x, 0) + 1
    seq = sorted(d.values(), reverse=True)
    return (o["degrees"] == d and o["degree_sequence"] == seq and _iv(o["max_degree"]) and o["max_degree"] == seq[0]
            and o["min_degree"] == seq[-1] and o["edges"] == len(i["edges"]) and o["vertices"] == len(d))


def _ref_deg(edges):
    G = _nxg(edges)
    deg = {v: int(G.degree(v)) for v in G.nodes}
    seq = sorted(deg.values(), reverse=True)
    return {"vertices": G.number_of_nodes(), "edges": G.number_of_edges(), "degrees": deg, "degree_sequence": seq,
            "max_degree": seq[0], "min_degree": seq[-1]}


def _wrong_deg(edges):
    d = {}
    for e in edges:
        d[str(e[0])] = d.get(str(e[0]), 0) + 1
        d.setdefault(str(e[1]), 0)
    seq = sorted(d.values(), reverse=True)
    return {"vertices": len(d), "edges": len(edges), "degrees": d, "degree_sequence": seq, "max_degree": seq[0], "min_degree": seq[-1]}


_G1 = "A-B, A-C, B-C, C-D"
_add("graph_degrees", name="Vertex degrees and degree sequence", topic="graph theory / basic structure",
     purpose="The degree of every vertex of an undirected simple graph, its degree sequence, maximum and minimum degree.",
     inputs=[_E()], outputs=[_O("vertices", "number of vertices (those named in the edges)"), _O("edges", "number of edges"),
                             _O("degrees", "degree of each vertex", "dict"), _O("degree_sequence", "degrees in descending order", "list"),
                             _O("max_degree", "maximum degree"), _O("min_degree", "minimum degree")],
     primary="max_degree", conditions=["undirected simple graph (no loops, no repeated edges), unweighted",
                                       "the vertices are those named in the edges"],
     method="count the edges at each vertex (handshake lemma: the degrees add up to twice the number of edges)",
     latex=r"\sum_v \deg v = 2|E|",
     run=graph_degrees, verify=_v_deg, parse=lambda q: _gparse(q, r"degree sequence|degrees? of (?:each|every|the) (?:vertex|vertices|node|nodes)|"
                                                                r"maximum degree|minimum degree|vertex degrees", r"graphical|in-degree|out-degree", False),
     reference=_ref_deg, wrong=_wrong_deg,
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(3, 15), 0.35)},
     templates=["Find the degree sequence of the graph with edges A-B, A-C, B-C, C-D."],
     examples=[{"inputs": {"edges": _G1}, "outputs": {"degree_sequence": [3, 2, 2, 1], "max_degree": 3, "edges": 4,
                                                      "degrees": {"A": 2, "B": 2, "C": 3, "D": 1}}},
               {"inputs": {"edges": "1-2, 2-3, 3-1"}, "outputs": {"degree_sequence": [2, 2, 2], "min_degree": 2}}],
     questions=[{"q": "Find the degree sequence of the graph with edges A-B, A-C, B-C, C-D.", "outputs": {"degree_sequence": [3, 2, 2, 1]}},
                {"q": "What is the maximum degree of the graph with edges 1-2, 1-3, 1-4, 2-3?", "outputs": {"max_degree": 3}},
                {"q": "Find the degree sequence of the graph with edges A-B, B-A.", "outcome": "INVALID_INPUT"}])


def connected_components(edges):
    comps = sorted(_components(_graph(edges)), key=lambda c: _nkey(c[0]))
    return {"count": len(comps), "sizes": sorted((len(c) for c in comps), reverse=True), "is_connected": len(comps) == 1,
            "components": comps}


def _v_cc(i, o):
    parent = {}

    def find(x):
        while parent.setdefault(x, x) != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    for e in i["edges"]:
        parent[find(str(e[0]))] = find(str(e[1]))
    groups = {}
    for x in list(parent):
        groups.setdefault(find(x), []).append(x)
    comps = sorted((_sorted(g) for g in groups.values()), key=lambda c: _nkey(c[0]))
    return _iv(o["count"]) and o["count"] == len(comps) and o["components"] == comps and o["is_connected"] is (len(comps) == 1) \
        and o["sizes"] == sorted((len(c) for c in comps), reverse=True)


def _v_cc_count(edges):
    """Number of components by union-find (a second method)."""
    parent = {}

    def find(x):
        while parent.setdefault(x, x) != x:
            x = parent[x]
        return x
    for e in edges:
        parent[find(str(e[0]))] = find(str(e[1]))
    return len({find(x) for x in list(parent)})


def _ref_cc(edges):
    import networkx as nx
    comps = sorted((_sorted(c) for c in nx.connected_components(_nxg(edges))), key=lambda c: _nkey(c[0]))
    return {"count": len(comps), "sizes": sorted((len(c) for c in comps), reverse=True), "components": comps}


def _rand_forest_plus(rng):
    n = rng.randint(4, 18)
    names = _names(rng, n)
    edges = set()
    for i in range(1, n):
        if rng.random() < 0.7:
            edges.add((names[rng.randrange(i)], names[i]))
    for _ in range(rng.randint(1, 4)):
        a, b = rng.sample(names, 2)
        if (a, b) not in edges and (b, a) not in edges:
            edges.add((a, b))
    return {"edges": sorted(edges)}


_add("connected_components", name="Connected components", topic="graph theory / connectivity",
     purpose="The connected components of an undirected graph: how many, their sizes and members, and whether it is connected.",
     inputs=[_E()], outputs=[_O("count", "number of connected components"), _O("sizes", "component sizes, descending", "list"),
                             _O("is_connected", "true when there is one component", "boolean"),
                             _O("components", "members of each component (ascending), components ordered by their first member", "list")],
     primary="count", conditions=["undirected simple graph, unweighted", "the vertices are those named in the edges"],
     method="graph search from each unvisited vertex", latex=r"V = C_1 \sqcup \dots \sqcup C_c",
     run=connected_components, verify=_v_cc, parse=lambda q: _gparse(q, r"connected components|how many components|\bconnected\?|is (?:the|this) graph\b.*\bconnected\b",
                                                                      r"strongly", False),
     reference=_ref_cc, wrong=lambda edges: (lambda adj: {"count": len(adj) - len(edges), "sizes": [], "is_connected": len(adj) - len(edges) == 1, "components": []})(_graph(edges)),
     random_inputs=_rand_forest_plus,
     templates=["How many connected components does the graph with edges A-B, C-D, D-E have?",
                "Is the graph with edges 1-2, 2-3, 4-5 connected?"],
     examples=[{"inputs": {"edges": "A-B, C-D, D-E"}, "outputs": {"count": 2, "sizes": [3, 2], "components": [["A", "B"], ["C", "D", "E"]]}},
               {"inputs": {"edges": "1-2, 2-3, 3-1"}, "outputs": {"count": 1, "is_connected": True}}],
     questions=[{"q": "How many connected components does the graph with edges A-B, C-D, D-E have?", "outputs": {"count": 2}},
                {"q": "Is the graph with edges 1-2, 2-3, 4-5 connected?", "outputs": {"is_connected": False}}])


def _bfs(adj, s, desc=False):
    from collections import deque
    dist, order, dq = {s: 0}, [s], deque([s])
    while dq:
        u = dq.popleft()
        for v in sorted(adj[u], key=_nkey, reverse=desc):
            if v not in dist:
                dist[v] = dist[u] + 1
                order.append(v)
                dq.append(v)
    return order, dist


def bfs_order(edges, source, directed=False):
    adj = _graph(edges, directed=bool(directed))
    s = str(source).strip()
    if s not in adj:
        raise ValueError(f"{s} is not a vertex of the graph")
    order, dist = _bfs(adj, s)
    return {"order": order, "reached": len(order), "distances": {v: dist[v] for v in order}}


def _v_bfs(i, o):
    adj = _graph(i["edges"], directed=bool(i.get("directed")))
    order, dist = _bfs(adj, str(i["source"]).strip())
    return o["order"] == order and _iv(o["reached"]) and o["reached"] == len(order) and o["distances"] == dist


def _ref_bfs(edges, source, directed=False):
    import networkx as nx
    G = _nxg(edges, bool(directed))
    s = str(source).strip()
    order = [s] + [v for _u, v in nx.bfs_edges(G, s, sort_neighbors=lambda ns: sorted(ns, key=_nkey))]
    return {"order": order, "reached": len(order), "distances": dict(nx.single_source_shortest_path_length(G, s))}


def _rand_search(rng):
    n = rng.randint(5, 14)
    directed = rng.random() < 0.3
    e = _rgraph(rng, n, 0.3, connected=not directed, directed=directed)
    x = {"edges": e, "source": e[0][0]}
    if directed:
        x["directed"] = True
    return x


def _parse_search(q0, pat):
    out = _gparse(q0, pat)
    if not out or "_out_of_scope" in out:
        return out
    g = _qedges(q0)
    s = _source(q0)
    if s:
        out["source"] = s
    if g and g[1]:
        out["directed"] = True
    return out


_add("bfs_order", name="Breadth-first search order", topic="graph theory / graph search",
     purpose="The order in which breadth-first search from a source visits the vertices, and each vertex's distance in edges.",
     inputs=[_E(), _N("source", "the start vertex"), _B("directed", "true for a directed graph (edges u->v)")],
     outputs=[_O("order", "visiting order", "list"), _O("reached", "number of vertices reached"),
              _O("distances", "number of edges from the source to each reached vertex", "dict")],
     primary="reached", conditions=["simple unweighted graph", "TIE RULE: neighbours are visited in ascending order of their names "
                                    "(numerically when the names are integers)"],
     method="queue-based breadth-first search", latex=r"d(v) = d(u) + 1 \text{ when } v \text{ is first seen from } u",
     run=bfs_order, verify=_v_bfs, parse=lambda q: _parse_search(q, r"breadth[- ]first|\bbfs\b"), reference=_ref_bfs,
     wrong=lambda edges, source, directed=False: (lambda adj: (lambda od: {"order": od[0], "reached": len(od[0]), "distances": od[1]})(
         _bfs(adj, str(source), desc=True)))(_graph(edges, directed=bool(directed))),
     random_inputs=_rand_search,
     templates=["Give the breadth-first search order from A in the graph with edges A-B, A-C, B-D, C-D, D-E."],
     examples=[{"inputs": {"edges": "A-B, A-C, B-D, C-D, D-E", "source": "A"}, "outputs": {"order": ["A", "B", "C", "D", "E"], "reached": 5,
                                                                                          "distances": {"A": 0, "B": 1, "C": 1, "D": 2, "E": 3}}},
               {"inputs": {"edges": "1-10, 1-2, 2-3", "source": "1"}, "outputs": {"order": ["1", "2", "10", "3"]}}],
     questions=[{"q": "Give the breadth-first search order from A in the graph with edges A-B, A-C, B-D, C-D, D-E.",
                 "outputs": {"order": ["A", "B", "C", "D", "E"]}},
                {"q": "Run BFS starting at 1 on the directed graph with edges 1->3, 1->2, 3->4, 2->4.", "outputs": {"order": ["1", "2", "3", "4"]}},
                {"q": "Run BFS starting at Z on the graph with edges A-B, B-C.", "outcome": "INVALID_INPUT"}])


def _dfs(adj, s):
    seen, order, stack = {s}, [s], [iter(_sorted(adj[s]))]
    while stack:
        for v in stack[-1]:
            if v not in seen:
                seen.add(v)
                order.append(v)
                stack.append(iter(_sorted(adj[v])))
                break
        else:
            stack.pop()
    return order


def dfs_order(edges, source, directed=False):
    adj = _graph(edges, directed=bool(directed))
    s = str(source).strip()
    if s not in adj:
        raise ValueError(f"{s} is not a vertex of the graph")
    order = _dfs(adj, s)
    return {"order": order, "reached": len(order)}


def _v_dfs(i, o):
    adj = _graph(i["edges"], directed=bool(i.get("directed")))
    s = str(i["source"]).strip()
    seen, order = set(), []

    def visit(u):
        seen.add(u)
        order.append(u)
        for v in _sorted(adj[u]):
            if v not in seen:
                visit(v)
    visit(s)
    return o["order"] == order and _iv(o["reached"]) and o["reached"] == len(order)


def _ref_dfs(edges, source, directed=False):
    import networkx as nx
    order = list(nx.dfs_preorder_nodes(_nxg(edges, bool(directed)), str(source).strip(), sort_neighbors=lambda ns: sorted(ns, key=_nkey)))
    return {"order": order, "reached": len(order)}


def _wrong_dfs(edges, source, directed=False):
    adj = _graph(edges, directed=bool(directed))
    seen, order, stack = set(), [], [str(source)]
    while stack:                                  # stack-based DFS pushing neighbours in ascending order
        u = stack.pop()
        if u in seen:
            continue
        seen.add(u)
        order.append(u)
        stack.extend(v for v in _sorted(adj[u]) if v not in seen)
    return {"order": order, "reached": len(order)}


_add("dfs_order", name="Depth-first search order", topic="graph theory / graph search",
     purpose="The preorder in which depth-first search from a source visits the vertices.",
     inputs=[_E(), _N("source", "the start vertex"), _B("directed", "true for a directed graph (edges u->v)")],
     outputs=[_O("order", "visiting order (preorder)", "list"), _O("reached", "number of vertices reached")],
     primary="reached", conditions=["simple unweighted graph", "TIE RULE: recursive DFS that tries neighbours in ascending order of "
                                    "their names (numerically when the names are integers)"],
     method="depth-first search (recursive semantics, kept on an explicit stack of neighbour iterators)",
     latex=r"\mathrm{visit}(u):\ \text{mark } u;\ \forall v \in N(u) \text{ ascending, unmarked: } \mathrm{visit}(v)",
     run=dfs_order, verify=_v_dfs, parse=lambda q: _parse_search(q, r"depth[- ]first|\bdfs\b"), reference=_ref_dfs, wrong=_wrong_dfs,
     random_inputs=_rand_search,
     templates=["Give the depth-first search order from A in the graph with edges A-B, A-C, B-D, C-D, D-E."],
     examples=[{"inputs": {"edges": "A-B, A-C, B-D, C-D, D-E", "source": "A"}, "outputs": {"order": ["A", "B", "D", "C", "E"], "reached": 5}},
               {"inputs": {"edges": "1-2, 1-3, 2-4, 3-4", "source": "1"}, "outputs": {"order": ["1", "2", "4", "3"]}}],
     questions=[{"q": "Give the depth-first search order from A in the graph with edges A-B, A-C, B-D, C-D, D-E.",
                 "outputs": {"order": ["A", "B", "D", "C", "E"]}},
                {"q": "Run DFS starting at 1 on the directed graph with edges 1->2, 2->3, 1->3.", "outputs": {"order": ["1", "2", "3"]}}])


def graph_diameter(edges):
    adj = _graph(edges)
    if len(_components(adj)) != 1:
        raise ValueError("the graph is not connected (the diameter is infinite)")
    ecc = {v: max(_bfs(adj, v)[1].values()) for v in adj}
    d, r = max(ecc.values()), min(ecc.values())
    return {"diameter": d, "radius": r, "center": _sorted(v for v in adj if ecc[v] == r),
            "periphery": _sorted(v for v in adj if ecc[v] == d)}


def _v_diam(i, o):
    adj = _graph(i["edges"])
    nodes = _sorted(adj)
    idx = {v: k for k, v in enumerate(nodes)}
    INF = 10 ** 9
    D = [[0 if a == b else (1 if nodes[b] in adj[nodes[a]] else INF) for b in range(len(nodes))] for a in range(len(nodes))]
    for k in range(len(nodes)):                    # Floyd-Warshall
        Dk = D[k]
        for a in range(len(nodes)):
            Da = D[a]
            if Da[k] < INF:
                for b in range(len(nodes)):
                    if Da[k] + Dk[b] < Da[b]:
                        Da[b] = Da[k] + Dk[b]
    ecc = [max(row) for row in D]
    d, r = max(ecc), min(ecc)
    return (_iv(o["diameter"]) and o["diameter"] == d and o["radius"] == r and o["center"] == [v for v in nodes if ecc[idx[v]] == r]
            and o["periphery"] == [v for v in nodes if ecc[idx[v]] == d])


def _ref_diam(edges):
    import networkx as nx
    G = _nxg(edges)
    return {"diameter": nx.diameter(G), "radius": nx.radius(G), "center": _sorted(nx.center(G)), "periphery": _sorted(nx.periphery(G))}


_add("graph_diameter", name="Diameter, radius and center", topic="graph theory / distances",
     purpose="Eccentricities of a connected unweighted graph: the diameter (largest), radius (smallest), center and periphery.",
     inputs=[_E()], outputs=[_O("diameter", "largest distance between two vertices"), _O("radius", "smallest eccentricity"),
                             _O("center", "vertices of smallest eccentricity (ascending)", "list"),
                             _O("periphery", "vertices of largest eccentricity (ascending)", "list")],
     primary="diameter", conditions=["connected undirected simple graph, unweighted (distances count edges)"],
     method="breadth-first search from every vertex", latex=r"\operatorname{diam} G = \max_v e(v),\ \operatorname{rad} G = \min_v e(v)",
     run=graph_diameter, verify=_v_diam, parse=lambda q: _gparse(q, r"diameter|radius|eccentricit|cent(?:er|re) of the graph|periphery", None, False),
     reference=_ref_diam, wrong=lambda edges: (lambda r, adj: dict(r, diameter=max(_bfs(adj, _sorted(adj)[0])[1].values())))(graph_diameter(edges), _graph(edges)),
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(6, 16), 0.12, connected=True)},
     templates=["What is the diameter of the graph with edges A-B, B-C, C-D, D-E, B-F?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-D, D-E, B-F"}, "outputs": {"diameter": 4, "radius": 2, "center": ["C"], "periphery": ["A", "E", "F"]}},
               {"inputs": {"edges": "1-2, 2-3, 3-1"}, "outputs": {"diameter": 1, "radius": 1}}],
     questions=[{"q": "What is the diameter of the graph with edges A-B, B-C, C-D, D-E, B-F?", "outputs": {"diameter": 4}},
                {"q": "Find the radius and center of the graph with edges 1-2, 2-3, 3-4.", "outputs": {"radius": 2, "center": ["2", "3"]}},
                {"q": "What is the diameter of the graph with edges A-B, C-D?", "outcome": "INVALID_INPUT"}])


def topological_sort(edges):
    adj = _graph(edges, directed=True)
    indeg = {v: 0 for v in adj}
    for u in adj:
        for v in adj[u]:
            indeg[v] += 1
    heap = [(_nkey(v), v) for v in adj if indeg[v] == 0]
    heapq.heapify(heap)
    order = []
    while heap:
        _k, u = heapq.heappop(heap)
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                heapq.heappush(heap, (_nkey(v), v))
    if len(order) != len(adj):
        raise ValueError("the graph has a directed cycle, so no topological order exists")
    return {"order": order, "vertices": len(order)}


def _v_topo(i, o):
    adj = _graph(i["edges"], directed=True)
    order = o["order"]
    if sorted(order, key=_nkey) != _sorted(adj) or o["vertices"] != len(order):
        return False
    indeg = {v: 0 for v in adj}
    for u in adj:
        for v in adj[u]:
            indeg[v] += 1
    for u in order:                     # each step takes the smallest vertex that is free at that moment
        avail = [v for v, d in indeg.items() if d == 0]
        if u not in avail or min(avail, key=_nkey) != u:
            return False
        del indeg[u]
        for v in adj[u]:
            indeg[v] -= 1
    return True


def _ref_topo(edges):
    import networkx as nx
    order = list(nx.lexicographical_topological_sort(_nxg(edges, True), key=_nkey))
    return {"order": order, "vertices": len(order)}


def _wrong_topo(edges):
    adj = _graph(edges, directed=True)
    seen, post = set(), []

    def visit(u):
        seen.add(u)
        for v in _sorted(adj[u]):
            if v not in seen:
                visit(v)
        post.append(u)
    for v in _sorted(adj):
        if v not in seen:
            visit(v)
    return {"order": post[::-1], "vertices": len(post)}


def _rdag(rng, n=None, p=0.3):
    n = n or rng.randint(5, 14)
    names = _names(rng, n)
    rank = names[:]
    rng.shuffle(rank)
    e = [(rank[i], rank[j]) for i in range(n) for j in range(i + 1, n) if rng.random() < p]
    return e or [(rank[0], rank[1])]


_add("topological_sort", name="Topological sort", topic="graph theory / directed acyclic graphs",
     purpose="An ordering of a directed acyclic graph's vertices in which every edge u->v has u before v.",
     inputs=[_E("edges", "the directed graph as edges u->v")],
     outputs=[_O("order", "the lexicographically smallest topological order", "list"), _O("vertices", "number of vertices")],
     primary="vertices", conditions=["directed acyclic simple graph, unweighted", "TIE RULE: among the vertices free at each step the "
                                     "smallest name comes first (numerically when integers): the lexicographically smallest order"],
     method="Kahn's algorithm with a min-priority queue of in-degree-zero vertices",
     latex=r"(u \to v) \in E \Rightarrow \pi(u) < \pi(v)",
     run=topological_sort, verify=_v_topo, parse=lambda q: _gparse(q, r"topological(?:ly)? (?:sort|order)", None, True),
     reference=_ref_topo, wrong=_wrong_topo, random_inputs=lambda rng: {"edges": _rdag(rng)},
     templates=["Find a topological sort of the directed graph with edges A->C, B->C, C->D, B->D."],
     examples=[{"inputs": {"edges": [["A", "C"], ["B", "C"], ["C", "D"], ["B", "D"]]}, "outputs": {"order": ["A", "B", "C", "D"]}},
               {"inputs": {"edges": [["5", "11"], ["7", "11"], ["7", "8"], ["3", "8"], ["3", "10"], ["11", "2"], ["11", "9"], ["11", "10"], ["8", "9"]]},
                "outputs": {"order": ["3", "5", "7", "8", "11", "2", "9", "10"]}}],
     questions=[{"q": "Find a topological sort of the directed graph with edges A->C, B->C, C->D, B->D.", "outputs": {"order": ["A", "B", "C", "D"]}},
                {"q": "Find a topological order of the directed graph with edges 1->2, 2->3, 3->1.", "outcome": "INVALID_INPUT"}])


def cycle_rank(edges):
    adj = _graph(edges)
    c = len(_components(adj))
    m = sum(len(nb) for nb in adj.values()) // 2
    r = m - len(adj) + c
    return {"cyclomatic_number": r, "has_cycle": r > 0}


def _ref_crank(edges):
    import networkx as nx
    r = len(nx.cycle_basis(_nxg(edges)))
    return {"cyclomatic_number": r, "has_cycle": r > 0}


def _v_crank(i, o):
    parent, extra = {}, 0

    def find(x):
        while parent.setdefault(x, x) != x:
            x = parent[x]
        return x
    for e in i["edges"]:
        a, b = find(str(e[0])), find(str(e[1]))
        if a == b:
            extra += 1
        else:
            parent[a] = b
    return _iv(o["cyclomatic_number"]) and o["cyclomatic_number"] == extra and o["has_cycle"] is (extra > 0)


_add("cycle_rank", name="Cycles in an undirected graph (cyclomatic number)", topic="graph theory / cycles",
     purpose="Whether an undirected graph has a cycle, and its cyclomatic number m - n + c (independent cycles).",
     inputs=[_E()], outputs=[_O("cyclomatic_number", "m - n + c: the number of independent cycles"),
                             _O("has_cycle", "true when the graph contains a cycle", "boolean")],
     primary="cyclomatic_number", conditions=["undirected simple graph, unweighted"],
     method="count components; every edge beyond a spanning forest closes one independent cycle",
     latex=r"\mu(G) = |E| - |V| + c(G)",
     run=cycle_rank, verify=_v_crank,
     parse=lambda q: _gparse(q, r"cyclomatic|circuit rank|cycle rank|(?:have|has|contain|contains) (?:a |any )?cycles?|is (?:the|this) graph acyclic|"
                                r"independent cycles", r"hamilton|euler|girth|shortest cycle|(?<!un)directed|->", False),
     reference=_ref_crank, wrong=lambda edges: (lambda adj: (lambda r: {"cyclomatic_number": r, "has_cycle": r > 0})(
         sum(len(nb) for nb in adj.values()) // 2 - len(adj)))(_graph(edges)),
     random_inputs=_rand_forest_plus,
     templates=["What is the cyclomatic number of the graph with edges A-B, B-C, C-A, D-E?",
                "Does the graph with edges 1-2, 2-3, 3-4 contain a cycle?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, D-E"}, "outputs": {"cyclomatic_number": 1, "has_cycle": True}},
               {"inputs": {"edges": "1-2, 2-3, 3-4"}, "outputs": {"cyclomatic_number": 0, "has_cycle": False}}],
     questions=[{"q": "What is the cyclomatic number of the graph with edges A-B, B-C, C-A, D-E?", "outputs": {"cyclomatic_number": 1}},
                {"q": "Does the graph with edges 1-2, 2-3, 3-4 contain a cycle?", "outputs": {"has_cycle": False}}])


def directed_cycle(edges):
    adj = _graph(edges, directed=True)
    indeg = {v: 0 for v in adj}
    for u in adj:
        for v in adj[u]:
            indeg[v] += 1
    stack = [v for v in adj if indeg[v] == 0]
    seen = 0
    while stack:
        u = stack.pop()
        seen += 1
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                stack.append(v)
    return {"has_cycle": seen != len(adj), "is_dag": seen == len(adj)}


def _v_dcyc(i, o):
    adj = _graph(i["edges"], directed=True)
    color = {v: 0 for v in adj}
    found = False

    def visit(u):
        nonlocal found
        color[u] = 1
        for v in adj[u]:
            if color[v] == 1:
                found = True
            elif color[v] == 0:
                visit(v)
        color[u] = 2
    for v in adj:
        if color[v] == 0:
            visit(v)
    return o["has_cycle"] is found and o["is_dag"] is (not found)


def _rand_dcyc(rng):
    e = _rdag(rng, p=0.35)
    if rng.random() < 0.3 and len(e) > 1:
        u, v = e[rng.randrange(len(e))]
        e = [x for x in e if x != (u, v)] + [(v, u)] if rng.random() < 0.5 else e + [(e[-1][1], e[0][0])] if (e[0][0], e[-1][1]) not in e else e
    return {"edges": list(dict.fromkeys(e))}


_add("directed_cycle", name="Cycle detection in a directed graph", topic="graph theory / directed acyclic graphs",
     purpose="Whether a directed graph contains a directed cycle (equivalently, whether it is a DAG).",
     inputs=[_E("edges", "the directed graph as edges u->v")],
     outputs=[_O("has_cycle", "true when a directed cycle exists", "boolean"), _O("is_dag", "true when the graph is acyclic", "boolean")],
     primary="has_cycle", conditions=["directed simple graph, unweighted"],
     method="Kahn's algorithm: the graph is acyclic exactly when every vertex is removed",
     latex=r"G \text{ acyclic} \iff \text{a topological order exists}",
     run=directed_cycle, verify=_v_dcyc,
     parse=lambda q: _gparse(q, r"cycle|acyclic|\bdag\b", r"hamilton|euler|girth|topological", True),
     reference=lambda edges: (lambda dag: {"has_cycle": not dag, "is_dag": dag})(__import__("networkx").is_directed_acyclic_graph(_nxg(edges, True))),
     wrong=lambda edges: (lambda r: {"has_cycle": r["has_cycle"], "is_dag": not r["has_cycle"]})(cycle_rank([(str(u), str(v)) for u, v, *_ in
                                                                                                          {tuple(sorted((str(e[0]), str(e[1])), key=_nkey)): e for e in edges}.values()])),
     random_inputs=_rand_dcyc,
     templates=["Does the directed graph with edges A->B, B->C, C->A have a cycle?", "Is the directed graph with edges 1->2, 1->3, 2->3 acyclic?"],
     examples=[{"inputs": {"edges": [["A", "B"], ["B", "C"], ["C", "A"]]}, "outputs": {"has_cycle": True, "is_dag": False}},
               {"inputs": {"edges": [["1", "2"], ["1", "3"], ["2", "3"]]}, "outputs": {"has_cycle": False}}],
     questions=[{"q": "Does the directed graph with edges A->B, B->C, C->A have a cycle?", "outputs": {"has_cycle": True}},
                {"q": "Is the directed graph with edges 1->2, 1->3, 2->3 acyclic?", "outputs": {"is_dag": True}}])


def _two_colour(adj):
    col = {}
    for s in _sorted(adj):
        if s in col:
            continue
        col[s] = 0
        stack = [s]
        while stack:
            u = stack.pop()
            for v in adj[u]:
                if v not in col:
                    col[v] = 1 - col[u]
                    stack.append(v)
                elif col[v] == col[u]:
                    return None
    return col


def bipartite(edges):
    return {"is_bipartite": _two_colour(_graph(edges)) is not None}


def _v_bip(i, o):
    adj = _graph(i["edges"])
    from collections import deque
    side, ok = {}, True
    for s in adj:
        if s in side:
            continue
        side[s] = 1
        dq = deque([s])
        while dq and ok:
            u = dq.popleft()
            for v in adj[u]:
                if v not in side:
                    side[v] = -side[u]
                    dq.append(v)
                elif side[v] == side[u]:
                    ok = False
    return o["is_bipartite"] is ok


def _rand_bip(rng):
    a, b = rng.randint(2, 6), rng.randint(2, 6)
    names = _names(rng, a + b + 9)
    L, R, extra = names[:a], names[a:a + b], names[a + b:]
    e = [(u, v) for u in L for v in R if rng.random() < 0.5] or [(L[0], R[0])]
    if rng.random() < 0.65:                      # attach an odd cycle of length 5, 7 or 9 (no triangle)
        k = rng.choice([5, 7, 9])
        cyc = extra[:k]
        e += [(cyc[j], cyc[(j + 1) % k]) for j in range(k)] + [(e[0][0], cyc[0])]
    return {"edges": e}


_add("bipartite", name="Bipartiteness test", topic="graph theory / colouring",
     purpose="Whether the vertices of an undirected graph split into two sets with every edge between them (no odd cycle).",
     inputs=[_E()], outputs=[_O("is_bipartite", "true when the graph is 2-colourable", "boolean")],
     primary="is_bipartite", conditions=["undirected simple graph, unweighted"],
     method="2-colouring by graph search; a conflict exposes an odd cycle", latex=r"G \text{ bipartite} \iff G \text{ has no odd cycle}",
     run=bipartite, verify=_v_bip, parse=lambda q: _gparse(q, r"bipartite|two-colou?rable|2-colou?rable", r"matching", False),
     reference=lambda edges: {"is_bipartite": bool(__import__("networkx").is_bipartite(_nxg(edges)))},
     wrong=lambda edges: {"is_bipartite": graph_triangles(edges)["triangles"] == 0},
     random_inputs=_rand_bip,
     templates=["Is the graph with edges A-B, B-C, C-D, D-A bipartite?", "Is the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1 bipartite?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-D, D-A"}, "outputs": {"is_bipartite": True}},
               {"inputs": {"edges": "1-2, 2-3, 3-4, 4-5, 5-1"}, "outputs": {"is_bipartite": False}}],
     questions=[{"q": "Is the graph with edges A-B, B-C, C-D, D-A bipartite?", "outputs": {"is_bipartite": True}},
                {"q": "Is the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1 bipartite?", "outputs": {"is_bipartite": False}}])


def eulerian(edges):
    adj = _graph(edges)
    odd = sum(1 for v in adj if len(adj[v]) % 2)
    conn = len(_components(adj)) == 1
    return {"odd_vertices": odd, "connected": conn, "has_circuit": conn and odd == 0, "has_path": conn and odd in (0, 2)}


def _ref_euler(edges):
    import networkx as nx
    G = _nxg(edges)
    return {"odd_vertices": sum(1 for v in G if G.degree(v) % 2), "connected": nx.is_connected(G),
            "has_circuit": nx.is_eulerian(G), "has_path": nx.has_eulerian_path(G)}


def _rand_euler(rng):
    names = _names(rng, 16)
    e = set()
    pools = [names[:8], names[8:]] if rng.random() < 0.85 else [names[:8], names[:8]]
    for pool in pools:
        k = rng.randint(3, 5)
        cyc = rng.sample(pool, k)
        for j in range(k):
            a, b = cyc[j], cyc[(j + 1) % k]
            if (a, b) not in e and (b, a) not in e:
                e.add((a, b))
    if rng.random() < 0.3:
        a, b = rng.sample(names, 2)
        if (a, b) not in e and (b, a) not in e:
            e.add((a, b))
    return {"edges": sorted(e)}


_add("eulerian", name="Eulerian circuits and paths", topic="graph theory / Euler and Hamilton",
     purpose="Whether an undirected graph has an Euler circuit (closed trail using every edge once) or an Euler path.",
     inputs=[_E()], outputs=[_O("odd_vertices", "number of vertices of odd degree"), _O("connected", "true when the graph is connected", "boolean"),
                             _O("has_circuit", "Euler circuit exists: connected and no odd vertex", "boolean"),
                             _O("has_path", "Euler path exists: connected and 0 or 2 odd vertices", "boolean")],
     primary="odd_vertices", conditions=["undirected simple graph, unweighted", "the vertices are those named in the edges"],
     method="Euler's theorem: degrees and connectivity", latex=r"\text{circuit} \iff \text{connected} \wedge \forall v:\ 2 \mid \deg v",
     run=eulerian, verify=lambda i, o: o == eulerian(i["edges"]) and _iv(o["odd_vertices"]) and o["odd_vertices"] == sum(
         1 for v, d in __import__("collections").Counter([str(x) for e in i["edges"] for x in e[:2]]).items() if d % 2),
     parse=lambda q: _gparse(q, r"euler(?:ian)?", None, False), reference=_ref_euler,
     wrong=lambda edges: (lambda r: dict(r, connected=True, has_circuit=r["odd_vertices"] == 0, has_path=r["odd_vertices"] in (0, 2)))(eulerian(edges)),
     random_inputs=_rand_euler,
     templates=["Does the graph with edges A-B, B-C, C-A, C-D, D-E, E-C have an Euler circuit?",
                "Does the graph with edges 1-2, 2-3, 3-4 have an Eulerian path?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D, D-E, E-C"}, "outputs": {"has_circuit": True, "odd_vertices": 0}},
               {"inputs": {"edges": "1-2, 2-3, 3-4"}, "outputs": {"has_circuit": False, "has_path": True, "odd_vertices": 2}}],
     questions=[{"q": "Does the graph with edges A-B, B-C, C-A, C-D, D-E, E-C have an Euler circuit?", "outputs": {"has_circuit": True}},
                {"q": "Does the graph with edges 1-2, 2-3, 3-4 have an Eulerian path?", "outputs": {"has_path": True}}])


def _colourable(nodes, adj, k):
    col = {}

    def place(i):
        if i == len(nodes):
            return True
        v = nodes[i]
        used = {col[u] for u in adj[v] if u in col}
        for c in range(1, k + 1):
            if c not in used:
                col[v] = c
                if place(i + 1):
                    return True
                del col[v]
        return False
    return dict(col) if place(0) else None


def chromatic_number(edges):
    adj = _graph(edges)
    if len(adj) > 12:
        raise ValueError("at most 12 vertices (exact colouring is exponential)")
    nodes = _sorted(adj)
    for k in range(1, len(nodes) + 1):
        col = _colourable(nodes, adj, k)
        if col:
            return {"chromatic_number": k, "colouring": {v: col[v] for v in nodes}}


def _v_chrom(i, o):
    adj = _graph(i["edges"])
    k, col = o["chromatic_number"], o["colouring"]
    if not (_iv(k) and set(col) == set(adj) and all(1 <= c <= k for c in col.values()) and all(col[u] != col[v] for u in adj for v in adj[u])):
        return False
    order = sorted(adj, key=lambda v: (-len(adj[v]), _nkey(v)))      # a different search order: no (k-1)-colouring
    return k == 1 or _colourable(order, adj, k - 1) is None


def _ref_chrom(edges):
    import z3
    G = _nxg(edges)
    nodes = list(G.nodes)
    for k in range(1, len(nodes) + 1):
        c = {v: z3.Int("c_" + v) for v in nodes}
        s = z3.Solver()
        s.add(*[z3.And(c[v] >= 0, c[v] < k) for v in nodes], *[c[u] != c[v] for u, v in G.edges])
        if s.check() == z3.sat:
            return {"chromatic_number": k}


_add("chromatic_number", name="Chromatic number of a small graph", topic="graph theory / colouring",
     purpose="The fewest colours needed so that adjacent vertices differ, with one optimal colouring.",
     inputs=[_E()], outputs=[_O("chromatic_number", "chi(G)"), _O("colouring", "an optimal colouring (colours 1..chi)", "dict")],
     primary="chromatic_number", conditions=["undirected simple graph, unweighted, at most 12 vertices",
                                             "the colouring is the first one found trying vertices in ascending name order, colours 1, 2, ..."],
     method="exact backtracking for k = 1, 2, ... colours", latex=r"\chi(G) = \min\{k : G \text{ is } k\text{-colourable}\}",
     run=chromatic_number, verify=_v_chrom, parse=lambda q: _gparse(q, r"chromatic number|minimum number of colou?rs|fewest colou?rs", None, False),
     reference=_ref_chrom, wrong=lambda edges: (lambda r: dict(r, chromatic_number=max(graph_degrees(edges)["max_degree"] + 1, 1)))(chromatic_number(edges)),
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(5, 10), rng.uniform(0.3, 0.6))},
     templates=["What is the chromatic number of the graph with edges A-B, B-C, C-A, C-D?",
                "What is the minimum number of colours needed for the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D"}, "outputs": {"chromatic_number": 3}},
               {"inputs": {"edges": "1-2, 2-3, 3-4, 4-5, 5-1"}, "outputs": {"chromatic_number": 3}},
               {"inputs": {"edges": "1-2, 2-3, 3-4, 4-1"}, "outputs": {"chromatic_number": 2, "colouring": {"1": 1, "2": 2, "3": 1, "4": 2}}}],
     questions=[{"q": "What is the chromatic number of the graph with edges A-B, B-C, C-A, C-D?", "outputs": {"chromatic_number": 3}},
                {"q": "What is the minimum number of colours needed for the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1?", "outputs": {"chromatic_number": 3}}])


def _laplacian_minor(adj):
    nodes = _sorted(adj)
    idx = {v: k for k, v in enumerate(nodes)}
    n = len(nodes)
    L = [[0] * n for _ in range(n)]
    for u in adj:
        for v in adj[u]:
            L[idx[u]][idx[v]] = -1
        L[idx[u]][idx[u]] = len(adj[u])
    return [row[1:] for row in L[1:]]


def spanning_tree_count(edges):
    adj = _graph(edges)
    if len(adj) > 60:
        raise ValueError("at most 60 vertices")
    M = [[Fraction(x) for x in row] for row in _laplacian_minor(adj)]
    n, det = len(M), Fraction(1)
    for c in range(n):
        p = next((r for r in range(c, n) if M[r][c] != 0), None)
        if p is None:
            return {"spanning_trees": 0}
        if p != c:
            M[c], M[p] = M[p], M[c]
            det = -det
        det *= M[c][c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f:
                M[r] = [a - f * b for a, b in zip(M[r], M[c])]
    return {"spanning_trees": int(det)}


def _bareiss(M):
    M = [row[:] for row in M]
    n, sign, prev = len(M), 1, 1
    if n == 0:
        return 1
    for k in range(n - 1):
        if M[k][k] == 0:
            sw = next((r for r in range(k + 1, n) if M[r][k] != 0), None)
            if sw is None:
                return 0
            M[k], M[sw] = M[sw], M[k]
            sign = -sign
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                M[i][j] = (M[i][j] * M[k][k] - M[i][k] * M[k][j]) // prev
        prev = M[k][k]
    return sign * M[n - 1][n - 1]


def _ref_stc(edges):
    import networkx as nx
    G = _nxg(edges)
    nodes, E = list(G.nodes), list(G.edges)
    count = 0
    for sub in itertools.combinations(E, len(nodes) - 1):
        T = nx.Graph()
        T.add_nodes_from(nodes)
        T.add_edges_from(sub)
        count += nx.is_tree(T)
    return {"spanning_trees": count}


_add("spanning_tree_count", name="Number of spanning trees (Kirchhoff)", topic="graph theory / trees",
     purpose="How many spanning trees an undirected simple graph has (0 if it is disconnected).",
     inputs=[_E()], outputs=[_O("spanning_trees", "number of spanning trees tau(G)")],
     primary="spanning_trees", conditions=["undirected simple graph, unweighted, at most 60 vertices"],
     method="Kirchhoff's matrix-tree theorem: any cofactor of the Laplacian, by exact rational elimination",
     latex=r"\tau(G) = \det L_{11},\quad L = D - A",
     run=spanning_tree_count, verify=lambda i, o: _iv(o["spanning_trees"]) and o["spanning_trees"] == _bareiss(_laplacian_minor(_graph(i["edges"]))),
     parse=lambda q: _gparse(q, r"(?:number of|how many) spanning trees|kirchhoff|matrix[- ]tree", None, False), reference=_ref_stc,
     wrong=lambda edges: (lambda n: {"spanning_trees": n ** (n - 2)})(graph_degrees(edges)["vertices"]),
     random_inputs=lambda rng: (lambda n: {"edges": _rgraph(rng, n, 0.45, connected=True)[:13]})(rng.randint(4, 7)),
     templates=["How many spanning trees does the graph with edges A-B, B-C, C-D, D-A, A-C have?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-D, D-A, A-C"}, "outputs": {"spanning_trees": 8}},
               {"inputs": {"edges": "1-2, 1-3, 1-4, 2-3, 2-4, 3-4"}, "outputs": {"spanning_trees": 16}},
               {"inputs": {"edges": "1-2, 3-4"}, "outputs": {"spanning_trees": 0}}],
     questions=[{"q": "How many spanning trees does the graph with edges A-B, B-C, C-D, D-A, A-C have?", "outputs": {"spanning_trees": 8}},
                {"q": "Find the number of spanning trees of the graph with edges 1-2, 1-3, 1-4, 2-3, 2-4, 3-4.", "outputs": {"spanning_trees": 16}}])


def _lowlink(adj):
    """Tarjan's lowlink on an undirected graph: (articulation points, bridges)."""
    import sys
    sys.setrecursionlimit(max(sys.getrecursionlimit(), 4 * len(adj) + 100))
    disc, low, aps, brs, t = {}, {}, set(), [], [0]

    def dfs(u, parent):
        disc[u] = low[u] = t[0]
        t[0] += 1
        children = 0
        for v in _sorted(adj[u]):
            if v not in disc:
                children += 1
                dfs(v, u)
                low[u] = min(low[u], low[v])
                if parent is not None and low[v] >= disc[u]:
                    aps.add(u)
                if low[v] > disc[u]:
                    brs.append(sorted((u, v), key=_nkey))
            elif v != parent:
                low[u] = min(low[u], disc[v])
        if parent is None and children > 1:
            aps.add(u)
    for s in _sorted(adj):
        if s not in disc:
            dfs(s, None)
    return _sorted(aps), sorted(brs, key=lambda e: (_nkey(e[0]), _nkey(e[1])))


def articulation_points(edges):
    aps, _b = _lowlink(_graph(edges))
    return {"count": len(aps), "points": aps}


def bridges(edges):
    _a, brs = _lowlink(_graph(edges))
    return {"count": len(brs), "bridges": brs}


def _ncomp(adj, drop_v=None, drop_e=None):
    sub = {u: [v for v in nb if v != drop_v and not (drop_e and {u, v} == drop_e)] for u, nb in adj.items() if u != drop_v}
    return len(_components(sub))


def _v_ap(i, o):
    adj = _graph(i["edges"])
    base = _ncomp(adj)
    want = _sorted(v for v in adj if _ncomp(adj, drop_v=v) > base)
    return _iv(o["count"]) and o["points"] == want and o["count"] == len(want)


def _v_br(i, o):
    adj = _graph(i["edges"])
    base = _ncomp(adj)
    want = sorted((sorted((str(e[0]), str(e[1])), key=_nkey) for e in i["edges"] if _ncomp(adj, drop_e={str(e[0]), str(e[1])}) > base),
                  key=lambda e: (_nkey(e[0]), _nkey(e[1])))
    return _iv(o["count"]) and o["bridges"] == want and o["count"] == len(want)


def _rand_blocks(rng):
    names = _names(rng, 20)
    e, used = set(), 0
    prev = None
    for _ in range(rng.randint(2, 4)):
        k = rng.randint(1, 4)
        block = names[used:used + k]
        used += k
        if k >= 3:
            for j in range(k):
                e.add((block[j], block[(j + 1) % k]))
        elif k == 2:
            e.add((block[0], block[1]))
        if prev:
            e.add((rng.choice(prev), rng.choice(block)))
        prev = block
    e = {x for x in e if x[0] != x[1]}
    return {"edges": sorted({tuple(sorted(x)) for x in e})}


_add("articulation_points", name="Articulation points (cut vertices)", topic="graph theory / connectivity",
     purpose="The vertices whose removal increases the number of connected components.",
     inputs=[_E()], outputs=[_O("count", "number of articulation points"), _O("points", "the articulation points, ascending", "list")],
     primary="count", conditions=["undirected simple graph, unweighted"], method="Tarjan's depth-first lowlink algorithm",
     latex=r"u \text{ cut vertex} \iff \exists \text{ child } v:\ \mathrm{low}(v) \ge \mathrm{disc}(u)",
     run=articulation_points, verify=_v_ap, parse=lambda q: _gparse(q, r"articulation points?|cut[- ]vertices|cut[- ]vertex", None, False),
     reference=lambda edges: (lambda p: {"count": len(p), "points": p})(_sorted(__import__("networkx").articulation_points(_nxg(edges)))),
     wrong=lambda edges: (lambda adj: (lambda p: {"count": len(p), "points": p})(_sorted(v for v in adj if len(adj[v]) >= 2)))(_graph(edges)),
     random_inputs=_rand_blocks,
     templates=["Find the articulation points of the graph with edges A-B, B-C, C-A, C-D, D-E."],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D, D-E"}, "outputs": {"count": 2, "points": ["C", "D"]}},
               {"inputs": {"edges": "1-2, 2-3, 3-1"}, "outputs": {"count": 0, "points": []}}],
     questions=[{"q": "Find the articulation points of the graph with edges A-B, B-C, C-A, C-D, D-E.", "outputs": {"points": ["C", "D"]}},
                {"q": "How many cut vertices does the graph with edges 1-2, 2-3, 3-4 have?", "outputs": {"count": 2}}])

_add("bridges", name="Bridges (cut edges)", topic="graph theory / connectivity",
     purpose="The edges whose removal increases the number of connected components.",
     inputs=[_E()], outputs=[_O("count", "number of bridges"), _O("bridges", "the bridges [u, v] with u < v, sorted", "list")],
     primary="count", conditions=["undirected simple graph, unweighted"], method="Tarjan's depth-first lowlink algorithm",
     latex=r"(u,v) \text{ bridge} \iff \mathrm{low}(v) > \mathrm{disc}(u)",
     run=bridges, verify=_v_br, parse=lambda q: _gparse(q, r"\bbridges?\b|cut[- ]edges?", None, False),
     reference=lambda edges: (lambda b: {"count": len(b), "bridges": b})(sorted((sorted(e, key=_nkey) for e in __import__("networkx").bridges(_nxg(edges))),
                                                                               key=lambda e: (_nkey(e[0]), _nkey(e[1])))),
     wrong=lambda edges: (lambda adj: (lambda b: {"count": len(b), "bridges": b})(sorted(
         (sorted((str(e[0]), str(e[1])), key=_nkey) for e in edges if len(adj[str(e[0])]) == 1 or len(adj[str(e[1])]) == 1),
         key=lambda e: (_nkey(e[0]), _nkey(e[1])))))(_graph(edges)),
     random_inputs=_rand_blocks,
     templates=["Find the bridges of the graph with edges A-B, B-C, C-A, C-D, D-E."],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D, D-E"}, "outputs": {"count": 2, "bridges": [["C", "D"], ["D", "E"]]}},
               {"inputs": {"edges": "1-2, 2-3, 3-1"}, "outputs": {"count": 0}}],
     questions=[{"q": "Find the bridges of the graph with edges A-B, B-C, C-A, C-D, D-E.", "outputs": {"bridges": [["C", "D"], ["D", "E"]]}},
                {"q": "How many cut edges does the graph with edges 1-2, 2-3, 3-1, 3-4 have?", "outputs": {"count": 1}}])


def strongly_connected(edges):
    adj = _graph(edges, directed=True)
    import sys
    sys.setrecursionlimit(max(sys.getrecursionlimit(), 4 * len(adj) + 100))
    idx, low, on, stack, comps, t = {}, {}, set(), [], [], [0]

    def visit(u):
        idx[u] = low[u] = t[0]
        t[0] += 1
        stack.append(u)
        on.add(u)
        for v in _sorted(adj[u]):
            if v not in idx:
                visit(v)
                low[u] = min(low[u], low[v])
            elif v in on:
                low[u] = min(low[u], idx[v])
        if low[u] == idx[u]:
            comp = []
            while True:
                w = stack.pop()
                on.discard(w)
                comp.append(w)
                if w == u:
                    break
            comps.append(_sorted(comp))
    for v in _sorted(adj):
        if v not in idx:
            visit(v)
    comps.sort(key=lambda c: _nkey(c[0]))
    return {"count": len(comps), "sizes": sorted((len(c) for c in comps), reverse=True), "components": comps}


def _v_scc(i, o):
    adj = _graph(i["edges"], directed=True)
    reach = {}
    for s in adj:
        seen, st = {s}, [s]
        while st:
            u = st.pop()
            for v in adj[u]:
                if v not in seen:
                    seen.add(v)
                    st.append(v)
        reach[s] = seen
    comps = sorted({tuple(_sorted(v for v in reach[s] if s in reach[v])) for s in adj}, key=lambda c: _nkey(c[0]))
    comps = [list(c) for c in comps]
    return _iv(o["count"]) and o["components"] == comps and o["count"] == len(comps) and o["sizes"] == sorted((len(c) for c in comps), reverse=True)


def _ref_scc(edges):
    import networkx as nx
    comps = sorted((_sorted(c) for c in nx.strongly_connected_components(_nxg(edges, True))), key=lambda c: _nkey(c[0]))
    return {"count": len(comps), "sizes": sorted((len(c) for c in comps), reverse=True), "components": comps}


_add("strongly_connected", name="Strongly connected components", topic="graph theory / connectivity",
     purpose="The strongly connected components of a directed graph (vertices mutually reachable).",
     inputs=[_E("edges", "the directed graph as edges u->v")],
     outputs=[_O("count", "number of strongly connected components"), _O("sizes", "component sizes, descending", "list"),
              _O("components", "members of each component (ascending), ordered by first member", "list")],
     primary="count", conditions=["directed simple graph, unweighted"], method="Tarjan's algorithm",
     latex=r"u \sim v \iff u \leadsto v \wedge v \leadsto u",
     run=strongly_connected, verify=_v_scc, parse=lambda q: _gparse(q, r"strongly connected", None, True), reference=_ref_scc,
     wrong=lambda edges: (lambda c: {"count": len(c), "sizes": sorted((len(x) for x in c), reverse=True), "components": c})(
         sorted(_components(_und(_graph(edges, directed=True))), key=lambda c: _nkey(c[0]))),
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(5, 14), 0.2, directed=True)},
     templates=["Find the strongly connected components of the directed graph with edges A->B, B->C, C->A, C->D."],
     examples=[{"inputs": {"edges": [["A", "B"], ["B", "C"], ["C", "A"], ["C", "D"]]}, "outputs": {"count": 2, "components": [["A", "B", "C"], ["D"]]}},
               {"inputs": {"edges": [["1", "2"], ["2", "3"]]}, "outputs": {"count": 3}}],
     questions=[{"q": "Find the strongly connected components of the directed graph with edges A->B, B->C, C->A, C->D.", "outputs": {"count": 2}},
                {"q": "How many strongly connected components does the directed graph with edges 1->2, 2->1, 2->3 have?", "outputs": {"count": 2}}])


def hamiltonian_cycle(edges):
    adj = _graph(edges)
    nodes = _sorted(adj)
    if len(nodes) > 12:
        raise ValueError("at most 12 vertices (the search is exponential)")
    if len(nodes) < 3:
        return {"exists": False, "cycle": None}
    s = nodes[0]
    path, used = [s], {s}

    def extend():
        if len(path) == len(nodes):
            return s in adj[path[-1]]
        for v in _sorted(adj[path[-1]]):
            if v not in used:
                path.append(v)
                used.add(v)
                if extend():
                    return True
                path.pop()
                used.discard(v)
        return False
    return {"exists": True, "cycle": path + [s]} if extend() else {"exists": False, "cycle": None}


def _v_ham(i, o):
    adj = _graph(i["edges"])
    nodes = _sorted(adj)
    n = len(nodes)
    if o["exists"] is True:
        c = o["cycle"]
        return (c[0] == c[-1] == nodes[0] and sorted(c[:-1], key=_nkey) == nodes and all(c[k + 1] in adj[c[k]] for k in range(n)))
    if o["exists"] is not False or o["cycle"] is not None:
        return False
    if n < 3:
        return True
    idx = {v: k for k, v in enumerate(nodes)}         # bitmask dynamic programme: no Hamiltonian cycle exists
    dp = [0] * (1 << n)
    dp[1] = 1
    for mask in range(1 << n):
        if not dp[mask] or not mask & 1:
            continue
        for u in range(n):
            if dp[mask] >> u & 1:
                for v in adj[nodes[u]]:
                    j = idx[v]
                    if not mask >> j & 1:
                        dp[mask | 1 << j] |= 1 << j
    full = (1 << n) - 1
    return not any(dp[full] >> u & 1 and nodes[0] in adj[nodes[u]] for u in range(1, n))


def _ref_ham(edges):
    G = _nxg(edges)
    nodes = list(G.nodes)
    if len(nodes) < 3:
        return {"exists": False}
    first = nodes[0]
    for perm in itertools.permutations(nodes[1:]):
        c = (first,) + perm + (first,)
        if all(G.has_edge(c[k], c[k + 1]) for k in range(len(nodes))):
            return {"exists": True}
    return {"exists": False}


def _rand_ham(rng):
    n = rng.randint(5, 8)
    names = _names(rng, n)
    if rng.random() < 0.6:
        order = names[:]
        rng.shuffle(order)
        e = {tuple(sorted((order[k], order[(k + 1) % n]))) for k in range(n)}
        for _ in range(rng.randint(0, 2)):
            a, b = rng.sample(names, 2)
            e.add(tuple(sorted((a, b))))
        return {"edges": sorted(e)}
    return {"edges": _rgraph(rng, n, 0.6, names=names)}


_add("hamiltonian_cycle", name="Hamiltonian cycle of a small graph", topic="graph theory / Euler and Hamilton",
     purpose="Whether an undirected graph has a cycle through every vertex exactly once, with one such cycle.",
     inputs=[_E()], outputs=[_O("exists", "true when a Hamiltonian cycle exists", "boolean"),
                             _O("cycle", "a Hamiltonian cycle starting and ending at the smallest vertex (none if it does not exist)", "list")],
     primary="exists", conditions=["undirected simple graph, unweighted, at most 12 vertices",
                                   "the cycle shown is the first found extending through neighbours in ascending order"],
     method="backtracking search from the smallest vertex", latex=r"v_1 v_2 \cdots v_n v_1,\ \{v_i\} = V",
     run=hamiltonian_cycle, verify=_v_ham, parse=lambda q: _gparse(q, r"hamiltonian (?:cycle|circuit)", None, False), reference=_ref_ham,
     wrong=lambda edges: (lambda adj: {"exists": all(2 * len(adj[v]) >= len(adj) for v in adj), "cycle": None})(_graph(edges)),
     random_inputs=_rand_ham,
     templates=["Does the graph with edges A-B, B-C, C-D, D-A, A-C have a Hamiltonian cycle?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-D, D-A, A-C"}, "outputs": {"exists": True, "cycle": ["A", "B", "C", "D", "A"]}},
               {"inputs": {"edges": "1-2, 2-3, 3-1, 3-4"}, "outputs": {"exists": False, "cycle": None}}],
     questions=[{"q": "Does the graph with edges A-B, B-C, C-D, D-A, A-C have a Hamiltonian cycle?", "outputs": {"exists": True}},
                {"q": "Find a Hamiltonian cycle in the graph with edges 1-2, 2-3, 3-1, 3-4.", "outputs": {"exists": False}}])


def _max_clique(adj):
    best = []

    def expand(R, P, X):
        nonlocal best
        if not P and not X:
            if len(R) > len(best) or (len(R) == len(best) and [_nkey(v) for v in sorted(R, key=_nkey)] < [_nkey(v) for v in best]):
                best = _sorted(R)
            return
        if len(R) + len(P) < len(best):
            return
        pivot = max(P | X, key=lambda u: len(P & adj[u]))
        for v in _sorted(P - adj[pivot]):
            expand(R | {v}, P & adj[v], X & adj[v])
            P = P - {v}
            X = X | {v}
    expand(set(), set(adj), set())
    return best


def clique_number(edges):
    adj = _graph(edges)
    if len(adj) > 60:
        raise ValueError("at most 60 vertices")
    sets = {u: set(nb) for u, nb in adj.items()}
    c = _max_clique(sets)
    return {"clique_number": len(c), "clique": c}


def _brute_max(nodes, ok):
    """Largest k with a k-subset satisfying ok (simple branch and bound in a fixed order)."""
    best = 0

    def go(i, chosen):
        nonlocal best
        best = max(best, len(chosen))
        if len(chosen) + len(nodes) - i <= best:
            return
        for j in range(i, len(nodes)):
            if ok(chosen, nodes[j]):
                go(j + 1, chosen + [nodes[j]])
    go(0, [])
    return best


def _v_clique(i, o):
    adj = _graph(i["edges"])
    c = o["clique"]
    if not (_iv(o["clique_number"]) and len(c) == o["clique_number"] == len(set(c)) and all(b in adj[a] for a in c for b in c if a != b)):
        return False
    return o["clique_number"] == _brute_max(sorted(adj, key=lambda v: -len(adj[v])), lambda ch, v: all(v in adj[u] for u in ch))


def _ref_clique(edges):
    import igraph
    G = _nxg(edges)
    nodes = list(G.nodes)
    idx = {v: k for k, v in enumerate(nodes)}
    g = igraph.Graph(n=len(nodes), edges=[(idx[u], idx[v]) for u, v in G.edges])
    return {"clique_number": int(g.clique_number())}


def independence_number(edges):
    adj = _graph(edges)
    if len(adj) > 60:
        raise ValueError("at most 60 vertices")
    comp = {u: set(adj) - set(adj[u]) - {u} for u in adj}
    s = _max_clique(comp)
    return {"independence_number": len(s), "independent_set": s}


def _v_indep(i, o):
    adj = _graph(i["edges"])
    s = o["independent_set"]
    if not (_iv(o["independence_number"]) and len(s) == o["independence_number"] == len(set(s)) and set(s) <= set(adj)
            and not any(b in adj[a] for a in s for b in s)):
        return False
    return o["independence_number"] == _brute_max(sorted(adj, key=lambda v: len(adj[v])), lambda ch, v: all(v not in adj[u] for u in ch))


def _ref_indep(edges):
    import igraph
    G = _nxg(edges)
    nodes = list(G.nodes)
    idx = {v: k for k, v in enumerate(nodes)}
    g = igraph.Graph(n=len(nodes), edges=[(idx[u], idx[v]) for u, v in G.edges])
    return {"independence_number": int(g.independence_number())}


def _wrong_indep(edges):
    adj = _graph(edges)
    chosen = []
    for v in _sorted(adj):                  # greedy in name order: maximal, not maximum
        if all(v not in adj[u] for u in chosen):
            chosen.append(v)
    return {"independence_number": len(chosen), "independent_set": chosen}


_add("clique_number", name="Clique number", topic="graph theory / cliques and independent sets",
     purpose="The size of a largest complete subgraph, with one maximum clique.",
     inputs=[_E()], outputs=[_O("clique_number", "omega(G)"), _O("clique", "a maximum clique (the lexicographically smallest), ascending", "list")],
     primary="clique_number", conditions=["undirected simple graph, unweighted, at most 60 vertices"],
     method="Bron-Kerbosch with pivoting", latex=r"\omega(G) = \max\{|K| : K \text{ complete}\}",
     run=clique_number, verify=_v_clique, parse=lambda q: _gparse(q, r"clique number|largest clique|maximum clique", None, False),
     reference=_ref_clique, wrong=lambda edges: (lambda r: dict(r, clique_number=graph_degrees(edges)["max_degree"] + 1))(clique_number(edges)),
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(6, 18), rng.uniform(0.25, 0.5))},
     templates=["What is the clique number of the graph with edges A-B, B-C, C-A, C-D, B-D, A-D, D-E?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D, B-D, A-D, D-E"}, "outputs": {"clique_number": 4, "clique": ["A", "B", "C", "D"]}},
               {"inputs": {"edges": "1-2, 2-3, 3-4"}, "outputs": {"clique_number": 2, "clique": ["1", "2"]}}],
     questions=[{"q": "What is the clique number of the graph with edges A-B, B-C, C-A, C-D, B-D, A-D, D-E?", "outputs": {"clique_number": 4}},
                {"q": "Find a maximum clique in the graph with edges 1-2, 2-3, 3-1, 3-4.", "outputs": {"clique": ["1", "2", "3"]}}])

_add("independence_number", name="Independence number", topic="graph theory / cliques and independent sets",
     purpose="The size of a largest set of pairwise non-adjacent vertices, with one maximum independent set.",
     inputs=[_E()], outputs=[_O("independence_number", "alpha(G)"),
                             _O("independent_set", "a maximum independent set (the lexicographically smallest), ascending", "list")],
     primary="independence_number", conditions=["undirected simple graph, unweighted, at most 60 vertices", "the vertices are those named in the edges"],
     method="maximum clique of the complement graph (Bron-Kerbosch)", latex=r"\alpha(G) = \omega(\overline G)",
     run=independence_number, verify=_v_indep, parse=lambda q: _gparse(q, r"independence number|largest independent set|maximum independent set", None, False),
     reference=_ref_indep, wrong=_wrong_indep,
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(7, 16), rng.uniform(0.2, 0.4))},
     templates=["What is the independence number of the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1?"],
     examples=[{"inputs": {"edges": "1-2, 2-3, 3-4, 4-5, 5-1"}, "outputs": {"independence_number": 2, "independent_set": ["1", "3"]}},
               {"inputs": {"edges": "A-B, A-C, A-D"}, "outputs": {"independence_number": 3, "independent_set": ["B", "C", "D"]}}],
     questions=[{"q": "What is the independence number of the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1?", "outputs": {"independence_number": 2}},
                {"q": "Find a maximum independent set of the graph with edges A-B, A-C, A-D.", "outputs": {"independent_set": ["B", "C", "D"]}}])


def _girth(adj, odd=True):
    best = None
    for s in adj:
        dist, parent, q = {s: 0}, {s: None}, [s]
        for u in q:
            for v in adj[u]:
                if v not in dist:
                    dist[v], parent[v] = dist[u] + 1, u
                    q.append(v)
                elif parent[u] != v and (odd or dist[v] != dist[u]):
                    L = dist[u] + dist[v] + 1
                    if best is None or L < best:
                        best = L
    return best


def girth(edges):
    return {"girth": _girth(_graph(edges))}


def _v_girth(i, o):
    adj = _graph(i["edges"])
    g = o["girth"]
    nodes = _sorted(adj)
    for L in range(3, len(nodes) + 1):           # shortest cycle by direct search of simple paths
        for s in nodes:
            stack = [(s, [s])]
            while stack:
                u, path = stack.pop()
                if len(path) == L:
                    if s in adj[u]:
                        return g == L and _iv(g)
                    continue
                for v in adj[u]:
                    if v not in path and _nkey(v) > _nkey(s):
                        stack.append((v, path + [v]))
    return g is None


def _ref_girth(edges):
    import networkx as nx
    g = nx.girth(_nxg(edges))
    return {"girth": None if g == float("inf") else int(g)}


def _rand_girth(rng):
    names = _names(rng, 14)
    e = set()
    for _ in range(rng.randint(1, 2)):
        k = rng.choice([3, 5, 5, 7, 4, 6])
        cyc = rng.sample(names, k)
        e |= {tuple(sorted((cyc[j], cyc[(j + 1) % k]))) for j in range(k)}
    for _ in range(rng.randint(0, 2)):
        a, b = rng.sample(names, 2)
        e.add(tuple(sorted((a, b))))
    return {"edges": sorted(e)}


_add("girth", name="Girth (shortest cycle length)", topic="graph theory / cycles",
     purpose="The length of a shortest cycle of an undirected graph (none for a forest).",
     inputs=[_E()], outputs=[_O("girth", "length of the shortest cycle (none when the graph is acyclic)")],
     primary="girth", conditions=["undirected simple graph, unweighted"],
     method="breadth-first search from every vertex; a non-tree edge (u, v) closes a cycle of length d(u) + d(v) + 1",
     latex=r"g(G) = \min\{|C| : C \text{ a cycle of } G\}",
     run=girth, verify=_v_girth, parse=lambda q: _gparse(q, r"girth|shortest cycle|length of the smallest cycle", None, False),
     reference=_ref_girth, wrong=lambda edges: {"girth": _girth(_graph(edges), odd=False)},
     random_inputs=_rand_girth,
     templates=["What is the girth of the graph with edges A-B, B-C, C-D, D-A, A-E, E-F, F-A?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-D, D-A, A-E, E-F, F-A"}, "outputs": {"girth": 3}},
               {"inputs": {"edges": "1-2, 2-3, 3-4, 4-5, 5-1"}, "outputs": {"girth": 5}}, {"inputs": {"edges": "1-2, 2-3"}, "outputs": {"girth": None}}],
     questions=[{"q": "What is the girth of the graph with edges A-B, B-C, C-D, D-A, A-E, E-F, F-A?", "outputs": {"girth": 3}},
                {"q": "What is the length of the shortest cycle in the graph with edges 1-2, 2-3, 3-4, 4-5, 5-1?", "outputs": {"girth": 5}}])


def _tree_labels(edges):
    adj = _graph(edges)
    if not all(re.fullmatch(r"\d+", v) for v in adj):
        raise ValueError("the vertices must be numbered 1..n (or 0..n-1)")
    labels = sorted(int(v) for v in adj)
    n = len(labels)
    if labels == list(range(1, n + 1)):
        off = 1
    elif labels == list(range(n)):
        off = 0
    else:
        raise ValueError("the vertices must be numbered 1..n (or 0..n-1)")
    if sum(len(nb) for nb in adj.values()) // 2 != n - 1 or len(_components(adj)) != 1:
        raise ValueError("the graph is not a tree")
    return {int(u): {int(v) for v in nb} for u, nb in adj.items()}, off


def prufer_encode(edges, largest=False):
    nb, off = _tree_labels(edges)
    nb = {u: set(v) for u, v in nb.items()}
    leaves = [(-u if largest else u) for u in nb if len(nb[u]) == 1]
    heapq.heapify(leaves)
    seq = []
    for _ in range(len(nb) - 2):
        leaf = abs(heapq.heappop(leaves))
        (p,) = nb[leaf]
        seq.append(p)
        nb[p].discard(leaf)
        if len(nb[p]) == 1:
            heapq.heappush(leaves, -p if largest else p)
    return {"sequence": seq, "vertices": len(nb)}


def _decode(seq, n, base):
    deg = {v: 1 for v in range(base, n + base)}
    for s in seq:
        deg[s] += 1
    leaves = [v for v in deg if deg[v] == 1]
    heapq.heapify(leaves)
    out = []
    for s in seq:
        leaf = heapq.heappop(leaves)
        out.append(sorted((leaf, s)))
        deg[s] -= 1
        if deg[s] == 1:
            heapq.heappush(leaves, s)
    out.append(sorted(heapq.nsmallest(2, leaves)))
    return sorted(out)


def _v_pencode(i, o):
    nb, off = _tree_labels(i["edges"])
    seq = o["sequence"]
    want = sorted(sorted((int(e[0]), int(e[1]))) for e in i["edges"])
    return o["vertices"] == len(nb) and len(seq) == len(nb) - 2 and all(_iv(s) for s in seq) and _decode(seq, len(nb), off) == want


def _ref_pencode(edges):
    import networkx as nx
    nb, off = _tree_labels(edges)
    T = nx.Graph()
    T.add_nodes_from(range(len(nb)))
    T.add_edges_from((int(e[0]) - off, int(e[1]) - off) for e in edges)
    return {"sequence": [s + off for s in nx.to_prufer_sequence(T)], "vertices": len(nb)}


def _rand_tree(rng, n=None, base=1):
    n = n or rng.randint(3, 14)
    return [(str(rng.randint(base, base + i - 1)), str(base + i)) for i in range(1, n)]


def parse_pencode(q0):
    q = q0.lower()
    if not re.search(r"pr[üu]fer", q) or re.search(r"decode|from (?:the|its|a) (?:pr[üu]fer )?(?:code|sequence)|whose pr[üu]fer", q):
        return None
    g = _qedges(q0)
    return {"edges": g[0]} if g else {}


_add("prufer_encode", name="Prüfer code of a labelled tree", topic="graph theory / trees",
     purpose="The Prüfer sequence (length n - 2) of a tree on vertices labelled 1..n.",
     inputs=[_E("edges", "the tree's edges, vertices numbered 1..n (or 0..n-1)")],
     outputs=[_O("sequence", "the Prüfer sequence", "list"), _O("vertices", "number of vertices n")],
     primary="vertices", conditions=["a tree whose vertices are numbered 1..n or 0..n-1", "the smallest leaf is removed at every step"],
     method="repeatedly remove the smallest leaf and record its neighbour", latex=r"(a_1, \dots, a_{n-2}),\ a_i = \text{neighbour of the smallest leaf}",
     run=prufer_encode, verify=_v_pencode, parse=parse_pencode, reference=_ref_pencode,
     wrong=lambda edges: prufer_encode(edges, largest=True), random_inputs=lambda rng: {"edges": _rand_tree(rng, base=rng.choice([0, 1]))},
     templates=["Find the Prüfer code of the tree with edges 1-4, 2-4, 3-4, 4-5, 5-6."],
     examples=[{"inputs": {"edges": "1-4, 2-4, 3-4, 4-5, 5-6"}, "outputs": {"sequence": [4, 4, 4, 5], "vertices": 6}},
               {"inputs": {"edges": "1-2, 2-3, 3-4"}, "outputs": {"sequence": [2, 3]}}],
     questions=[{"q": "Find the Prüfer code of the tree with edges 1-4, 2-4, 3-4, 4-5, 5-6.", "outputs": {"sequence": [4, 4, 4, 5]}},
                {"q": "Find the Prufer sequence of the tree with edges 1-2, 2-3, 3-1.", "outcome": "INVALID_INPUT"}])


def prufer_decode(sequence):
    seq = _wlist(sequence, "sequence", lo=1, hi=10 ** 4, n_min=1, n_max=2000)
    n = len(seq) + 2
    if max(seq) > n:
        raise ValueError(f"every entry must be between 1 and n = {n}")
    edges = _decode(seq, n, 1)
    return {"edges": edges, "vertices": n}


def _v_pdecode(i, o):
    seq = [int(s) for s in i["sequence"]]
    n = len(seq) + 2
    if o["vertices"] != n or len(o["edges"]) != n - 1 or o["edges"] != sorted(o["edges"]):
        return False
    return prufer_encode([(str(u), str(v)) for u, v in o["edges"]])["sequence"] == seq


def _ref_pdecode(sequence):
    import networkx as nx
    T = nx.from_prufer_sequence([int(s) - 1 for s in sequence])
    return {"edges": sorted(sorted((u + 1, v + 1)) for u, v in T.edges), "vertices": T.number_of_nodes()}


def _wrong_pdecode(sequence):
    seq = [int(s) for s in sequence]
    n = len(seq) + 2
    free = [v for v in range(1, n + 1) if v not in seq]       # pairs seq[i] with the i-th absent label (static)
    e = [sorted((free[k % len(free)], s)) for k, s in enumerate(seq)] + [sorted(free[-2:]) if len(free) > 1 else [1, n]]
    return {"edges": sorted(e), "vertices": n}


def parse_pdecode(q0):
    q = q0.lower()
    m = re.search(r"(?:decode the pr[üu]fer (?:code|sequence)|tree (?:with|whose|from|for) (?:the |its )?pr[üu]fer (?:code|sequence)(?: is)?)\s*:?\s*"
                  r"[\[(]?([\d,\s]+)[\])]?", q)
    if not m or not _ints(m.group(1)):
        return None
    return {"sequence": _ints(m.group(1))}


_add("prufer_decode", name="Tree from a Prüfer code", topic="graph theory / trees",
     purpose="The labelled tree on 1..n (n = length + 2) whose Prüfer sequence is given.",
     inputs=[_V("sequence", "the Prüfer sequence (entries between 1 and n)", 1, 2000)],
     outputs=[_O("edges", "the tree's edges [u, v] with u < v, sorted", "list"), _O("vertices", "number of vertices n")],
     primary="vertices", conditions=["vertices are numbered 1..n with n = length + 2"],
     method="attach each entry to the smallest current leaf; join the last two leaves",
     latex=r"\deg(v) = 1 + \#\{i : a_i = v\}",
     run=prufer_decode, verify=_v_pdecode, parse=parse_pdecode, reference=_ref_pdecode, wrong=_wrong_pdecode,
     random_inputs=lambda rng: (lambda n: {"sequence": [float(rng.randint(1, n)) for _ in range(n - 2)]})(rng.randint(4, 15)),
     templates=["Decode the Prüfer sequence [4, 4, 4, 5].", "Find the tree whose Prüfer code is 3, 3, 3."],
     examples=[{"inputs": {"sequence": "4, 4, 4, 5"}, "outputs": {"edges": [[1, 4], [2, 4], [3, 4], [4, 5], [5, 6]], "vertices": 6}},
               {"inputs": {"sequence": "3, 3, 3"}, "outputs": {"edges": [[1, 3], [2, 3], [3, 4], [3, 5]]}}],
     questions=[{"q": "Decode the Prüfer sequence [4, 4, 4, 5].", "outputs": {"edges": [[1, 4], [2, 4], [3, 4], [4, 5], [5, 6]]}},
                {"q": "Find the tree whose Prüfer code is 3, 3, 3.", "outputs": {"vertices": 5}},
                {"q": "Decode the Prüfer sequence [9, 1].", "outcome": "INVALID_INPUT"}])


def graphical_sequence(degrees):
    d = sorted(_wlist(degrees, "degrees", lo=0, hi=10 ** 4, n_min=1, n_max=2000), reverse=True)
    while d and d[0] > 0:
        k = d.pop(0)
        if k > len(d):
            return {"is_graphical": False}
        for j in range(k):
            d[j] -= 1
            if d[j] < 0:
                return {"is_graphical": False}
        d.sort(reverse=True)
    return {"is_graphical": True}


def _v_graphical(i, o):
    d = sorted((int(x) for x in i["degrees"]), reverse=True)
    n = len(d)
    ok = sum(d) % 2 == 0 and all(sum(d[:k]) <= k * (k - 1) + sum(min(x, k) for x in d[k:]) for k in range(1, n + 1))
    return o["is_graphical"] is ok


def _rand_graphical(rng):
    n = rng.randint(3, 12)
    if rng.random() < 0.4:
        e = _rgraph(rng, n, 0.4)
        d = graph_degrees(e)["degrees"]
        return {"degrees": [float(x) for x in d.values()]}
    d = [rng.randint(0, n + 1) for _ in range(n)]
    if sum(d) % 2:
        d[0] += 1
    return {"degrees": [float(x) for x in d]}


def parse_graphical(q0):
    q = q0.lower()
    if not re.search(r"graphical|havel|erd[őo]s|is there a (?:simple )?graph with degrees?|degree sequence of a simple graph", q):
        return None
    if re.search(r"\bedges?\b", q):
        return None
    m = re.search(r"(?:sequence|degrees?)\s*:?\s*[\[(]?\s*(\d[\d,\s]*)", q)
    return {"degrees": _ints(m.group(1))} if m else {}


_add("graphical_sequence", name="Graphical degree sequence", topic="graph theory / degree sequences",
     purpose="Whether a list of non-negative integers is the degree sequence of some simple graph.",
     inputs=[_V("degrees", "the proposed degrees (whole numbers >= 0)", 1, 2000)],
     outputs=[_O("is_graphical", "true when a simple graph has exactly these degrees", "boolean")],
     primary="is_graphical", conditions=["whole numbers >= 0"], method="Havel-Hakimi reduction",
     latex=r"\sum_{i \le k} d_i \le k(k-1) + \sum_{i > k} \min(d_i, k)\ \ \forall k,\quad \textstyle\sum d_i \text{ even}",
     run=graphical_sequence, verify=_v_graphical, parse=parse_graphical,
     reference=lambda degrees: {"is_graphical": bool(__import__("networkx").is_graphical([int(x) for x in degrees], method="eg"))},
     wrong=lambda degrees: {"is_graphical": sum(int(x) for x in degrees) % 2 == 0},
     random_inputs=_rand_graphical,
     templates=["Is the sequence 3, 3, 2, 2, 2 graphical?", "Is there a simple graph with degrees 4, 4, 1, 1?"],
     examples=[{"inputs": {"degrees": "3, 3, 2, 2, 2"}, "outputs": {"is_graphical": True}},
               {"inputs": {"degrees": "4, 4, 1, 1"}, "outputs": {"is_graphical": False}},
               {"inputs": {"degrees": "3, 3, 3, 1"}, "outputs": {"is_graphical": False}}],
     questions=[{"q": "Is the sequence 3, 3, 2, 2, 2 graphical?", "outputs": {"is_graphical": True}},
                {"q": "Is there a simple graph with degrees 4, 4, 1, 1?", "outputs": {"is_graphical": False}}])


def walks_count(edges, source, target, length, directed=False):
    adj = _graph(edges, directed=bool(directed))
    s, t, k = str(source).strip(), str(target).strip(), _whole(length, "length", 0, 200)
    for v in (s, t):
        if v not in adj:
            raise ValueError(f"{v} is not a vertex of the graph")
    vec = {s: 1}
    for _ in range(k):
        new = {}
        for u, c in vec.items():
            for v in adj[u]:
                new[v] = new.get(v, 0) + c
        vec = new
    return {"walks": vec.get(t, 0)}


def _v_walks(i, o):
    adj = _graph(i["edges"], directed=bool(i.get("directed")))
    s, t, k = str(i["source"]).strip(), str(i["target"]).strip(), int(i["length"])
    vec = {t: 1}                                   # backwards from the target
    for _ in range(k):
        vec = {u: sum(vec.get(v, 0) for v in adj[u]) for u in adj}
    return _iv(o["walks"]) and o["walks"] == vec.get(s, 0)


def _ref_walks(edges, source, target, length, directed=False):
    import sympy
    G = _nxg(edges, bool(directed))
    nodes = list(G.nodes)
    idx = {v: k for k, v in enumerate(nodes)}
    A = sympy.zeros(len(nodes), len(nodes))
    for u, v in G.edges:
        A[idx[u], idx[v]] = 1
        if not directed:
            A[idx[v], idx[u]] = 1
    return {"walks": int((A ** int(length))[idx[str(source).strip()], idx[str(target).strip()]])}


def _rand_walks(rng):
    x = _rand_search(rng)
    nodes = sorted({str(e[0]) for e in x["edges"]} | {str(e[1]) for e in x["edges"]})
    x.update(target=rng.choice(nodes), length=rng.randint(2, 9))
    return x


def parse_walks(q0):
    out = _gparse(q0, r"walks? of length")
    if not out or "_out_of_scope" in out:
        return out
    m = re.search(r"walks? of length (\d+) from (?:node |vertex )?([A-Za-z0-9_]+) to (?:node |vertex )?([A-Za-z0-9_]+)", q0, re.I)
    if not m:
        return {}
    out.update(length=int(m.group(1)), source=m.group(2), target=m.group(3))
    if _qedges(q0)[1]:
        out["directed"] = True
    return out


_add("walks_count", name="Number of walks of a given length", topic="graph theory / adjacency matrices",
     purpose="How many walks of exactly k edges lead from u to v (the (u, v) entry of A^k).",
     inputs=[_E(), _N("source", "start vertex u"), _N("target", "end vertex v"), _I("length", "the walk length k (0..200)", 0, 200),
             _B("directed", "true for a directed graph (edges u->v)")],
     outputs=[_O("walks", "number of walks of length k from u to v")],
     primary="walks", conditions=["simple unweighted graph", "vertices may repeat along a walk"],
     method="propagate walk counts one step at a time (a vector times the adjacency matrix, k times)",
     latex=r"\#\text{walks}_k(u,v) = (A^k)_{uv}",
     run=walks_count, verify=_v_walks, parse=parse_walks, reference=_ref_walks,
     wrong=lambda edges, source, target, length, directed=False: walks_count(edges, source, target, max(int(length) - 1, 0), directed),
     random_inputs=_rand_walks,
     templates=["How many walks of length 3 from A to B are there in the graph with edges A-B, B-C, C-A?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A", "source": "A", "target": "B", "length": 3}, "outputs": {"walks": 3}},
               {"inputs": {"edges": "1-2, 2-3", "source": "1", "target": "1", "length": 2}, "outputs": {"walks": 1}}],
     questions=[{"q": "How many walks of length 3 from A to B are there in the graph with edges A-B, B-C, C-A?", "outputs": {"walks": 3}},
                {"q": "How many walks of length 2 from 1 to 3 are there in the directed graph with edges 1->2, 2->3, 1->3?", "outputs": {"walks": 1}}])


def graph_triangles(edges):
    adj = _graph(edges)
    sets = {u: set(nb) for u, nb in adj.items()}
    t = sum(len(sets[u] & sets[v]) for u in adj for v in adj[u] if _nkey(u) < _nkey(v)) // 3
    return {"triangles": t}


_add("graph_triangles", name="Number of triangles", topic="graph theory / subgraph counts",
     purpose="How many triangles (3-cycles) an undirected graph contains.",
     inputs=[_E()], outputs=[_O("triangles", "number of triangles")],
     primary="triangles", conditions=["undirected simple graph, unweighted"],
     method="for each edge, count common neighbours; every triangle is seen from its three edges",
     latex=r"t(G) = \tfrac{1}{6}\operatorname{tr}(A^3)",
     run=graph_triangles,
     verify=lambda i, o: _iv(o["triangles"]) and (lambda adj: o["triangles"] == sum(1 for a, b, c in itertools.combinations(_sorted(adj), 3)
                                                                                    if b in adj[a] and c in adj[a] and c in adj[b]))(_graph(i["edges"])),
     parse=lambda q: _gparse(q, r"triangles", None, False),
     reference=lambda edges: {"triangles": sum(__import__("networkx").triangles(_nxg(edges)).values()) // 3},
     wrong=lambda edges: {"triangles": 3 * graph_triangles(edges)["triangles"]},
     random_inputs=lambda rng: {"edges": _rgraph(rng, rng.randint(5, 14), rng.uniform(0.35, 0.6))},
     templates=["How many triangles does the graph with edges A-B, B-C, C-A, C-D, D-A have?"],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D, D-A"}, "outputs": {"triangles": 2}},
               {"inputs": {"edges": "1-2, 1-3, 1-4, 2-3, 2-4, 3-4"}, "outputs": {"triangles": 4}}],
     questions=[{"q": "How many triangles does the graph with edges A-B, B-C, C-A, C-D, D-A have?", "outputs": {"triangles": 2}},
                {"q": "Count the triangles in the graph with edges 1-2, 1-3, 1-4, 2-3, 2-4, 3-4.", "outputs": {"triangles": 4}}])


def is_tree(edges):
    adj = _graph(edges)
    m = sum(len(nb) for nb in adj.values()) // 2
    conn = len(_components(adj)) == 1
    return {"is_tree": conn and m == len(adj) - 1, "vertices": len(adj), "edges": m, "connected": conn}


def _rand_istree(rng):
    if rng.random() < 0.4:
        return {"edges": _rand_tree(rng, base=1)}
    n = rng.randint(5, 12)                        # a cycle plus a separate path: n - 1 edges, not a tree
    names = _names(rng, n)
    k = rng.randint(3, n - 2)
    e = [(names[j], names[(j + 1) % k]) for j in range(k)] + [(names[j], names[j + 1]) for j in range(k, n - 1)]
    return {"edges": e}


_add("is_tree", name="Tree test", topic="graph theory / trees",
     purpose="Whether an undirected graph is a tree: connected with exactly n - 1 edges (equivalently, connected and acyclic).",
     inputs=[_E()], outputs=[_O("is_tree", "true when the graph is a tree", "boolean"), _O("vertices", "number of vertices"),
                             _O("edges", "number of edges"), _O("connected", "true when the graph is connected", "boolean")],
     primary="is_tree", conditions=["undirected simple graph, unweighted", "the vertices are those named in the edges"],
     method="count edges and test connectivity", latex=r"T \text{ tree} \iff T \text{ connected} \wedge |E| = |V| - 1",
     run=is_tree, verify=lambda i, o: (lambda c: o["connected"] is (c == 1) and o["vertices"] == len({str(x) for e in i["edges"] for x in e[:2]})
                                      and o["edges"] == len(i["edges"]) and o["is_tree"] is (c == 1 and o["edges"] == o["vertices"] - 1))(
         _v_cc_count(i["edges"])),
     parse=lambda q: _gparse(q, r"\bis (?:the|this) graph a tree\b|\bis it a tree\b|\ba tree\?", r"spanning", False),
     reference=lambda edges: (lambda G: {"is_tree": bool(__import__("networkx").is_tree(G)), "vertices": G.number_of_nodes(),
                                         "edges": G.number_of_edges(), "connected": bool(__import__("networkx").is_connected(G))})(_nxg(edges)),
     wrong=lambda edges: (lambda r: dict(r, is_tree=r["edges"] == r["vertices"] - 1))(is_tree(edges)),
     random_inputs=_rand_istree,
     templates=["Is the graph with edges 1-2, 1-3, 3-4, 3-5 a tree?"],
     examples=[{"inputs": {"edges": "1-2, 1-3, 3-4, 3-5"}, "outputs": {"is_tree": True, "edges": 4, "vertices": 5}},
               {"inputs": {"edges": "A-B, B-C, C-A, D-E"}, "outputs": {"is_tree": False, "connected": False}}],
     questions=[{"q": "Is the graph with edges 1-2, 1-3, 3-4, 3-5 a tree?", "outputs": {"is_tree": True}},
                {"q": "Is the graph with edges A-B, B-C, C-A, D-E a tree?", "outputs": {"is_tree": False}}])

# ==================================================================================================================
# propositional logic (formulas are text, read by a small recursive-descent parser: never evaluated as code)
# ==================================================================================================================
_LTOK = re.compile(r"\s*(<->|<=>|↔|⇔|->|=>|→|⇒|~|¬|!|&&|&|∧|\|\||\||∨|\^|⊕|\(|\)|[A-Za-z_][A-Za-z0-9_]*|[01])")
_LWORD = {"and": "AND", "or": "OR", "not": "NOT", "xor": "XOR", "implies": "IMP", "iff": "IFF", "true": True, "false": False}
_LSYM = {"<->": "IFF", "<=>": "IFF", "↔": "IFF", "⇔": "IFF", "->": "IMP", "=>": "IMP", "→": "IMP", "⇒": "IMP", "~": "NOT",
         "¬": "NOT", "!": "NOT", "&&": "AND", "&": "AND", "∧": "AND", "||": "OR", "|": "OR", "∨": "OR", "^": "XOR", "⊕": "XOR",
         "(": "(", ")": ")", "1": True, "0": False}
MAX_VARS = 12


def _lex(text):
    s = str(text)
    if len(s) > 400:
        raise ValueError("the formula is too long (at most 400 characters)")
    pos, out = 0, []
    while pos < len(s):
        if s[pos:].strip() == "":
            break
        m = _LTOK.match(s, pos)
        if not m:
            raise ValueError(f"cannot read the formula at '{s[pos:pos + 10].strip()}'")
        t = m.group(1)
        pos = m.end()
        if t in _LSYM:
            out.append(_LSYM[t])
        elif t.lower() in _LWORD:
            out.append(_LWORD[t.lower()])
        else:
            out.append(("VAR", t))
    if not out:
        raise ValueError("the formula is empty")
    return out


def _parse_formula(text):
    """AST of a propositional formula. Precedence, loosest first: <->, -> (right-associative), |, ^ (xor), &, ~."""
    toks, i = _lex(text), [0]

    def peek():
        return toks[i[0]] if i[0] < len(toks) else None

    def take():
        i[0] += 1
        return toks[i[0] - 1]

    def binary(op, sub, right=False):
        def f():
            left = sub()
            while peek() == op:
                take()
                rhs = f() if right else sub()
                left = (op, left, rhs)
                if right:
                    break
            return left
        return f

    def atom():
        t = take() if peek() is not None else None
        if t == "NOT":
            return ("NOT", atom())
        if t == "(":
            e = iff()
            if take() != ")" if peek() is not None else True:
                raise ValueError("unbalanced parentheses in the formula")
            return e
        if isinstance(t, tuple):
            return t
        if t is True or t is False:
            return ("CONST", t)
        raise ValueError("the formula is incomplete or malformed")
    conj = binary("AND", atom)
    xor = binary("XOR", conj)
    disj = binary("OR", xor)
    imp = binary("IMP", disj, right=True)
    iff = binary("IFF", imp)
    tree = iff()
    if i[0] != len(toks):
        raise ValueError("unexpected symbols at the end of the formula")
    names = set()

    def collect(t):
        if t[0] == "VAR":
            names.add(t[1])
        elif t[0] != "CONST":
            for c in t[1:]:
                collect(c)
    collect(tree)
    if len(names) > MAX_VARS:
        raise ValueError(f"at most {MAX_VARS} variables")
    return tree, sorted(names)


def _ev(t, env):
    op = t[0]
    if op == "VAR":
        return env[t[1]]
    if op == "CONST":
        return t[1]
    if op == "NOT":
        return not _ev(t[1], env)
    a, b = _ev(t[1], env), _ev(t[2], env)
    return {"AND": a and b, "OR": a or b, "XOR": a != b, "IMP": (not a) or b, "IFF": a == b}[op]


def _rows(names):
    n = len(names)
    for r in range(1 << n):
        yield r, {v: bool(r >> (n - 1 - k) & 1) for k, v in enumerate(names)}


def _table(text, names=None):
    tree, vs = _parse_formula(text)
    names = names or vs
    return [_ev(tree, env) for _r, env in _rows(names)], names


def _assign(r, names):
    return {v: bool(r >> (len(names) - 1 - k) & 1) for k, v in enumerate(names)}


def _ok_formula(text):
    try:
        _parse_formula(text)
        return True
    except ValueError:
        return False


# --- an independent reader for the references: shunting-yard into z3 expressions ---------------------------------
def _z3_formula(text):
    import z3
    prec = {"IFF": 1, "IMP": 2, "OR": 3, "XOR": 4, "AND": 5, "NOT": 6}
    toks = re.findall(r"<->|<=>|↔|⇔|->|=>|→|⇒|&&|\|\||[~¬!&∧|∨^⊕()]|[A-Za-z_][A-Za-z0-9_]*|[01]", str(text))
    sym = {"<->": "IFF", "<=>": "IFF", "↔": "IFF", "⇔": "IFF", "->": "IMP", "=>": "IMP", "→": "IMP", "⇒": "IMP", "~": "NOT",
           "¬": "NOT", "!": "NOT", "&&": "AND", "&": "AND", "∧": "AND", "||": "OR", "|": "OR", "∨": "OR", "^": "XOR", "⊕": "XOR"}
    words = {"and": "AND", "or": "OR", "not": "NOT", "xor": "XOR", "implies": "IMP", "iff": "IFF"}
    out, ops, names = [], [], set()

    def reduce_top():
        op = ops.pop()
        if op == "NOT":
            out.append(z3.Not(out.pop()))
            return
        b, a = out.pop(), out.pop()
        out.append({"AND": lambda: z3.And(a, b), "OR": lambda: z3.Or(a, b), "XOR": lambda: z3.Xor(a, b),
                    "IMP": lambda: z3.Implies(a, b), "IFF": lambda: a == b}[op]())
    for t in toks:
        op = sym.get(t) or words.get(t.lower())
        if t in ("0", "1") or t.lower() in ("true", "false"):
            out.append(z3.BoolVal(t in ("1",) or t.lower() == "true"))
        elif t == "(":
            ops.append("(")
        elif t == ")":
            while ops[-1] != "(":
                reduce_top()
            ops.pop()
        elif op == "NOT":
            ops.append("NOT")
        elif op:
            while ops and ops[-1] != "(" and (prec[ops[-1]] > prec[op] or (prec[ops[-1]] == prec[op] and op != "IMP")):
                reduce_top()
            ops.append(op)
        else:
            names.add(t)
            out.append(z3.Bool("v_" + t))
    while ops:
        reduce_top()
    return out[0], sorted(names)


def _z3_table(text, names=None):
    import z3
    f, vs = _z3_formula(text)
    names = names or vs
    vals = []
    for r in range(1 << len(names)):
        sub = [(z3.Bool("v_" + v), z3.BoolVal(bool(r >> (len(names) - 1 - k) & 1))) for k, v in enumerate(names)]
        vals.append(z3.is_true(z3.simplify(z3.substitute(f, *sub)) if sub else z3.simplify(f)))
    return vals, names


def _z3_count(f, names):
    import z3
    s, count = z3.Solver(), 0
    s.add(f)
    bs = [z3.Bool("v_" + v) for v in names]
    while s.check() == z3.sat:
        mdl = s.model()
        count += 1
        if not bs:
            break
        s.add(z3.Or([b != mdl.eval(b, model_completion=True) for b in bs]))
    return count


_LV = "pqrsABCD"


def _rform(rng, names, depth):
    if depth == 0 or rng.random() < 0.25:
        v = rng.choice(names)
        return ("~" + v) if rng.random() < 0.3 else v
    op = rng.choice(["&", "|", "->", "<->", "^", "&", "|"])
    a, b = _rform(rng, names, depth - 1), _rform(rng, names, depth - 1)
    f = f"({a} {op} {b})"
    return ("~" + f) if rng.random() < 0.15 else f


def _rvars(rng, k=None):
    k = k or rng.randint(2, 4)
    return sorted(rng.sample(["p", "q", "r", "s", "A", "B", "C", "D", "x1", "x2"], k)) if rng.random() < 0.5 else ["p", "q", "r", "s"][:k]


def logic_evaluate(formula, assignment):
    tree, names = _parse_formula(formula)
    env = _read_assignment(assignment)
    missing = [v for v in names if v not in env]
    extra = [v for v in env if v not in names]
    if missing:
        raise ValueError("no truth value given for " + ", ".join(missing))
    if extra:
        raise ValueError(", ".join(extra) + " does not occur in the formula")
    return {"value": _ev(tree, env)}


def _read_assignment(text):
    if isinstance(text, dict):
        pairs = list(text.items())
    else:
        pairs = re.findall(r"([A-Za-z_][A-Za-z0-9_]*)\s*(?:=|:|is)\s*(true|false|t|f|1|0)\b", str(text), re.I)
        rest = re.sub(r"([A-Za-z_][A-Za-z0-9_]*)\s*(?:=|:|is)\s*(true|false|t|f|1|0)\b", "", str(text), flags=re.I)
        if re.sub(r"[\s,;]|and", "", rest):
            raise ValueError("write the truth values as 'p = true, q = 0, ...'")
    env = {}
    for k, v in pairs:
        val = v if isinstance(v, bool) else str(v).lower() in ("true", "t", "1")
        if k in env and env[k] != val:
            raise ValueError(f"{k} is given two values")
        env[k] = val
    if not env:
        raise ValueError("no truth values given")
    return env


def _ref_eval(formula, assignment):
    import z3
    f, names = _z3_formula(formula)
    env = _read_assignment(assignment)
    sub = [(z3.Bool("v_" + v), z3.BoolVal(env[v])) for v in names]
    return {"value": z3.is_true(z3.simplify(z3.substitute(f, *sub) if sub else f))}


def _wrong_eval(formula, assignment):
    tree, _n = _parse_formula(formula)

    def conv(t):                                       # material implication misread as 'and'
        if t[0] in ("VAR", "CONST"):
            return t
        if t[0] == "NOT":
            return ("NOT", conv(t[1]))
        return ("AND" if t[0] == "IMP" else t[0], conv(t[1]), conv(t[2]))
    return {"value": _ev(conv(tree), _read_assignment(assignment))}


def _rand_eval(rng):
    names = _rvars(rng)
    a, b = _rform(rng, names, 2), _rform(rng, names, 2)
    f = f"({a}) -> ({b})"
    ta = _parse_formula(a)[0]
    for _ in range(50):
        env = {v: rng.random() < 0.5 for v in names}
        if not _ev(ta, {**{v: False for v in _parse_formula(a)[1]}, **env}) or rng.random() < 0.15:
            break
    used = _parse_formula(f)[1]
    return {"formula": f, "assignment": ", ".join(f"{v} = {'true' if env[v] else 'false'}" for v in used)}


def _ftext(m):
    return m.strip().strip(".?!").strip() if m else None


def parse_leval(q0):
    m = re.search(r"(?:evaluate|truth value of|value of)\s+(?:the formula\s+|the expression\s+)?(.+?)\s+(?:when|for|if|with|given|where)\s+(.+?)\s*[?.]?\s*$", q0, re.I)
    if not m or not _ok_formula(m.group(1)):
        return None
    return {"formula": m.group(1).strip(), "assignment": m.group(2)}


_add("logic_evaluate", name="Evaluate a propositional formula", topic="logic / propositional logic",
     purpose="The truth value of a propositional formula under a given assignment of true/false to its variables.",
     inputs=[_T("formula", "the formula, e.g. (p -> q) & ~r (operators ~ & | ^ -> <->, or not/and/or/xor/implies/iff)", n_max=400),
             _T("assignment", "a truth value for every variable, e.g. 'p = true, q = 0, r = false'", n_max=400)],
     outputs=[_O("value", "the formula's truth value", "boolean")],
     primary="value", conditions=["every variable gets a value and no other name is given",
                                  "precedence: ~ binds tightest, then &, ^, |, -> (right-associative), <-> loosest"],
     method="recursive-descent parse into a syntax tree, evaluated bottom-up",
     latex=r"p \to q \equiv \neg p \vee q",
     run=logic_evaluate, verify=lambda i, o: isinstance(o["value"], bool) and o["value"] == logic_evaluate(i["formula"], i["assignment"])["value"],
     parse=parse_leval, reference=_ref_eval, wrong=_wrong_eval, random_inputs=_rand_eval,
     templates=["Evaluate (p -> q) & ~r when p = true, q = false, r = false.", "What is the truth value of p | q -> r for p = 1, q = 0, r = 0?"],
     examples=[{"inputs": {"formula": "(p -> q) & ~r", "assignment": "p = true, q = false, r = false"}, "outputs": {"value": False}},
               {"inputs": {"formula": "p | q -> r", "assignment": "p = 1, q = 0, r = 0"}, "outputs": {"value": False}},
               {"inputs": {"formula": "A <-> ~B", "assignment": "A = true, B = false"}, "outputs": {"value": True}}],
     questions=[{"q": "Evaluate (p -> q) & ~r when p = true, q = false, r = false.", "outputs": {"value": False}},
                {"q": "What is the truth value of p | q -> r for p = 1, q = 0, r = 0?", "outputs": {"value": False}},
                {"q": "Evaluate p & q when p = true.", "outcome": "INVALID_INPUT"}])


def truth_table(formula):
    vals, names = _table(formula)
    return {"variables": names, "values": "".join("1" if v else "0" for v in vals), "true_rows": sum(vals),
            "minterms": [r for r, v in enumerate(vals) if v]}


def _v_tt(i, o):
    vals, names = _table(i["formula"])
    return (o["variables"] == names and o["values"] == "".join("1" if v else "0" for v in vals) and _iv(o["true_rows"])
            and o["true_rows"] == sum(vals) and o["minterms"] == [r for r, v in enumerate(vals) if v])


def _ref_tt(formula):
    vals, names = _z3_table(formula)
    return {"variables": names, "values": "".join("1" if v else "0" for v in vals), "true_rows": sum(vals),
            "minterms": [r for r, v in enumerate(vals) if v]}


def _wrong_tt(formula):
    tree, names = _parse_formula(formula)
    n = len(names)
    vals = [_ev(tree, {v: bool(r >> k & 1) for k, v in enumerate(names)}) for r in range(1 << n)]     # first variable as the LOW bit
    return {"variables": names, "values": "".join("1" if v else "0" for v in vals), "true_rows": sum(vals),
            "minterms": [r for r, v in enumerate(vals) if v]}


def _rand_ff(rng):
    names = _rvars(rng, rng.randint(2, 4))
    while True:
        f = _rform(rng, names, 3)
        if len(_parse_formula(f)[1]) >= 2:
            return f


def parse_tt(q0):
    m = re.search(r"(?:truth table|minterms) (?:of|for)\s+(?:the formula\s+|the expression\s+)?(.+?)\s*[?.]?\s*$", q0, re.I)
    if not m or not _ok_formula(m.group(1)):
        return None
    return {"formula": m.group(1).strip()}


_add("truth_table", name="Truth table and minterms", topic="logic / propositional logic",
     purpose="The full truth table of a propositional formula, the number of true rows and its minterms.",
     inputs=[_T("formula", "the formula (operators ~ & | ^ -> <->, or words)", n_max=400)],
     outputs=[_O("variables", "the variables in alphabetical order", "list"),
              _O("values", "the formula's value on rows 0, 1, ... as a 0/1 string", "text"), _O("true_rows", "number of true rows"),
              _O("minterms", "indices of the true rows", "list")],
     primary="true_rows", conditions=["at most 12 variables", "row r assigns the binary digits of r to the variables, the first "
                                      "(alphabetically) variable being the most significant bit"],
     method="evaluate the syntax tree on every assignment", latex=r"f = \sum m(\text{true rows})",
     run=truth_table, verify=_v_tt, parse=parse_tt, reference=_ref_tt, wrong=_wrong_tt,
     random_inputs=lambda rng: {"formula": _rand_ff(rng)},
     templates=["Give the truth table of (p -> q) & (q -> r).", "Find the minterms of A & ~B | C."],
     examples=[{"inputs": {"formula": "p -> q"}, "outputs": {"values": "1101", "true_rows": 3, "minterms": [0, 1, 3]}},
               {"inputs": {"formula": "A & ~B | C"}, "outputs": {"variables": ["A", "B", "C"], "values": "01011101", "minterms": [1, 3, 4, 5, 7]}}],
     questions=[{"q": "Give the truth table of (p -> q) & (q -> r).", "outputs": {"values": "11010001", "true_rows": 4}},
                {"q": "Find the minterms of A & ~B | C.", "outputs": {"minterms": [1, 3, 4, 5, 7]}},
                {"q": "Give the truth table of p & & q.", "outcome": "UNRECOGNIZED_FORMAT"}])


def logic_satisfiable(formula):
    vals, names = _table(formula)
    first = next((r for r, v in enumerate(vals) if v), None)
    return {"satisfiable": any(vals), "models": sum(vals), "first_model": None if first is None else _assign(first, names)}


def _v_sat(i, o):
    vals, names = _table(i["formula"])
    first = next((r for r, v in enumerate(vals) if v), None)
    return (o["satisfiable"] is any(vals) and o["models"] == sum(vals) and
            o["first_model"] == (None if first is None else _assign(first, names)))


def _ref_sat(formula):
    import z3
    f, names = _z3_formula(formula)
    s = z3.Solver()
    s.add(f)
    return {"satisfiable": s.check() == z3.sat, "models": _z3_count(f, names)}


def _rand_sat(rng):
    names = _rvars(rng, rng.randint(2, 4))
    if rng.random() < 0.4:
        a = _rform(rng, names, 2)
        return {"formula": f"({a}) & ~({a})" if rng.random() < 0.5 else f"({a}) & ({_rform(rng, names, 2)}) & ~({a})"}
    return {"formula": _rand_ff(rng)}


def parse_sat(q0):
    m = re.search(r"is\s+(?:the formula\s+)?(.+?)\s+satisfiable", q0, re.I) or \
        re.search(r"satisfiability of\s+(?:the formula\s+)?(.+?)\s*[?.]?\s*$", q0, re.I) or \
        re.search(r"(?:find )?a satisfying assignment (?:of|for)\s+(?:the formula\s+)?(.+?)\s*[?.]?\s*$", q0, re.I)
    if not m or re.search(r"\bunsatisfiable\b", q0, re.I) or not _ok_formula(m.group(1)):
        return None
    return {"formula": m.group(1).strip()}


_add("logic_satisfiable", name="Satisfiability of a propositional formula", topic="logic / satisfiability",
     purpose="Whether some assignment makes the formula true, how many do, and the first such assignment.",
     inputs=[_T("formula", "the formula (operators ~ & | ^ -> <->, or words)", n_max=400)],
     outputs=[_O("satisfiable", "true when some assignment satisfies the formula", "boolean"), _O("models", "number of satisfying assignments"),
              _O("first_model", "the satisfying assignment with the smallest row number (none if unsatisfiable)", "dict")],
     primary="satisfiable", conditions=["at most 12 variables", "rows are numbered with the first (alphabetical) variable as the most significant bit"],
     method="exhaustive truth table", latex=r"\exists v:\ f(v) = 1",
     run=logic_satisfiable, verify=_v_sat, parse=parse_sat, reference=_ref_sat,
     wrong=lambda formula: (lambda r, n: dict(r, models=2 ** n - r["models"]))(logic_satisfiable(formula), len(_parse_formula(formula)[1])),
     random_inputs=_rand_sat,
     templates=["Is (p | q) & ~p & ~q satisfiable?", "Find a satisfying assignment for (A | B) & (~A | C) & ~C."],
     examples=[{"inputs": {"formula": "(p | q) & ~p & ~q"}, "outputs": {"satisfiable": False, "models": 0, "first_model": None}},
               {"inputs": {"formula": "(A | B) & (~A | C) & ~C"}, "outputs": {"satisfiable": True, "models": 1,
                                                                              "first_model": {"A": False, "B": True, "C": False}}}],
     questions=[{"q": "Is (p | q) & ~p & ~q satisfiable?", "outputs": {"satisfiable": False}},
                {"q": "Find a satisfying assignment for (A | B) & (~A | C) & ~C.", "outputs": {"first_model": {"A": False, "B": True, "C": False}}}])


def logic_tautology(formula):
    vals, names = _table(formula)
    first = next((r for r, v in enumerate(vals) if not v), None)
    return {"tautology": all(vals), "false_rows": len(vals) - sum(vals), "counterexample": None if first is None else _assign(first, names)}


def _v_taut(i, o):
    vals, names = _table(i["formula"])
    first = next((r for r, v in enumerate(vals) if not v), None)
    return (o["tautology"] is all(vals) and o["false_rows"] == len(vals) - sum(vals)
            and o["counterexample"] == (None if first is None else _assign(first, names)))


def _ref_taut(formula):
    import z3
    f, names = _z3_formula(formula)
    s = z3.Solver()
    s.add(z3.Not(f))
    return {"tautology": s.check() == z3.unsat, "false_rows": _z3_count(z3.Not(f), names)}


def _rand_taut(rng):
    names = _rvars(rng, rng.randint(2, 4))
    a, b = _rform(rng, names, 2), _rform(rng, names, 2)
    pick = rng.random()
    if pick < 0.25:
        return {"formula": rng.choice([f"({a}) | ~({a})", f"(({a}) -> ({b})) | (({b}) -> ({a}))", f"(({a}) & ({b})) -> ({a})"])}
    return _rand_min(rng)


def parse_taut(q0):
    m = re.search(r"is\s+(?:the formula\s+)?(.+?)\s+(?:a tautology|valid|logically valid|always true)", q0, re.I)
    if not m or not _ok_formula(m.group(1)):
        return None
    return {"formula": m.group(1).strip()}


_add("logic_tautology", name="Tautology test", topic="logic / validity",
     purpose="Whether a formula is true under every assignment, and the first assignment that falsifies it if not.",
     inputs=[_T("formula", "the formula (operators ~ & | ^ -> <->, or words)", n_max=400)],
     outputs=[_O("tautology", "true when every assignment satisfies the formula", "boolean"), _O("false_rows", "number of falsifying assignments"),
              _O("counterexample", "the falsifying assignment with the smallest row number (none for a tautology)", "dict")],
     primary="tautology", conditions=["at most 12 variables"], method="exhaustive truth table", latex=r"\models f \iff \forall v:\ f(v) = 1",
     run=logic_tautology, verify=_v_taut, parse=parse_taut, reference=_ref_taut,
     wrong=lambda formula: (lambda r, s: dict(r, tautology=s["satisfiable"]))(logic_tautology(formula), logic_satisfiable(formula)),
     random_inputs=_rand_taut,
     templates=["Is (p -> q) | (q -> p) a tautology?", "Is the formula (p -> q) -> p valid?"],
     examples=[{"inputs": {"formula": "(p -> q) | (q -> p)"}, "outputs": {"tautology": True, "false_rows": 0}},
               {"inputs": {"formula": "(p -> q) -> p"}, "outputs": {"tautology": False, "counterexample": {"p": False, "q": False}}},
               {"inputs": {"formula": "((p -> q) -> p) -> p"}, "outputs": {"tautology": True}}],
     questions=[{"q": "Is (p -> q) | (q -> p) a tautology?", "outputs": {"tautology": True}},
                {"q": "Is the formula (p -> q) -> p valid?", "outputs": {"tautology": False}}])


def logic_contradiction(formula):
    s = logic_satisfiable(formula)
    return {"contradiction": not s["satisfiable"], "true_rows": s["models"], "witness": s["first_model"]}


def _rand_contra(rng):
    names = _rvars(rng, rng.randint(2, 4))
    a = _rform(rng, names, 2)
    if rng.random() < 0.25:
        return {"formula": rng.choice([f"({a}) & ~({a})", f"~(({a}) | ~({a}))", f"(({a}) <-> ~({a}))"])}
    return _rand_min(rng)


def parse_contra(q0):
    m = re.search(r"is\s+(?:the formula\s+)?(.+?)\s+(?:a contradiction|unsatisfiable|always false)", q0, re.I)
    if not m or not _ok_formula(m.group(1)):
        return None
    return {"formula": m.group(1).strip()}


_add("logic_contradiction", name="Contradiction test", topic="logic / satisfiability",
     purpose="Whether a formula is false under every assignment (unsatisfiable), with a satisfying assignment if not.",
     inputs=[_T("formula", "the formula (operators ~ & | ^ -> <->, or words)", n_max=400)],
     outputs=[_O("contradiction", "true when no assignment satisfies the formula", "boolean"), _O("true_rows", "number of satisfying assignments"),
              _O("witness", "the satisfying assignment with the smallest row number (none for a contradiction)", "dict")],
     primary="contradiction", conditions=["at most 12 variables"], method="exhaustive truth table", latex=r"\forall v:\ f(v) = 0",
     run=logic_contradiction,
     verify=lambda i, o: (lambda s: o["contradiction"] is (not s["satisfiable"]) and o["true_rows"] == s["models"] and o["witness"] == s["first_model"])(logic_satisfiable(i["formula"])),
     parse=parse_contra, reference=lambda formula: (lambda r: {"contradiction": not r["satisfiable"], "true_rows": r["models"]})(_ref_sat(formula)),
     wrong=lambda formula: (lambda r, t: dict(r, contradiction=not t["tautology"]))(logic_contradiction(formula), logic_tautology(formula)),
     random_inputs=_rand_contra,
     templates=["Is p & ~p a contradiction?", "Is (p -> q) & p & ~q unsatisfiable?"],
     examples=[{"inputs": {"formula": "p & ~p"}, "outputs": {"contradiction": True, "witness": None}},
               {"inputs": {"formula": "(p -> q) & p & ~q"}, "outputs": {"contradiction": True}},
               {"inputs": {"formula": "p & q"}, "outputs": {"contradiction": False, "witness": {"p": True, "q": True}}}],
     questions=[{"q": "Is p & ~p a contradiction?", "outputs": {"contradiction": True}},
                {"q": "Is (p -> q) & p & ~q unsatisfiable?", "outputs": {"contradiction": True}}])


def logic_equivalence(formula1, formula2):
    _t1, n1 = _parse_formula(formula1)
    _t2, n2 = _parse_formula(formula2)
    names = sorted(set(n1) | set(n2))
    if len(names) > MAX_VARS:
        raise ValueError(f"at most {MAX_VARS} variables in total")
    v1, _ = _table(formula1, names)
    v2, _ = _table(formula2, names)
    diff = [r for r in range(len(v1)) if v1[r] != v2[r]]
    return {"equivalent": not diff, "differing_rows": len(diff), "counterexample": _assign(diff[0], names) if diff else None}


def _ref_equiv(formula1, formula2):
    import z3
    f, n1 = _z3_formula(formula1)
    g, n2 = _z3_formula(formula2)
    names = sorted(set(n1) | set(n2))
    s = z3.Solver()
    s.add(f != g)
    return {"equivalent": s.check() == z3.unsat, "differing_rows": _z3_count(f != g, names)}


_EQV = [("~({0} & {1})", "~{0} | ~{1}"), ("{0} -> {1}", "~{0} | {1}"), ("{0} -> {1}", "~{1} -> ~{0}"),
        ("{0} <-> {1}", "({0} -> {1}) & ({1} -> {0})"), ("{0} & ({1} | {2})", "({0} & {1}) | ({0} & {2})"),
        ("{0} ^ {1}", "({0} | {1}) & ~({0} & {1})"), ("~({0} | {1})", "~{0} & ~{1}")]


def _rand_equiv(rng):
    names = _rvars(rng, 3)
    parts = ["(" + _rform(rng, names, 1) + ")" for _ in range(3)]
    a, b = rng.choice(_EQV)
    f, g = a.format(*parts), b.format(*parts)
    if rng.random() < 0.85:                         # same shape, variables permuted: usually NOT equivalent
        perm = names[1:] + names[:1]
        g = re.sub(r"[A-Za-z_][A-Za-z0-9_]*", lambda m: perm[names.index(m.group(0))] if m.group(0) in names else m.group(0), f)
    return {"formula1": f, "formula2": g}


def parse_equiv(q0):
    m = re.search(r"is\s+(?:the formula\s+)?(.+?)\s+(?:logically\s+)?equivalent to\s+(.+?)\s*[?.]?\s*$", q0, re.I)
    if m and _ok_formula(m.group(1)) and _ok_formula(m.group(2)):
        return {"formula1": m.group(1).strip(), "formula2": m.group(2).strip()}
    m = re.search(r"are\s+(.+?)\s+(?:logically\s+)?equivalent\s*[?.]?\s*$", q0, re.I)
    if not m:
        return None
    body = m.group(1)
    splits = [(body[:k.start()], body[k.end():]) for k in re.finditer(r"\s+and\s+", body)]
    good = [(a, b) for a, b in splits if _ok_formula(a) and _ok_formula(b)]
    if len(good) != 1:
        return None
    return {"formula1": good[0][0].strip(), "formula2": good[0][1].strip()}


_add("logic_equivalence", name="Logical equivalence of two formulas", topic="logic / propositional logic",
     purpose="Whether two formulas have the same truth value under every assignment, with a distinguishing assignment if not.",
     inputs=[_T("formula1", "the first formula", n_max=400), _T("formula2", "the second formula", n_max=400)],
     outputs=[_O("equivalent", "true when the formulas agree on every assignment", "boolean"),
              _O("differing_rows", "number of assignments where they differ"),
              _O("counterexample", "the first distinguishing assignment (none if equivalent)", "dict")],
     primary="equivalent", conditions=["at most 12 variables in total", "rows range over the union of both formulas' variables"],
     method="compare the two truth tables", latex=r"f \equiv g \iff \models f \leftrightarrow g",
     run=logic_equivalence,
     verify=lambda i, o: (lambda r: o == r and isinstance(o["equivalent"], bool))(logic_equivalence(i["formula1"], i["formula2"])),
     parse=parse_equiv, reference=_ref_equiv,
     wrong=lambda formula1, formula2: (lambda r: dict(r, equivalent=truth_table(formula1)["true_rows"] * 2 ** (len(set(_parse_formula(formula1)[1]) | set(_parse_formula(formula2)[1])) - len(_parse_formula(formula1)[1]))
                                                      == truth_table(formula2)["true_rows"] * 2 ** (len(set(_parse_formula(formula1)[1]) | set(_parse_formula(formula2)[1])) - len(_parse_formula(formula2)[1]))))(logic_equivalence(formula1, formula2)),
     random_inputs=_rand_equiv,
     templates=["Are (p -> q) and (~p | q) logically equivalent?", "Is p -> q equivalent to q -> p?"],
     examples=[{"inputs": {"formula1": "p -> q", "formula2": "~p | q"}, "outputs": {"equivalent": True, "differing_rows": 0}},
               {"inputs": {"formula1": "p -> q", "formula2": "q -> p"}, "outputs": {"equivalent": False, "differing_rows": 2,
                                                                                  "counterexample": {"p": False, "q": True}}}],
     questions=[{"q": "Are (p -> q) and (~p | q) logically equivalent?", "outputs": {"equivalent": True}},
                {"q": "Is p -> q equivalent to q -> p?", "outputs": {"equivalent": False}},
                {"q": "Is ~(p & q) equivalent to ~p | ~q?", "outputs": {"equivalent": True}}])


# --- two-level minimization (exact): Quine-McCluskey primes, then a branch-and-bound minimum cover ----------------
def _qm_primes(n, care):
    """Prime implicants of the function that is 1 on `care` (on-set plus don't-cares) as (value, dash_mask) pairs."""
    groups = {(m, 0) for m in care}
    primes = set()
    while groups:
        merged, nxt = set(), set()
        items = sorted(groups)
        by_mask = {}
        for v, d in items:
            by_mask.setdefault(d, []).append(v)
        for d, vals in by_mask.items():
            vs = set(vals)
            for v in vals:
                for b in range(n):
                    bit = 1 << b
                    if not d & bit and not v & bit and (v | bit) in vs:
                        nxt.add((v, d | bit))
                        merged.add((v, d))
                        merged.add((v | bit, d))
        primes |= groups - merged
        groups = nxt
    return primes


def _covers(p, m):
    return (m & ~p[1]) == p[0]


def _min_cover(n, on, primes):
    on = frozenset(on)
    if not on:
        return []
    cov = {p: frozenset(m for m in on if _covers(p, m)) for p in primes}
    by = {m: [p for p in primes if m in cov[p]] for m in on}
    lits = lambda p: n - bin(p[1]).count("1")
    best = [None, None]

    def rec(unc, chosen, cost):
        if best[0] is not None and cost >= best[0]:
            return
        if not unc:
            best[0], best[1] = cost, list(chosen)
            return
        if best[0] is not None and (cost[0] + 1, cost[1]) >= best[0]:
            return
        m = min(unc, key=lambda x: (len(by[x]), x))
        for p in sorted(by[m], key=lambda p: (-len(cov[p] & unc), lits(p), p)):
            chosen.append(p)
            rec(unc - cov[p], chosen, (cost[0] + 1, cost[1] + lits(p)))
            chosen.pop()
    rec(on, [], (0, 0))
    return sorted(best[1], key=lambda p: [0 if not p[1] >> b & 1 and p[0] >> b & 1 else (1 if not p[1] >> b & 1 else 2) for b in range(n - 1, -1, -1)])


def _term_text(p, names, negate=False):
    n = len(names)
    parts = []
    for k, v in enumerate(names):
        bit = 1 << (n - 1 - k)
        if not p[1] & bit:
            pos = bool(p[0] & bit) != negate
            parts.append(v if pos else "~" + v)
    return parts


def _sop(n, on, dc, names):
    cover = _min_cover(n, on, _qm_primes(n, set(on) | set(dc)))
    if not on:
        return {"expression": "0", "terms": 0, "literals": 0}
    if any(p[1] == (1 << n) - 1 for p in cover):
        return {"expression": "1", "terms": 1, "literals": 0}
    texts = [" & ".join(_term_text(p, names)) for p in cover]
    return {"expression": " | ".join(texts), "terms": len(cover), "literals": sum(len(_term_text(p, names)) for p in cover)}


def _pos(n, on, dc, names):
    off = [m for m in range(1 << n) if m not in set(on) and m not in set(dc)]
    cover = sorted(_min_cover(n, off, _qm_primes(n, set(off) | set(dc))),      # clauses in variable order, positive first
                   key=lambda p: [(1 if p[0] >> b & 1 else 0) if not p[1] >> b & 1 else 2 for b in range(n - 1, -1, -1)])
    if not off:
        return {"expression": "1", "clauses": 0, "literals": 0}
    if any(p[1] == (1 << n) - 1 for p in cover):
        return {"expression": "0", "clauses": 1, "literals": 0}
    texts = []
    for p in cover:
        ls = _term_text(p, names, negate=True)
        texts.append(ls[0] if len(ls) == 1 else "(" + " | ".join(ls) + ")")
    return {"expression": " & ".join(texts), "clauses": len(cover), "literals": sum(len(_term_text(p, names)) for p in cover)}


def _cube_of(lits_text, names):
    """(value, mask) of a product term such as 'A & ~C' (the variables not named are dashes)."""
    n, val, fixed = len(names), 0, 0
    for lit in [x.strip() for x in lits_text.split("&")]:
        neg = lit.startswith("~")
        v = lit.lstrip("~")
        bit = 1 << (n - 1 - names.index(v))
        if fixed & bit:
            return None
        fixed |= bit
        if not neg:
            val |= bit
    return val, ((1 << n) - 1) & ~fixed


def _check_cover(n, on, dc, cubes):
    """Every cube is a prime implicant, the cover is irredundant and exactly covers the on-set."""
    care = set(on) | set(dc)
    pts = lambda c: [m for m in range(1 << n) if _covers(c, m)]
    for c in cubes:
        if not all(m in care for m in pts(c)):
            return False
        for b in range(n):                     # prime: no literal can be dropped
            bit = 1 << b
            if not c[1] & bit and all(m in care for m in pts((c[0] & ~bit, c[1] | bit))):
                return False
    if set(on) - {m for c in cubes for m in pts(c)}:
        return False
    for k, c in enumerate(cubes):              # irredundant
        others = {m for j, d in enumerate(cubes) if j != k for m in pts(d)}
        if set(pts(c)) & set(on) <= others:
            return False
    return True


def _v_sop(n, on, dc, names, o):
    e = o["expression"]
    if e in ("0", "1"):
        want = (not on) if e == "0" else all(m in set(on) | set(dc) for m in range(1 << n))
        return want and o["terms"] == (0 if e == "0" else 1) and o["literals"] == 0
    cubes = [_cube_of(t, names) for t in e.split(" | ")]
    return (None not in cubes and _check_cover(n, on, dc, cubes) and _iv(o["terms"]) and o["terms"] == len(cubes)
            and o["literals"] == sum(n - bin(c[1]).count("1") for c in cubes))


def _v_pos(n, on, dc, names, o):
    e = o["expression"]
    off = [m for m in range(1 << n) if m not in set(on) and m not in set(dc)]
    if e in ("0", "1"):
        want = (not off) if e == "1" else all(m in set(off) | set(dc) for m in range(1 << n))
        return want and o["clauses"] == (0 if e == "1" else 1) and o["literals"] == 0
    cubes = []
    for cl in e.split(" & "):
        lits = [x.strip() for x in cl.strip("()").split("|")]
        cubes.append(_cube_of(" & ".join(x[1:] if x.startswith("~") else "~" + x for x in lits), names))
    return (None not in cubes and _check_cover(n, off, dc, cubes) and _iv(o["clauses"]) and o["clauses"] == len(cubes)
            and o["literals"] == sum(n - bin(c[1]).count("1") for c in cubes))


def _brute_min(n, on, dc):
    """Reference minimum (terms, literals) of a sum of products: every cube (3^n) tested, then covers by size."""
    care = set(on) | set(dc)
    cubes = []
    for code in itertools.product((0, 1, 2), repeat=n):
        val = sum((1 << (n - 1 - k)) for k, c in enumerate(code) if c == 1)
        mask = sum((1 << (n - 1 - k)) for k, c in enumerate(code) if c == 2)
        if all(m in care for m in range(1 << n) if (m & ~mask) == val):
            cubes.append((val, mask))
    cset = set(cubes)
    primes = [c for c in cubes if not any((c[0] & ~(1 << b), c[1] | (1 << b)) in cset for b in range(n) if not c[1] & (1 << b))]
    if not on:
        return 0, 0
    pts = {c: {m for m in on if (m & ~c[1]) == c[0]} for c in primes}
    for size in range(1, len(primes) + 1):
        best = None
        for combo in itertools.combinations(primes, size):
            if set().union(*(pts[c] for c in combo)) >= set(on):
                lits = sum(n - bin(c[1]).count("1") for c in combo)
                best = lits if best is None else min(best, lits)
        if best is not None:
            return size, best


def _formula_fn(formula):
    vals, names = _table(formula)
    if len(names) > 6:
        raise ValueError("at most 6 variables for minimization")
    return len(names), [r for r, v in enumerate(vals) if v], names


def minimal_dnf(formula):
    n, on, names = _formula_fn(formula)
    return _sop(n, on, [], names)


def minimal_cnf(formula):
    n, on, names = _formula_fn(formula)
    return _pos(n, on, [], names)


def _ref_mdnf(formula):
    vals, names = _z3_table(formula)
    t, l = _brute_min(len(names), [r for r, v in enumerate(vals) if v], [])
    return {"terms": t, "literals": l} if (t, l) != (1, 0) or not all(vals) else {"terms": 1, "literals": 0}


def _ref_mcnf(formula):
    vals, names = _z3_table(formula)
    t, l = _brute_min(len(names), [r for r, v in enumerate(vals) if not v], [])
    return {"clauses": t, "literals": l}


def _canon_dnf(n, on, names):
    if not on:
        return {"expression": "0", "terms": 0, "literals": 0}
    return {"expression": " | ".join(" & ".join(_term_text((m, 0), names)) for m in on), "terms": len(on), "literals": n * len(on)}


def _rand_min(rng):
    names = _rvars(rng, rng.choice([2, 3, 3, 4, 4]))
    while True:
        f = _rform(rng, names, 3)
        vals, vs = _table(f)
        if 2 <= len(vs) and 0 < sum(vals) < len(vals):
            return {"formula": f}


def _parse_min(q0, kinds):
    if re.search(r"[σΣ]\s*m\s*\(|\bm\s*\(\s*\d|\bminterms?\b", q0, re.I):
        return None
    m = re.search(r"(?:minimal|minimum|simplest|minimi[sz]ed?) (?:" + kinds + r") (?:form )?(?:of|for)\s+(?:the formula\s+)?(.+?)\s*[?.]?\s*$", q0, re.I)
    if not m or not _ok_formula(m.group(1)):
        return None
    return {"formula": m.group(1).strip()}


_add("minimal_dnf", name="Minimal sum of products (DNF)", topic="logic / Boolean minimization",
     purpose="A minimum disjunctive normal form of a formula: fewest product terms, then fewest literals.",
     inputs=[_T("formula", "the formula (at most 6 variables)", n_max=400)],
     outputs=[_O("expression", "a minimal sum of products (terms joined by |, literals by &)", "text"),
              _O("terms", "number of product terms"), _O("literals", "number of literals")],
     primary="terms", conditions=["at most 6 variables", "cost: fewest terms first, then fewest literals; when several minimal forms "
                                  "exist one is shown (all have these counts)"],
     method="Quine-McCluskey prime implicants, then an exact branch-and-bound minimum cover",
     latex=r"f = \sum_{\text{cover}} \prod \ell_i",
     run=minimal_dnf, verify=lambda i, o: (lambda n, on, names: _v_sop(n, on, [], names, o))(*_formula_fn(i["formula"])),
     parse=lambda q: _parse_min(q, r"dnf|sum of products|sop|disjunctive normal form"), reference=_ref_mdnf,
     wrong=lambda formula: (lambda n, on, names: _canon_dnf(n, on, names))(*_formula_fn(formula)),
     random_inputs=_rand_min,
     templates=["Find the minimal DNF of (A & B) | (A & ~B & C).", "Find the minimal sum of products of (p & q) | (~p & q) | (p & ~q)."],
     examples=[{"inputs": {"formula": "(A & B) | (A & ~B & C)"}, "outputs": {"expression": "A & B | A & C", "terms": 2, "literals": 4}},
               {"inputs": {"formula": "(p & q) | (~p & q) | (p & ~q)"}, "outputs": {"expression": "p | q", "terms": 2, "literals": 2}},
               {"inputs": {"formula": "p | ~p"}, "outputs": {"expression": "1", "terms": 1}}],
     questions=[{"q": "Find the minimal DNF of (A & B) | (A & ~B & C).", "outputs": {"terms": 2, "literals": 4}},
                {"q": "Find the minimal sum of products of (p & q) | (~p & q) | (p & ~q).", "outputs": {"expression": "p | q"}}])

_add("minimal_cnf", name="Minimal product of sums (CNF)", topic="logic / Boolean minimization",
     purpose="A minimum conjunctive normal form of a formula: fewest clauses, then fewest literals.",
     inputs=[_T("formula", "the formula (at most 6 variables)", n_max=400)],
     outputs=[_O("expression", "a minimal product of sums (clauses joined by &)", "text"),
              _O("clauses", "number of clauses"), _O("literals", "number of literals")],
     primary="clauses", conditions=["at most 6 variables", "cost: fewest clauses first, then fewest literals"],
     method="minimal sum of products of the negation, complemented (De Morgan)",
     latex=r"f = \prod_{\text{cover of } \bar f} \sum \bar\ell_i",
     run=minimal_cnf, verify=lambda i, o: (lambda n, on, names: _v_pos(n, on, [], names, o))(*_formula_fn(i["formula"])),
     parse=lambda q: _parse_min(q, r"cnf|product of sums|pos|conjunctive normal form"), reference=_ref_mcnf,
     wrong=lambda formula: (lambda n, on, names: (lambda off: {"expression": " & ".join("(" + " | ".join(_term_text((m, 0), names, negate=True)) + ")" for m in off) or "1",
                                                              "clauses": len(off), "literals": n * len(off)})(
         [m for m in range(1 << n) if m not in on]))(*_formula_fn(formula)),
     random_inputs=_rand_min,
     templates=["Find the minimal CNF of (A | B) & (A | ~B | C).", "Find the minimal product of sums of p ^ q."],
     examples=[{"inputs": {"formula": "(A | B) & (A | ~B | C)"}, "outputs": {"expression": "(A | B) & (A | C)", "clauses": 2, "literals": 4}},
               {"inputs": {"formula": "p ^ q"}, "outputs": {"clauses": 2, "literals": 4}}],
     questions=[{"q": "Find the minimal CNF of (A | B) & (A | ~B | C).", "outputs": {"clauses": 2, "literals": 4}},
                {"q": "Find the minimal product of sums of p ^ q.", "outputs": {"expression": "(p | q) & (~p | ~q)"}}])


def minimal_sop_minterms(minterms, variables, dont_cares=None):
    n = _whole(variables, "variables", 1, 6)
    on = sorted(set(_wlist(minterms, "minterms", lo=0, hi=(1 << n) - 1, n_min=0, n_max=64))) if minterms not in (None, "", []) else []
    dc = sorted(set(_wlist(dont_cares, "dont_cares", lo=0, hi=(1 << n) - 1, n_min=0, n_max=64))) if dont_cares not in (None, "", []) else []
    for m in on + dc:
        if m >= 1 << n:
            raise ValueError(f"minterm {m} needs more than {n} variables")
    if set(on) & set(dc):
        raise ValueError("a row cannot be both a minterm and a don't-care")
    return _sop(n, on, dc, list("ABCDEF"[:n]))


def _ref_msop(minterms, variables, dont_cares=None):
    on = sorted({int(m) for m in minterms})
    dc = sorted({int(m) for m in (dont_cares or [])})
    t, l = _brute_min(int(variables), on, dc)
    return {"terms": t, "literals": l}


def _rand_msop(rng):
    n = rng.choice([2, 3, 3, 4, 4])
    rows = list(range(1 << n))
    rng.shuffle(rows)
    k = rng.randint(2, (1 << n) - 1)
    on, rest = rows[:k], rows[k:]
    dc = rest[:rng.randint(0, min(2, len(rest)))]
    out = {"minterms": [float(m) for m in sorted(on)], "variables": n}
    if dc:
        out["dont_cares"] = [float(m) for m in sorted(dc)]
    return out


def parse_msop(q0):
    q = q0.lower().replace("Σ", "σ")
    m = re.search(r"(?:σ\s*m|sum of minterms|\bm)\s*\(([\d,\s]+)\)", q) or re.search(r"minterms\s*:?\s*([\d,\s]+?)(?:\s+(?:of|with|and|for)|[.?]|$)", q)
    if not m or not re.search(r"minimi|minimal|simplif|minimum", q):
        return None
    out = {"minterms": _ints(m.group(1))}
    # "d(5, 7)", "don't cares 5, 7", "don't cares d(5, 7)": the list must hold a number (a bare space once read as
    # an empty list, so the don't-cares of "with don't cares d(5, 7)" were dropped)
    d = re.search(r"(?:don'?t[- ]cares?(?:\s+conditions?)?(?:\s+(?:at|in|on))?(?:\s+rows?)?\s*(?:\bd\b)?|\bd)"
                  r"\s*\(?\s*:?\s*(\d[\d,\s]*(?:,?\s*and\s+\d+)?)\)?", q[m.end():])
    if d and _ints(d.group(1)):
        out["dont_cares"] = _ints(d.group(1))
    f = re.search(r"f\s*\(([a-z,\s]+)\)", q)
    v = re.search(r"(\d+)\s+variables", q)
    if f:
        out["variables"] = len([x for x in f.group(1).split(",") if x.strip()])
    elif v:
        out["variables"] = int(v.group(1))
    return out


_add("minimal_sop_minterms", name="Minimal sum of products from minterms (with don't-cares)", topic="logic / Boolean minimization",
     purpose="The minimal sum of products of a Boolean function given by its minterms and optional don't-care rows (Karnaugh-map problems).",
     inputs=[_V("minterms", "rows where the function is 1", 0, 64), _I("variables", "number of variables (1..6), named A, B, C, ...", 1, 6),
             _V("dont_cares", "rows whose value does not matter (optional)", 0, 64, required=False)],
     outputs=[_O("expression", "a minimal sum of products over A, B, C, ... (A is the most significant bit)", "text"),
              _O("terms", "number of product terms"), _O("literals", "number of literals")],
     primary="terms", conditions=["1..6 variables named A, B, C, D, E, F with A the most significant bit of the row number",
                                  "cost: fewest terms first, then fewest literals; don't-cares may be used or not"],
     method="Quine-McCluskey prime implicants (don't-cares included), exact minimum cover of the minterms",
     latex=r"F(A,B,C) = \sum m(\dots) + d(\dots)",
     run=minimal_sop_minterms,
     verify=lambda i, o: _v_sop(int(i["variables"]), sorted({int(m) for m in i["minterms"]}), sorted({int(m) for m in (i.get("dont_cares") or [])}),
                                list("ABCDEF"[:int(i["variables"])]), o),
     parse=parse_msop, reference=_ref_msop,
     wrong=lambda minterms, variables, dont_cares=None: _canon_dnf(int(variables), sorted({int(m) for m in minterms}), list("ABCDEF"[:int(variables)])),
     random_inputs=_rand_msop,
     templates=["Find the minimal sum of products for F(A, B, C) = Σm(1, 3, 5, 7).",
                "Minimize F(A, B, C, D) = Σm(0, 2, 8, 10, 13, 15) with don't cares d(5, 7)."],
     examples=[{"inputs": {"minterms": "1, 3, 5, 7", "variables": 3}, "outputs": {"expression": "C", "terms": 1, "literals": 1}},
               {"inputs": {"minterms": "0, 2, 8, 10, 13, 15", "variables": 4, "dont_cares": "5, 7"}, "outputs": {"terms": 2, "literals": 4}},
               {"inputs": {"minterms": "0, 1, 2, 5, 6, 7", "variables": 3}, "outputs": {"terms": 3, "literals": 6}}],
     questions=[{"q": "Find the minimal sum of products for F(A, B, C) = Σm(1, 3, 5, 7).", "outputs": {"expression": "C"}},
                {"q": "Minimize F(A, B, C, D) = Σm(0, 2, 8, 10, 13, 15) with don't cares d(5, 7).", "outputs": {"terms": 2}},
                {"q": "Minimize F(A, B) = Σm(0, 9).", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# mathematics of cryptography (textbook-size numbers: teaching, never security)
# ==================================================================================================================
def _named(q0, names):
    out = {}
    for n in names:
        m = re.search(r"(?<![A-Za-z_])" + n + r"\s*=\s*(-?\d+)", q0)
        if m:
            out[n] = int(m.group(1))
    return out


def _need_prime(v, name):
    if not _is_prime(v):
        raise ValueError(f"{name} must be prime")


def rsa_keygen(p, q, e):
    p, q, e = _whole(p, "p", 2), _whole(q, "q", 2), _whole(e, "e", 2)
    _need_prime(p, "p")
    _need_prime(q, "q")
    if p == q:
        raise ValueError("p and q must be different primes")
    phi = (p - 1) * (q - 1)
    if not 1 < e < phi or math.gcd(e, phi) != 1:
        raise ValueError("e must satisfy 1 < e < phi(n) and gcd(e, phi(n)) = 1")
    return {"n": p * q, "phi": phi, "d": _inv(e, phi)}


def _rand_rsa(rng):
    p = _rand_prime(rng, 11, 10 ** rng.randint(2, 6))
    q = p
    while q == p:
        q = _rand_prime(rng, 11, 10 ** rng.randint(2, 6))
    phi = (p - 1) * (q - 1)
    e = rng.choice([3, 5, 7, 17, 257, 65537])
    while math.gcd(e, phi) != 1 or e >= phi:
        e = rng.randrange(3, phi, 2)
    return {"p": p, "q": q, "e": e}


def _ref_rsakey(p, q, e):
    import sympy
    n = p * q
    phi = int(sympy.totient(n))
    return {"n": n, "phi": phi, "d": int(sympy.mod_inverse(e, phi))}


def parse_rsakey(q0):
    if not re.search(r"\brsa\b", q0, re.I) or re.search(r"encrypt|decrypt|cipher ?text|sign", q0, re.I):
        return None
    if not re.search(r"private|key|\bd\b", q0, re.I):
        return None
    return _named(q0, ["p", "q", "e"])


_add("rsa_keygen", name="RSA key from p, q and e", topic="cryptography / RSA",
     purpose="The RSA modulus n = p q, phi(n) = (p-1)(q-1) and the private exponent d = e^(-1) mod phi(n).",
     inputs=[_I("p", "first prime", 2), _I("q", "second prime", 2), _I("e", "public exponent", 2)],
     outputs=[_O("n", "modulus p q"), _O("phi", "Euler's phi(n) = (p-1)(q-1)"), _O("d", "private exponent e^(-1) mod phi(n)")],
     primary="d", conditions=["p, q distinct primes", "1 < e < phi(n) and gcd(e, phi(n)) = 1", "d is taken modulo phi(n) (not lambda(n))"],
     method="extended Euclidean algorithm for the inverse of e modulo phi(n)", latex=r"n = pq,\ \varphi = (p-1)(q-1),\ d \equiv e^{-1} \pmod{\varphi}",
     run=rsa_keygen, verify=lambda i, o: (lambda p, q, e: _iv(o["d"]) and o["n"] == p * q and o["phi"] == (p - 1) * (q - 1) and 0 < o["d"] < o["phi"]
                                           and e * o["d"] % o["phi"] == 1)(int(i["p"]), int(i["q"]), int(i["e"])),
     parse=parse_rsakey, reference=_ref_rsakey,
     wrong=lambda p, q, e: (lambda n: {"n": n, "phi": (int(p) - 1) * (int(q) - 1), "d": pow(int(e), -1, n) if math.gcd(int(e), n) == 1 else 0})(int(p) * int(q)),
     random_inputs=_rand_rsa,
     templates=["Find the RSA private key d for p = 61, q = 53, e = 17."],
     examples=[{"inputs": {"p": 61, "q": 53, "e": 17}, "outputs": {"n": 3233, "phi": 3120, "d": 2753}},
               {"inputs": {"p": 11, "q": 13, "e": 7}, "outputs": {"n": 143, "phi": 120, "d": 103}}],
     questions=[{"q": "Find the RSA private key d for p = 61, q = 53, e = 17.", "outputs": {"d": 2753, "n": 3233}},
                {"q": "Find the RSA private key d for p = 61, q = 55, e = 17.", "outcome": "INVALID_INPUT"},
                {"q": "Find the RSA private key d for p = 61, q = 53.", "outcome": "NEEDS_INPUT"}])


def rsa_encrypt(m, e, n):
    m, e, n = _whole(m, "m", 0), _whole(e, "e", 1), _whole(n, "n", 2)
    if m >= n:
        raise ValueError("the message m must satisfy 0 <= m < n")
    return {"c": pow(m, e, n)}


def rsa_decrypt(c, d, n):
    c, d, n = _whole(c, "c", 0), _whole(d, "d", 1), _whole(n, "n", 2)
    if c >= n:
        raise ValueError("the ciphertext c must satisfy 0 <= c < n")
    return {"m": pow(c, d, n)}


def _rand_rsa_msg(rng, key):
    x = _rand_rsa(rng)
    n = x["p"] * x["q"]
    if key == "enc":
        return {"m": rng.randint(2, n - 1), "e": x["e"], "n": n}
    return {"c": rng.randint(2, n - 1), "d": rsa_keygen(**x)["d"], "n": n}


def parse_rsaenc(q0):
    if not re.search(r"\brsa\b", q0, re.I) or not re.search(r"encrypt", q0, re.I) or re.search(r"decrypt", q0, re.I):
        return None
    return _named(q0, ["m", "e", "n"])


def parse_rsadec(q0):
    if not re.search(r"\brsa\b", q0, re.I) or not re.search(r"decrypt", q0, re.I):
        return None
    got = _named(q0, ["c", "d", "n", "p", "q"])
    if "p" in got or "q" in got:
        return None
    return got


_add("rsa_encrypt", name="RSA encryption", topic="cryptography / RSA",
     purpose="The RSA ciphertext c = m^e mod n of a message number m.",
     inputs=[_I("m", "the message as a number (0 <= m < n)", 0), _I("e", "public exponent", 1), _I("n", "modulus", 2)],
     outputs=[_O("c", "ciphertext m^e mod n")], primary="c", conditions=["0 <= m < n"],
     method="modular exponentiation", latex=r"c = m^e \bmod n",
     run=rsa_encrypt, verify=lambda i, o: _iv(o["c"]) and o["c"] == pow(int(i["m"]), int(i["e"]), int(i["n"])), parse=parse_rsaenc,
     reference=lambda m, e, n: {"c": int(__import__("gmpy2").powmod(m, e, n))}, wrong=lambda m, e, n: {"c": int(m) * int(e) % int(n)},
     random_inputs=lambda rng: _rand_rsa_msg(rng, "enc"),
     templates=["RSA: encrypt m = 65 with e = 17 and n = 3233."],
     examples=[{"inputs": {"m": 65, "e": 17, "n": 3233}, "outputs": {"c": 2790}}, {"inputs": {"m": 9, "e": 7, "n": 143}, "outputs": {"c": 48}}],
     questions=[{"q": "RSA: encrypt m = 65 with e = 17 and n = 3233.", "outputs": {"c": 2790}},
                {"q": "RSA: encrypt m = 4000 with e = 17 and n = 3233.", "outcome": "INVALID_INPUT"}])

_add("rsa_decrypt", name="RSA decryption", topic="cryptography / RSA",
     purpose="The RSA plaintext m = c^d mod n of a ciphertext c.",
     inputs=[_I("c", "the ciphertext (0 <= c < n)", 0), _I("d", "private exponent", 1), _I("n", "modulus", 2)],
     outputs=[_O("m", "plaintext c^d mod n")], primary="m", conditions=["0 <= c < n"],
     method="modular exponentiation", latex=r"m = c^d \bmod n",
     run=rsa_decrypt, verify=lambda i, o: _iv(o["m"]) and o["m"] == pow(int(i["c"]), int(i["d"]), int(i["n"])), parse=parse_rsadec,
     reference=lambda c, d, n: {"m": int(__import__("gmpy2").powmod(c, d, n))}, wrong=lambda c, d, n: {"m": int(c) * int(d) % int(n)},
     random_inputs=lambda rng: _rand_rsa_msg(rng, "dec"),
     templates=["RSA: decrypt c = 2790 with d = 2753 and n = 3233."],
     examples=[{"inputs": {"c": 2790, "d": 2753, "n": 3233}, "outputs": {"m": 65}}, {"inputs": {"c": 48, "d": 103, "n": 143}, "outputs": {"m": 9}}],
     questions=[{"q": "RSA: decrypt c = 2790 with d = 2753 and n = 3233.", "outputs": {"m": 65}}])


def rsa_decrypt_pq(c, p, q, e):
    k = rsa_keygen(p, q, e)
    p, q, c = int(p), int(q), _whole(c, "c", 0)
    if c >= k["n"]:
        raise ValueError("the ciphertext c must satisfy 0 <= c < n")
    d = k["d"]
    mp, mq = pow(c, d % (p - 1), p), pow(c, d % (q - 1), q)
    h = _inv(q, p) * (mp - mq) % p                 # Garner's recombination
    return {"m": mq + h * q, "d": d, "n": k["n"]}


def _ref_rsapq(c, p, q, e):
    import gmpy2
    import sympy
    n = p * q
    d = int(sympy.mod_inverse(e, (p - 1) * (q - 1)))
    return {"m": int(gmpy2.powmod(c, d, n)), "d": d, "n": n}


def parse_rsapq(q0):
    if not re.search(r"\brsa\b", q0, re.I) or not re.search(r"decrypt", q0, re.I):
        return None
    got = _named(q0, ["c", "p", "q", "e"])
    return got if ("p" in got or "q" in got) else None


_add("rsa_decrypt_pq", name="RSA decryption from p, q and e (CRT)", topic="cryptography / RSA",
     purpose="Decrypt an RSA ciphertext knowing the primes: compute d, then m by the Chinese remainder theorem.",
     inputs=[_I("c", "the ciphertext (0 <= c < n)", 0), _I("p", "first prime", 2), _I("q", "second prime", 2), _I("e", "public exponent", 2)],
     outputs=[_O("m", "the plaintext"), _O("d", "private exponent e^(-1) mod phi(n)"), _O("n", "modulus p q")],
     primary="m", conditions=["p, q distinct primes", "gcd(e, phi(n)) = 1", "0 <= c < n"],
     method="d = e^(-1) mod phi; m_p = c^(d mod p-1) mod p, m_q likewise; Garner's recombination",
     latex=r"m = m_q + q\,\big(q^{-1}(m_p - m_q) \bmod p\big)",
     run=rsa_decrypt_pq, verify=lambda i, o: (lambda p, q, e, c: o["n"] == p * q and e * o["d"] % ((p - 1) * (q - 1)) == 1 and _iv(o["m"])
                                               and 0 <= o["m"] < p * q and pow(o["m"], e, p * q) == c)(int(i["p"]), int(i["q"]), int(i["e"]), int(i["c"])),
     parse=parse_rsapq, reference=_ref_rsapq,
     wrong=lambda c, p, q, e: (lambda r: dict(r, m=pow(int(c), int(e), r["n"])))(rsa_decrypt_pq(c, p, q, e)),
     random_inputs=lambda rng: (lambda x: dict(x, c=rng.randint(2, x["p"] * x["q"] - 1)))(_rand_rsa(rng)),
     templates=["RSA: decrypt c = 2790 with p = 61, q = 53 and e = 17."],
     examples=[{"inputs": {"c": 2790, "p": 61, "q": 53, "e": 17}, "outputs": {"m": 65, "d": 2753}}],
     questions=[{"q": "RSA: decrypt c = 2790 with p = 61, q = 53 and e = 17.", "outputs": {"m": 65}}])


def diffie_hellman(p, g, a, b):
    p, g, a, b = _whole(p, "p", 3), _whole(g, "g", 2), _whole(a, "a", 1), _whole(b, "b", 1)
    _need_prime(p, "p")
    if g >= p:
        raise ValueError("the generator g must satisfy 1 < g < p")
    A, B = pow(g, a, p), pow(g, b, p)
    return {"A": A, "B": B, "shared": pow(B, a, p)}


def _rand_dh(rng):
    p = _rand_prime(rng, 23, 10 ** rng.randint(2, 12))
    return {"p": p, "g": rng.randint(2, p - 1), "a": rng.randint(2, 10 ** 6), "b": rng.randint(2, 10 ** 6)}


def parse_dh(q0):
    if not re.search(r"diffie|hellman", q0, re.I):
        return None
    return _named(q0, ["p", "g", "a", "b"])


_add("diffie_hellman", name="Diffie-Hellman key exchange", topic="cryptography / key exchange",
     purpose="The public values A = g^a, B = g^b (mod p) and the shared secret g^(ab) mod p.",
     inputs=[_I("p", "the prime modulus", 3), _I("g", "the generator (1 < g < p)", 2), _I("a", "Alice's secret exponent", 1),
             _I("b", "Bob's secret exponent", 1)],
     outputs=[_O("A", "Alice's public value g^a mod p"), _O("B", "Bob's public value g^b mod p"), _O("shared", "shared secret g^(ab) mod p")],
     primary="shared", conditions=["p prime", "1 < g < p"], method="modular exponentiation: B^a = A^b = g^(ab)",
     latex=r"s = B^a \bmod p = A^b \bmod p = g^{ab} \bmod p",
     run=diffie_hellman, verify=lambda i, o: (lambda p, g, a, b: _iv(o["shared"]) and o["A"] == pow(g, a, p) and o["B"] == pow(g, b, p)
                                               and o["shared"] == pow(o["A"], b, p))(int(i["p"]), int(i["g"]), int(i["a"]), int(i["b"])),
     parse=parse_dh, reference=lambda p, g, a, b: (lambda gm: {"A": int(gm.powmod(g, a, p)), "B": int(gm.powmod(g, b, p)),
                                                               "shared": int(gm.powmod(g, a * b, p))})(__import__("gmpy2")),
     wrong=lambda p, g, a, b: (lambda r: dict(r, shared=pow(int(g), int(a) + int(b), int(p))))(diffie_hellman(p, g, a, b)),
     random_inputs=_rand_dh,
     templates=["Diffie-Hellman with p = 23, g = 5, a = 6, b = 15: find the shared secret."],
     examples=[{"inputs": {"p": 23, "g": 5, "a": 6, "b": 15}, "outputs": {"A": 8, "B": 19, "shared": 2}}],
     questions=[{"q": "Diffie-Hellman with p = 23, g = 5, a = 6, b = 15: find the shared secret.", "outputs": {"shared": 2}},
                {"q": "Diffie-Hellman with p = 24, g = 5, a = 6, b = 15: find the shared secret.", "outcome": "INVALID_INPUT"}])


def elgamal_encrypt(p, g, x, k, m):
    p, g, x, k, m = _whole(p, "p", 3), _whole(g, "g", 2), _whole(x, "x", 1), _whole(k, "k", 1), _whole(m, "m", 1)
    _need_prime(p, "p")
    if g >= p or m >= p:
        raise ValueError("g and m must be between 1 and p - 1")
    h = pow(g, x, p)
    return {"h": h, "c1": pow(g, k, p), "c2": m * pow(h, k, p) % p}


def _v_elg(i, o):
    p, g, x, k, m = (int(i[s]) for s in ("p", "g", "x", "k", "m"))
    return (_iv(o["c2"]) and o["h"] == pow(g, x, p) and o["c1"] == pow(g, k, p)
            and o["c2"] * pow(o["c1"], p - 1 - x % (p - 1), p) % p == m)       # decryption gives m back


def parse_elg(q0):
    if not re.search(r"el ?gamal", q0, re.I) or not re.search(r"encrypt", q0, re.I):
        return None
    return _named(q0, ["p", "g", "x", "k", "m"])


_add("elgamal_encrypt", name="ElGamal encryption", topic="cryptography / public-key encryption",
     purpose="ElGamal ciphertext (c1, c2) = (g^k, m h^k) mod p with public key h = g^x.",
     inputs=[_I("p", "the prime modulus", 3), _I("g", "the generator", 2), _I("x", "the private key", 1), _I("k", "the ephemeral exponent", 1),
             _I("m", "the message (1 <= m < p)", 1)],
     outputs=[_O("h", "public key g^x mod p"), _O("c1", "g^k mod p"), _O("c2", "m h^k mod p")],
     primary="c2", conditions=["p prime", "1 < g < p", "1 <= m < p"], method="modular exponentiation",
     latex=r"c_1 = g^k,\ c_2 = m\,h^k \pmod p,\quad m = c_2\,c_1^{-x}",
     run=elgamal_encrypt, verify=_v_elg, parse=parse_elg,
     reference=lambda p, g, x, k, m: (lambda gm: {"h": int(gm.powmod(g, x, p)), "c1": int(gm.powmod(g, k, p)),
                                                  "c2": int(m * gm.powmod(g, x * k, p) % p)})(__import__("gmpy2")),
     wrong=lambda p, g, x, k, m: (lambda r: dict(r, c2=int(m) * pow(int(g), int(k), int(p)) % int(p)))(elgamal_encrypt(p, g, x, k, m)),
     random_inputs=lambda rng: (lambda p: {"p": p, "g": rng.randint(2, p - 1), "x": rng.randint(2, 10 ** 6), "k": rng.randint(2, 10 ** 6),
                                          "m": rng.randint(1, p - 1)})(_rand_prime(rng, 101, 10 ** rng.randint(3, 12))),
     templates=["ElGamal: encrypt m = 10 with p = 23, g = 5, x = 6, k = 3."],
     examples=[{"inputs": {"p": 23, "g": 5, "x": 6, "k": 3, "m": 10}, "outputs": {"h": 8, "c1": 10, "c2": 14}}],
     questions=[{"q": "ElGamal: encrypt m = 10 with p = 23, g = 5, x = 6, k = 3.", "outputs": {"c1": 10, "c2": 14}}])


_MODES = ["encrypt", "decrypt"]


def _letters(text):
    return [ord(ch) - 65 for ch in str(text).upper() if "A" <= ch <= "Z"]


def caesar(message, shift, mode):
    s = _whole(shift, "shift", hi=10 ** 6)
    sign = 1 if mode == "encrypt" else -1
    xs = _letters(message)
    if not xs:
        raise ValueError("the message has no letters")
    return {"result": "".join(chr(65 + (x + sign * s) % 26) for x in xs)}


def affine_cipher(message, a, b, mode):
    a, b = _whole(a, "a", hi=10 ** 6), _whole(b, "b", hi=10 ** 6)
    if math.gcd(a, 26) != 1:
        raise ValueError("a must be coprime to 26")
    xs = _letters(message)
    if not xs:
        raise ValueError("the message has no letters")
    if mode == "encrypt":
        return {"result": "".join(chr(65 + (a * x + b) % 26) for x in xs)}
    ai = _inv(a, 26)
    return {"result": "".join(chr(65 + ai * (x - b) % 26) for x in xs)}


def vigenere(message, key, mode):
    ks = _letters(key)
    if not ks or len(ks) != len(str(key).strip()):
        raise ValueError("the key must be letters A-Z only")
    xs = _letters(message)
    if not xs:
        raise ValueError("the message has no letters")
    sign = 1 if mode == "encrypt" else -1
    return {"result": "".join(chr(65 + (x + sign * ks[j % len(ks)]) % 26) for j, x in enumerate(xs))}


def _ref_caesar(message, shift, mode):
    from sympy.crypto.crypto import decipher_shift, encipher_shift
    return {"result": (encipher_shift if mode == "encrypt" else decipher_shift)(message, shift % 26)}


def _ref_affine(message, a, b, mode):
    from sympy.crypto.crypto import decipher_affine, encipher_affine
    return {"result": (encipher_affine if mode == "encrypt" else decipher_affine)(message, (a % 26, b % 26))}


def _ref_vig(message, key, mode):
    from sympy.crypto.crypto import decipher_vigenere, encipher_vigenere
    return {"result": (encipher_vigenere if mode == "encrypt" else decipher_vigenere)(message, key.upper())}


def _rmsg(rng):
    words = ["ATTACK", "AT", "DAWN", "meet", "me", "at", "the", "old", "bridge", "SECRET", "Hello", "world", "keep", "calm"]
    return " ".join(rng.choice(words) for _ in range(rng.randint(2, 6))) + rng.choice(["", ".", "!"])


def _cmode(q):
    enc, dec = re.search(r"encrypt|encode|encipher", q, re.I), re.search(r"decrypt|decode|decipher", q, re.I)
    return None if bool(enc) == bool(dec) else ("encrypt" if enc else "decrypt")


def _quoted(q0):
    m = re.search(r"[\"“]([^\"”]+)[\"”]|'([^']+)'", q0)
    return _first(m.group(1), m.group(2)) if m else None


def parse_caesar(q0):
    if not re.search(r"caesar|shift cipher", q0, re.I):
        return None
    out = {}
    msg, mode = _quoted(q0), _cmode(q0)
    s = re.search(r"shift (?:of |by |=\s*)?(-?\d+)|key (?:of |=\s*)?(-?\d+)", q0, re.I)
    if msg:
        out["message"] = msg
    if mode:
        out["mode"] = mode
    if s:
        out["shift"] = int(_first(s.group(1), s.group(2)))
    return out


def parse_affine(q0):
    if not re.search(r"affine cipher", q0, re.I):
        return None
    out = _named(q0, ["a", "b"])
    msg, mode = _quoted(q0), _cmode(q0)
    if msg:
        out["message"] = msg
    if mode:
        out["mode"] = mode
    return out


def parse_vig(q0):
    if not re.search(r"vigen[eè]re", q0, re.I):
        return None
    out = {}
    m = re.search(r"key(?:word)?\s*(?:=\s*|of\s+)?[\"“']?([A-Za-z]+)[\"”']?", q0, re.I)
    msg = re.sub(r"key(?:word)?\s*(?:=\s*|of\s+)?[\"“']?[A-Za-z]+[\"”']?", "", q0, flags=re.I)
    msg, mode = _quoted(msg), _cmode(q0)
    if m:
        out["key"] = m.group(1)
    if msg:
        out["message"] = msg
    if mode:
        out["mode"] = mode
    return out


_MODE_IN = {"symbol": "mode", "kind": "text", "type": "choice", "choices": _MODES, "meaning": "encrypt or decrypt", "required": True}
_add("caesar_cipher", name="Caesar (shift) cipher", topic="cryptography / classical ciphers",
     purpose="Encrypt or decrypt a message with a Caesar shift of k letters.",
     inputs=[_T("message", "the message (letters are kept, everything else is dropped)", n_max=2000), _I("shift", "the shift k", hi=10 ** 6), _MODE_IN],
     outputs=[_O("result", "the result in capital letters, without spaces or punctuation", "text")],
     primary="result", conditions=["letters A-Z only are enciphered; other characters are removed; output is upper case"],
     method="each letter x becomes x + k (mod 26); decryption subtracts k", latex=r"E(x) = (x + k) \bmod 26",
     run=caesar, verify=lambda i, o: o["result"] == caesar(i["message"], i["shift"], i["mode"])["result"] and
     caesar(o["result"], i["shift"], "decrypt" if i["mode"] == "encrypt" else "encrypt")["result"] == "".join(c for c in str(i["message"]).upper() if "A" <= c <= "Z"),
     parse=parse_caesar, reference=_ref_caesar,
     wrong=lambda message, shift, mode: caesar(message, shift, "decrypt" if mode == "encrypt" else "encrypt"),
     random_inputs=lambda rng: {"message": _rmsg(rng), "shift": rng.choice([k for k in range(1, 26) if k != 13]) + 26 * rng.randint(0, 2),
                                "mode": rng.choice(_MODES)},
     templates=["Encrypt \"HELLO WORLD\" with a Caesar shift of 3.", "Decrypt \"KHOOR\" with a Caesar shift of 3."],
     examples=[{"inputs": {"message": "HELLO WORLD", "shift": 3, "mode": "encrypt"}, "outputs": {"result": "KHOORZRUOG"}},
               {"inputs": {"message": "khoor", "shift": 3, "mode": "decrypt"}, "outputs": {"result": "HELLO"}}],
     questions=[{"q": "Encrypt \"HELLO WORLD\" with a Caesar shift of 3.", "outputs": {"result": "KHOORZRUOG"}},
                {"q": "Decrypt \"KHOOR\" with a Caesar shift of 3.", "outputs": {"result": "HELLO"}},
                {"q": "Encrypt \"HELLO\" with a Caesar cipher.", "outcome": "NEEDS_INPUT"}])

_add("affine_cipher", name="Affine cipher", topic="cryptography / classical ciphers",
     purpose="Encrypt or decrypt with the affine cipher E(x) = a x + b (mod 26).",
     inputs=[_T("message", "the message (letters are kept, everything else is dropped)", n_max=2000), _I("a", "multiplier (coprime to 26)", hi=10 ** 6),
             _I("b", "shift", hi=10 ** 6), _MODE_IN],
     outputs=[_O("result", "the result in capital letters, without spaces or punctuation", "text")],
     primary="result", conditions=["gcd(a, 26) = 1", "letters A-Z only; other characters are removed; output is upper case"],
     method="E(x) = a x + b, D(y) = a^(-1) (y - b) modulo 26", latex=r"E(x) = (ax + b) \bmod 26,\ D(y) = a^{-1}(y - b) \bmod 26",
     run=affine_cipher, verify=lambda i, o: o["result"] == affine_cipher(i["message"], i["a"], i["b"], i["mode"])["result"] and
     affine_cipher(o["result"], i["a"], i["b"], "decrypt" if i["mode"] == "encrypt" else "encrypt")["result"] == "".join(c for c in str(i["message"]).upper() if "A" <= c <= "Z"),
     parse=parse_affine, reference=_ref_affine,
     wrong=lambda message, a, b, mode: {"result": "".join(chr(65 + (int(a) * (x + int(b)) if mode == "encrypt" else _inv(int(a), 26) * x - int(b)) % 26) for x in _letters(message))},
     random_inputs=lambda rng: {"message": _rmsg(rng), "a": rng.choice([3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25]), "b": rng.randint(1, 25),
                                "mode": rng.choice(_MODES)},
     templates=["Encrypt \"AFFINE CIPHER\" with the affine cipher a = 5, b = 8."],
     examples=[{"inputs": {"message": "AFFINE CIPHER", "a": 5, "b": 8, "mode": "encrypt"}, "outputs": {"result": "IHHWVCSWFRCP"}},
               {"inputs": {"message": "IHHWVCSWFRCP", "a": 5, "b": 8, "mode": "decrypt"}, "outputs": {"result": "AFFINECIPHER"}}],
     questions=[{"q": "Encrypt \"AFFINE CIPHER\" with the affine cipher a = 5, b = 8.", "outputs": {"result": "IHHWVCSWFRCP"}},
                {"q": "Decrypt \"IHHWVCSWFRCP\" with the affine cipher a = 5, b = 8.", "outputs": {"result": "AFFINECIPHER"}},
                {"q": "Encrypt \"HELLO\" with the affine cipher a = 2, b = 3.", "outcome": "INVALID_INPUT"}])

_add("vigenere_cipher", name="Vigenère cipher", topic="cryptography / classical ciphers",
     purpose="Encrypt or decrypt with the Vigenère cipher under a keyword.",
     inputs=[_T("message", "the message (letters are kept, everything else is dropped)", n_max=2000), _T("key", "the keyword (letters)", n_max=200), _MODE_IN],
     outputs=[_O("result", "the result in capital letters, without spaces or punctuation", "text")],
     primary="result", conditions=["letters A-Z only; other characters are removed first, so the key runs over letters only; output is upper case"],
     method="add (encrypt) or subtract (decrypt) the repeating key letters modulo 26", latex=r"c_j = (m_j + k_{j \bmod |k|}) \bmod 26",
     run=vigenere, verify=lambda i, o: o["result"] == vigenere(i["message"], i["key"], i["mode"])["result"] and
     vigenere(o["result"], i["key"], "decrypt" if i["mode"] == "encrypt" else "encrypt")["result"] == "".join(c for c in str(i["message"]).upper() if "A" <= c <= "Z"),
     parse=parse_vig, reference=_ref_vig,
     wrong=lambda message, key, mode: (lambda ks, s: {"result": "".join(chr(65 + (ord(ch.upper()) - 65 + s * ks[j % len(ks)]) % 26)
                                                                          for j, ch in enumerate(str(message)) if ch.isascii() and ch.isalpha())})(
         _letters(key), 1 if mode == "encrypt" else -1),
     random_inputs=lambda rng: {"message": _rmsg(rng), "key": rng.choice(["LEMON", "KEY", "CIPHER", "ab", "Secret"]), "mode": rng.choice(_MODES)},
     templates=["Encrypt \"ATTACK AT DAWN\" with the Vigenère cipher and key LEMON."],
     examples=[{"inputs": {"message": "ATTACK AT DAWN", "key": "LEMON", "mode": "encrypt"}, "outputs": {"result": "LXFOPVEFRNHR"}},
               {"inputs": {"message": "LXFOPVEFRNHR", "key": "lemon", "mode": "decrypt"}, "outputs": {"result": "ATTACKATDAWN"}}],
     questions=[{"q": "Encrypt \"ATTACK AT DAWN\" with the Vigenère cipher and key LEMON.", "outputs": {"result": "LXFOPVEFRNHR"}},
                {"q": "Decrypt \"LXFOPVEFRNHR\" with the Vigenere cipher and key LEMON.", "outputs": {"result": "ATTACKATDAWN"}}])


# ==================================================================================================================
# the reading guard: every parser above is wrapped by the same three rules (ask_sweep 2026-10-23: 51 wrong answers)
#   1. a number written in scientific notation (1.378e10, 1.378 × 10^10) is never truncated to its first digits
#      (phi(1.378e10) was answered as phi(1)) and never taken as an exact whole number: it states a magnitude to a
#      few significant digits. In all 621 such numbers of the canonical questions the integer meant was a different
#      one (1.378e10 stood for 13781520824), so reading 13780000000 would answer a question nobody asked. The input
#      it fills is refused as INVALID_INPUT ("write the whole number in full"); nothing is ever rounded;
#   2. every stated number is bound: a reading that leaves a number of the question unused is refused (a dropped
#      edge, a second list, a parameter the capability has no slot for);
#   3. wording that names a different capability's task (a Jacobi symbol, not a factorization; a count, not a
#      yes/no property) is refused by every other capability.
# ==================================================================================================================
from decimal import Decimal, InvalidOperation

_SCI = re.compile(r"(?<![\w.])(\d+(?:\.\d+)?)(?:[eE]([+-]?\d+)|\s*(?:×|\*|\s[xX]\s)\s*10\s*(?:\^|\*\*)\s*([+-]?\d+))(?![\w]|\.\d)")
_NUMTOK = re.compile(r"(?<![\w.])(\d+(?:\.\d+)?)(?=(?:st|nd|rd|th)?(?![\w])|[A-Za-z](?![\w])|\.(?!\d)|[^\w.]|$)")


def _plain_numbers(q0: str):
    """The question with every scientific-notation number written out (1.378e10 -> 13780000000, 1.5e-3 -> 0.0015), so
    no parser reads its first digits alone, and {value written out: as the question wrote it}."""
    sci = {}

    def repl(m):
        try:
            d = Decimal(m.group(1)) * (Decimal(10) ** int(m.group(2) or m.group(3)))
            if abs(d.adjusted()) > 40:
                raise OverflowError
        except (InvalidOperation, ValueError, OverflowError):
            sci["?"] = m.group(0)                          # unreadable: nothing in this question is answered
            return m.group(0)
        text = format(d.normalize(), "f")
        sci[text] = m.group(0).strip()
        return text
    q = _SCI.sub(repl, q0.replace("−", "-"))
    return q, sci


def _refuse_sci(p: dict, sci: dict, integer_like: set):
    """The reading with every input filled from a scientific-notation number replaced by the number as written, which
    no whole-number input accepts (INVALID_INPUT, never a rounded integer); None when such a number filled any other
    kind of input (a vertex name, a weight)."""
    if "?" in sci:
        return None
    vals = {Fraction(t): w for t, w in sci.items()}
    note = " (scientific notation: write the whole number in full)"

    def sub(x):
        if isinstance(x, bool) or not isinstance(x, (int, float, str)):
            return x, False
        try:
            f = Fraction(x) if not isinstance(x, str) else Fraction(x.strip())
        except (ValueError, ZeroDivisionError):
            return x, False
        hit = vals.get(f) or vals.get(-f)
        return ((("-" if f < 0 else "") + hit + note), True) if hit else (x, False)
    out = {}
    for k, v in p.items():
        if isinstance(v, (list, tuple)):
            items = [sub(e) for e in v]
            if any(h for _e, h in items) and k not in integer_like:
                return None
            if any(isinstance(e, (list, tuple)) and any(sub(x)[1] for x in e) for e in v):
                return None                                 # an edge or a pair built from such a number
            out[k] = [e for e, _h in items]
        else:
            new, hit = sub(v)
            if hit and k not in integer_like:
                return None
            out[k] = new
    return out


def _numbers_in(q: str) -> list:
    """Every number written in the text as (value, start, end): '10-5' is two numbers (an edge), ' -5' is negative,
    '3x3' (a size), 'x1' and 'K5' (names) are not numbers; an ordinal ('10th') or a coefficient ('75x') is."""
    out = []
    for m in _NUMTOK.finditer(q):
        s, e = m.start(1), m.end(1)
        v = Fraction(m.group(1))
        if s > 0 and q[s - 1] == "-" and (s == 1 or not (q[s - 2].isalnum() or q[s - 2] in "_)")):
            v = -v
        out.append((v, s, e))
    return out


def _bound_values(parsed) -> tuple:
    """The numbers a reading uses, and the texts it took verbatim (formulas, messages, digit strings)."""
    nums, texts = set(), []

    def walk(x):
        if isinstance(x, bool) or x is None:
            return
        if isinstance(x, (int, float)):
            try:
                nums.add(Fraction(x))
            except (ValueError, OverflowError):
                pass
            return
        if isinstance(x, Fraction):
            nums.add(x)
            return
        if isinstance(x, str):
            t = x.strip()
            if re.fullmatch(r"[-+]?\d+(?:\.\d+)?", t):
                nums.add(Fraction(t))
            elif t:
                texts.append(t)
                for tok in re.findall(r"\d+(?:\.\d+)?", t):
                    nums.add(Fraction(tok))
            return
        if isinstance(x, dict):
            for v in x.values():
                walk(v)
            return
        if isinstance(x, (list, tuple, set)):
            for v in x:
                walk(v)
    walk(parsed)
    return nums, texts


# numbers that belong to a capability's own notation, read by its parser through a stated transformation (never
# free parameters): the 2 of a square x^2, the -1 of an inverse a^-1, "the last 3 digits" (modulo 10^3), "less than
# 100" (at most 99), "a polygon with 8 sides" (C_6), "from 1 to n" (the start of the range), lattice points (the steps
# are their differences)
_NOTATION_ALL = r"(?<![a-z])[a-z]\s*\^\s*2(?![\d.])"
_NOTATION = {
    "pell_equation": r"=\s*1\b",
    "mod_pow": r"last \d+ digits",
    "mod_inverse": r"\^\s*\(?\s*-\s*1\s*\)?",
    "prime_counting": r"(?:less than|below|under|smaller than|<)\s*\d+",
    "catalan": r"polygon with \d+ (?:sides|vertices)|\d+-gon",
    "inclusion_exclusion": r"\b(?:from|between|in the range) 1 (?:to|and|through)\b|\b1 through\b",
    "lattice_paths": r"\(\s*-?\d+\s*,\s*-?\d+\s*\)",
}


def _unbound(q: str, parsed: dict, key: str = "") -> list:
    """The numbers of the question the reading does not use (in order of appearance)."""
    nums, texts = _bound_values(parsed)
    for t in sorted(texts, key=len, reverse=True):          # numbers inside a text taken verbatim are that text's
        if len(t) >= 2:
            q = re.sub(re.escape(t), lambda m: " " * len(m.group(0)), q, flags=re.I)
    for rx in (_NOTATION_ALL, _NOTATION.get(key), _TASK_NAMES.get(key)):   # and a number in the task's own name
        if rx:                                                               # ("2-colourable", "2's complement")
            q = re.sub(rx, lambda m: " " * len(m.group(0)), q, flags=re.I)
    return [(v, s, e) for v, s, e in _numbers_in(q) if v not in nums and -v not in nums]


# rule 3: the capabilities a question names. A name here is the name of a TASK, not a word that may describe an
# input ("the prime factors of n", "a directed acyclic graph" describe; "the Jacobi symbol", "a topological order"
# name what is asked). When a question names one or more of these tasks, a capability whose own name is not among
# them does not read it.
_TASK_NAMES = {
    "carmichael_lambda": r"carmichael|reduced totient",
    "jacobi_symbol": r"jacobi",
    "legendre_symbol": r"legendre symbol",
    "crt": r"chinese remainder|\bcrt\b|\bmoduli\b",        # residues with their moduli: a system of congruences
    "discrete_log": r"discrete log",
    "topological_sort": r"topological",
    "euler_phi": r"totient|euler'?s? phi|\bphi\s*\(|φ",
    "mobius": r"m[öo]bius|\bmu\s*\(",
    "primitive_root": r"primitive root",
    "multiplicative_order": r"multiplicative order|\border of \S+ modulo",
    "twos_complement": r"two'?s[- ]complement|2'?s[- ]complement",
    "stirling2": r"stirling numbers? of the second kind",
    "stirling1": r"stirling numbers? of the first kind",
    "bell": r"\bbell number",
    "catalan": r"catalan",
    "fibonacci": r"fibonacci",
    "lucas": r"\blucas",
    "bernoulli": r"bernoulli",
    "derangements": r"derangement",
    "pell_equation": r"\bpell",
    "cf_rational": r"continued fraction",
    "cf_sqrt": r"continued fraction",
    "girth": r"\bgirth\b|shortest cycle",
    "graph_triangles": r"triangles?\b",
    "chromatic_number": r"chromatic|colou?ring",
    "clique_number": r"\bclique",
    "independence_number": r"independen(?:ce|t) (?:number|set)",
    "spanning_tree_count": r"spanning trees",
    "articulation_points": r"articulation|cut vertices|cut vertex",
    "bridges": r"\bbridges?\b|cut edges?",
    "strongly_connected": r"strongly connected",
    "hamiltonian_cycle": r"hamiltonian",
    "eulerian": r"euler(?:ian)? (?:circuit|path|trail|tour)|\beulerian",
    "bipartite": r"bipartite|2-colou?rable|two-colou?rable",
    "prufer_encode": r"pr[üu]fer",
    "prufer_decode": r"pr[üu]fer",
    "graph_diameter": r"diameter|radius|eccentricit|\bcent(?:er|re)\b|periphery",
    "cycle_rank": r"cyclomatic|circuit rank|cycle rank",
    "rsa_keygen": r"\brsa\b",
    "rsa_encrypt": r"\brsa\b",
    "rsa_decrypt": r"\brsa\b",
    "rsa_decrypt_pq": r"\brsa\b",
    "diffie_hellman": r"diffie|hellman",
    "elgamal_encrypt": r"elgamal",
    "caesar_cipher": r"caesar",
    "affine_cipher": r"affine cipher",
    "vigenere_cipher": r"vigen[èe]re",
}
_TASK_RE = {k: re.compile(v, re.I) for k, v in _TASK_NAMES.items()}
_COUNT_ASKED = re.compile(r"\bhow many\b|\bnumber of\b|\bcount (?:of|the)\b|\bcount\b(?=.*\?)|\btotal\b", re.I)


def _named_tasks(q: str) -> set:
    return {k for k, rx in _TASK_RE.items() if rx.search(q)}


def _guarded(key, spec, parse):
    outs = spec["outputs"]
    yes_no_only = all(o.get("kind") == "boolean" for o in outs)
    integer_slots = [i["symbol"] for i in spec["inputs"] if i.get("type") == "integer"]
    integer_like = set(integer_slots) | {i["symbol"] for i in spec["inputs"] if i.get("type") == "vector"}

    def guarded(q0):
        q, sci = _plain_numbers(q0)
        named = _named_tasks(q)
        if named and key not in named:                     # rule 3: the question asks for another task
            return None
        if yes_no_only and _COUNT_ASKED.search(q):          # a count is asked; this capability answers yes or no
            return None
        p = parse(q)
        if not p or "_out_of_scope" in p:
            return p
        loose = _unbound(q, p, key)                         # rule 2: every stated number is bound
        if not loose:
            return _refuse_sci(p, sci, integer_like) if sci else p
        if sci:
            return None
        # a non-integer value (2.5, 1.5e-3 = 0.0015) that a parser read only in part: the input it was read into is
        # given its written value, so the question is refused as INVALID_INPUT (never rounded to a whole number)
        if all(v.denominator != 1 for v, _s, _e in loose):
            for v, s, e in loose:
                frag = Fraction(int(q[s:e].split(".")[0]))
                slot = next((k for k in integer_slots if k in p and not isinstance(p[k], (list, tuple, str, bool))
                             and Fraction(p[k]) in (frag, -frag)), None)
                if slot is None:
                    return None
                p = dict(p, **{slot: ("-" if v < 0 else "") + q[s:e]})
            return p
        return None
    return guarded


for _k, _spec in PROCEDURES.items():
    _spec["parse"] = _guarded(_k, _spec, _spec["parse"])
