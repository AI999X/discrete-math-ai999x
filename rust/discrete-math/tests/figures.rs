//! Every Discrete Mathematics capability's live figures (src/figures.rs plus the generic engine): its worked example
//! gives each published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or
//! text outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use discrete_math_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/discrete-math-ai999x/figures, except the
/// generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("affine_cipher", &["letters"]),
    ("articulation_points", &["graph"]),
    ("bfs_order", &["bars", "graph", "graph-2"]),
    ("bipartite", &["graph"]),
    ("bracelets", &["field"]),
    ("bridges", &["graph"]),
    ("caesar_cipher", &["letters"]),
    ("cf_rational", &["series"]),
    ("cf_sqrt", &["series"]),
    ("chromatic_number", &["bars", "graph", "graph-2"]),
    ("clique_number", &["graph"]),
    ("combinations", &["field"]),
    ("connected_components", &["graph"]),
    ("crt", &["io"]),
    ("cycle_rank", &["graph"]),
    ("dfs_order", &["graph"]),
    ("directed_cycle", &["graph"]),
    ("divisor_functions", &["decision", "series"]),
    ("eulerian", &["graph"]),
    ("extended_gcd", &["field"]),
    ("gcd_lcm", &["io"]),
    ("girth", &["graph"]),
    ("graph_degrees", &["bars", "series"]),
    ("graph_diameter", &["graph"]),
    ("graph_triangles", &["graph"]),
    ("graphical_sequence", &["io"]),
    ("hamiltonian_cycle", &["graph"]),
    ("independence_number", &["graph"]),
    ("integer_partitions", &["stems"]),
    ("integer_sqrt", &["decision"]),
    ("is_prime", &["decision"]),
    ("is_tree", &["graph"]),
    ("jacobi_symbol", &["field"]),
    ("lattice_paths", &["field"]),
    ("legendre_symbol", &["table"]),
    ("linear_diophantine", &["decision"]),
    ("logic_contradiction", &["truth"]),
    ("logic_equivalence", &["truth"]),
    ("logic_evaluate", &["truth"]),
    ("logic_satisfiable", &["truth"]),
    ("logic_tautology", &["truth"]),
    ("minimal_cnf", &["truth"]),
    ("minimal_dnf", &["truth"]),
    ("minimal_sop_minterms", &["io"]),
    ("mobius", &["decision"]),
    ("mod_reduce", &["field"]),
    ("multinomial", &["io"]),
    ("multisets", &["field"]),
    ("necklaces", &["field"]),
    ("partitions_k", &["field"]),
    ("permutations", &["field"]),
    ("pigeonhole_max", &["field"]),
    ("pigeonhole_needed", &["field"]),
    ("prime_factorization", &["matrix"]),
    ("prufer_decode", &["matrix", "tree"]),
    ("prufer_encode", &["graph", "series"]),
    ("sequences_with_repetition", &["field"]),
    ("spanning_tree_count", &["graph"]),
    ("sqrt_mod", &["complex"]),
    ("stirling2", &["heatmap"]),
    ("strongly_connected", &["graph"]),
    ("topological_sort", &["graph"]),
    ("truth_table", &["io"]),
    ("two_squares", &["matrix"]),
    ("twos_complement", &["field"]),
    ("vigenere_cipher", &["letters"]),
    ("walks_count", &["graph"]),
    ("word_arrangements", &["bars"]),
];

/// Replacement inputs for a second, different example: (capability or "*" for any, input, value).
const OTHER: &[(&str, &str, &str)] = &[
    ("affine_cipher", "message", "HELLO THERE"),
    ("caesar_cipher", "message", "DISCRETE"),
    ("vigenere_cipher", "message", "DEFEND THE EAST"),
    ("word_arrangements", "word", "BANANA"),
    ("crt", "residues", "1, 2, 3"),
    ("gcd_lcm", "numbers", "60, 48, 18"),
    ("graphical_sequence", "degrees", "2, 2, 2, 2"),
    ("minimal_sop_minterms", "minterms", "0, 2, 5, 7"),
    ("multinomial", "counts", "1, 2, 4"),
    ("prufer_decode", "sequence", "3, 3, 3, 7, 7, 8, 8"),
    ("logic_contradiction", "formula", "p & ~q & q"),
    ("logic_equivalence", "formula2", "p & q"),
    ("logic_evaluate", "assignment", "p = true, q = true, r = false"),
    ("logic_satisfiable", "formula", "(p | q) & ~p"),
    ("logic_tautology", "formula", "p | ~p | q"),
    ("minimal_cnf", "formula", "(A | B) & (B | C) & (A | C)"),
    ("minimal_dnf", "formula", "(A & B) | (B & C) | (A & ~C)"),
    ("truth_table", "formula", "p & (q | r)"),
    ("directed_cycle", "edges", "[[\"A\", \"B\"], [\"B\", \"C\"], [\"C\", \"D\"], [\"D\", \"B\"]]"),
    ("strongly_connected", "edges", "[[\"A\", \"B\"], [\"B\", \"A\"], [\"C\", \"D\"], [\"D\", \"E\"], [\"E\", \"C\"]]"),
    ("topological_sort", "edges", "[[\"A\", \"B\"], [\"B\", \"C\"], [\"A\", \"D\"], [\"D\", \"C\"], [\"C\", \"E\"]]"),
    ("prufer_encode", "edges", "1-2, 2-3, 3-4, 4-5, 2-6"),
    ("is_tree", "edges", "1-2, 2-3, 2-4, 4-5, 4-6"),
    ("independence_number", "edges", "1-2, 2-3, 3-4, 4-5, 5-6, 6-1"),
    ("*", "edges", "A-B, B-C, C-D, D-A, A-C, D-E"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c=[['A', 'B']]).
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: Input;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = Input::Text(cs[s0..i].iter().collect::<String>().trim().to_string());
            i += 1;
        } else if cs[i] == '[' {
            let s0 = i;
            let mut depth = 0;
            loop {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                i += 1;
            }
            i += 1;
            let t: String = cs[s0..i].iter().collect::<String>().replace('\'', "\"");
            value = Input::Json(serde_json::from_str(&t).expect("a list"));
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = Input::Text(cs[s0..i].iter().collect::<String>().trim().to_string());
        }
        out.push((name.trim().to_string(), value));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    }
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf\""), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The value with each whole number in it moved by `d` ("3" -> "4"; "2, 3" -> "3, 4"); None when it holds none.
fn shift_ints(t: &str, d: i64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let mut flush = |cur: &mut String, out: &mut String| {
        if !cur.is_empty() {
            match cur.parse::<i64>() {
                Ok(x) => {
                    out.push_str(&(x + d).to_string());
                    changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || (c == '-' && cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for (cap, sym, val) in OTHER.iter().filter(|o| o.0 == *key || o.0 == "*") {
            if !given.iter().any(|g| g.0 == *sym) || (*cap == "*" && OTHER.iter().any(|o| o.0 == *key && o.1 == *sym)) {
                continue;
            }
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = if val.starts_with('[') { Input::Json(serde_json::from_str(val).unwrap()) } else {
                        Input::Text(val.to_string())
                    };
                }
            }
            tries.push(g);
        }
        for d in [1, 2, -1, 3] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if t.chars().any(|c| c.is_ascii_alphabetic()) {
                        continue;
                    }
                    if let Some(t2) = shift_ints(t, d) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // a Legendre symbol needs an odd prime: refused by the model, so no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = vec![("a".into(), Input::Text("5".into())), ("p".into(), Input::Text("12".into()))];
    let a = ax_engine::solve(p, "legendre_symbol", &given, None);
    assert!(!a.solved());
    assert!(figures(p, "legendre_symbol", &given, None, None).is_empty());
    // a cipher message without letters cannot be mapped letter by letter
    let given: Vec<(String, Input)> = vec![("message".into(), Input::Text("123".into())),
                                           ("shift".into(), Input::Text("3".into())),
                                           ("mode".into(), Input::Text("encrypt".into()))];
    let figs = figures(p, "caesar_cipher", &given, None, None);
    assert!(figs.iter().all(|f| f.kind != "letters"));
}
