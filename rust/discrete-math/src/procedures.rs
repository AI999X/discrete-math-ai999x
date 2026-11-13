//! discrete_math: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/discrete_math/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "affine_cipher",
    "apportionment",
    "articulation_points",
    "base_convert",
    "bell",
    "bernoulli",
    "bfs_order",
    "binomial_term",
    "bipartite",
    "bracelets",
    "bridges",
    "caesar_cipher",
    "carmichael_lambda",
    "catalan",
    "cf_rational",
    "cf_sqrt",
    "chromatic_number",
    "clique_number",
    "combinations",
    "connected_components",
    "crt",
    "cycle_rank",
    "derangements",
    "dfs_order",
    "diffie_hellman",
    "directed_cycle",
    "discrete_log",
    "divisor_functions",
    "elgamal_encrypt",
    "euler_phi",
    "eulerian",
    "extended_gcd",
    "fibonacci",
    "gcd_lcm",
    "girth",
    "graph_degrees",
    "graph_diameter",
    "graph_triangles",
    "graphical_sequence",
    "hamiltonian_cycle",
    "inclusion_exclusion",
    "independence_number",
    "integer_partitions",
    "integer_sqrt",
    "is_prime",
    "is_tree",
    "jacobi_symbol",
    "lattice_paths",
    "legendre_symbol",
    "linear_congruence",
    "linear_diophantine",
    "logic_contradiction",
    "logic_equivalence",
    "logic_evaluate",
    "logic_satisfiable",
    "logic_tautology",
    "lucas",
    "minimal_cnf",
    "minimal_dnf",
    "minimal_sop_minterms",
    "mobius",
    "mod_inverse",
    "mod_pow",
    "mod_reduce",
    "multinomial",
    "multiplicative_order",
    "multisets",
    "necklaces",
    "next_prime",
    "nth_prime",
    "partitions_distinct",
    "partitions_k",
    "pell_equation",
    "permutations",
    "pigeonhole_max",
    "pigeonhole_needed",
    "previous_prime",
    "prime_counting",
    "prime_factorization",
    "primitive_root",
    "prufer_decode",
    "prufer_encode",
    "rsa_decrypt",
    "rsa_decrypt_pq",
    "rsa_encrypt",
    "rsa_keygen",
    "sequences_with_repetition",
    "spanning_tree_count",
    "sqrt_mod",
    "stars_and_bars",
    "stirling1",
    "stirling2",
    "strongly_connected",
    "topological_sort",
    "truth_table",
    "two_squares",
    "twos_complement",
    "venn_three_sets",
    "venn_two_sets",
    "vigenere_cipher",
    "voting_methods",
    "walks_count",
    "word_arrangements",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "affine_cipher" => {
            let r = f_affine_cipher(match v.get("message") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mode") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_33(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "apportionment" => {
            let r = f_apportionment(match v.get("populations") { Some(x) => py::from_val(x), None => { return None } }, match v.get("seats") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_142(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "articulation_points" => {
            let r = f_articulation_points(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_235(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "base_convert" => {
            let r = f_base_convert(match v.get("digits") { Some(x) => py::from_val(x), None => { return None } }, match v.get("from_base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("to_base") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_305(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bell" => {
            let r = f_bell(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_352(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bernoulli" => {
            let r = f_bernoulli(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_383(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bfs_order" => {
            let r = f_bfs_order(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_433(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_term" => {
            let r = f_binomial_term(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bipartite" => {
            let r = f_bipartite(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_515(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bracelets" => {
            let r = s_536(match v.get("beads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("colours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_598(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bridges" => {
            let r = f_bridges(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_661(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "caesar_cipher" => {
            let r = f_caesar(match v.get("message") { Some(x) => py::from_val(x), None => { return None } }, match v.get("shift") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mode") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_720(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "carmichael_lambda" => {
            let r = f_carmichael_lambda(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "catalan" => {
            let r = f_catalan(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cf_rational" => {
            let r = f_cf_rational(match v.get("numerator") { Some(x) => py::from_val(x), None => { return None } }, match v.get("denominator") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_887(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cf_sqrt" => {
            let r = f_cf_sqrt(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_934(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chromatic_number" => {
            let r = f_chromatic_number(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_995(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clique_number" => {
            let r = f_clique_number(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1061(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combinations" => {
            let r = f_combinations(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1111(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "connected_components" => {
            let r = f_connected_components(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1154(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crt" => {
            let r = f_crt(match v.get("residues") { Some(x) => py::from_val(x), None => { return None } }, match v.get("moduli") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1223(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cycle_rank" => {
            let r = f_cycle_rank(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1279(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "derangements" => {
            let r = f_derangements(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1323(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dfs_order" => {
            let r = f_dfs_order(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1360(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diffie_hellman" => {
            let r = f_diffie_hellman(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1421(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "directed_cycle" => {
            let r = f_directed_cycle(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1476(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "discrete_log" => {
            let r = f_discrete_log(match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("modulus") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1530(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "divisor_functions" => {
            let r = f_divisor_functions(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1610(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elgamal_encrypt" => {
            let r = f_elgamal_encrypt(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1699(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "euler_phi" => {
            let r = f_euler_phi(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1773(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eulerian" => {
            let r = f_eulerian(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1829(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "extended_gcd" => {
            let r = f_extended_gcd(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1870(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fibonacci" => {
            let r = f_fibonacci(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1911(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gcd_lcm" => {
            let r = f_gcd_lcm(match v.get("numbers") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1954(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "girth" => {
            let r = f_girth(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2024(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_degrees" => {
            let r = f_graph_degrees(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2071(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_diameter" => {
            let r = f_graph_diameter(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2141(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_triangles" => {
            let r = f_graph_triangles(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2210(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graphical_sequence" => {
            let r = f_graphical_sequence(match v.get("degrees") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2267(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hamiltonian_cycle" => {
            let r = f_hamiltonian_cycle(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2317(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inclusion_exclusion" => {
            let r = f_inclusion_exclusion(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("divisors") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2384(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "independence_number" => {
            let r = f_independence_number(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2445(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "integer_partitions" => {
            let r = f_integer_partitions(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2496(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "integer_sqrt" => {
            let r = f_integer_sqrt(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "is_prime" => {
            let r = f_is_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2611(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "is_tree" => {
            let r = f_is_tree(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2684(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "jacobi_symbol" => {
            let r = f_jacobi_symbol(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2747(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lattice_paths" => {
            let r = f_lattice_paths(match v.get("right") { Some(x) => py::from_val(x), None => { return None } }, match v.get("up") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2818(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "legendre_symbol" => {
            let r = f_legendre_symbol(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2884(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_congruence" => {
            let r = f_linear_congruence(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2922(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_diophantine" => {
            let r = f_linear_diophantine(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_contradiction" => {
            let r = f_logic_contradiction(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3089(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_equivalence" => {
            let r = f_logic_equivalence(match v.get("formula1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_evaluate" => {
            let r = f_logic_evaluate(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("assignment") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_satisfiable" => {
            let r = f_logic_satisfiable(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_tautology" => {
            let r = f_logic_tautology(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lucas" => {
            let r = f_lucas(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3496(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimal_cnf" => {
            let r = f_minimal_cnf(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3638(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimal_dnf" => {
            let r = f_minimal_dnf(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3824(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimal_sop_minterms" => {
            let r = f_minimal_sop_minterms(match v.get("minterms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dont_cares") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3960(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mobius" => {
            let r = f_mobius(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4080(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mod_inverse" => {
            let r = f_mod_inverse(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mod_pow" => {
            let r = f_mod_pow(match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("exponent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("modulus") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4166(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mod_reduce" => {
            let r = f_mod_reduce(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4196(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multinomial" => {
            let r = f_multinomial(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multiplicative_order" => {
            let r = f_multiplicative_order(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4333(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multisets" => {
            let r = f_multisets(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4380(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "necklaces" => {
            let r = f_necklaces(match v.get("beads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("colours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4455(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "next_prime" => {
            let r = f_next_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4510(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nth_prime" => {
            let r = f_nth_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4542(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partitions_distinct" => {
            let r = f_partitions_distinct(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4590(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partitions_k" => {
            let r = f_partitions_k(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4629(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pell_equation" => {
            let r = f_pell(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4676(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "permutations" => {
            let r = f_permutations(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4720(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pigeonhole_max" => {
            let r = f_pigeonhole_max(match v.get("items") { Some(x) => py::from_val(x), None => { return None } }, match v.get("boxes") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pigeonhole_needed" => {
            let r = f_pigeonhole_needed(match v.get("boxes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at_least") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4779(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "previous_prime" => {
            let r = f_previous_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4830(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prime_counting" => {
            let r = f_prime_counting(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4862(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prime_factorization" => {
            let r = f_prime_factorization(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4934(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "primitive_root" => {
            let r = f_primitive_root(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5047(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prufer_decode" => {
            let r = f_prufer_decode(match v.get("sequence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5119(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prufer_encode" => {
            let r = f_prufer_encode(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5243(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_decrypt" => {
            let r = f_rsa_decrypt(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5294(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_decrypt_pq" => {
            let r = f_rsa_decrypt_pq(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5364(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_encrypt" => {
            let r = f_rsa_encrypt(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_keygen" => {
            let r = f_rsa_keygen(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5471(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sequences_with_repetition" => {
            let r = f_sequences_with_repetition(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5512(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spanning_tree_count" => {
            let r = f_spanning_tree_count(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5568(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sqrt_mod" => {
            let r = f_sqrt_mod(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5655(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stars_and_bars" => {
            let r = f_stars_and_bars(match v.get("total") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lower") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("upper") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stirling1" => {
            let r = f_stirling1(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5761(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stirling2" => {
            let r = f_stirling2(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "strongly_connected" => {
            let r = f_strongly_connected(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5860(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "topological_sort" => {
            let r = f_topological_sort(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5924(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "truth_table" => {
            let r = f_truth_table(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6026(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_squares" => {
            let r = f_two_squares(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6122(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "twos_complement" => {
            let r = f_twos_complement(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6188(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "venn_three_sets" => {
            let r = f_venn_three_sets(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ab") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ac") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("abc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("universe") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6236(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "venn_two_sets" => {
            let r = f_venn_two_sets(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("both") { Some(x) => py::from_val(x), None => { return None } }, match v.get("universe") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6310(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vigenere_cipher" => {
            let r = f_vigenere(match v.get("message") { Some(x) => py::from_val(x), None => { return None } }, match v.get("key") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mode") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6354(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "voting_methods" => {
            let r = f_voting_methods(match v.get("ballots") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6452(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "walks_count" => {
            let r = f_walks_count(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("length") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6553(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "word_arrangements" => {
            let r = f_word_arrangements(match v.get("word") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6577(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_MAXI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Int(1000000000000000i64)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MAX_VARS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Int(12i64)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__APP_METHODS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("hamilton"), Py::str("jefferson"), Py::str("webster"), Py::str("adams"), Py::str("huntington-hill")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__DIG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::str("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LSYM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("<->"), Py::str("IFF")), (Py::str("<=>"), Py::str("IFF")), (Py::str("↔"), Py::str("IFF")), (Py::str("⇔"), Py::str("IFF")), (Py::str("->"), Py::str("IMP")), (Py::str("=>"), Py::str("IMP")), (Py::str("→"), Py::str("IMP")), (Py::str("⇒"), Py::str("IMP")), (Py::str("~"), Py::str("NOT")), (Py::str("¬"), Py::str("NOT")), (Py::str("!"), Py::str("NOT")), (Py::str("&&"), Py::str("AND")), (Py::str("&"), Py::str("AND")), (Py::str("∧"), Py::str("AND")), (Py::str("||"), Py::str("OR")), (Py::str("|"), Py::str("OR")), (Py::str("∨"), Py::str("OR")), (Py::str("^"), Py::str("XOR")), (Py::str("⊕"), Py::str("XOR")), (Py::str("("), Py::str("(")), (Py::str(")"), Py::str(")")), (Py::str("1"), Py::Bool(true)), (Py::str("0"), Py::Bool(false))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LTOK() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*(<->|<=>|↔|⇔|->|=>|→|⇒|~|¬|!|&&|&|∧|\\|\\||\\||∨|\\^|⊕|\\(|\\)|[A-Za-z_][A-Za-z0-9_]*|[01])"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LWORD() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("and"), Py::str("AND")), (Py::str("or"), Py::str("OR")), (Py::str("not"), Py::str("NOT")), (Py::str("xor"), Py::str("XOR")), (Py::str("implies"), Py::str("IMP")), (Py::str("iff"), Py::str("IFF")), (Py::str("true"), Py::Bool(true)), (Py::str("false"), Py::Bool(false))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MR_BASES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Int(2i64), Py::Int(325i64), Py::Int(9375i64), Py::Int(28178i64), Py::Int(450775i64), Py::Int(9780504i64), Py::Int(1795265022i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NCAP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Int(170i64)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SMALL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Int(2i64), Py::Int(3i64), Py::Int(5i64), Py::Int(7i64), Py::Int(11i64), Py::Int(13i64), Py::Int(17i64), Py::Int(19i64), Py::Int(23i64), Py::Int(29i64), Py::Int(31i64), Py::Int(37i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__VENN3_OUT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("only_a"), Py::str("only_b"), Py::str("only_c"), Py::str("only_a_and_b"), Py::str("only_a_and_c"), Py::str("only_b_and_c")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ap_gt(mut v_method: Py, mut v_pi: Py, mut v_k: Py, mut v_pj: Py, mut v_l: Py) -> R {
    if ({ let t109 = v_method.clone(); Py::Bool({ let t110 = Py::str("jefferson"); if py::eq(&t109, &t110) { true } else { false } }) }).truthy() {
        return Ok({ let t111 = py::mul(&v_pi.clone(), &v_l.clone())?; Py::Bool({ let t112 = py::mul(&v_pj.clone(), &v_k.clone())?; if py::gt(&t111, &t112)? { true } else { false } }) });
    }
    if ({ let t113 = v_method.clone(); Py::Bool({ let t114 = Py::str("webster"); if py::eq(&t113, &t114) { true } else { false } }) }).truthy() {
        return Ok({ let t115 = py::mul(&v_pi.clone(), &py::sub(&py::mul(&Py::Int(2i64), &v_l.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t116 = py::mul(&v_pj.clone(), &py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?)?; if py::gt(&t115, &t116)? { true } else { false } }) });
    }
    if ({ let t117 = v_method.clone(); Py::Bool({ let t118 = Py::str("adams"); if py::eq(&t117, &t118) { true } else { false } }) }).truthy() {
        if ({ let t119 = v_k.clone(); Py::Bool({ let t120 = Py::Int(1i64); if py::eq(&t119, &t120) { true } else { false } }) }).truthy() {
            return Ok({ let t121 = v_l.clone(); Py::Bool({ let t122 = Py::Int(1i64); if py::ne(&t121, &t122) { true } else { false } }) });
        }
        if ({ let t123 = v_l.clone(); Py::Bool({ let t124 = Py::Int(1i64); if py::eq(&t123, &t124) { true } else { false } }) }).truthy() {
            return Ok(Py::Bool(false));
        }
        return Ok({ let t125 = py::mul(&v_pi.clone(), &py::sub(&v_l.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t126 = py::mul(&v_pj.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t125, &t126)? { true } else { false } }) });
    }
    if ({ let t127 = v_k.clone(); Py::Bool({ let t128 = Py::Int(1i64); if py::eq(&t127, &t128) { true } else { false } }) }).truthy() {
        return Ok({ let t129 = v_l.clone(); Py::Bool({ let t130 = Py::Int(1i64); if py::ne(&t129, &t130) { true } else { false } }) });
    }
    if ({ let t131 = v_l.clone(); Py::Bool({ let t132 = Py::Int(1i64); if py::eq(&t131, &t132) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t133 = py::mul(&py::mul(&py::mul(&v_pi.clone(), &v_pi.clone())?, &v_l.clone())?, &py::sub(&v_l.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t134 = py::mul(&py::mul(&py::mul(&v_pj.clone(), &v_pj.clone())?, &v_k.clone())?, &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t133, &t134)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__assign(mut v_r: Py, mut v_names: Py) -> R {
    return Ok({ let mut out = vec![]; for t3454 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t3455 = (t3454).items()?;
if t3455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3455[0].clone();
v_v = t3455[1].clone();
out.push((v_v.clone(), Py::Bool((py::bitand(&py::rshift(&v_r.clone(), &py::sub(&py::sub(&py::len(&v_names.clone())?, &Py::Int(1i64))?, &v_k.clone())?)?, &Py::Int(1i64))?).truthy()))); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bareiss(mut v_M: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_sw: Py = Py::None;
    v_M = { let mut out = vec![]; for t5573 in py::iter(&(v_M.clone()))? { let mut v_row: Py; 
v_row = t5573;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    let t5574 = (Py::tuple(vec![py::len(&v_M.clone())?, Py::Int(1i64), Py::Int(1i64)])).items()?;
    if t5574.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5574[0].clone();
    v_sign = t5574[1].clone();
    v_prev = t5574[2].clone();
    if ({ let t5575 = v_n.clone(); Py::Bool({ let t5576 = Py::Int(0i64); if py::eq(&t5575, &t5576) { true } else { false } }) }).truthy() {
        return Ok(Py::Int(1i64));
    }
    for t5577 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t5577;
        if ({ let t5578 = py::getitem(&py::getitem(&v_M.clone(), &v_k.clone())?, &v_k.clone())?; Py::Bool({ let t5579 = Py::Int(0i64); if py::eq(&t5578, &t5579) { true } else { false } }) }).truthy() {
            v_sw = (match ('g5580: { let mut out = vec![]; for t5581 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5581;
if ({ let t5582 = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?; Py::Bool({ let t5583 = Py::Int(0i64); if py::ne(&t5582, &t5583) { true } else { false } }) }).truthy() { out.push(v_r.clone()); break 'g5580 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
            if ({ let t5584 = v_sw.clone(); Py::Bool({ let t5585 = Py::None; if py::is(&t5584, &t5585) { true } else { false } }) }).truthy() {
                return Ok(Py::Int(0i64));
            }
            let t5586 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_sw.clone())?, py::getitem(&v_M.clone(), &v_k.clone())?])).items()?;
            if t5586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_M.clone(), &v_k.clone(), t5586[0].clone())?;
            py::setitem(&v_M.clone(), &v_sw.clone(), t5586[1].clone())?;
            v_sign = py::neg(&v_sign.clone())?;
        }
        for t5587 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t5587;
            for t5588 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_j = t5588;
                py::setitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone(), py::floordiv(&py::sub(&py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_k.clone())?, &v_k.clone())?)?, &py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_k.clone())?, &v_j.clone())?)?)?, &v_prev.clone())?)?;
            }
        }
        v_prev = py::getitem(&py::getitem(&v_M.clone(), &v_k.clone())?, &v_k.clone())?;
    }
    return Ok(py::mul(&v_sign.clone(), &py::getitem(&py::getitem(&v_M.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bfs(mut v_adj: Py, mut v_s: Py, mut v_desc: Py) -> R {
    let mut v_dist: Py = Py::None;
    let mut v_dq: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2130 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), Py::Int(0i64))]), Py::list(vec![v_s.clone()]), Py::list((Py::list(vec![v_s.clone()])).items()?)])).items()?;
    if t2130.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dist = t2130[0].clone();
    v_order = t2130[1].clone();
    v_dq = t2130[2].clone();
    while (v_dq.clone()).truthy() {
        v_u = py::method(&v_dq.clone(), "popleft", &[])?;
        for t2131 in py::iter(&(py::sorted_key(&py::getitem(&v_adj.clone(), &v_u.clone())?, &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (v_desc.clone()).truthy())?))? {
            v_v = t2131;
            if ({ let t2132 = v_v.clone(); Py::Bool({ let t2133 = v_dist.clone(); if !py::contains(&t2133, &t2132)? { true } else { false } }) }).truthy() {
                py::setitem(&v_dist.clone(), &v_v.clone(), py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &Py::Int(1i64))?)?;
                let _ = py::method(&v_order.clone(), "append", &[v_v.clone()])?;
                let _ = py::method(&v_dq.clone(), "append", &[v_v.clone()])?;
            }
        }
    }
    return Ok(Py::tuple(vec![v_order.clone(), v_dist.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__brute_max(mut v_nodes: Py, mut v_ok: Py) -> R {
    let mut v_go: Py = Py::None;
    let c_best = py::cell_new();
    py::cell_set(&c_best, Py::Int(0i64));
    let rec2461 = py::rec_cell();
    v_go = { let v_nodes = v_nodes.clone(); let v_ok = v_ok.clone(); let c_best = c_best.clone(); let rec2461 = rec2461.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_chosen: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_go: Py = py::rec_get(&rec2461)?;
            let mut v_best: Py = Py::None;
            let mut v_j: Py = Py::None;
            py::cell_set(&c_best, py::minmax(&[py::cell_get(&c_best)?, py::len(&v_chosen.clone())?], true)?);
            if ({ let t2462 = py::sub(&py::add(&py::len(&v_chosen.clone())?, &py::len(&v_nodes.clone())?)?, &v_i.clone())?; Py::Bool({ let t2463 = py::cell_get(&c_best)?; if py::le(&t2462, &t2463)? { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            for t2464 in py::range_iter(&[(v_i.clone()).i()?, (py::len(&v_nodes.clone())?).i()?]) {
                v_j = t2464;
                if (py::call(&v_ok.clone(), &[v_chosen.clone(), py::getitem(&v_nodes.clone(), &v_j.clone())?])?).truthy() {
                    let _ = py::call(&v_go.clone(), &[py::add(&v_j.clone(), &Py::Int(1i64))?, py::add(&v_chosen.clone(), &Py::list(vec![py::getitem(&v_nodes.clone(), &v_j.clone())?]))?])?;
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2461, &v_go);
    let _ = py::call(&v_go.clone(), &[Py::Int(0i64), Py::list(vec![])])?;
    return Ok(py::cell_get(&c_best)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__carmichael_from(mut v_f: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_lam = Py::Int(1i64);
    for t4281 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? {
        let t4288 = (t4281).items()?;
        if t4288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t4288[0].clone();
        v_e = t4288[1].clone();
        v_l = (if ({ let t4282 = v_p.clone(); Py::Bool({ let t4283 = Py::Int(2i64); if py::eq(&t4282, &t4283) { true } else { false } }) }).truthy() { (if ({ let t4284 = v_e.clone(); Py::Bool({ let t4285 = Py::Int(3i64); if py::ge(&t4284, &t4285)? { true } else { false } }) }).truthy() { py::pow(&Py::Int(2i64), &py::sub(&v_e.clone(), &Py::Int(2i64))?)? } else { (if ({ let t4286 = v_e.clone(); Py::Bool({ let t4287 = Py::Int(1i64); if py::eq(&t4286, &t4287) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) }) } else { py::mul(&py::pow(&v_p.clone(), &py::sub(&v_e.clone(), &Py::Int(1i64))?)?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)? });
        v_lam = py::floordiv(&py::mul(&v_lam.clone(), &v_l.clone())?, &py::gcd(&v_lam.clone(), &v_l.clone())?)?;
    }
    return Ok(v_lam.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_cover(mut v_n: Py, mut v_on: Py, mut v_dc: Py, mut v_cubes: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_bit: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_care: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_others: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_care = py::bitor(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?;
    v_pts = { let v_care = v_care.clone(); let v_cubes = v_cubes.clone(); let v_dc = v_dc.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t3983 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3983;
if (f__covers(v_c.clone(), v_m.clone())?).truthy() { out.push(v_m.clone()); }} Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t3984 in py::iter(&(v_cubes.clone()))? {
        v_c = t3984;
        if (py::not(&Py::Bool(({ let mut out = vec![]; for t3985 in py::iter(&(py::call(&v_pts.clone(), &[v_c.clone()])?))? { let mut v_m: Py; 
v_m = t3985;
out.push({ let t3986 = v_m.clone(); Py::Bool({ let t3987 = v_care.clone(); if py::contains(&t3987, &t3986)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
            return Ok(Py::Bool(false));
        }
        for t3988 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_b = t3988;
            v_bit = py::lshift(&Py::Int(1i64), &v_b.clone())?;
            if ({ let t3992 = py::not(&py::bitand(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_bit.clone())?); if !t3992.truthy() { t3992 } else { Py::Bool(({ let mut out = vec![]; for t3989 in py::iter(&(py::call(&v_pts.clone(), &[Py::tuple(vec![py::bitand(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::invert(&v_bit.clone())?)?, py::bitor(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_bit.clone())?])])?))? { let mut v_m: Py; 
v_m = t3989;
out.push({ let t3990 = v_m.clone(); Py::Bool({ let t3991 = v_care.clone(); if py::contains(&t3991, &t3990)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }).truthy() {
                return Ok(Py::Bool(false));
            }
        }
    }
    if (py::sub(&py::set(&v_on.clone())?, &py::set(&{ let mut out = vec![]; for t3993 in py::iter(&(v_cubes.clone()))? { let mut v_c: Py; 
v_c = t3993;
for t3994 in py::iter(&(py::call(&v_pts.clone(), &[v_c.clone()])?))? { let mut v_m: Py; 
v_m = t3994;
out.push(v_m.clone()); }} Py::list(out) })?)?).truthy() {
        return Ok(Py::Bool(false));
    }
    for t3995 in py::iter(&(py::enumerate(&v_cubes.clone(), 0)?))? {
        let t4003 = (t3995).items()?;
        if t4003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4003[0].clone();
        v_c = t4003[1].clone();
        v_others = py::set(&{ let mut out = vec![]; for t3996 in py::iter(&(py::enumerate(&v_cubes.clone(), 0)?))? { let mut v_d: Py; let mut v_j: Py; 
let t3997 = (t3996).items()?;
if t3997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t3997[0].clone();
v_d = t3997[1].clone();
if ({ let t3998 = v_j.clone(); Py::Bool({ let t3999 = v_k.clone(); if py::ne(&t3998, &t3999) { true } else { false } }) }).truthy() { for t4000 in py::iter(&(py::call(&v_pts.clone(), &[v_d.clone()])?))? { let mut v_m: Py; 
v_m = t4000;
out.push(v_m.clone()); }}} Py::list(out) })?;
        if ({ let t4001 = py::bitand(&py::set(&py::call(&v_pts.clone(), &[v_c.clone()])?)?, &py::set(&v_on.clone())?)?; Py::Bool({ let t4002 = v_others.clone(); if py::le(&t4001, &t4002)? { true } else { false } }) }).truthy() {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__colourable(mut v_nodes: Py, mut v_adj: Py, mut v_k: Py) -> R {
    let mut v_col: Py = Py::None;
    let mut v_place: Py = Py::None;
    v_col = Py::dict(vec![]);
    let rec985 = py::rec_cell();
    v_place = { let v_adj = v_adj.clone(); let v_col = v_col.clone(); let v_k = v_k.clone(); let v_nodes = v_nodes.clone(); let rec985 = rec985.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_place: Py = py::rec_get(&rec985)?;
            let mut v_c: Py = Py::None;
            let mut v_used: Py = Py::None;
            let mut v_v: Py = Py::None;
            if ({ let t986 = v_i.clone(); Py::Bool({ let t987 = py::len(&v_nodes.clone())?; if py::eq(&t986, &t987) { true } else { false } }) }).truthy() {
                return Ok(Py::Bool(true));
            }
            v_v = py::getitem(&v_nodes.clone(), &v_i.clone())?;
            v_used = py::set(&{ let mut out = vec![]; for t988 in py::iter(&(py::getitem(&v_adj.clone(), &v_v.clone())?))? { let mut v_u: Py; 
v_u = t988;
if ({ let t989 = v_u.clone(); Py::Bool({ let t990 = v_col.clone(); if py::contains(&t990, &t989)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_col.clone(), &v_u.clone())?); }} Py::list(out) })?;
            for t991 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
                v_c = t991;
                if ({ let t992 = v_c.clone(); Py::Bool({ let t993 = v_used.clone(); if !py::contains(&t993, &t992)? { true } else { false } }) }).truthy() {
                    py::setitem(&v_col.clone(), &v_v.clone(), v_c.clone())?;
                    if (py::call(&v_place.clone(), &[py::add(&v_i.clone(), &Py::Int(1i64))?])?).truthy() {
                        return Ok(Py::Bool(true));
                    }
                    py::delitem(&v_col.clone(), &v_v.clone())?;
                }
            }
            return Ok(Py::Bool(false));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec985, &v_place);
    return Ok((if (py::call(&v_place.clone(), &[Py::Int(0i64)])?).truthy() { py::dict_from(&v_col.clone(), vec![])? } else { Py::None }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__components(mut v_adj: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_comps: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t5221 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, Py::list(vec![])])).items()?;
    if t5221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_seen = t5221[0].clone();
    v_comps = t5221[1].clone();
    for t5222 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_s = t5222;
        if ({ let t5223 = v_s.clone(); Py::Bool({ let t5224 = v_seen.clone(); if py::contains(&t5224, &t5223)? { true } else { false } }) }).truthy() {
            continue;
        }
        let t5225 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![v_s.clone()])])).items()?;
        if t5225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_comp = t5225[0].clone();
        v_stack = t5225[1].clone();
        let _ = py::method(&v_seen.clone(), "add", &[v_s.clone()])?;
        while (v_stack.clone()).truthy() {
            v_u = py::method(&v_stack.clone(), "pop", &[])?;
            let _ = py::method(&v_comp.clone(), "append", &[v_u.clone()])?;
            for t5226 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t5226;
                if ({ let t5227 = v_v.clone(); Py::Bool({ let t5228 = v_seen.clone(); if !py::contains(&t5228, &t5227)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[v_v.clone()])?;
                    let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
                }
            }
        }
        let _ = py::method(&v_comps.clone(), "append", &[f__sorted(v_comp.clone())?])?;
    }
    return Ok(v_comps.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__convergent(mut v_terms: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_h1: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let t4670 = (Py::tuple(vec![Py::Int(1i64), py::getitem(&v_terms.clone(), &Py::Int(0i64))?, Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t4670.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t4670[0].clone();
    v_h1 = t4670[1].clone();
    v_k0 = t4670[2].clone();
    v_k1 = t4670[3].clone();
    for t4671 in py::iter(&(py::slice(&v_terms.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_a = t4671;
        let t4672 = (Py::tuple(vec![v_h1.clone(), py::add(&py::mul(&v_a.clone(), &v_h1.clone())?, &v_h0.clone())?])).items()?;
        if t4672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h0 = t4672[0].clone();
        v_h1 = t4672[1].clone();
        let t4673 = (Py::tuple(vec![v_k1.clone(), py::add(&py::mul(&v_a.clone(), &v_k1.clone())?, &v_k0.clone())?])).items()?;
        if t4673.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k0 = t4673[0].clone();
        v_k1 = t4673[1].clone();
    }
    return Ok(Py::tuple(vec![v_h1.clone(), v_k1.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__covers(mut v_p: Py, mut v_m: Py) -> R {
    return Ok({ let t3918 = py::bitand(&v_m.clone(), &py::invert(&py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t3919 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; if py::eq(&t3918, &t3919) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cube_of(mut v_lits_text: Py, mut v_names: Py) -> R {
    let mut v_bit: Py = Py::None;
    let mut v_fixed: Py = Py::None;
    let mut v_lit: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_neg: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t3978 = (Py::tuple(vec![py::len(&v_names.clone())?, Py::Int(0i64), Py::Int(0i64)])).items()?;
    if t3978.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3978[0].clone();
    v_val = t3978[1].clone();
    v_fixed = t3978[2].clone();
    for t3980 in py::iter(&({ let mut out = vec![]; for t3979 in py::iter(&(py::method(&v_lits_text.clone(), "split", &[Py::str("&")])?))? { let mut v_x: Py; 
v_x = t3979;
out.push(py::method(&v_x.clone(), "strip", &[])?); } Py::list(out) }))? {
        v_lit = t3980;
        v_neg = py::method(&v_lit.clone(), "startswith", &[Py::str("~")])?;
        v_v = py::method(&v_lit.clone(), "lstrip", &[Py::str("~")])?;
        v_bit = py::lshift(&Py::Int(1i64), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::method(&v_names.clone(), "index", &[v_v.clone()])?)?)?;
        if (py::bitand(&v_fixed.clone(), &v_bit.clone())?).truthy() {
            return Ok(Py::None);
        }
        v_fixed = py::bitor(&v_fixed, &v_bit.clone())?;
        if (py::not(&v_neg.clone())).truthy() {
            v_val = py::bitor(&v_val, &v_bit.clone())?;
        }
    }
    return Ok(Py::tuple(vec![v_val.clone(), py::bitand(&py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?, &py::invert(&v_fixed.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__decode(mut v_seq: Py, mut v_n: Py, mut v_base: Py) -> R {
    let mut v_deg: Py = Py::None;
    let mut v_leaf: Py = Py::None;
    let mut v_leaves: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_deg = { let mut out = vec![]; for t5254 in py::range_iter(&[(v_base.clone()).i()?, (py::add(&v_n.clone(), &v_base.clone())?).i()?]) { let mut v_v: Py; 
v_v = t5254;
out.push((v_v.clone(), Py::Int(1i64))); } Py::dict(out) };
    for t5255 in py::iter(&(v_seq.clone()))? {
        v_s = t5255;
        let t5256 = v_deg.clone(); let t5257 = v_s.clone();
        py::setitem(&t5256, &t5257, py::iadd(&py::getitem(&t5256, &t5257)?, &Py::Int(1i64))?)?;
    }
    v_leaves = { let mut out = vec![]; for t5258 in py::iter(&(v_deg.clone()))? { let mut v_v: Py; 
v_v = t5258;
if ({ let t5259 = py::getitem(&v_deg.clone(), &v_v.clone())?; Py::Bool({ let t5260 = Py::Int(1i64); if py::eq(&t5259, &t5260) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    let _ = py::heapq("heapify", &[v_leaves.clone()])?;
    v_out = Py::list(vec![]);
    for t5261 in py::iter(&(v_seq.clone()))? {
        v_s = t5261;
        v_leaf = py::heapq("heappop", &[v_leaves.clone()])?;
        let _ = py::method(&v_out.clone(), "append", &[py::sorted(&Py::tuple(vec![v_leaf.clone(), v_s.clone()]), (Py::Bool(false)).truthy())?])?;
        let t5262 = v_deg.clone(); let t5263 = v_s.clone();
        py::setitem(&t5262, &t5263, py::sub(&py::getitem(&t5262, &t5263)?, &Py::Int(1i64))?)?;
        if ({ let t5264 = py::getitem(&v_deg.clone(), &v_s.clone())?; Py::Bool({ let t5265 = Py::Int(1i64); if py::eq(&t5264, &t5265) { true } else { false } }) }).truthy() {
            let _ = py::heapq("heappush", &[v_leaves.clone(), v_s.clone()])?;
        }
    }
    let _ = py::method(&v_out.clone(), "append", &[py::sorted(&py::nsmallest(&Py::Int(2i64), &v_leaves.clone(), None, false)?, (Py::Bool(false)).truthy())?])?;
    return Ok(py::sorted(&v_out.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__divisors_from(mut v_f: Py) -> R {
    let mut v_ds: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_ds = Py::list(vec![Py::Int(1i64)]);
    for t6073 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? {
        let t6076 = (t6073).items()?;
        if t6076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t6076[0].clone();
        v_e = t6076[1].clone();
        v_ds = { let mut out = vec![]; for t6074 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t6074;
for t6075 in py::range_iter(&[(py::add(&v_e.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t6075;
out.push(py::mul(&v_d.clone(), &py::pow(&v_p.clone(), &v_k.clone())?)?); }} Py::list(out) };
    }
    return Ok(py::sorted(&v_ds.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__euclid(mut v_a: Py, mut v_b: Py) -> R {
    while (v_b.clone()).truthy() {
        let t1953 = (Py::tuple(vec![v_b.clone(), py::modulo(&v_a.clone(), &v_b.clone())?])).items()?;
        if t1953.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1953[0].clone();
        v_b = t1953[1].clone();
    }
    return Ok(py::abs(&v_a.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ev(mut v_t: Py, mut v_env: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_op: Py = Py::None;
    v_op = py::getitem(&v_t.clone(), &Py::Int(0i64))?;
    if ({ let t6008 = v_op.clone(); Py::Bool({ let t6009 = Py::str("VAR"); if py::eq(&t6008, &t6009) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_env.clone(), &py::getitem(&v_t.clone(), &Py::Int(1i64))?)?);
    }
    if ({ let t6010 = v_op.clone(); Py::Bool({ let t6011 = Py::str("CONST"); if py::eq(&t6010, &t6011) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_t.clone(), &Py::Int(1i64))?);
    }
    if ({ let t6012 = v_op.clone(); Py::Bool({ let t6013 = Py::str("NOT"); if py::eq(&t6012, &t6013) { true } else { false } }) }).truthy() {
        return Ok(py::not(&f__ev(py::getitem(&v_t.clone(), &Py::Int(1i64))?, v_env.clone())?));
    }
    let t6014 = (Py::tuple(vec![f__ev(py::getitem(&v_t.clone(), &Py::Int(1i64))?, v_env.clone())?, f__ev(py::getitem(&v_t.clone(), &Py::Int(2i64))?, v_env.clone())?])).items()?;
    if t6014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6014[0].clone();
    v_b = t6014[1].clone();
    return Ok(py::getitem(&Py::dict(vec![(Py::str("AND"), { let t6015 = v_a.clone(); if !t6015.truthy() { t6015 } else { v_b.clone() } }), (Py::str("OR"), { let t6016 = v_a.clone(); if t6016.truthy() { t6016 } else { v_b.clone() } }), (Py::str("XOR"), { let t6017 = v_a.clone(); Py::Bool({ let t6018 = v_b.clone(); if py::ne(&t6017, &t6018) { true } else { false } }) }), (Py::str("IMP"), { let t6019 = py::not(&v_a.clone()); if t6019.truthy() { t6019 } else { v_b.clone() } }), (Py::str("IFF"), { let t6020 = v_a.clone(); Py::Bool({ let t6021 = v_b.clone(); if py::eq(&t6020, &t6021) { true } else { false } }) })]), &v_op.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ext(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_old_r: Py = Py::None;
    let mut v_old_s: Py = Py::None;
    let mut v_old_t: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t5464 = (Py::tuple(vec![v_a.clone(), v_b.clone(), Py::Int(1i64), Py::Int(0i64), Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t5464.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_old_r = t5464[0].clone();
    v_r = t5464[1].clone();
    v_old_s = t5464[2].clone();
    v_s = t5464[3].clone();
    v_old_t = t5464[4].clone();
    v_t = t5464[5].clone();
    while (v_r.clone()).truthy() {
        v_q = py::floordiv(&v_old_r.clone(), &v_r.clone())?;
        let t5465 = (Py::tuple(vec![v_r.clone(), py::sub(&v_old_r.clone(), &py::mul(&v_q.clone(), &v_r.clone())?)?])).items()?;
        if t5465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_old_r = t5465[0].clone();
        v_r = t5465[1].clone();
        let t5466 = (Py::tuple(vec![v_s.clone(), py::sub(&v_old_s.clone(), &py::mul(&v_q.clone(), &v_s.clone())?)?])).items()?;
        if t5466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_old_s = t5466[0].clone();
        v_s = t5466[1].clone();
        let t5467 = (Py::tuple(vec![v_t.clone(), py::sub(&v_old_t.clone(), &py::mul(&v_q.clone(), &v_t.clone())?)?])).items()?;
        if t5467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_old_t = t5467[0].clone();
        v_t = t5467[1].clone();
    }
    return Ok(Py::tuple(vec![v_old_r.clone(), v_old_s.clone(), v_old_t.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__factor(mut v_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_stack: Py = Py::None;
    v_out = Py::dict(vec![]);
    for t6077 in py::iter(&(Py::tuple(vec![Py::Int(2i64), Py::Int(3i64), Py::Int(5i64), Py::Int(7i64), Py::Int(11i64), Py::Int(13i64), Py::Int(17i64), Py::Int(19i64), Py::Int(23i64), Py::Int(29i64), Py::Int(31i64), Py::Int(37i64), Py::Int(41i64), Py::Int(43i64), Py::Int(47i64)])))? {
        v_p = t6077;
        while ({ let t6078 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t6079 = Py::Int(0i64); if py::eq(&t6078, &t6079) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &v_p.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_p.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
            v_n = py::floordiv(&v_n, &v_p.clone())?;
        }
    }
    v_stack = (if ({ let t6080 = v_n.clone(); Py::Bool({ let t6081 = Py::Int(1i64); if py::gt(&t6080, &t6081)? { true } else { false } }) }).truthy() { Py::list(vec![v_n.clone()]) } else { Py::list(vec![]) });
    while (v_stack.clone()).truthy() {
        v_m = py::method(&v_stack.clone(), "pop", &[])?;
        if ({ let t6082 = v_m.clone(); Py::Bool({ let t6083 = Py::Int(1i64); if py::eq(&t6082, &t6083) { true } else { false } }) }).truthy() {
            continue;
        }
        if (f__is_prime(v_m.clone())?).truthy() {
            py::setitem(&v_out.clone(), &v_m.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_m.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
            continue;
        }
        v_r = py::isqrt(&v_m.clone())?;
        if ({ let t6106 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t6107 = v_m.clone(); if py::eq(&t6106, &t6107) { true } else { false } }) }).truthy() {
            v_stack = py::iadd(&v_stack, &Py::list(vec![v_r.clone(), v_r.clone()]))?;
            continue;
        }
        v_d = f__rho(v_m.clone())?;
        v_stack = py::iadd(&v_stack, &Py::list(vec![v_d.clone(), py::floordiv(&v_m.clone(), &v_d.clone())?]))?;
    }
    return Ok(py::dict_from(&py::sorted(&py::method(&v_out.clone(), "items", &[])?, (Py::Bool(false)).truthy())?, vec![])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fib_pair(mut v_n: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    if ({ let t3501 = v_n.clone(); Py::Bool({ let t3502 = Py::Int(0i64); if py::eq(&t3501, &t3502) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]));
    }
    let t3503 = (f__fib_pair(py::rshift(&v_n.clone(), &Py::Int(1i64))?)?).items()?;
    if t3503.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3503[0].clone();
    v_b = t3503[1].clone();
    v_c = py::mul(&v_a.clone(), &py::sub(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_a.clone())?)?;
    v_d = py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?;
    return Ok((if (py::bitand(&v_n.clone(), &Py::Int(1i64))?).truthy() { Py::tuple(vec![v_d.clone(), py::add(&v_c.clone(), &v_d.clone())?]) } else { Py::tuple(vec![v_c.clone(), v_d.clone()]) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__formula_fn(mut v_formula: Py) -> R {
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3774 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t3774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3774[0].clone();
    v_names = t3774[1].clone();
    if ({ let t3775 = py::len(&v_names.clone())?; Py::Bool({ let t3776 = Py::Int(6i64); if py::gt(&t3775, &t3776)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 6 variables for minimization"))));
    }
    return Ok(Py::tuple(vec![py::len(&v_names.clone())?, { let mut out = vec![]; for t3777 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3778 = (t3777).items()?;
if t3778.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3778[0].clone();
v_v = t3778[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); }} Py::list(out) }, v_names.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__girth(mut v_adj: Py, mut v_odd: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_best = Py::None;
    for t1988 in py::iter(&(v_adj.clone()))? {
        v_s = t1988;
        let t1989 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), Py::Int(0i64))]), Py::dict(vec![(v_s.clone(), Py::None)]), Py::list(vec![v_s.clone()])])).items()?;
        if t1989.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_dist = t1989[0].clone();
        v_parent = t1989[1].clone();
        v_q = t1989[2].clone();
        for t1990 in py::iter(&(v_q.clone()))? {
            v_u = t1990;
            for t1991 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t1991;
                if ({ let t1992 = v_v.clone(); Py::Bool({ let t1993 = v_dist.clone(); if !py::contains(&t1993, &t1992)? { true } else { false } }) }).truthy() {
                    let t1994 = (Py::tuple(vec![py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &Py::Int(1i64))?, v_u.clone()])).items()?;
                    if t1994.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&v_dist.clone(), &v_v.clone(), t1994[0].clone())?;
                    py::setitem(&v_parent.clone(), &v_v.clone(), t1994[1].clone())?;
                    let _ = py::method(&v_q.clone(), "append", &[v_v.clone()])?;
                } else {
                    if ({ let t2000 = { let t1995 = py::getitem(&v_parent.clone(), &v_u.clone())?; Py::Bool({ let t1996 = v_v.clone(); if py::ne(&t1995, &t1996) { true } else { false } }) }; if !t2000.truthy() { t2000 } else { { let t1999 = v_odd.clone(); if t1999.truthy() { t1999 } else { { let t1997 = py::getitem(&v_dist.clone(), &v_v.clone())?; Py::Bool({ let t1998 = py::getitem(&v_dist.clone(), &v_u.clone())?; if py::ne(&t1997, &t1998) { true } else { false } }) } } } } }).truthy() {
                        v_L = py::add(&py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &py::getitem(&v_dist.clone(), &v_v.clone())?)?, &Py::Int(1i64))?;
                        if ({ let t2005 = { let t2001 = v_best.clone(); Py::Bool({ let t2002 = Py::None; if py::is(&t2001, &t2002) { true } else { false } }) }; if t2005.truthy() { t2005 } else { { let t2003 = v_L.clone(); Py::Bool({ let t2004 = v_best.clone(); if py::lt(&t2003, &t2004)? { true } else { false } }) } } }).truthy() {
                            v_best = v_L.clone();
                        }
                    }
                }
            }
        }
    }
    return Ok(v_best.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__graph(mut v_edges: Py, mut v_directed: Py, mut v_weighted: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t6516 = py::not(&v_weighted.clone()); if !t6516.truthy() { t6516 } else { Py::Bool(({ let mut out = vec![]; for t6513 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t6513;
out.push({ let t6514 = py::len(&v_e.clone())?; Py::Bool({ let t6515 = Py::Int(3i64); if py::eq(&t6514, &t6515) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability takes an unweighted graph: give edges without weights"))));
    }
    v_adj = Py::dict(vec![]);
    for t6517 in py::iter(&(v_edges.clone()))? {
        v_e = t6517;
        let t6518 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)), "strip", &[])?])).items()?;
        if t6518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t6518[0].clone();
        v_v = t6518[1].clone();
        if ({ let t6519 = v_u.clone(); Py::Bool({ let t6520 = v_v.clone(); if py::eq(&t6519, &t6520) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["self-loop at ".to_string(), py::format(&v_u.clone(), "")?, ": simple graphs only".to_string()].concat()))));
        }
        let _ = py::method(&v_adj.clone(), "setdefault", &[v_u.clone(), Py::dict(vec![])])?;
        let _ = py::method(&v_adj.clone(), "setdefault", &[v_v.clone(), Py::dict(vec![])])?;
        if ({ let t6526 = { let t6521 = v_v.clone(); Py::Bool({ let t6522 = py::getitem(&v_adj.clone(), &v_u.clone())?; if py::contains(&t6522, &t6521)? { true } else { false } }) }; if t6526.truthy() { t6526 } else { { let t6525 = py::not(&v_directed.clone()); if !t6525.truthy() { t6525 } else { { let t6523 = v_u.clone(); Py::Bool({ let t6524 = py::getitem(&v_adj.clone(), &v_v.clone())?; if py::contains(&t6524, &t6523)? { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["the edge ".to_string(), py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&v_v.clone(), "")?, " is given twice".to_string()].concat()))));
        }
        py::setitem(&py::getitem(&v_adj.clone(), &v_u.clone())?, &v_v.clone(), (if ({ let t6527 = py::len(&v_e.clone())?; Py::Bool({ let t6528 = Py::Int(3i64); if py::eq(&t6527, &t6528) { true } else { false } }) }).truthy() { py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)? } else { Py::Float(1.0) }))?;
        if (py::not(&v_directed.clone())).truthy() {
            py::setitem(&py::getitem(&v_adj.clone(), &v_v.clone())?, &v_u.clone(), py::getitem(&py::getitem(&v_adj.clone(), &v_u.clone())?, &v_v.clone())?)?;
        }
    }
    if (py::not(&v_adj.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph has no edges"))));
    }
    if ({ let t6529 = py::len(&v_adj.clone())?; Py::Bool({ let t6530 = Py::Int(400i64); if py::gt(&t6529, &t6530)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 400 vertices"))));
    }
    return Ok(v_adj.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inv(mut v_a: Py, mut v_m: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t5468 = (f__ext(py::modulo(&v_a.clone(), &v_m.clone())?, v_m.clone())?).items()?;
    if t5468.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t5468[0].clone();
    v_x = t5468[1].clone();
    v__ = t5468[2].clone();
    if ({ let t5469 = v_g.clone(); Py::Bool({ let t5470 = Py::Int(1i64); if py::ne(&t5469, &t5470) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_a.clone(), "")?, " has no inverse modulo ".to_string(), py::format(&v_m.clone(), "")?, " (gcd is ".to_string(), py::format(&v_g.clone(), "")?, ")".to_string()].concat()))));
    }
    return Ok(py::modulo(&v_x.clone(), &v_m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__is_prime(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t6084 = v_n.clone(); Py::Bool({ let t6085 = Py::Int(2i64); if py::lt(&t6084, &t6085)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    for t6086 in py::iter(&(c__SMALL()?))? {
        v_p = t6086;
        if ({ let t6087 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t6088 = Py::Int(0i64); if py::eq(&t6087, &t6088) { true } else { false } }) }).truthy() {
            return Ok({ let t6089 = v_n.clone(); Py::Bool({ let t6090 = v_p.clone(); if py::eq(&t6089, &t6090) { true } else { false } }) });
        }
    }
    let t6091 = (Py::tuple(vec![py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
    if t6091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t6091[0].clone();
    v_s = t6091[1].clone();
    while ({ let t6092 = py::modulo(&v_d.clone(), &Py::Int(2i64))?; Py::Bool({ let t6093 = Py::Int(0i64); if py::eq(&t6092, &t6093) { true } else { false } }) }).truthy() {
        v_d = py::floordiv(&v_d, &Py::Int(2i64))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    for t6094 in py::iter(&(c__MR_BASES()?))? {
        v_a = t6094;
        v_a = py::modulo(&v_a, &v_n.clone())?;
        if ({ let t6095 = v_a.clone(); Py::Bool({ let t6096 = Py::Int(0i64); if py::eq(&t6095, &t6096) { true } else { false } }) }).truthy() {
            continue;
        }
        v_x = py::pow_mod(&v_a.clone(), &v_d.clone(), &v_n.clone())?;
        if ({ let t6101 = { let t6097 = v_x.clone(); Py::Bool({ let t6098 = Py::Int(1i64); if py::eq(&t6097, &t6098) { true } else { false } }) }; if t6101.truthy() { t6101 } else { { let t6099 = v_x.clone(); Py::Bool({ let t6100 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t6099, &t6100) { true } else { false } }) } } }).truthy() {
            continue;
        }
        let mut t6102 = false;
        for t6103 in py::range_iter(&[(py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
            v__ = t6103;
            v_x = py::modulo(&py::mul(&v_x.clone(), &v_x.clone())?, &v_n.clone())?;
            if ({ let t6104 = v_x.clone(); Py::Bool({ let t6105 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t6104, &t6105) { true } else { false } }) }).truthy() {
                { t6102 = true; break; }
            }
        }
        if !t6102 {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__is_prime_alt(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t4944 = v_n.clone(); Py::Bool({ let t4945 = Py::Int(2i64); if py::lt(&t4944, &t4945)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    for t4946 in py::iter(&(Py::tuple(vec![Py::Int(2i64), Py::Int(3i64), Py::Int(5i64), Py::Int(7i64), Py::Int(11i64), Py::Int(13i64), Py::Int(17i64), Py::Int(19i64), Py::Int(23i64), Py::Int(29i64), Py::Int(31i64), Py::Int(37i64), Py::Int(41i64), Py::Int(43i64), Py::Int(47i64)])))? {
        v_p = t4946;
        if ({ let t4947 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t4948 = Py::Int(0i64); if py::eq(&t4947, &t4948) { true } else { false } }) }).truthy() {
            return Ok({ let t4949 = v_n.clone(); Py::Bool({ let t4950 = v_p.clone(); if py::eq(&t4949, &t4950) { true } else { false } }) });
        }
    }
    let t4951 = (Py::tuple(vec![py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
    if t4951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4951[0].clone();
    v_s = t4951[1].clone();
    while (py::not(&py::bitand(&v_d.clone(), &Py::Int(1i64))?)).truthy() {
        v_d = py::rshift(&v_d, &Py::Int(1i64))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    for t4952 in py::iter(&(c__SMALL()?))? {
        v_a = t4952;
        v_x = py::pow_mod(&v_a.clone(), &v_d.clone(), &v_n.clone())?;
        if ({ let t4953 = v_x.clone(); Py::Bool({ let t4954 = Py::tuple(vec![Py::Int(1i64), py::sub(&v_n.clone(), &Py::Int(1i64))?]); if py::contains(&t4954, &t4953)? { true } else { false } }) }).truthy() {
            continue;
        }
        let mut t4955 = false;
        for t4956 in py::range_iter(&[(py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
            v__ = t4956;
            v_x = py::pow_mod(&v_x.clone(), &Py::Int(2i64), &v_n.clone())?;
            if ({ let t4957 = v_x.clone(); Py::Bool({ let t4958 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t4957, &t4958) { true } else { false } }) }).truthy() {
                { t4955 = true; break; }
            }
        }
        if !t4955 {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__iv(mut v_x: Py) -> R {
    return Ok({ let t6579 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t6579.truthy() { t6579 } else { py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jacobi(mut v_a: Py, mut v_n: Py) -> R {
    let mut v_result: Py = Py::None;
    v_a = py::modulo(&v_a, &v_n.clone())?;
    v_result = Py::Int(1i64);
    while (v_a.clone()).truthy() {
        while ({ let t2874 = py::modulo(&v_a.clone(), &Py::Int(2i64))?; Py::Bool({ let t2875 = Py::Int(0i64); if py::eq(&t2874, &t2875) { true } else { false } }) }).truthy() {
            v_a = py::floordiv(&v_a, &Py::Int(2i64))?;
            if ({ let t2872 = py::modulo(&v_n.clone(), &Py::Int(8i64))?; Py::Bool({ let t2873 = Py::tuple(vec![Py::Int(3i64), Py::Int(5i64)]); if py::contains(&t2873, &t2872)? { true } else { false } }) }).truthy() {
                v_result = py::neg(&v_result.clone())?;
            }
        }
        let t2876 = (Py::tuple(vec![v_n.clone(), v_a.clone()])).items()?;
        if t2876.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2876[0].clone();
        v_n = t2876[1].clone();
        if ({ let t2881 = { let t2877 = py::modulo(&v_a.clone(), &Py::Int(4i64))?; Py::Bool({ let t2878 = Py::Int(3i64); if py::eq(&t2877, &t2878) { true } else { false } }) }; if !t2881.truthy() { t2881 } else { { let t2879 = py::modulo(&v_n.clone(), &Py::Int(4i64))?; Py::Bool({ let t2880 = Py::Int(3i64); if py::eq(&t2879, &t2880) { true } else { false } }) } } }).truthy() {
            v_result = py::neg(&v_result.clone())?;
        }
        v_a = py::modulo(&v_a, &v_n.clone())?;
    }
    return Ok((if ({ let t2882 = v_n.clone(); Py::Bool({ let t2883 = Py::Int(1i64); if py::eq(&t2882, &t2883) { true } else { false } }) }).truthy() { v_result.clone() } else { Py::Int(0i64) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__laplacian_minor(mut v_adj: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_nodes = f__sorted(v_adj.clone())?;
    v_idx = { let mut out = vec![]; for t5546 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t5547 = (t5546).items()?;
if t5547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5547[0].clone();
v_v = t5547[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_n = py::len(&v_nodes.clone())?;
    v_L = { let mut out = vec![]; for t5548 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t5548;
out.push(py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?); } Py::list(out) };
    for t5549 in py::iter(&(v_adj.clone()))? {
        v_u = t5549;
        for t5550 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5550;
            py::setitem(&py::getitem(&v_L.clone(), &py::getitem(&v_idx.clone(), &v_u.clone())?)?, &py::getitem(&v_idx.clone(), &v_v.clone())?, py::neg(&Py::Int(1i64))?)?;
        }
        py::setitem(&py::getitem(&v_L.clone(), &py::getitem(&v_idx.clone(), &v_u.clone())?)?, &py::getitem(&v_idx.clone(), &v_u.clone())?, py::len(&py::getitem(&v_adj.clone(), &v_u.clone())?)?)?;
    }
    return Ok({ let mut out = vec![]; for t5551 in py::iter(&(py::slice(&v_L.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_row: Py; 
v_row = t5551;
out.push(py::slice(&v_row.clone(), Some((Py::Int(1i64)).i()?), None, None)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__letters(mut v_text: Py) -> R {
    return Ok({ let mut out = vec![]; for t6343 in py::iter(&(py::method(&Py::str(&py::to_str(&v_text.clone())), "upper", &[])?))? { let mut v_ch: Py; 
v_ch = t6343;
if ({ let t6344 = Py::str("A"); Py::Bool({ let t6345 = v_ch.clone(); if py::le(&t6344, &t6345)? { { let t6346 = Py::str("Z"); if py::le(&t6345, &t6346)? { true } else { false } } } else { false } }) }).truthy() { out.push(py::sub(&py::ord(&v_ch.clone())?, &Py::Int(65i64))?); }} Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lex(mut v_text: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_s = Py::str(&py::to_str(&v_text.clone()));
    if ({ let t5956 = py::len(&v_s.clone())?; Py::Bool({ let t5957 = Py::Int(400i64); if py::gt(&t5956, &t5957)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the formula is too long (at most 400 characters)"))));
    }
    let t5958 = (Py::tuple(vec![Py::Int(0i64), Py::list(vec![])])).items()?;
    if t5958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t5958[0].clone();
    v_out = t5958[1].clone();
    while ({ let t5965 = v_pos.clone(); Py::Bool({ let t5966 = py::len(&v_s.clone())?; if py::lt(&t5965, &t5966)? { true } else { false } }) }).truthy() {
        if ({ let t5959 = py::method(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), None, None)?, "strip", &[])?; Py::Bool({ let t5960 = Py::str(""); if py::eq(&t5959, &t5960) { true } else { false } }) }).truthy() {
            break;
        }
        v_m = py::method(&c__LTOK()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if (py::not(&v_m.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the formula at '".to_string(), py::format(&py::method(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(10i64))?).i()?), None)?, "strip", &[])?, "")?, "'".to_string()].concat()))));
        }
        v_t = py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?;
        v_pos = py::method(&v_m.clone(), "end", &[])?;
        if ({ let t5961 = v_t.clone(); Py::Bool({ let t5962 = c__LSYM()?; if py::contains(&t5962, &t5961)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::getitem(&c__LSYM()?, &v_t.clone())?])?;
        } else {
            if ({ let t5963 = py::method(&v_t.clone(), "lower", &[])?; Py::Bool({ let t5964 = c__LWORD()?; if py::contains(&t5964, &t5963)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_out.clone(), "append", &[py::getitem(&c__LWORD()?, &py::method(&v_t.clone(), "lower", &[])?)?])?;
            } else {
                let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![Py::str("VAR"), v_t.clone()])])?;
            }
        }
    }
    if (py::not(&v_out.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the formula is empty"))));
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lowlink(mut v_adj: Py) -> R {
    let mut v_aps: Py = Py::None;
    let mut v_brs: Py = Py::None;
    let mut v_dfs: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_low: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = { let _ = py::minmax(&[Py::Int(1000), py::add(&py::mul(&Py::Int(4i64), &py::len(&v_adj.clone())?)?, &Py::Int(100i64))?], true)?; Py::None };
    let t617 = (Py::tuple(vec![Py::dict(vec![]), Py::dict(vec![]), py::set(&Py::list(vec![]))?, Py::list(vec![]), Py::list(vec![Py::Int(0i64)])])).items()?;
    if t617.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_disc = t617[0].clone();
    v_low = t617[1].clone();
    v_aps = t617[2].clone();
    v_brs = t617[3].clone();
    v_t = t617[4].clone();
    let rec618 = py::rec_cell();
    v_dfs = { let v_adj = v_adj.clone(); let v_aps = v_aps.clone(); let v_brs = v_brs.clone(); let v_disc = v_disc.clone(); let v_low = v_low.clone(); let v_t = v_t.clone(); let rec618 = rec618.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_parent: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_dfs: Py = py::rec_get(&rec618)?;
            let mut v_children: Py = Py::None;
            let mut v_v: Py = Py::None;
            let t619 = py::getitem(&v_t.clone(), &Py::Int(0i64))?;
            py::setitem(&v_disc.clone(), &v_u.clone(), t619.clone())?;
            py::setitem(&v_low.clone(), &v_u.clone(), t619.clone())?;
            let t620 = v_t.clone(); let t621 = Py::Int(0i64);
            py::setitem(&t620, &t621, py::iadd(&py::getitem(&t620, &t621)?, &Py::Int(1i64))?)?;
            v_children = Py::Int(0i64);
            for t622 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &v_u.clone())?)?))? {
                v_v = t622;
                if ({ let t623 = v_v.clone(); Py::Bool({ let t624 = v_disc.clone(); if !py::contains(&t624, &t623)? { true } else { false } }) }).truthy() {
                    v_children = py::iadd(&v_children, &Py::Int(1i64))?;
                    let _ = py::call(&v_dfs.clone(), &[v_v.clone(), v_u.clone()])?;
                    py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_low.clone(), &v_v.clone())?], false)?)?;
                    if ({ let t629 = { let t625 = v_parent.clone(); Py::Bool({ let t626 = Py::None; if !py::is(&t625, &t626) { true } else { false } }) }; if !t629.truthy() { t629 } else { { let t627 = py::getitem(&v_low.clone(), &v_v.clone())?; Py::Bool({ let t628 = py::getitem(&v_disc.clone(), &v_u.clone())?; if py::ge(&t627, &t628)? { true } else { false } }) } } }).truthy() {
                        let _ = py::method(&v_aps.clone(), "add", &[v_u.clone()])?;
                    }
                    if ({ let t630 = py::getitem(&v_low.clone(), &v_v.clone())?; Py::Bool({ let t631 = py::getitem(&v_disc.clone(), &v_u.clone())?; if py::gt(&t630, &t631)? { true } else { false } }) }).truthy() {
                        let _ = py::method(&v_brs.clone(), "append", &[py::sorted_key(&Py::tuple(vec![v_u.clone(), v_v.clone()]), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?])?;
                    }
                } else {
                    if ({ let t632 = v_v.clone(); Py::Bool({ let t633 = v_parent.clone(); if py::ne(&t632, &t633) { true } else { false } }) }).truthy() {
                        py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_disc.clone(), &v_v.clone())?], false)?)?;
                    }
                }
            }
            if ({ let t638 = { let t634 = v_parent.clone(); Py::Bool({ let t635 = Py::None; if py::is(&t634, &t635) { true } else { false } }) }; if !t638.truthy() { t638 } else { { let t636 = v_children.clone(); Py::Bool({ let t637 = Py::Int(1i64); if py::gt(&t636, &t637)? { true } else { false } }) } } }).truthy() {
                let _ = py::method(&v_aps.clone(), "add", &[v_u.clone()])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec618, &v_dfs);
    for t639 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_s = t639;
        if ({ let t640 = v_s.clone(); Py::Bool({ let t641 = v_disc.clone(); if !py::contains(&t641, &t640)? { true } else { false } }) }).truthy() {
            let _ = py::call(&v_dfs.clone(), &[v_s.clone(), Py::None])?;
        }
    }
    return Ok(Py::tuple(vec![f__sorted(v_aps.clone())?, py::sorted_key(&v_brs.clone(), &{ let v_adj = v_adj.clone(); let v_aps = v_aps.clone(); let v_brs = v_brs.clone(); let v_dfs = v_dfs.clone(); let v_disc = v_disc.clone(); let v_low = v_low.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![f__nkey(py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, f__nkey(py::getitem(&v_e.clone(), &Py::Int(1i64))?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__max_clique(mut v_adj: Py) -> R {
    let mut v_expand: Py = Py::None;
    let c_best = py::cell_new();
    py::cell_set(&c_best, Py::list(vec![]));
    let rec2430 = py::rec_cell();
    v_expand = { let v_adj = v_adj.clone(); let c_best = c_best.clone(); let rec2430 = rec2430.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_R: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_P: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_X: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_expand: Py = py::rec_get(&rec2430)?;
            let mut v_best: Py = Py::None;
            let mut v_pivot: Py = Py::None;
            let mut v_v: Py = Py::None;
            let c_P = py::cell_new();
            py::cell_set(&c_P, v_P.clone());
            if ({ let t2431 = py::not(&py::cell_get(&c_P)?); if !t2431.truthy() { t2431 } else { py::not(&v_X.clone()) } }).truthy() {
                if ({ let t2441 = { let t2432 = py::len(&v_R.clone())?; Py::Bool({ let t2433 = py::len(&py::cell_get(&c_best)?)?; if py::gt(&t2432, &t2433)? { true } else { false } }) }; if t2441.truthy() { t2441 } else { { let t2440 = { let t2434 = py::len(&v_R.clone())?; Py::Bool({ let t2435 = py::len(&py::cell_get(&c_best)?)?; if py::eq(&t2434, &t2435) { true } else { false } }) }; if !t2440.truthy() { t2440 } else { { let t2436 = { let mut out = vec![]; for t2438 in py::iter(&(py::sorted_key(&v_R.clone(), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?))? { let mut v_v: Py; 
v_v = t2438;
out.push(f__nkey(v_v.clone())?); } Py::list(out) }; Py::Bool({ let t2437 = { let mut out = vec![]; for t2439 in py::iter(&(py::cell_get(&c_best)?))? { let mut v_v: Py; 
v_v = t2439;
out.push(f__nkey(v_v.clone())?); } Py::list(out) }; if py::lt(&t2436, &t2437)? { true } else { false } }) } } } } }).truthy() {
                    py::cell_set(&c_best, f__sorted(v_R.clone())?);
                }
                return Ok(Py::None);
            }
            if ({ let t2442 = py::add(&py::len(&v_R.clone())?, &py::len(&py::cell_get(&c_P)?)?)?; Py::Bool({ let t2443 = py::len(&py::cell_get(&c_best)?)?; if py::lt(&t2442, &t2443)? { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            v_pivot = py::minmax_key(&[py::bitor(&py::cell_get(&c_P)?, &v_X.clone())?], &{ let v_R = v_R.clone(); let v_X = v_X.clone(); let v_adj = v_adj.clone(); let v_expand = v_expand.clone(); let c_P = c_P.clone(); let c_best = c_best.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::len(&py::bitand(&py::cell_get(&c_P)?, &py::getitem(&v_adj.clone(), &v_u.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
            for t2444 in py::iter(&(f__sorted(py::sub(&py::cell_get(&c_P)?, &py::getitem(&v_adj.clone(), &v_pivot.clone())?)?)?))? {
                v_v = t2444;
                let _ = py::call(&v_expand.clone(), &[py::bitor(&v_R.clone(), &py::set(&Py::list(vec![v_v.clone()]))?)?, py::bitand(&py::cell_get(&c_P)?, &py::getitem(&v_adj.clone(), &v_v.clone())?)?, py::bitand(&v_X.clone(), &py::getitem(&v_adj.clone(), &v_v.clone())?)?])?;
                py::cell_set(&c_P, py::sub(&py::cell_get(&c_P)?, &py::set(&Py::list(vec![v_v.clone()]))?)?);
                v_X = py::bitor(&v_X.clone(), &py::set(&Py::list(vec![v_v.clone()]))?)?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2430, &v_expand);
    let _ = py::call(&v_expand.clone(), &[py::set(&Py::list(vec![]))?, py::set(&v_adj.clone())?, py::set(&Py::list(vec![]))?])?;
    return Ok(py::cell_get(&c_best)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__min_cover(mut v_n: Py, mut v_on: Py, mut v_primes: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cov: Py = Py::None;
    let mut v_lits: Py = Py::None;
    let mut v_rec: Py = Py::None;
    v_on = py::set(&v_on.clone())?;
    if (py::not(&v_on.clone())).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_cov = { let mut out = vec![]; for t3916 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t3916;
out.push((v_p.clone(), py::set(&{ let mut out = vec![]; for t3917 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3917;
if (f__covers(v_p.clone(), v_m.clone())?).truthy() { out.push(v_m.clone()); }} Py::list(out) })?)); } Py::dict(out) };
    v_by = { let mut out = vec![]; for t3920 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3920;
out.push((v_m.clone(), { let mut out = vec![]; for t3921 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t3921;
if ({ let t3922 = v_m.clone(); Py::Bool({ let t3923 = py::getitem(&v_cov.clone(), &v_p.clone())?; if py::contains(&t3923, &t3922)? { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) })); } Py::dict(out) };
    v_lits = { let v_by = v_by.clone(); let v_cov = v_cov.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&v_n.clone(), &py::method(&py::int_text_base(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, "bin")?, "count", &[Py::str("1")])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_best = Py::list(vec![Py::None, Py::None]);
    let rec3924 = py::rec_cell();
    v_rec = { let v_best = v_best.clone(); let v_by = v_by.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let rec3924 = rec3924.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_unc: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_chosen: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_cost: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_rec: Py = py::rec_get(&rec3924)?;
            let mut v_m: Py = Py::None;
            let mut v_p: Py = Py::None;
            if ({ let t3929 = { let t3925 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t3926 = Py::None; if !py::is(&t3925, &t3926) { true } else { false } }) }; if !t3929.truthy() { t3929 } else { { let t3927 = v_cost.clone(); Py::Bool({ let t3928 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t3927, &t3928)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::None);
            }
            if (py::not(&v_unc.clone())).truthy() {
                let t3930 = (Py::tuple(vec![v_cost.clone(), Py::list((v_chosen.clone()).items()?)])).items()?;
                if t3930.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &Py::Int(0i64), t3930[0].clone())?;
                py::setitem(&v_best.clone(), &Py::Int(1i64), t3930[1].clone())?;
                return Ok(Py::None);
            }
            if ({ let t3935 = { let t3931 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t3932 = Py::None; if !py::is(&t3931, &t3932) { true } else { false } }) }; if !t3935.truthy() { t3935 } else { { let t3933 = Py::tuple(vec![py::add(&py::getitem(&v_cost.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::getitem(&v_cost.clone(), &Py::Int(1i64))?]); Py::Bool({ let t3934 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t3933, &t3934)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::None);
            }
            v_m = py::minmax_key(&[v_unc.clone()], &{ let v_best = v_best.clone(); let v_by = v_by.clone(); let v_chosen = v_chosen.clone(); let v_cost = v_cost.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let v_rec = v_rec.clone(); let v_unc = v_unc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::len(&py::getitem(&v_by.clone(), &v_x.clone())?)?, v_x.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
            for t3936 in py::iter(&(py::sorted_key(&py::getitem(&v_by.clone(), &v_m.clone())?, &{ let v_best = v_best.clone(); let v_by = v_by.clone(); let v_chosen = v_chosen.clone(); let v_cost = v_cost.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let v_rec = v_rec.clone(); let v_unc = v_unc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::len(&py::bitand(&py::getitem(&v_cov.clone(), &v_p.clone())?, &v_unc.clone())?)?)?, py::call(&v_lits.clone(), &[v_p.clone()])?, v_p.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?))? {
                v_p = t3936;
                let _ = py::method(&v_chosen.clone(), "append", &[v_p.clone()])?;
                let _ = py::call(&v_rec.clone(), &[py::sub(&v_unc.clone(), &py::getitem(&v_cov.clone(), &v_p.clone())?)?, v_chosen.clone(), Py::tuple(vec![py::add(&py::getitem(&v_cost.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::add(&py::getitem(&v_cost.clone(), &Py::Int(1i64))?, &py::call(&v_lits.clone(), &[v_p.clone()])?)?])])?;
                let _ = py::method(&v_chosen.clone(), "pop", &[])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3924, &v_rec);
    let _ = py::call(&v_rec.clone(), &[v_on.clone(), Py::list(vec![]), Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)])])?;
    return Ok(py::sorted_key(&py::getitem(&v_best.clone(), &Py::Int(1i64))?, &{ let v_best = v_best.clone(); let v_by = v_by.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let v_rec = v_rec.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t3937 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_b: Py; 
v_b = t3937;
out.push((if ({ let t3938 = py::not(&py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_b.clone())?, &Py::Int(1i64))?); if !t3938.truthy() { t3938 } else { py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_b.clone())?, &Py::Int(1i64))? } }).truthy() { Py::Int(0i64) } else { (if (py::not(&py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_b.clone())?, &Py::Int(1i64))?)).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) })); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mult(mut v_x: Py, mut v_y: Py) -> R {
    if ({ let t6148 = { let t6144 = v_x.clone(); Py::Bool({ let t6145 = Py::Int(0i64); if py::eq(&t6144, &t6145) { true } else { false } }) }; if !t6148.truthy() { t6148 } else { { let t6146 = v_y.clone(); Py::Bool({ let t6147 = Py::Int(0i64); if py::eq(&t6146, &t6147) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Int(1i64));
    }
    if ({ let t6153 = { let t6149 = v_x.clone(); Py::Bool({ let t6150 = Py::Int(0i64); if py::eq(&t6149, &t6150) { true } else { false } }) }; if t6153.truthy() { t6153 } else { { let t6151 = v_x.clone(); Py::Bool({ let t6152 = v_y.clone(); if py::eq(&t6151, &t6152) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Int(4i64));
    }
    return Ok(Py::Int(8i64));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ncomp(mut v_adj: Py, mut v_drop_v: Py, mut v_drop_e: Py) -> R {
    let mut v_sub: Py = Py::None;
    v_sub = { let mut out = vec![]; for t662 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t663 = (t662).items()?;
if t663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t663[0].clone();
v_nb = t663[1].clone();
if ({ let t664 = v_u.clone(); Py::Bool({ let t665 = v_drop_v.clone(); if py::ne(&t664, &t665) { true } else { false } }) }).truthy() { out.push((v_u.clone(), { let mut out = vec![]; for t666 in py::iter(&(v_nb.clone()))? { let mut v_v: Py; 
v_v = t666;
if ({ let t672 = { let t667 = v_v.clone(); Py::Bool({ let t668 = v_drop_v.clone(); if py::ne(&t667, &t668) { true } else { false } }) }; if !t672.truthy() { t672 } else { py::not(&{ let t671 = v_drop_e.clone(); if !t671.truthy() { t671 } else { { let t669 = py::set(&Py::list(vec![v_u.clone(), v_v.clone()]))?; Py::Bool({ let t670 = v_drop_e.clone(); if py::eq(&t669, &t670) { true } else { false } }) } } }) } }).truthy() { out.push(v_v.clone()); }} Py::list(out) })); }} Py::dict(out) };
    return Ok(py::len(&f__components(v_sub.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__necklace_count(mut v_n: Py, mut v_k: Py) -> R {
    return Ok((if ({ let t4409 = v_n.clone(); Py::Bool({ let t4410 = Py::Int(1i64); if py::gt(&t4409, &t4410)? { true } else { false } }) }).truthy() { py::floordiv(&py::sum(&{ let mut out = vec![]; for t4453 in py::iter(&(f__divisors_from(f__factor(v_n.clone())?)?))? { let mut v_d: Py; 
v_d = t4453;
if (Py::Bool(true)).truthy() { out.push(py::mul(&py::getitem(&f_euler_phi(v_d.clone())?, &Py::str("phi"))?, &py::pow(&v_k.clone(), &py::floordiv(&v_n.clone(), &v_d.clone())?)?)?); }} Py::list(out) }, None)?, &v_n.clone())? } else { v_k.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need_prime(mut v_v: Py, mut v_name: Py) -> R {
    if (py::not(&f__is_prime(v_v.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be prime".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nkey(mut v_x: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = Py::str(&py::to_str(&v_x.clone()));
    return Ok((if (ax_engine::pyre::module_call("fullmatch", &Py::str("-?\\d+"), &[v_s.clone()], 0)?).truthy() { Py::tuple(vec![Py::Int(0i64), py::int(&v_s.clone())?, Py::str("")]) } else { Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), v_s.clone()]) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__order(mut v_a: Py, mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_k = (if ({ let t4279 = v_n.clone(); Py::Bool({ let t4280 = Py::Int(1i64); if py::gt(&t4279, &t4280)? { true } else { false } }) }).truthy() { f__carmichael_from(f__factor(v_n.clone())?)? } else { Py::Int(1i64) });
    for t4327 in py::iter(&(f__factor(v_k.clone())?))? {
        v_p = t4327;
        while ({ let t4332 = { let t4328 = py::modulo(&v_k.clone(), &v_p.clone())?; Py::Bool({ let t4329 = Py::Int(0i64); if py::eq(&t4328, &t4329) { true } else { false } }) }; if !t4332.truthy() { t4332 } else { { let t4330 = py::pow_mod(&v_a.clone(), &py::floordiv(&v_k.clone(), &v_p.clone())?, &v_n.clone())?; Py::Bool({ let t4331 = Py::Int(1i64); if py::eq(&t4330, &t4331) { true } else { false } }) } } }).truthy() {
            v_k = py::floordiv(&v_k, &v_p.clone())?;
        }
    }
    return Ok(v_k.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__parse_formula(mut v_text: Py) -> R {
    let mut v_atom: Py = Py::None;
    let mut v_binary: Py = Py::None;
    let mut v_collect: Py = Py::None;
    let mut v_conj: Py = Py::None;
    let mut v_disj: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_imp: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_peek: Py = Py::None;
    let mut v_take: Py = Py::None;
    let mut v_toks: Py = Py::None;
    let mut v_tree: Py = Py::None;
    let mut v_xor: Py = Py::None;
    let c_iff = py::cell_new();
    let t5967 = (Py::tuple(vec![f__lex(v_text.clone())?, Py::list(vec![Py::Int(0i64)])])).items()?;
    if t5967.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_toks = t5967[0].clone();
    v_i = t5967[1].clone();
    v_peek = { let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok((if ({ let t5968 = py::getitem(&v_i.clone(), &Py::Int(0i64))?; Py::Bool({ let t5969 = py::len(&v_toks.clone())?; if py::lt(&t5968, &t5969)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_i.clone(), &Py::Int(0i64))?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let t5970 = v_i.clone(); let t5971 = Py::Int(0i64);
            py::setitem(&t5970, &t5971, py::iadd(&py::getitem(&t5970, &t5971)?, &Py::Int(1i64))?)?;
            return Ok(py::getitem(&v_toks.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_binary = { let v_binary = v_binary.clone(); let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_op: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_sub: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_right: Py = a.get(2).cloned().unwrap_or(Py::Bool(false));
            let mut v_f: Py = Py::None;
            let rec5972 = py::rec_cell();
            v_f = { let v_binary = v_binary.clone(); let v_i = v_i.clone(); let v_op = v_op.clone(); let v_peek = v_peek.clone(); let v_right = v_right.clone(); let v_sub = v_sub.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); let rec5972 = rec5972.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_f: Py = py::rec_get(&rec5972)?;
            let mut v_left: Py = Py::None;
            let mut v_rhs: Py = Py::None;
            v_left = py::call(&v_sub.clone(), &[])?;
            while ({ let t5973 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t5974 = v_op.clone(); if py::eq(&t5973, &t5974) { true } else { false } }) }).truthy() {
                let _ = py::call(&v_take.clone(), &[])?;
                v_rhs = (if (v_right.clone()).truthy() { py::call(&v_f.clone(), &[])? } else { py::call(&v_sub.clone(), &[])? });
                v_left = Py::tuple(vec![v_op.clone(), v_left.clone(), v_rhs.clone()]);
                if (v_right.clone()).truthy() {
                    break;
                }
            }
            return Ok(v_left.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            py::rec_set(&rec5972, &v_f);
            return Ok(v_f.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec5975 = py::rec_cell();
    v_atom = { let v_binary = v_binary.clone(); let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); let rec5975 = rec5975.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_atom: Py = py::rec_get(&rec5975)?;
            let mut v_e: Py = Py::None;
            let mut v_t: Py = Py::None;
            v_t = (if ({ let t5976 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t5977 = Py::None; if !py::is(&t5976, &t5977) { true } else { false } }) }).truthy() { py::call(&v_take.clone(), &[])? } else { Py::None });
            if ({ let t5978 = v_t.clone(); Py::Bool({ let t5979 = Py::str("NOT"); if py::eq(&t5978, &t5979) { true } else { false } }) }).truthy() {
                return Ok(Py::tuple(vec![Py::str("NOT"), py::call(&v_atom.clone(), &[])?]));
            }
            if ({ let t5980 = v_t.clone(); Py::Bool({ let t5981 = Py::str("("); if py::eq(&t5980, &t5981) { true } else { false } }) }).truthy() {
                v_e = py::call(&py::cell_get(&c_iff)?, &[])?;
                if ((if ({ let t5982 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t5983 = Py::None; if !py::is(&t5982, &t5983) { true } else { false } }) }).truthy() { { let t5984 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t5985 = Py::str(")"); if py::ne(&t5984, &t5985) { true } else { false } }) } } else { Py::Bool(true) })).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses in the formula"))));
                }
                return Ok(v_e.clone());
            }
            if (Py::Bool(matches!(v_t.clone(), Py::Tuple(_)))).truthy() {
                return Ok(v_t.clone());
            }
            if ({ let t5990 = { let t5986 = v_t.clone(); Py::Bool({ let t5987 = Py::Bool(true); if py::is(&t5986, &t5987) { true } else { false } }) }; if t5990.truthy() { t5990 } else { { let t5988 = v_t.clone(); Py::Bool({ let t5989 = Py::Bool(false); if py::is(&t5988, &t5989) { true } else { false } }) } } }).truthy() {
                return Ok(Py::tuple(vec![Py::str("CONST"), v_t.clone()]));
            }
            return Err(py::value_error(py::to_str(&Py::str("the formula is incomplete or malformed"))));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec5975, &v_atom);
    v_conj = py::call(&v_binary.clone(), &[Py::str("AND"), v_atom.clone()])?;
    v_xor = py::call(&v_binary.clone(), &[Py::str("XOR"), v_conj.clone()])?;
    v_disj = py::call(&v_binary.clone(), &[Py::str("OR"), v_xor.clone()])?;
    v_imp = py::call(&v_binary.clone(), &[Py::str("IMP"), v_disj.clone(), Py::Bool(true)])?;
    py::cell_set(&c_iff, py::call(&v_binary.clone(), &[Py::str("IFF"), v_imp.clone()])?);
    v_tree = py::call(&py::cell_get(&c_iff)?, &[])?;
    if ({ let t5991 = py::getitem(&v_i.clone(), &Py::Int(0i64))?; Py::Bool({ let t5992 = py::len(&v_toks.clone())?; if py::ne(&t5991, &t5992) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("unexpected symbols at the end of the formula"))));
    }
    v_names = py::set(&Py::list(vec![]))?;
    let rec5993 = py::rec_cell();
    v_collect = { let v_atom = v_atom.clone(); let v_binary = v_binary.clone(); let v_conj = v_conj.clone(); let v_disj = v_disj.clone(); let v_i = v_i.clone(); let v_imp = v_imp.clone(); let v_names = v_names.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_tree = v_tree.clone(); let v_xor = v_xor.clone(); let c_iff = c_iff.clone(); let rec5993 = rec5993.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_collect: Py = py::rec_get(&rec5993)?;
            let mut v_c: Py = Py::None;
            if ({ let t5994 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t5995 = Py::str("VAR"); if py::eq(&t5994, &t5995) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_names.clone(), "add", &[py::getitem(&v_t.clone(), &Py::Int(1i64))?])?;
            } else {
                if ({ let t5996 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t5997 = Py::str("CONST"); if py::ne(&t5996, &t5997) { true } else { false } }) }).truthy() {
                    for t5998 in py::iter(&(py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
                        v_c = t5998;
                        let _ = py::call(&v_collect.clone(), &[v_c.clone()])?;
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec5993, &v_collect);
    let _ = py::call(&v_collect.clone(), &[v_tree.clone()])?;
    if ({ let t5999 = py::len(&v_names.clone())?; Py::Bool({ let t6000 = c_MAX_VARS()?; if py::gt(&t5999, &t6000)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&c_MAX_VARS()?, "")?, " variables".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![v_tree.clone(), py::sorted(&v_names.clone(), (Py::Bool(false)).truthy())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_n: Py, mut v_on: Py, mut v_dc: Py, mut v_names: Py) -> R {
    let mut v_cover: Py = Py::None;
    let mut v_ls: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_texts: Py = Py::None;
    v_off = { let mut out = vec![]; for t3585 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3585;
if ({ let t3590 = { let t3586 = v_m.clone(); Py::Bool({ let t3587 = py::set(&v_on.clone())?; if !py::contains(&t3587, &t3586)? { true } else { false } }) }; if !t3590.truthy() { t3590 } else { { let t3588 = v_m.clone(); Py::Bool({ let t3589 = py::set(&v_dc.clone())?; if !py::contains(&t3589, &t3588)? { true } else { false } }) } } }).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    v_cover = py::sorted_key(&f__min_cover(v_n.clone(), v_off.clone(), f__qm_primes(v_n.clone(), py::bitor(&py::set(&v_off.clone())?, &py::set(&v_dc.clone())?)?)?)?, &{ let v_dc = v_dc.clone(); let v_n = v_n.clone(); let v_names = v_names.clone(); let v_off = v_off.clone(); let v_on = v_on.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t3626 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_b: Py; 
v_b = t3626;
out.push((if (py::not(&py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_b.clone())?, &Py::Int(1i64))?)).truthy() { (if (py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_b.clone())?, &Py::Int(1i64))?).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) } else { Py::Int(2i64) })); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    if (py::not(&v_off.clone())).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("1")), (Py::str("clauses"), Py::Int(0i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3627 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3627;
out.push({ let t3628 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t3629 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t3628, &t3629) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("0")), (Py::str("clauses"), Py::Int(1i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    v_texts = Py::list(vec![]);
    for t3630 in py::iter(&(v_cover.clone()))? {
        v_p = t3630;
        v_ls = f__term_text(v_p.clone(), v_names.clone(), Py::Bool(true))?;
        let _ = py::method(&v_texts.clone(), "append", &[(if ({ let t3635 = py::len(&v_ls.clone())?; Py::Bool({ let t3636 = Py::Int(1i64); if py::eq(&t3635, &t3636) { true } else { false } }) }).truthy() { py::getitem(&v_ls.clone(), &Py::Int(0i64))? } else { py::add(&py::add(&Py::str("("), &py::method(&Py::str(" | "), "join", &[v_ls.clone()])?)?, &Py::str(")"))? })])?;
    }
    return Ok(Py::dict(vec![(Py::str("expression"), py::method(&Py::str(" & "), "join", &[v_texts.clone()])?), (Py::str("clauses"), py::len(&v_cover.clone())?), (Py::str("literals"), py::sum(&{ let mut out = vec![]; for t3637 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3637;
out.push(py::len(&f__term_text(v_p.clone(), v_names.clone(), Py::Bool(false))?)?); } Py::list(out) }, None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qm_primes(mut v_n: Py, mut v_care: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_bit: Py = Py::None;
    let mut v_by_mask: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_groups: Py = Py::None;
    let mut v_items: Py = Py::None;
    let mut v_merged: Py = Py::None;
    let mut v_nxt: Py = Py::None;
    let mut v_primes: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let mut v_vs: Py = Py::None;
    v_groups = py::set(&{ let mut out = vec![]; for t3939 in py::iter(&(v_care.clone()))? { let mut v_m: Py; 
v_m = t3939;
out.push(Py::tuple(vec![v_m.clone(), Py::Int(0i64)])); } Py::list(out) })?;
    v_primes = py::set(&Py::list(vec![]))?;
    while (v_groups.clone()).truthy() {
        let t3940 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, py::set(&Py::list(vec![]))?])).items()?;
        if t3940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_merged = t3940[0].clone();
        v_nxt = t3940[1].clone();
        v_items = py::sorted(&v_groups.clone(), (Py::Bool(false)).truthy())?;
        v_by_mask = Py::dict(vec![]);
        for t3941 in py::iter(&(v_items.clone()))? {
            let t3942 = (t3941).items()?;
            if t3942.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t3942[0].clone();
            v_d = t3942[1].clone();
            let _ = py::method(&py::method(&v_by_mask.clone(), "setdefault", &[v_d.clone(), Py::list(vec![])])?, "append", &[v_v.clone()])?;
        }
        for t3943 in py::iter(&(py::method(&v_by_mask.clone(), "items", &[])?))? {
            let t3950 = (t3943).items()?;
            if t3950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_d = t3950[0].clone();
            v_vals = t3950[1].clone();
            v_vs = py::set(&v_vals.clone())?;
            for t3944 in py::iter(&(v_vals.clone()))? {
                v_v = t3944;
                for t3945 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_b = t3945;
                    v_bit = py::lshift(&Py::Int(1i64), &v_b.clone())?;
                    if ({ let t3949 = py::not(&py::bitand(&v_d.clone(), &v_bit.clone())?); if !t3949.truthy() { t3949 } else { { let t3948 = py::not(&py::bitand(&v_v.clone(), &v_bit.clone())?); if !t3948.truthy() { t3948 } else { { let t3946 = py::bitor(&v_v.clone(), &v_bit.clone())?; Py::Bool({ let t3947 = v_vs.clone(); if py::contains(&t3947, &t3946)? { true } else { false } }) } } } } }).truthy() {
                        let _ = py::method(&v_nxt.clone(), "add", &[Py::tuple(vec![v_v.clone(), py::bitor(&v_d.clone(), &v_bit.clone())?])])?;
                        let _ = py::method(&v_merged.clone(), "add", &[Py::tuple(vec![v_v.clone(), v_d.clone()])])?;
                        let _ = py::method(&v_merged.clone(), "add", &[Py::tuple(vec![py::bitor(&v_v.clone(), &v_bit.clone())?, v_d.clone()])])?;
                    }
                }
            }
        }
        v_primes = py::bitor(&v_primes, &py::sub(&v_groups.clone(), &v_merged.clone())?)?;
        v_groups = v_nxt.clone();
    }
    return Ok(v_primes.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__read_assignment(mut v_text: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_rest: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_val: Py = Py::None;
    if (Py::Bool(matches!(v_text.clone(), Py::Dict(_)))).truthy() {
        v_pairs = Py::list((py::method(&v_text.clone(), "items", &[])?).items()?);
    } else {
        v_pairs = ax_engine::pyre::module_call("findall", &Py::str("([A-Za-z_][A-Za-z0-9_]*)\\s*(?:=|:|is)\\s*(true|false|t|f|1|0)\\b"), &[Py::str(&py::to_str(&v_text.clone()))], (Py::Int(2i64)).i()?)?;
        v_rest = ax_engine::pyre::module_call("sub", &Py::str("([A-Za-z_][A-Za-z0-9_]*)\\s*(?:=|:|is)\\s*(true|false|t|f|1|0)\\b"), &[Py::str(""), Py::str(&py::to_str(&v_text.clone())), Py::Int(0)], (Py::Int(2i64)).i()?)?;
        if (ax_engine::pyre::module_call("sub", &Py::str("[\\s,;]|and"), &[Py::str(""), v_rest.clone(), Py::Int(0)], 0)?).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("write the truth values as 'p = true, q = 0, ...'"))));
        }
    }
    v_env = Py::dict(vec![]);
    for t3243 in py::iter(&(v_pairs.clone()))? {
        let t3251 = (t3243).items()?;
        if t3251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3251[0].clone();
        v_v = t3251[1].clone();
        v_val = (if (Py::Bool(matches!(v_v.clone(), Py::Bool(_)))).truthy() { v_v.clone() } else { { let t3244 = py::method(&Py::str(&py::to_str(&v_v.clone())), "lower", &[])?; Py::Bool({ let t3245 = Py::tuple(vec![Py::str("true"), Py::str("t"), Py::str("1")]); if py::contains(&t3245, &t3244)? { true } else { false } }) } });
        if ({ let t3250 = { let t3246 = v_k.clone(); Py::Bool({ let t3247 = v_env.clone(); if py::contains(&t3247, &t3246)? { true } else { false } }) }; if !t3250.truthy() { t3250 } else { { let t3248 = py::getitem(&v_env.clone(), &v_k.clone())?; Py::Bool({ let t3249 = v_val.clone(); if py::ne(&t3248, &t3249) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " is given two values".to_string()].concat()))));
        }
        py::setitem(&v_env.clone(), &v_k.clone(), v_val.clone())?;
    }
    if (py::not(&v_env.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no truth values given"))));
    }
    return Ok(v_env.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rho(mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    if ({ let t6108 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t6109 = Py::Int(0i64); if py::eq(&t6108, &t6109) { true } else { false } }) }).truthy() {
        return Ok(Py::Int(2i64));
    }
    v_c = Py::Int(1i64);
    while (Py::Bool(true)).truthy() {
        let t6110 = Py::Int(2i64);
        v_x = t6110.clone();
        v_y = t6110.clone();
        v_d = Py::Int(1i64);
        while ({ let t6111 = v_d.clone(); Py::Bool({ let t6112 = Py::Int(1i64); if py::eq(&t6111, &t6112) { true } else { false } }) }).truthy() {
            v_x = py::modulo(&py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &v_c.clone())?, &v_n.clone())?;
            v_y = py::modulo(&py::add(&py::mul(&v_y.clone(), &v_y.clone())?, &v_c.clone())?, &v_n.clone())?;
            v_y = py::modulo(&py::add(&py::mul(&v_y.clone(), &v_y.clone())?, &v_c.clone())?, &v_n.clone())?;
            v_d = py::gcd(&py::abs(&py::sub(&v_x.clone(), &v_y.clone())?)?, &v_n.clone())?;
        }
        if ({ let t6113 = v_d.clone(); Py::Bool({ let t6114 = v_n.clone(); if py::ne(&t6113, &t6114) { true } else { false } }) }).truthy() {
            return Ok(v_d.clone());
        }
        v_c = py::iadd(&v_c, &Py::Int(1i64))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rows(mut v_names: Py) -> R {
    let mut gen_out: Vec<Py> = vec![];
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::len(&v_names.clone())?;
    for t6003 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) {
        v_r = t6003;
        gen_out.push(Py::tuple(vec![v_r.clone(), { let mut out = vec![]; for t6004 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t6005 = (t6004).items()?;
if t6005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6005[0].clone();
v_v = t6005[1].clone();
out.push((v_v.clone(), Py::Bool((py::bitand(&py::rshift(&v_r.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?)?, &Py::Int(1i64))?).truthy()))); } Py::dict(out) }]));
    }
    #[allow(unreachable_code)]
    return Ok(Py::list(gen_out));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sop(mut v_n: Py, mut v_on: Py, mut v_dc: Py, mut v_names: Py) -> R {
    let mut v_cover: Py = Py::None;
    let mut v_texts: Py = Py::None;
    v_cover = f__min_cover(v_n.clone(), v_on.clone(), f__qm_primes(v_n.clone(), py::bitor(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?)?)?;
    if (py::not(&v_on.clone())).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("0")), (Py::str("terms"), Py::Int(0i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3951 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3951;
out.push({ let t3952 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t3953 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t3952, &t3953) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("1")), (Py::str("terms"), Py::Int(1i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    v_texts = { let mut out = vec![]; for t3954 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3954;
out.push(py::method(&Py::str(" & "), "join", &[f__term_text(v_p.clone(), v_names.clone(), Py::Bool(false))?])?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("expression"), py::method(&Py::str(" | "), "join", &[v_texts.clone()])?), (Py::str("terms"), py::len(&v_cover.clone())?), (Py::str("literals"), py::sum(&{ let mut out = vec![]; for t3959 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3959;
out.push(py::len(&f__term_text(v_p.clone(), v_names.clone(), Py::Bool(false))?)?); } Py::list(out) }, None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sorted(mut v_nodes: Py) -> R {
    return Ok(py::sorted_key(&v_nodes.clone(), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__table(mut v_text: Py, mut v_names: Py) -> R {
    let mut v_tree: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let t6001 = (f__parse_formula(v_text.clone())?).items()?;
    if t6001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tree = t6001[0].clone();
    v_vs = t6001[1].clone();
    v_names = { let t6002 = v_names.clone(); if t6002.truthy() { t6002 } else { v_vs.clone() } };
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t6006 in py::iter(&(f__rows(v_names.clone())?))? { let mut v__r: Py; let mut v_env: Py; 
let t6007 = (t6006).items()?;
if t6007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__r = t6007[0].clone();
v_env = t6007[1].clone();
out.push(f__ev(v_tree.clone(), v_env.clone())?); } Py::list(out) }, v_names.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__term_text(mut v_p: Py, mut v_names: Py, mut v_negate: Py) -> R {
    let mut v_bit: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_names.clone())?;
    v_parts = Py::list(vec![]);
    for t3955 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? {
        let t3958 = (t3955).items()?;
        if t3958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3958[0].clone();
        v_v = t3958[1].clone();
        v_bit = py::lshift(&Py::Int(1i64), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?)?;
        if (py::not(&py::bitand(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_bit.clone())?)).truthy() {
            v_pos = { let t3956 = Py::Bool((py::bitand(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_bit.clone())?).truthy()); Py::Bool({ let t3957 = v_negate.clone(); if py::ne(&t3956, &t3957) { true } else { false } }) };
            let _ = py::method(&v_parts.clone(), "append", &[(if (v_pos.clone()).truthy() { v_v.clone() } else { py::add(&Py::str("~"), &v_v.clone())? })])?;
        }
    }
    return Ok(v_parts.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__to_base(mut v_v: Py, mut v_b: Py) -> R {
    let mut v_neg: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    if ({ let t299 = v_v.clone(); Py::Bool({ let t300 = Py::Int(0i64); if py::eq(&t299, &t300) { true } else { false } }) }).truthy() {
        return Ok(Py::str("0"));
    }
    let t303 = (Py::tuple(vec![{ let t301 = v_v.clone(); Py::Bool({ let t302 = Py::Int(0i64); if py::lt(&t301, &t302)? { true } else { false } }) }, py::abs(&v_v.clone())?, Py::list(vec![])])).items()?;
    if t303.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_neg = t303[0].clone();
    v_v = t303[1].clone();
    v_out = t303[2].clone();
    while (v_v.clone()).truthy() {
        let t304 = (Py::tuple(vec![py::floordiv(&v_v.clone(), &v_b.clone())?, py::modulo(&v_v.clone(), &v_b.clone())?])).items()?;
        if t304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t304[0].clone();
        v_r = t304[1].clone();
        let _ = py::method(&v_out.clone(), "append", &[py::getitem(&c__DIG()?, &v_r.clone())?])?;
    }
    return Ok(py::add(&(if (v_neg.clone()).truthy() { Py::str("-") } else { Py::str("") }), &py::method(&Py::str(""), "join", &[py::reversed(&v_out.clone())?])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tonelli(mut v_a: Py, mut v_p: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let mut v_z: Py = Py::None;
    if ({ let t5641 = v_p.clone(); Py::Bool({ let t5642 = Py::Int(2i64); if py::eq(&t5641, &t5642) { true } else { false } }) }).truthy() {
        return Ok(py::modulo(&v_a.clone(), &Py::Int(2i64))?);
    }
    let t5643 = (Py::tuple(vec![py::sub(&v_p.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
    if t5643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t5643[0].clone();
    v_s = t5643[1].clone();
    while ({ let t5644 = py::modulo(&v_q.clone(), &Py::Int(2i64))?; Py::Bool({ let t5645 = Py::Int(0i64); if py::eq(&t5644, &t5645) { true } else { false } }) }).truthy() {
        v_q = py::floordiv(&v_q, &Py::Int(2i64))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    v_z = Py::Int(2i64);
    while ({ let t5646 = py::pow_mod(&v_z.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?; Py::Bool({ let t5647 = py::sub(&v_p.clone(), &Py::Int(1i64))?; if py::ne(&t5646, &t5647) { true } else { false } }) }).truthy() {
        v_z = py::iadd(&v_z, &Py::Int(1i64))?;
    }
    let t5648 = (Py::tuple(vec![v_s.clone(), py::pow_mod(&v_z.clone(), &v_q.clone(), &v_p.clone())?, py::pow_mod(&v_a.clone(), &v_q.clone(), &v_p.clone())?, py::pow_mod(&v_a.clone(), &py::floordiv(&py::add(&v_q.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?])).items()?;
    if t5648.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5648[0].clone();
    v_c = t5648[1].clone();
    v_t = t5648[2].clone();
    v_r = t5648[3].clone();
    while ({ let t5653 = v_t.clone(); Py::Bool({ let t5654 = Py::Int(1i64); if py::ne(&t5653, &t5654) { true } else { false } }) }).truthy() {
        let t5649 = (Py::tuple(vec![Py::Int(0i64), v_t.clone()])).items()?;
        if t5649.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t5649[0].clone();
        v_t2 = t5649[1].clone();
        while ({ let t5650 = v_t2.clone(); Py::Bool({ let t5651 = Py::Int(1i64); if py::ne(&t5650, &t5651) { true } else { false } }) }).truthy() {
            v_t2 = py::modulo(&py::mul(&v_t2.clone(), &v_t2.clone())?, &v_p.clone())?;
            v_i = py::iadd(&v_i, &Py::Int(1i64))?;
        }
        v_b = py::pow_mod(&v_c.clone(), &py::lshift(&Py::Int(1i64), &py::sub(&py::sub(&v_m.clone(), &v_i.clone())?, &Py::Int(1i64))?)?, &v_p.clone())?;
        let t5652 = (Py::tuple(vec![v_i.clone(), py::modulo(&py::mul(&v_b.clone(), &v_b.clone())?, &v_p.clone())?, py::modulo(&py::mul(&py::mul(&v_t.clone(), &v_b.clone())?, &v_b.clone())?, &v_p.clone())?, py::modulo(&py::mul(&v_r.clone(), &v_b.clone())?, &v_p.clone())?])).items()?;
        if t5652.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_m = t5652[0].clone();
        v_c = t5652[1].clone();
        v_t = t5652[2].clone();
        v_r = t5652[3].clone();
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tree_labels(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_labels: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_off: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t5210 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5210;
out.push(ax_engine::pyre::module_call("fullmatch", &Py::str("\\d+"), &[v_v.clone()], 0)?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the vertices must be numbered 1..n (or 0..n-1)"))));
    }
    v_labels = py::sorted(&{ let mut out = vec![]; for t5211 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5211;
out.push(py::int(&v_v.clone())?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_labels.clone())?;
    if ({ let t5212 = v_labels.clone(); Py::Bool({ let t5213 = Py::list((Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::eq(&t5212, &t5213) { true } else { false } }) }).truthy() {
        v_off = Py::Int(1i64);
    } else {
        if ({ let t5214 = v_labels.clone(); Py::Bool({ let t5215 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::eq(&t5214, &t5215) { true } else { false } }) }).truthy() {
            v_off = Py::Int(0i64);
        } else {
            return Err(py::value_error(py::to_str(&Py::str("the vertices must be numbered 1..n (or 0..n-1)"))));
        }
    }
    if ({ let t5229 = { let t5216 = py::floordiv(&py::sum(&{ let mut out = vec![]; for t5218 in py::iter(&(py::method(&v_adj.clone(), "values", &[])?))? { let mut v_nb: Py; 
v_nb = t5218;
out.push(py::len(&v_nb.clone())?); } Py::list(out) }, None)?, &Py::Int(2i64))?; Py::Bool({ let t5217 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t5216, &t5217) { true } else { false } }) }; if t5229.truthy() { t5229 } else { { let t5219 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t5220 = Py::Int(1i64); if py::ne(&t5219, &t5220) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph is not a tree"))));
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t5230 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t5231 = (t5230).items()?;
if t5231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5231[0].clone();
v_nb = t5231[1].clone();
out.push((py::int(&v_u.clone())?, py::set(&{ let mut out = vec![]; for t5232 in py::iter(&(v_nb.clone()))? { let mut v_v: Py; 
v_v = t5232;
out.push(py::int(&v_v.clone())?); } Py::list(out) })?)); } Py::dict(out) }, v_off.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trial_factor(mut v_n: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t5056 = (Py::tuple(vec![Py::dict(vec![]), Py::Int(2i64)])).items()?;
    if t5056.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_out = t5056[0].clone();
    v_p = t5056[1].clone();
    while ({ let t5061 = py::mul(&v_p.clone(), &v_p.clone())?; Py::Bool({ let t5062 = v_n.clone(); if py::le(&t5061, &t5062)? { true } else { false } }) }).truthy() {
        while ({ let t5057 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t5058 = Py::Int(0i64); if py::eq(&t5057, &t5058) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &v_p.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_p.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
            v_n = py::floordiv(&v_n, &v_p.clone())?;
        }
        v_p = py::iadd(&v_p, &(if ({ let t5059 = v_p.clone(); Py::Bool({ let t5060 = Py::Int(2i64); if py::eq(&t5059, &t5060) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }))?;
    }
    if ({ let t5063 = v_n.clone(); Py::Bool({ let t5064 = Py::Int(1i64); if py::gt(&t5063, &t5064)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &v_n.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_n.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_colour(mut v_adj: Py) -> R {
    let mut v_col: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_col = Py::dict(vec![]);
    for t489 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_s = t489;
        if ({ let t490 = v_s.clone(); Py::Bool({ let t491 = v_col.clone(); if py::contains(&t491, &t490)? { true } else { false } }) }).truthy() {
            continue;
        }
        py::setitem(&v_col.clone(), &v_s.clone(), Py::Int(0i64))?;
        v_stack = Py::list(vec![v_s.clone()]);
        while (v_stack.clone()).truthy() {
            v_u = py::method(&v_stack.clone(), "pop", &[])?;
            for t492 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t492;
                if ({ let t493 = v_v.clone(); Py::Bool({ let t494 = v_col.clone(); if !py::contains(&t494, &t493)? { true } else { false } }) }).truthy() {
                    py::setitem(&v_col.clone(), &v_v.clone(), py::sub(&Py::Int(1i64), &py::getitem(&v_col.clone(), &v_u.clone())?)?)?;
                    let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
                } else {
                    if ({ let t495 = py::getitem(&v_col.clone(), &v_v.clone())?; Py::Bool({ let t496 = py::getitem(&v_col.clone(), &v_u.clone())?; if py::eq(&t495, &t496) { true } else { false } }) }).truthy() {
                        return Ok(Py::None);
                    }
                }
            }
        }
    }
    return Ok(v_col.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_ap(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_base = f__ncomp(v_adj.clone(), Py::None, Py::None)?;
    v_want = f__sorted({ let mut out = vec![]; for t255 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t255;
if ({ let t256 = f__ncomp(v_adj.clone(), v_v.clone(), Py::None)?; Py::Bool({ let t257 = v_base.clone(); if py::gt(&t256, &t257)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?;
    return Ok({ let t264 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t264.truthy() { t264 } else { { let t263 = { let t259 = py::getitem(&v_o.clone(), &Py::str("points"))?; Py::Bool({ let t260 = v_want.clone(); if py::eq(&t259, &t260) { true } else { false } }) }; if !t263.truthy() { t263 } else { { let t261 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t262 = py::len(&v_want.clone())?; if py::eq(&t261, &t262) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_app(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pops: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_up_a: Py = Py::None;
    let mut v_up_b: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_pops = f__wlist(py::getitem(&v_i.clone(), &Py::str("populations"))?, Py::str("populations"), Py::Int(1i64), c_MAXI()?, Py::Int(1i64), Py::None)?;
    let t143 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("seats"))?)?, py::getitem(&v_i.clone(), &Py::str("method"))?])).items()?;
    if t143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t143[0].clone();
    v_m = t143[1].clone();
    v_s = py::getitem(&v_o.clone(), &Py::str("seats"))?;
    v_n = py::len(&v_pops.clone())?;
    v_total = py::sum(&v_pops.clone(), None)?;
    if ({ let t151 = { let t144 = py::len(&v_s.clone())?; Py::Bool({ let t145 = v_n.clone(); if py::ne(&t144, &t145) { true } else { false } }) }; if t151.truthy() { t151 } else { { let t150 = { let t146 = py::getitem(&v_o.clone(), &Py::str("total_seats"))?; Py::Bool({ let t147 = v_h.clone(); if py::ne(&t146, &t147) { true } else { false } }) }; if t150.truthy() { t150 } else { { let t148 = py::len(&py::getitem(&v_o.clone(), &Py::str("quotas"))?)?; Py::Bool({ let t149 = v_n.clone(); if py::ne(&t148, &t149) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_given = Py::Int(0i64);
    for t152 in py::iter(&(v_s.clone()))? {
        v_x = t152;
        if ({ let t156 = py::not(&f__iv(v_x.clone())?); if t156.truthy() { t156 } else { { let t154 = v_x.clone(); Py::Bool({ let t155 = Py::Int(0i64); if py::lt(&t154, &t155)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Bool(false));
        }
        v_given = py::iadd(&v_given, &v_x.clone())?;
    }
    if ({ let t157 = v_given.clone(); Py::Bool({ let t158 = v_h.clone(); if py::ne(&t157, &t158) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_r = py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("standard_divisor"))?, &v_h.clone())?, &v_total.clone())?)?, &v_total.clone())?;
    for t159 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t159;
        v_r = py::minmax(&[v_r.clone(), py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("quotas"))?, &v_k.clone())?, &v_total.clone())?, &py::mul(&py::getitem(&v_pops.clone(), &v_k.clone())?, &v_h.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_pops.clone(), &v_k.clone())?, &v_h.clone())?], true)?)?], true)?;
    }
    if ({ let t160 = v_m.clone(); Py::Bool({ let t161 = Py::str("hamilton"); if py::eq(&t160, &t161) { true } else { false } }) }).truthy() {
        for t162 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_k = t162;
            if ({ let t167 = { let t163 = py::getitem(&v_s.clone(), &v_k.clone())?; Py::Bool({ let t164 = py::floordiv(&py::mul(&py::getitem(&v_pops.clone(), &v_k.clone())?, &v_h.clone())?, &v_total.clone())?; if py::ne(&t163, &t164) { true } else { false } }) }; if !t167.truthy() { t167 } else { { let t165 = py::getitem(&v_s.clone(), &v_k.clone())?; Py::Bool({ let t166 = py::add(&py::floordiv(&py::mul(&py::getitem(&v_pops.clone(), &v_k.clone())?, &v_h.clone())?, &v_total.clone())?, &Py::Int(1i64))?; if py::ne(&t165, &t166) { true } else { false } }) } } }).truthy() {
                return Ok(Py::Bool(false));
            }
        }
        for t168 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_a = t168;
            for t169 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_b = t169;
                let t174 = (Py::tuple(vec![{ let t170 = py::getitem(&v_s.clone(), &v_a.clone())?; Py::Bool({ let t171 = py::floordiv(&py::mul(&py::getitem(&v_pops.clone(), &v_a.clone())?, &v_h.clone())?, &v_total.clone())?; if py::gt(&t170, &t171)? { true } else { false } }) }, { let t172 = py::getitem(&v_s.clone(), &v_b.clone())?; Py::Bool({ let t173 = py::floordiv(&py::mul(&py::getitem(&v_pops.clone(), &v_b.clone())?, &v_h.clone())?, &v_total.clone())?; if py::gt(&t172, &t173)? { true } else { false } }) }])).items()?;
                if t174.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_up_a = t174[0].clone();
                v_up_b = t174[1].clone();
                if ({ let t178 = v_up_a.clone(); if !t178.truthy() { t178 } else { { let t177 = py::not(&v_up_b.clone()); if !t177.truthy() { t177 } else { { let t175 = py::modulo(&py::mul(&py::getitem(&v_pops.clone(), &v_a.clone())?, &v_h.clone())?, &v_total.clone())?; Py::Bool({ let t176 = py::modulo(&py::mul(&py::getitem(&v_pops.clone(), &v_b.clone())?, &v_h.clone())?, &v_total.clone())?; if py::le(&t175, &t176)? { true } else { false } }) } } } } }).truthy() {
                    return Ok(Py::Bool(false));
                }
            }
        }
        return Ok(v_r.clone());
    }
    for t179 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_a = t179;
        if ({ let t180 = py::getitem(&v_s.clone(), &v_a.clone())?; Py::Bool({ let t181 = Py::Int(0i64); if py::eq(&t180, &t181) { true } else { false } }) }).truthy() {
            continue;
        }
        for t182 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_b = t182;
            if (py::not(&f__ap_gt(v_m.clone(), py::getitem(&v_pops.clone(), &v_a.clone())?, py::getitem(&v_s.clone(), &v_a.clone())?, py::getitem(&v_pops.clone(), &v_b.clone())?, py::add(&py::getitem(&v_s.clone(), &v_b.clone())?, &Py::Int(1i64))?)?)).truthy() {
                return Ok(Py::Bool(false));
            }
        }
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_base(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_back: Py = Py::None;
    let mut v_canon: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_neg: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rn: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tb: Py = Py::None;
    let mut v_val: Py = Py::None;
    v_s = py::method(&py::method(&py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("digits"))?)), "strip", &[])?, "upper", &[])?, "replace", &[Py::str("_"), Py::str("")])?, "replace", &[Py::str(" "), Py::str("")])?;
    let t306 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("from_base"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("to_base"))?)?])).items()?;
    if t306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fb = t306[0].clone();
    v_tb = t306[1].clone();
    v_neg = py::method(&v_s.clone(), "startswith", &[Py::str("-")])?;
    v_val = py::sum(&{ let mut out = vec![]; for t307 in py::iter(&(py::enumerate(&py::reversed(&py::method(&v_s.clone(), "lstrip", &[Py::str("-")])?)?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t308 = (t307).items()?;
if t308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t308[0].clone();
v_c = t308[1].clone();
out.push(py::mul(&py::method(&c__DIG()?, "index", &[v_c.clone()])?, &py::pow(&v_fb.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?;
    v_val = (if (v_neg.clone()).truthy() { py::neg(&v_val.clone())? } else { v_val.clone() });
    v_r = py::getitem(&v_o.clone(), &Py::str("result"))?;
    v_rn = py::method(&v_r.clone(), "startswith", &[Py::str("-")])?;
    v_back = py::sum(&{ let mut out = vec![]; for t309 in py::iter(&(py::enumerate(&py::reversed(&py::method(&v_r.clone(), "lstrip", &[Py::str("-")])?)?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t310 = (t309).items()?;
if t310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t310[0].clone();
v_c = t310[1].clone();
out.push(py::mul(&py::method(&c__DIG()?, "index", &[v_c.clone()])?, &py::pow(&v_tb.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?;
    v_canon = { let t315 = { let t311 = v_r.clone(); Py::Bool({ let t312 = Py::str("0"); if py::eq(&t311, &t312) { true } else { false } }) }; if t315.truthy() { t315 } else { { let t313 = py::getitem(&py::method(&v_r.clone(), "lstrip", &[Py::str("-")])?, &Py::Int(0i64))?; Py::Bool({ let t314 = Py::str("0"); if py::ne(&t313, &t314) { true } else { false } }) } } };
    return Ok({ let t327 = f__iv(py::getitem(&v_o.clone(), &Py::str("decimal"))?)?; if !t327.truthy() { t327 } else { { let t326 = { let t317 = py::getitem(&v_o.clone(), &Py::str("decimal"))?; Py::Bool({ let t318 = v_val.clone(); if py::eq(&t317, &t318) { true } else { false } }) }; if !t326.truthy() { t326 } else { { let t325 = { let t319 = (if (v_rn.clone()).truthy() { py::neg(&v_back.clone())? } else { v_back.clone() }); Py::Bool({ let t320 = v_val.clone(); if py::eq(&t319, &t320) { true } else { false } }) }; if !t325.truthy() { t325 } else { { let t324 = v_canon.clone(); if !t324.truthy() { t324 } else { Py::Bool(({ let mut out = vec![]; for t321 in py::iter(&(py::method(&v_r.clone(), "lstrip", &[Py::str("-")])?))? { let mut v_c: Py; 
v_c = t321;
out.push({ let t322 = py::method(&c__DIG()?, "index", &[v_c.clone()])?; Py::Bool({ let t323 = v_tb.clone(); if py::lt(&t322, &t323)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_bell(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_b = Py::list(vec![Py::Int(1i64)]);
    for t353 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_m = t353;
        let _ = py::method(&v_b.clone(), "append", &[py::sum(&{ let mut out = vec![]; for t354 in py::range_iter(&[(py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t354;
out.push(py::mul(&py::comb(&v_m.clone(), &v_j.clone())?, &py::getitem(&v_b.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?])?;
    }
    return Ok({ let t358 = f__iv(py::getitem(&v_o.clone(), &Py::str("bell"))?)?; if !t358.truthy() { t358 } else { { let t356 = py::getitem(&v_o.clone(), &Py::str("bell"))?; Py::Bool({ let t357 = py::getitem(&v_b.clone(), &v_n.clone())?; if py::eq(&t356, &t357) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_bern(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_a = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    for t384 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t384;
        py::setitem(&v_a.clone(), &v_m.clone(), py::fraction(&[Py::Int(1i64), py::add(&v_m.clone(), &Py::Int(1i64))?])?)?;
        for t385 in py::range_iter(&[(v_m.clone()).i()?, (Py::Int(0i64)).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_j = t385;
            py::setitem(&v_a.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?, py::mul(&v_j.clone(), &py::sub(&py::getitem(&v_a.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_a.clone(), &v_j.clone())?)?)?)?;
        }
    }
    v_b = (if ({ let t386 = v_n.clone(); Py::Bool({ let t387 = Py::Int(1i64); if py::eq(&t386, &t387) { true } else { false } }) }).truthy() { py::neg(&py::getitem(&v_a.clone(), &Py::Int(0i64))?)? } else { py::getitem(&v_a.clone(), &Py::Int(0i64))? });
    return Ok({ let t398 = f__iv(py::getitem(&v_o.clone(), &Py::str("numerator"))?)?; if !t398.truthy() { t398 } else { { let t397 = f__iv(py::getitem(&v_o.clone(), &Py::str("denominator"))?)?; if !t397.truthy() { t397 } else { { let t396 = { let t389 = py::fraction(&[py::getitem(&v_o.clone(), &Py::str("numerator"))?, py::getitem(&v_o.clone(), &Py::str("denominator"))?])?; Py::Bool({ let t390 = v_b.clone(); if py::eq(&t389, &t390) { true } else { false } }) }; if !t396.truthy() { t396 } else { { let t395 = { let t391 = py::getitem(&v_o.clone(), &Py::str("denominator"))?; Py::Bool({ let t392 = Py::Int(0i64); if py::gt(&t391, &t392)? { true } else { false } }) }; if !t395.truthy() { t395 } else { { let t393 = py::getitem(&v_o.clone(), &Py::str("value"))?; Py::Bool({ let t394 = py::float(&v_b.clone())?; if py::eq(&t393, &t394) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_bfs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_order: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool((py::method(&v_i.clone(), "get", &[Py::str("directed")])?).truthy()), Py::Bool(false))?;
    let t434 = (f__bfs(v_adj.clone(), py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, Py::Bool(false))?).items()?;
    if t434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_order = t434[0].clone();
    v_dist = t434[1].clone();
    return Ok({ let t444 = { let t435 = py::getitem(&v_o.clone(), &Py::str("order"))?; Py::Bool({ let t436 = v_order.clone(); if py::eq(&t435, &t436) { true } else { false } }) }; if !t444.truthy() { t444 } else { { let t443 = f__iv(py::getitem(&v_o.clone(), &Py::str("reached"))?)?; if !t443.truthy() { t443 } else { { let t442 = { let t438 = py::getitem(&v_o.clone(), &Py::str("reached"))?; Py::Bool({ let t439 = py::len(&v_order.clone())?; if py::eq(&t438, &t439) { true } else { false } }) }; if !t442.truthy() { t442 } else { { let t440 = py::getitem(&v_o.clone(), &Py::str("distances"))?; Py::Bool({ let t441 = v_dist.clone(); if py::eq(&t440, &t441) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_bip(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_dq: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    let t516 = (Py::tuple(vec![Py::dict(vec![]), Py::Bool(true)])).items()?;
    if t516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_side = t516[0].clone();
    v_ok = t516[1].clone();
    for t517 in py::iter(&(v_adj.clone()))? {
        v_s = t517;
        if ({ let t518 = v_s.clone(); Py::Bool({ let t519 = v_side.clone(); if py::contains(&t519, &t518)? { true } else { false } }) }).truthy() {
            continue;
        }
        py::setitem(&v_side.clone(), &v_s.clone(), Py::Int(1i64))?;
        v_dq = Py::list((Py::list(vec![v_s.clone()])).items()?);
        while ({ let t525 = v_dq.clone(); if !t525.truthy() { t525 } else { v_ok.clone() } }).truthy() {
            v_u = py::method(&v_dq.clone(), "popleft", &[])?;
            for t520 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t520;
                if ({ let t521 = v_v.clone(); Py::Bool({ let t522 = v_side.clone(); if !py::contains(&t522, &t521)? { true } else { false } }) }).truthy() {
                    py::setitem(&v_side.clone(), &v_v.clone(), py::neg(&py::getitem(&v_side.clone(), &v_u.clone())?)?)?;
                    let _ = py::method(&v_dq.clone(), "append", &[v_v.clone()])?;
                } else {
                    if ({ let t523 = py::getitem(&v_side.clone(), &v_v.clone())?; Py::Bool({ let t524 = py::getitem(&v_side.clone(), &v_u.clone())?; if py::eq(&t523, &t524) { true } else { false } }) }).truthy() {
                        v_ok = Py::Bool(false);
                    }
                }
            }
        }
    }
    return Ok({ let t526 = py::getitem(&v_o.clone(), &Py::str("is_bipartite"))?; Py::Bool({ let t527 = v_ok.clone(); if py::is(&t526, &t527) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_br(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_base = f__ncomp(v_adj.clone(), Py::None, Py::None)?;
    v_want = py::sorted_key(&{ let mut out = vec![]; for t681 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t681;
if ({ let t682 = f__ncomp(v_adj.clone(), Py::None, py::set(&Py::list(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))]))?)?; Py::Bool({ let t683 = v_base.clone(); if py::gt(&t682, &t683)? { true } else { false } }) }).truthy() { out.push(py::sorted_key(&Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))]), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?); }} Py::list(out) }, &{ let v_adj = v_adj.clone(); let v_base = v_base.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![f__nkey(py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, f__nkey(py::getitem(&v_e.clone(), &Py::Int(1i64))?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t690 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t690.truthy() { t690 } else { { let t689 = { let t685 = py::getitem(&v_o.clone(), &Py::str("bridges"))?; Py::Bool({ let t686 = v_want.clone(); if py::eq(&t685, &t686) { true } else { false } }) }; if !t689.truthy() { t689 } else { { let t687 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t688 = py::len(&v_want.clone())?; if py::eq(&t687, &t688) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_carm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t804 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("lambda"))?])).items()?;
    if t804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t804[0].clone();
    v_lam = t804[1].clone();
    if ({ let t808 = py::not(&f__iv(v_lam.clone())?); if t808.truthy() { t808 } else { { let t806 = v_lam.clone(); Py::Bool({ let t807 = Py::Int(1i64); if py::lt(&t806, &t807)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_f = (if ({ let t809 = v_n.clone(); Py::Bool({ let t810 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::lt(&t809, &t810)? { true } else { false } }) }).truthy() { f__trial_factor(v_n.clone())? } else { f__factor(v_n.clone())? });
    v_want = Py::Int(1i64);
    for t820 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? {
        let t826 = (t820).items()?;
        if t826.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t826[0].clone();
        v_e = t826[1].clone();
        v_l = py::mul(&py::pow(&v_p.clone(), &py::sub(&v_e.clone(), &Py::Int(1i64))?)?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)?;
        if ({ let t825 = { let t821 = v_p.clone(); Py::Bool({ let t822 = Py::Int(2i64); if py::eq(&t821, &t822) { true } else { false } }) }; if !t825.truthy() { t825 } else { { let t823 = v_e.clone(); Py::Bool({ let t824 = Py::Int(3i64); if py::ge(&t823, &t824)? { true } else { false } }) } } }).truthy() {
            v_l = py::floordiv(&v_l, &Py::Int(2i64))?;
        }
        v_want = py::gcd_lcm("lcm", &[v_want.clone(), v_l.clone()])?;
    }
    return Ok({ let t834 = { let t827 = v_lam.clone(); Py::Bool({ let t828 = v_want.clone(); if py::eq(&t827, &t828) { true } else { false } }) }; if !t834.truthy() { t834 } else { Py::Bool(({ let mut out = vec![]; for t829 in py::range_iter(&[(Py::Int(2i64)).i()?, (Py::Int(30i64)).i()?]) { let mut v_a: Py; 
v_a = t829;
if ({ let t830 = py::gcd(&v_a.clone(), &v_n.clone())?; Py::Bool({ let t831 = Py::Int(1i64); if py::eq(&t830, &t831) { true } else { false } }) }).truthy() { out.push({ let t832 = py::pow_mod(&v_a.clone(), &v_lam.clone(), &v_n.clone())?; Py::Bool({ let t833 = py::modulo(&Py::Int(1i64), &v_n.clone())?; if py::eq(&t832, &t833) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_catalan(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_c = Py::Int(1i64);
    for t858 in py::range_iter(&[(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?).i()?]) {
        v_k = t858;
        v_c = py::floordiv(&py::mul(&py::mul(&v_c.clone(), &Py::Int(2i64))?, &py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?)?, &py::add(&v_k.clone(), &Py::Int(2i64))?)?;
    }
    return Ok({ let t862 = f__iv(py::getitem(&v_o.clone(), &Py::str("catalan"))?)?; if !t862.truthy() { t862 } else { { let t860 = py::getitem(&v_o.clone(), &Py::str("catalan"))?; Py::Bool({ let t861 = v_c.clone(); if py::eq(&t860, &t861) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_cc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_comps: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_find: Py = Py::None;
    let mut v_groups: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_parent = Py::dict(vec![]);
    v_find = { let v_find = v_find.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_parent = v_parent.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t1155 = py::method(&v_parent.clone(), "setdefault", &[v_x.clone(), v_x.clone()])?; Py::Bool({ let t1156 = v_x.clone(); if py::ne(&t1155, &t1156) { true } else { false } }) }).truthy() {
                py::setitem(&v_parent.clone(), &v_x.clone(), py::getitem(&v_parent.clone(), &py::getitem(&v_parent.clone(), &v_x.clone())?)?)?;
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t1157 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? {
        v_e = t1157;
        py::setitem(&v_parent.clone(), &py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?))])?, py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])?)?;
    }
    v_groups = Py::dict(vec![]);
    for t1158 in py::iter(&(Py::list((v_parent.clone()).items()?)))? {
        v_x = t1158;
        let _ = py::method(&py::method(&v_groups.clone(), "setdefault", &[py::call(&v_find.clone(), &[v_x.clone()])?, Py::list(vec![])])?, "append", &[v_x.clone()])?;
    }
    v_comps = py::sorted_key(&{ let mut out = vec![]; for t1159 in py::iter(&(py::method(&v_groups.clone(), "values", &[])?))? { let mut v_g: Py; 
v_g = t1159;
out.push(f__sorted(v_g.clone())?); } Py::list(out) }, &{ let v_e = v_e.clone(); let v_find = v_find.clone(); let v_groups = v_groups.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_parent = v_parent.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__nkey(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t1175 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t1175.truthy() { t1175 } else { { let t1174 = { let t1161 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1162 = py::len(&v_comps.clone())?; if py::eq(&t1161, &t1162) { true } else { false } }) }; if !t1174.truthy() { t1174 } else { { let t1173 = { let t1163 = py::getitem(&v_o.clone(), &Py::str("components"))?; Py::Bool({ let t1164 = v_comps.clone(); if py::eq(&t1163, &t1164) { true } else { false } }) }; if !t1173.truthy() { t1173 } else { { let t1172 = { let t1165 = py::getitem(&v_o.clone(), &Py::str("is_connected"))?; Py::Bool({ let t1166 = { let t1167 = py::len(&v_comps.clone())?; Py::Bool({ let t1168 = Py::Int(1i64); if py::eq(&t1167, &t1168) { true } else { false } }) }; if py::is(&t1165, &t1166) { true } else { false } }) }; if !t1172.truthy() { t1172 } else { { let t1169 = py::getitem(&v_o.clone(), &Py::str("sizes"))?; Py::Bool({ let t1170 = py::sorted(&{ let mut out = vec![]; for t1171 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t1171;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?; if py::eq(&t1169, &t1170) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_cc_count(mut v_edges: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_find: Py = Py::None;
    let mut v_parent: Py = Py::None;
    v_parent = Py::dict(vec![]);
    v_find = { let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_parent = v_parent.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t2706 = py::method(&v_parent.clone(), "setdefault", &[v_x.clone(), v_x.clone()])?; Py::Bool({ let t2707 = v_x.clone(); if py::ne(&t2706, &t2707) { true } else { false } }) }).truthy() {
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t2708 in py::iter(&(v_edges.clone()))? {
        v_e = t2708;
        py::setitem(&v_parent.clone(), &py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?))])?, py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])?)?;
    }
    return Ok(py::len(&py::set(&{ let mut out = vec![]; for t2709 in py::iter(&(Py::list((v_parent.clone()).items()?)))? { let mut v_x: Py; 
v_x = t2709;
out.push(py::call(&v_find.clone(), &[v_x.clone()])?); } Py::list(out) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_cfr(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_t = py::getitem(&v_o.clone(), &Py::str("terms"))?;
    if ({ let t903 = py::not(&v_t.clone()); if t903.truthy() { t903 } else { { let t902 = py::not(&Py::Bool(({ let mut out = vec![]; for t888 in py::iter(&(v_t.clone()))? { let mut v_a: Py; 
v_a = t888;
out.push(f__iv(v_a.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))); if t902.truthy() { t902 } else { { let t901 = Py::Bool(({ let mut out = vec![]; for t890 in py::iter(&(py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_a: Py; 
v_a = t890;
out.push({ let t891 = v_a.clone(); Py::Bool({ let t892 = Py::Int(1i64); if py::lt(&t891, &t892)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t901.truthy() { t901 } else { { let t900 = { let t897 = { let t893 = py::len(&v_t.clone())?; Py::Bool({ let t894 = Py::Int(1i64); if py::gt(&t893, &t894)? { true } else { false } }) }; if !t897.truthy() { t897 } else { { let t895 = py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t896 = Py::Int(2i64); if py::lt(&t895, &t896)? { true } else { false } }) } } }; if t900.truthy() { t900 } else { { let t898 = py::getitem(&v_o.clone(), &Py::str("length"))?; Py::Bool({ let t899 = py::len(&v_t.clone())?; if py::ne(&t898, &t899) { true } else { false } }) } } } } } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_v = py::fraction(&[py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?])?;
    for t904 in py::iter(&(py::reversed(&py::slice(&v_t.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?))? {
        v_a = t904;
        v_v = py::add(&v_a.clone(), &py::truediv(&Py::Int(1i64), &v_v.clone())?)?;
    }
    return Ok({ let t905 = v_v.clone(); Py::Bool({ let t906 = py::fraction(&[py::int(&py::getitem(&v_i.clone(), &Py::str("numerator"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("denominator"))?)?])?; if py::eq(&t905, &t906) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_cfs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_per: Py = Py::None;
    let mut v_q: Py = Py::None;
    let t935 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_o.clone(), &Py::str("a0"))?, py::getitem(&v_o.clone(), &Py::str("period"))?])).items()?;
    if t935.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t935[0].clone();
    v_a0 = t935[1].clone();
    v_per = t935[2].clone();
    if (py::not(&{ let t946 = f__iv(v_a0.clone())?; if !t946.truthy() { t946 } else { { let t945 = { let t937 = v_a0.clone(); Py::Bool({ let t938 = py::isqrt(&v_d.clone())?; if py::eq(&t937, &t938) { true } else { false } }) }; if !t945.truthy() { t945 } else { { let t944 = v_per.clone(); if !t944.truthy() { t944 } else { { let t943 = { let t939 = py::getitem(&v_per.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t940 = py::mul(&Py::Int(2i64), &v_a0.clone())?; if py::eq(&t939, &t940) { true } else { false } }) }; if !t943.truthy() { t943 } else { { let t941 = py::getitem(&v_o.clone(), &Py::str("period_length"))?; Py::Bool({ let t942 = py::len(&v_per.clone())?; if py::eq(&t941, &t942) { true } else { false } }) } } } } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t947 = py::slice(&v_per.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?; Py::Bool({ let t948 = py::slice(&py::slice(&v_per.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?; if py::ne(&t947, &t948) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    let t953 = (f__convergent(py::add(&Py::list(vec![v_a0.clone()]), &py::slice(&v_per.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?)?).items()?;
    if t953.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t953[0].clone();
    v_q = t953[1].clone();
    return Ok({ let t954 = py::sub(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&py::mul(&v_d.clone(), &v_q.clone())?, &v_q.clone())?)?; Py::Bool({ let t955 = py::pow(&py::neg(&Py::Int(1i64))?, &py::len(&v_per.clone())?)?; if py::eq(&t954, &t955) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_chrom(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_order: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    let t996 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("chromatic_number"))?, py::getitem(&v_o.clone(), &Py::str("colouring"))?])).items()?;
    if t996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t996[0].clone();
    v_col = t996[1].clone();
    if (py::not(&{ let t1010 = f__iv(v_k.clone())?; if !t1010.truthy() { t1010 } else { { let t1009 = { let t998 = py::set(&v_col.clone())?; Py::Bool({ let t999 = py::set(&v_adj.clone())?; if py::eq(&t998, &t999) { true } else { false } }) }; if !t1009.truthy() { t1009 } else { { let t1008 = Py::Bool(({ let mut out = vec![]; for t1000 in py::iter(&(py::method(&v_col.clone(), "values", &[])?))? { let mut v_c: Py; 
v_c = t1000;
out.push({ let t1001 = Py::Int(1i64); Py::Bool({ let t1002 = v_c.clone(); if py::le(&t1001, &t1002)? { { let t1003 = v_k.clone(); if py::le(&t1002, &t1003)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1008.truthy() { t1008 } else { Py::Bool(({ let mut out = vec![]; for t1004 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t1004;
for t1005 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t1005;
out.push({ let t1006 = py::getitem(&v_col.clone(), &v_u.clone())?; Py::Bool({ let t1007 = py::getitem(&v_col.clone(), &v_v.clone())?; if py::ne(&t1006, &t1007) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_order = py::sorted_key(&v_adj.clone(), &{ let v_adj = v_adj.clone(); let v_col = v_col.clone(); let v_i = v_i.clone(); let v_k = v_k.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?)?, f__nkey(v_v.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t1015 = { let t1011 = v_k.clone(); Py::Bool({ let t1012 = Py::Int(1i64); if py::eq(&t1011, &t1012) { true } else { false } }) }; if t1015.truthy() { t1015 } else { { let t1013 = f__colourable(v_order.clone(), v_adj.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1014 = Py::None; if py::is(&t1013, &t1014) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_clique(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_c = py::getitem(&v_o.clone(), &Py::str("clique"))?;
    if (py::not(&{ let t1073 = f__iv(py::getitem(&v_o.clone(), &Py::str("clique_number"))?)?; if !t1073.truthy() { t1073 } else { { let t1072 = { let t1063 = py::len(&v_c.clone())?; Py::Bool({ let t1064 = py::getitem(&v_o.clone(), &Py::str("clique_number"))?; if py::eq(&t1063, &t1064) { { let t1065 = py::len(&py::set(&v_c.clone())?)?; if py::eq(&t1064, &t1065) { true } else { false } } } else { false } }) }; if !t1072.truthy() { t1072 } else { Py::Bool(({ let mut out = vec![]; for t1066 in py::iter(&(v_c.clone()))? { let mut v_a: Py; 
v_a = t1066;
for t1067 in py::iter(&(v_c.clone()))? { let mut v_b: Py; 
v_b = t1067;
if ({ let t1068 = v_a.clone(); Py::Bool({ let t1069 = v_b.clone(); if py::ne(&t1068, &t1069) { true } else { false } }) }).truthy() { out.push({ let t1070 = v_b.clone(); Py::Bool({ let t1071 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t1071, &t1070)? { true } else { false } }) }); }}} Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t1074 = py::getitem(&v_o.clone(), &Py::str("clique_number"))?; Py::Bool({ let t1075 = f__brute_max(py::sorted_key(&v_adj.clone(), &{ let v_adj = v_adj.clone(); let v_c = v_c.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::neg(&py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?, { let v_adj = v_adj.clone(); let v_c = v_c.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_ch: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(Py::Bool(({ let mut out = vec![]; for t1080 in py::iter(&(v_ch.clone()))? { let mut v_u: Py; 
v_u = t1080;
out.push({ let t1081 = v_v.clone(); Py::Bool({ let t1082 = py::getitem(&v_adj.clone(), &v_u.clone())?; if py::contains(&t1082, &t1081)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?; if py::eq(&t1074, &t1075) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_crank(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_extra: Py = Py::None;
    let mut v_find: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let t1280 = (Py::tuple(vec![Py::dict(vec![]), Py::Int(0i64)])).items()?;
    if t1280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_parent = t1280[0].clone();
    v_extra = t1280[1].clone();
    v_find = { let v_extra = v_extra.clone(); let v_find = v_find.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_parent = v_parent.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t1281 = py::method(&v_parent.clone(), "setdefault", &[v_x.clone(), v_x.clone()])?; Py::Bool({ let t1282 = v_x.clone(); if py::ne(&t1281, &t1282) { true } else { false } }) }).truthy() {
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t1283 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? {
        v_e = t1283;
        let t1284 = (Py::tuple(vec![py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?))])?, py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])?])).items()?;
        if t1284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1284[0].clone();
        v_b = t1284[1].clone();
        if ({ let t1285 = v_a.clone(); Py::Bool({ let t1286 = v_b.clone(); if py::eq(&t1285, &t1286) { true } else { false } }) }).truthy() {
            v_extra = py::iadd(&v_extra, &Py::Int(1i64))?;
        } else {
            py::setitem(&v_parent.clone(), &v_a.clone(), v_b.clone())?;
        }
    }
    return Ok({ let t1295 = f__iv(py::getitem(&v_o.clone(), &Py::str("cyclomatic_number"))?)?; if !t1295.truthy() { t1295 } else { { let t1294 = { let t1288 = py::getitem(&v_o.clone(), &Py::str("cyclomatic_number"))?; Py::Bool({ let t1289 = v_extra.clone(); if py::eq(&t1288, &t1289) { true } else { false } }) }; if !t1294.truthy() { t1294 } else { { let t1290 = py::getitem(&v_o.clone(), &Py::str("has_cycle"))?; Py::Bool({ let t1291 = { let t1292 = v_extra.clone(); Py::Bool({ let t1293 = Py::Int(0i64); if py::gt(&t1292, &t1293)? { true } else { false } }) }; if py::is(&t1290, &t1291) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_crt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1226 = (Py::tuple(vec![{ let mut out = vec![]; for t1224 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("residues"))?))? { let mut v_v: Py; 
v_v = t1224;
out.push(py::int(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1225 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("moduli"))?))? { let mut v_v: Py; 
v_v = t1225;
out.push(py::int(&v_v.clone())?); } Py::list(out) }])).items()?;
    if t1226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rs = t1226[0].clone();
    v_ms = t1226[1].clone();
    let t1227 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("modulus"))?])).items()?;
    if t1227.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1227[0].clone();
    v_M = t1227[1].clone();
    return Ok({ let t1241 = f__iv(v_x.clone())?; if !t1241.truthy() { t1241 } else { { let t1240 = f__iv(v_M.clone())?; if !t1240.truthy() { t1240 } else { { let t1239 = { let t1229 = v_M.clone(); Py::Bool({ let t1230 = py::gcd_lcm("lcm", &({ let mut v: Vec<Py> = vec![]; v.extend((v_ms.clone()).items()?); v })[..])?; if py::eq(&t1229, &t1230) { true } else { false } }) }; if !t1239.truthy() { t1239 } else { { let t1238 = { let t1231 = Py::Int(0i64); Py::Bool({ let t1232 = v_x.clone(); if py::le(&t1231, &t1232)? { { let t1233 = v_M.clone(); if py::lt(&t1232, &t1233)? { true } else { false } } } else { false } }) }; if !t1238.truthy() { t1238 } else { Py::Bool(({ let mut out = vec![]; for t1234 in py::iter(&(py::zip(&[v_rs.clone(), v_ms.clone()])?))? { let mut v_m: Py; let mut v_r: Py; 
let t1235 = (t1234).items()?;
if t1235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1235[0].clone();
v_m = t1235[1].clone();
out.push({ let t1236 = py::modulo(&py::sub(&v_x.clone(), &v_r.clone())?, &v_m.clone())?; Py::Bool({ let t1237 = Py::Int(0i64); if py::eq(&t1236, &t1237) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_dcyc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_color: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_visit: Py = Py::None;
    let c_found = py::cell_new();
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true), Py::Bool(false))?;
    v_color = { let mut out = vec![]; for t1477 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1477;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    py::cell_set(&c_found, Py::Bool(false));
    let rec1478 = py::rec_cell();
    v_visit = { let v_adj = v_adj.clone(); let v_color = v_color.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let c_found = c_found.clone(); let rec1478 = rec1478.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_visit: Py = py::rec_get(&rec1478)?;
            let mut v_found: Py = Py::None;
            let mut v_v: Py = Py::None;
            py::setitem(&v_color.clone(), &v_u.clone(), Py::Int(1i64))?;
            for t1479 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t1479;
                if ({ let t1480 = py::getitem(&v_color.clone(), &v_v.clone())?; Py::Bool({ let t1481 = Py::Int(1i64); if py::eq(&t1480, &t1481) { true } else { false } }) }).truthy() {
                    py::cell_set(&c_found, Py::Bool(true));
                } else {
                    if ({ let t1482 = py::getitem(&v_color.clone(), &v_v.clone())?; Py::Bool({ let t1483 = Py::Int(0i64); if py::eq(&t1482, &t1483) { true } else { false } }) }).truthy() {
                        let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
                    }
                }
            }
            py::setitem(&v_color.clone(), &v_u.clone(), Py::Int(2i64))?;
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1478, &v_visit);
    for t1484 in py::iter(&(v_adj.clone()))? {
        v_v = t1484;
        if ({ let t1485 = py::getitem(&v_color.clone(), &v_v.clone())?; Py::Bool({ let t1486 = Py::Int(0i64); if py::eq(&t1485, &t1486) { true } else { false } }) }).truthy() {
            let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
        }
    }
    return Ok({ let t1491 = { let t1487 = py::getitem(&v_o.clone(), &Py::str("has_cycle"))?; Py::Bool({ let t1488 = py::cell_get(&c_found)?; if py::is(&t1487, &t1488) { true } else { false } }) }; if !t1491.truthy() { t1491 } else { { let t1489 = py::getitem(&v_o.clone(), &Py::str("is_dag"))?; Py::Bool({ let t1490 = py::not(&py::cell_get(&c_found)?); if py::is(&t1489, &t1490) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_deg(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_d = Py::dict(vec![]);
    for t2072 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? {
        v_e = t2072;
        for t2073 in py::iter(&(Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])))? {
            v_x = t2073;
            py::setitem(&v_d.clone(), &v_x.clone(), py::add(&py::method(&v_d.clone(), "get", &[v_x.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
        }
    }
    v_seq = py::sorted(&py::method(&v_d.clone(), "values", &[])?, (Py::Bool(true)).truthy())?;
    return Ok({ let t2092 = { let t2074 = py::getitem(&v_o.clone(), &Py::str("degrees"))?; Py::Bool({ let t2075 = v_d.clone(); if py::eq(&t2074, &t2075) { true } else { false } }) }; if !t2092.truthy() { t2092 } else { { let t2091 = { let t2076 = py::getitem(&v_o.clone(), &Py::str("degree_sequence"))?; Py::Bool({ let t2077 = v_seq.clone(); if py::eq(&t2076, &t2077) { true } else { false } }) }; if !t2091.truthy() { t2091 } else { { let t2090 = f__iv(py::getitem(&v_o.clone(), &Py::str("max_degree"))?)?; if !t2090.truthy() { t2090 } else { { let t2089 = { let t2079 = py::getitem(&v_o.clone(), &Py::str("max_degree"))?; Py::Bool({ let t2080 = py::getitem(&v_seq.clone(), &Py::Int(0i64))?; if py::eq(&t2079, &t2080) { true } else { false } }) }; if !t2089.truthy() { t2089 } else { { let t2088 = { let t2081 = py::getitem(&v_o.clone(), &Py::str("min_degree"))?; Py::Bool({ let t2082 = py::getitem(&v_seq.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t2081, &t2082) { true } else { false } }) }; if !t2088.truthy() { t2088 } else { { let t2087 = { let t2083 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t2084 = py::len(&py::getitem(&v_i.clone(), &Py::str("edges"))?)?; if py::eq(&t2083, &t2084) { true } else { false } }) }; if !t2087.truthy() { t2087 } else { { let t2085 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t2086 = py::len(&v_d.clone())?; if py::eq(&t2085, &t2086) { true } else { false } }) } } } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_der(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_d = py::sum(&{ let mut out = vec![]; for t1324 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1324;
out.push(py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_k.clone())?, &py::floordiv(&py::factorial(&v_n.clone())?, &py::factorial(&v_k.clone())?)?)?); } Py::list(out) }, None)?;
    return Ok({ let t1331 = f__iv(py::getitem(&v_o.clone(), &Py::str("derangements"))?)?; if !t1331.truthy() { t1331 } else { { let t1330 = { let t1326 = py::getitem(&v_o.clone(), &Py::str("derangements"))?; Py::Bool({ let t1327 = v_d.clone(); if py::eq(&t1326, &t1327) { true } else { false } }) }; if !t1330.truthy() { t1330 } else { { let t1328 = py::getitem(&v_o.clone(), &Py::str("probability"))?; Py::Bool({ let t1329 = py::float(&py::fraction(&[v_d.clone(), py::factorial(&v_n.clone())?])?)?; if py::eq(&t1328, &t1329) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_dfs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_visit: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool((py::method(&v_i.clone(), "get", &[Py::str("directed")])?).truthy()), Py::Bool(false))?;
    v_s = py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?;
    let t1361 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, Py::list(vec![])])).items()?;
    if t1361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_seen = t1361[0].clone();
    v_order = t1361[1].clone();
    let rec1362 = py::rec_cell();
    v_visit = { let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_order = v_order.clone(); let v_s = v_s.clone(); let v_seen = v_seen.clone(); let rec1362 = rec1362.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_visit: Py = py::rec_get(&rec1362)?;
            let mut v_v: Py = Py::None;
            let _ = py::method(&v_seen.clone(), "add", &[v_u.clone()])?;
            let _ = py::method(&v_order.clone(), "append", &[v_u.clone()])?;
            for t1363 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &v_u.clone())?)?))? {
                v_v = t1363;
                if ({ let t1364 = v_v.clone(); Py::Bool({ let t1365 = v_seen.clone(); if !py::contains(&t1365, &t1364)? { true } else { false } }) }).truthy() {
                    let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1362, &v_visit);
    let _ = py::call(&v_visit.clone(), &[v_s.clone()])?;
    return Ok({ let t1372 = { let t1366 = py::getitem(&v_o.clone(), &Py::str("order"))?; Py::Bool({ let t1367 = v_order.clone(); if py::eq(&t1366, &t1367) { true } else { false } }) }; if !t1372.truthy() { t1372 } else { { let t1371 = f__iv(py::getitem(&v_o.clone(), &Py::str("reached"))?)?; if !t1371.truthy() { t1371 } else { { let t1369 = py::getitem(&v_o.clone(), &Py::str("reached"))?; Py::Bool({ let t1370 = py::len(&v_order.clone())?; if py::eq(&t1369, &t1370) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_diam(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Da: Py = Py::None;
    let mut v_Dk: Py = Py::None;
    let mut v_INF: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ecc: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_nodes = f__sorted(v_adj.clone())?;
    v_idx = { let mut out = vec![]; for t2142 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t2143 = (t2142).items()?;
if t2143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2143[0].clone();
v_v = t2143[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_INF = py::pow(&Py::Int(10i64), &Py::Int(9i64))?;
    v_D = { let mut out = vec![]; for t2144 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) { let mut v_a: Py; 
v_a = t2144;
out.push({ let mut out = vec![]; for t2145 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) { let mut v_b: Py; 
v_b = t2145;
out.push((if ({ let t2146 = v_a.clone(); Py::Bool({ let t2147 = v_b.clone(); if py::eq(&t2146, &t2147) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { (if ({ let t2148 = py::getitem(&v_nodes.clone(), &v_b.clone())?; Py::Bool({ let t2149 = py::getitem(&v_adj.clone(), &py::getitem(&v_nodes.clone(), &v_a.clone())?)?; if py::contains(&t2149, &t2148)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { v_INF.clone() }) })); } Py::list(out) }); } Py::list(out) };
    for t2150 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
        v_k = t2150;
        v_Dk = py::getitem(&v_D.clone(), &v_k.clone())?;
        for t2151 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
            v_a = t2151;
            v_Da = py::getitem(&v_D.clone(), &v_a.clone())?;
            if ({ let t2152 = py::getitem(&v_Da.clone(), &v_k.clone())?; Py::Bool({ let t2153 = v_INF.clone(); if py::lt(&t2152, &t2153)? { true } else { false } }) }).truthy() {
                for t2154 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
                    v_b = t2154;
                    if ({ let t2155 = py::add(&py::getitem(&v_Da.clone(), &v_k.clone())?, &py::getitem(&v_Dk.clone(), &v_b.clone())?)?; Py::Bool({ let t2156 = py::getitem(&v_Da.clone(), &v_b.clone())?; if py::lt(&t2155, &t2156)? { true } else { false } }) }).truthy() {
                        py::setitem(&v_Da.clone(), &v_b.clone(), py::add(&py::getitem(&v_Da.clone(), &v_k.clone())?, &py::getitem(&v_Dk.clone(), &v_b.clone())?)?)?;
                    }
                }
            }
        }
    }
    v_ecc = { let mut out = vec![]; for t2157 in py::iter(&(v_D.clone()))? { let mut v_row: Py; 
v_row = t2157;
out.push(py::minmax(&[v_row.clone()], true)?); } Py::list(out) };
    let t2158 = (Py::tuple(vec![py::minmax(&[v_ecc.clone()], true)?, py::minmax(&[v_ecc.clone()], false)?])).items()?;
    if t2158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2158[0].clone();
    v_r = t2158[1].clone();
    return Ok({ let t2177 = f__iv(py::getitem(&v_o.clone(), &Py::str("diameter"))?)?; if !t2177.truthy() { t2177 } else { { let t2176 = { let t2160 = py::getitem(&v_o.clone(), &Py::str("diameter"))?; Py::Bool({ let t2161 = v_d.clone(); if py::eq(&t2160, &t2161) { true } else { false } }) }; if !t2176.truthy() { t2176 } else { { let t2175 = { let t2162 = py::getitem(&v_o.clone(), &Py::str("radius"))?; Py::Bool({ let t2163 = v_r.clone(); if py::eq(&t2162, &t2163) { true } else { false } }) }; if !t2175.truthy() { t2175 } else { { let t2174 = { let t2164 = py::getitem(&v_o.clone(), &Py::str("center"))?; Py::Bool({ let t2165 = { let mut out = vec![]; for t2166 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2166;
if ({ let t2167 = py::getitem(&v_ecc.clone(), &py::getitem(&v_idx.clone(), &v_v.clone())?)?; Py::Bool({ let t2168 = v_r.clone(); if py::eq(&t2167, &t2168) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }; if py::eq(&t2164, &t2165) { true } else { false } }) }; if !t2174.truthy() { t2174 } else { { let t2169 = py::getitem(&v_o.clone(), &Py::str("periphery"))?; Py::Bool({ let t2170 = { let mut out = vec![]; for t2171 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2171;
if ({ let t2172 = py::getitem(&v_ecc.clone(), &py::getitem(&v_idx.clone(), &v_v.clone())?)?; Py::Bool({ let t2173 = v_d.clone(); if py::eq(&t2172, &t2173) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }; if py::eq(&t2169, &t2170) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_dioph(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t2982 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])).items()?;
    if t2982.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2982[0].clone();
    v_b = t2982[1].clone();
    v_c = t2982[2].clone();
    v_g = py::gcd(&v_a.clone(), &v_b.clone())?;
    if ({ let t2992 = { let t2983 = py::getitem(&v_o.clone(), &Py::str("solvable"))?; Py::Bool({ let t2984 = { let t2985 = py::modulo(&v_c.clone(), &v_g.clone())?; Py::Bool({ let t2986 = Py::Int(0i64); if py::eq(&t2985, &t2986) { true } else { false } }) }; if !py::is(&t2983, &t2984) { true } else { false } }) }; if t2992.truthy() { t2992 } else { { let t2991 = { let t2987 = py::getitem(&v_o.clone(), &Py::str("x_step"))?; Py::Bool({ let t2988 = py::floordiv(&v_b.clone(), &v_g.clone())?; if py::ne(&t2987, &t2988) { true } else { false } }) }; if t2991.truthy() { t2991 } else { { let t2989 = py::getitem(&v_o.clone(), &Py::str("y_step"))?; Py::Bool({ let t2990 = py::neg(&py::floordiv(&v_a.clone(), &v_g.clone())?)?; if py::ne(&t2989, &t2990) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if (py::not(&py::getitem(&v_o.clone(), &Py::str("solvable"))?)).truthy() {
        return Ok({ let t2997 = { let t2993 = py::getitem(&v_o.clone(), &Py::str("x0"))?; Py::Bool({ let t2994 = Py::None; if py::is(&t2993, &t2994) { true } else { false } }) }; if !t2997.truthy() { t2997 } else { { let t2995 = py::getitem(&v_o.clone(), &Py::str("y0"))?; Py::Bool({ let t2996 = Py::None; if py::is(&t2995, &t2996) { true } else { false } }) } } });
    }
    return Ok({ let t3006 = f__iv(py::getitem(&v_o.clone(), &Py::str("x0"))?)?; if !t3006.truthy() { t3006 } else { { let t3005 = f__iv(py::getitem(&v_o.clone(), &Py::str("y0"))?)?; if !t3005.truthy() { t3005 } else { { let t3004 = { let t2999 = py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("x0"))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("y0"))?)?)?; Py::Bool({ let t3000 = v_c.clone(); if py::eq(&t2999, &t3000) { true } else { false } }) }; if !t3004.truthy() { t3004 } else { { let t3001 = Py::Int(0i64); Py::Bool({ let t3002 = py::getitem(&v_o.clone(), &Py::str("x0"))?; if py::le(&t3001, &t3002)? { { let t3003 = py::floordiv(&py::abs(&v_b.clone())?, &v_g.clone())?; if py::lt(&t3002, &t3003)? { true } else { false } } } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_div(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cls: Py = Py::None;
    let mut v_ds: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_proper: Py = Py::None;
    let mut v_tau: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_ds = py::getitem(&v_o.clone(), &Py::str("divisors"))?;
    v_tau = (if ({ let t1611 = v_n.clone(); Py::Bool({ let t1612 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::le(&t1611, &t1612)? { true } else { false } }) }).truthy() { py::prod(&{ let mut out = vec![]; for t1622 in py::iter(&(py::method(&f__trial_factor(v_n.clone())?, "values", &[])?))? { let mut v_e: Py; 
v_e = t1622;
out.push(py::add(&v_e.clone(), &Py::Int(1i64))?); } Py::list(out) })? } else { py::prod(&{ let mut out = vec![]; for t1623 in py::iter(&(py::method(&f__factor(v_n.clone())?, "values", &[])?))? { let mut v_e: Py; 
v_e = t1623;
out.push(py::add(&v_e.clone(), &Py::Int(1i64))?); } Py::list(out) })? });
    v_proper = py::sub(&py::getitem(&v_o.clone(), &Py::str("sum"))?, &v_n.clone())?;
    v_cls = (if ({ let t1624 = v_proper.clone(); Py::Bool({ let t1625 = v_n.clone(); if py::eq(&t1624, &t1625) { true } else { false } }) }).truthy() { Py::str("perfect") } else { (if ({ let t1626 = v_proper.clone(); Py::Bool({ let t1627 = v_n.clone(); if py::gt(&t1626, &t1627)? { true } else { false } }) }).truthy() { Py::str("abundant") } else { Py::str("deficient") }) });
    return Ok({ let t1648 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t1648.truthy() { t1648 } else { { let t1647 = { let t1629 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1630 = v_tau.clone(); if py::eq(&t1629, &t1630) { { let t1631 = py::len(&v_ds.clone())?; if py::eq(&t1630, &t1631) { true } else { false } } } else { false } }) }; if !t1647.truthy() { t1647 } else { { let t1646 = { let t1632 = v_ds.clone(); Py::Bool({ let t1633 = py::sorted(&py::set(&v_ds.clone())?, (Py::Bool(false)).truthy())?; if py::eq(&t1632, &t1633) { true } else { false } }) }; if !t1646.truthy() { t1646 } else { { let t1645 = Py::Bool(({ let mut out = vec![]; for t1634 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t1634;
out.push({ let t1635 = py::modulo(&v_n.clone(), &v_d.clone())?; Py::Bool({ let t1636 = Py::Int(0i64); if py::eq(&t1635, &t1636) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1645.truthy() { t1645 } else { { let t1644 = { let t1637 = py::getitem(&v_o.clone(), &Py::str("sum"))?; Py::Bool({ let t1638 = py::sum(&v_ds.clone(), None)?; if py::eq(&t1637, &t1638) { true } else { false } }) }; if !t1644.truthy() { t1644 } else { { let t1643 = { let t1639 = py::getitem(&v_o.clone(), &Py::str("proper_sum"))?; Py::Bool({ let t1640 = v_proper.clone(); if py::eq(&t1639, &t1640) { true } else { false } }) }; if !t1643.truthy() { t1643 } else { { let t1641 = py::getitem(&v_o.clone(), &Py::str("classification"))?; Py::Bool({ let t1642 = v_cls.clone(); if py::eq(&t1641, &t1642) { true } else { false } }) } } } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_dlog(mut v_i: Py, mut v_o: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1531 = (Py::tuple(vec![py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("base"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?)?, py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("target"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?])).items()?;
    if t1531.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1531[0].clone();
    v_t = t1531[1].clone();
    v_m = t1531[2].clone();
    v_x = py::getitem(&v_o.clone(), &Py::str("x"))?;
    if (py::not(&{ let t1538 = f__iv(v_x.clone())?; if !t1538.truthy() { t1538 } else { { let t1537 = { let t1533 = v_x.clone(); Py::Bool({ let t1534 = Py::Int(0i64); if py::ge(&t1533, &t1534)? { true } else { false } }) }; if !t1537.truthy() { t1537 } else { { let t1535 = py::pow_mod(&v_b.clone(), &v_x.clone(), &v_m.clone())?; Py::Bool({ let t1536 = v_t.clone(); if py::eq(&t1535, &t1536) { true } else { false } }) } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_cur = py::modulo(&Py::Int(1i64), &v_m.clone())?;
    for t1539 in py::range_iter(&[(v_x.clone()).i()?]) {
        v__ = t1539;
        if ({ let t1540 = v_cur.clone(); Py::Bool({ let t1541 = v_t.clone(); if py::eq(&t1540, &t1541) { true } else { false } }) }).truthy() {
            return Ok(Py::Bool(false));
        }
        v_cur = py::modulo(&py::mul(&v_cur.clone(), &v_b.clone())?, &v_m.clone())?;
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_elg(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1701 = ({ let mut out = vec![]; for t1700 in py::iter(&(Py::tuple(vec![Py::str("p"), Py::str("g"), Py::str("x"), Py::str("k"), Py::str("m")])))? { let mut v_s: Py; 
v_s = t1700;
out.push(py::int(&py::getitem(&v_i.clone(), &v_s.clone())?)?); } Py::list(out) }).items()?;
    if t1701.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1701[0].clone();
    v_g = t1701[1].clone();
    v_x = t1701[2].clone();
    v_k = t1701[3].clone();
    v_m = t1701[4].clone();
    return Ok({ let t1711 = f__iv(py::getitem(&v_o.clone(), &Py::str("c2"))?)?; if !t1711.truthy() { t1711 } else { { let t1710 = { let t1703 = py::getitem(&v_o.clone(), &Py::str("h"))?; Py::Bool({ let t1704 = py::pow_mod(&v_g.clone(), &v_x.clone(), &v_p.clone())?; if py::eq(&t1703, &t1704) { true } else { false } }) }; if !t1710.truthy() { t1710 } else { { let t1709 = { let t1705 = py::getitem(&v_o.clone(), &Py::str("c1"))?; Py::Bool({ let t1706 = py::pow_mod(&v_g.clone(), &v_k.clone(), &v_p.clone())?; if py::eq(&t1705, &t1706) { true } else { false } }) }; if !t1709.truthy() { t1709 } else { { let t1707 = py::modulo(&py::mul(&py::getitem(&v_o.clone(), &Py::str("c2"))?, &py::pow_mod(&py::getitem(&v_o.clone(), &Py::str("c1"))?, &py::sub(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::modulo(&v_x.clone(), &py::sub(&v_p.clone(), &Py::Int(1i64))?)?)?, &v_p.clone())?)?, &v_p.clone())?; Py::Bool({ let t1708 = v_m.clone(); if py::eq(&t1707, &t1708) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_ext(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t1871 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?])).items()?;
    if t1871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1871[0].clone();
    v_b = t1871[1].clone();
    v_g = py::getitem(&v_o.clone(), &Py::str("gcd"))?;
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t1872 in py::iter(&(Py::tuple(vec![Py::str("gcd"), Py::str("x"), Py::str("y"), Py::str("x_min"), Py::str("y_min")])))? { let mut v_k: Py; 
v_k = t1872;
out.push(f__iv(py::getitem(&v_o.clone(), &v_k.clone())?)?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t1885 = { let t1874 = v_g.clone(); Py::Bool({ let t1875 = py::gcd(&v_a.clone(), &v_b.clone())?; if py::eq(&t1874, &t1875) { true } else { false } }) }; if !t1885.truthy() { t1885 } else { { let t1884 = { let t1876 = py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("y"))?)?)?; Py::Bool({ let t1877 = v_g.clone(); if py::eq(&t1876, &t1877) { true } else { false } }) }; if !t1884.truthy() { t1884 } else { { let t1883 = { let t1878 = py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("x_min"))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("y_min"))?)?)?; Py::Bool({ let t1879 = v_g.clone(); if py::eq(&t1878, &t1879) { true } else { false } }) }; if !t1883.truthy() { t1883 } else { { let t1880 = Py::Int(0i64); Py::Bool({ let t1881 = py::getitem(&v_o.clone(), &Py::str("x_min"))?; if py::le(&t1880, &t1881)? { { let t1882 = py::floordiv(&v_b.clone(), &v_g.clone())?; if py::lt(&t1881, &t1882)? { true } else { false } } } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_factor(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_fs: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ps: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_fs = py::getitem(&v_o.clone(), &Py::str("factorization"))?;
    v_ps = { let mut out = vec![]; for t4935 in py::iter(&(v_fs.clone()))? { let mut v__e: Py; let mut v_p: Py; 
let t4936 = (t4935).items()?;
if t4936.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4936[0].clone();
v__e = t4936[1].clone();
out.push(v_p.clone()); } Py::list(out) };
    if ({ let t4962 = { let t4937 = v_ps.clone(); Py::Bool({ let t4938 = py::sorted(&py::set(&v_ps.clone())?, (Py::Bool(false)).truthy())?; if py::ne(&t4937, &t4938) { true } else { false } }) }; if t4962.truthy() { t4962 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4939 in py::iter(&(v_fs.clone()))? { let mut v_e: Py; let mut v_p: Py; 
let t4940 = (t4939).items()?;
if t4940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4940[0].clone();
v_e = t4940[1].clone();
out.push({ let t4961 = f__iv(v_p.clone())?; if !t4961.truthy() { t4961 } else { { let t4960 = f__iv(v_e.clone())?; if !t4960.truthy() { t4960 } else { { let t4959 = { let t4942 = v_e.clone(); Py::Bool({ let t4943 = Py::Int(1i64); if py::ge(&t4942, &t4943)? { true } else { false } }) }; if !t4959.truthy() { t4959 } else { f__is_prime_alt(v_p.clone())? } } } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t4981 = { let t4963 = py::prod(&{ let mut out = vec![]; for t4965 in py::iter(&(v_fs.clone()))? { let mut v_e: Py; let mut v_p: Py; 
let t4966 = (t4965).items()?;
if t4966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4966[0].clone();
v_e = t4966[1].clone();
out.push(py::pow(&v_p.clone(), &v_e.clone())?); } Py::list(out) })?; Py::Bool({ let t4964 = v_n.clone(); if py::eq(&t4963, &t4964) { true } else { false } }) }; if !t4981.truthy() { t4981 } else { { let t4980 = { let t4967 = py::getitem(&v_o.clone(), &Py::str("largest_prime_factor"))?; Py::Bool({ let t4968 = py::getitem(&v_ps.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t4967, &t4968) { true } else { false } }) }; if !t4980.truthy() { t4980 } else { { let t4979 = f__iv(py::getitem(&v_o.clone(), &Py::str("largest_prime_factor"))?)?; if !t4979.truthy() { t4979 } else { { let t4978 = { let t4969 = py::getitem(&v_o.clone(), &Py::str("smallest_prime_factor"))?; Py::Bool({ let t4970 = py::getitem(&v_ps.clone(), &Py::Int(0i64))?; if py::eq(&t4969, &t4970) { true } else { false } }) }; if !t4978.truthy() { t4978 } else { { let t4977 = { let t4971 = py::getitem(&v_o.clone(), &Py::str("distinct_primes"))?; Py::Bool({ let t4972 = py::len(&v_ps.clone())?; if py::eq(&t4971, &t4972) { true } else { false } }) }; if !t4977.truthy() { t4977 } else { { let t4973 = py::getitem(&v_o.clone(), &Py::str("prime_factors_with_multiplicity"))?; Py::Bool({ let t4974 = py::sum(&{ let mut out = vec![]; for t4975 in py::iter(&(v_fs.clone()))? { let mut v__p: Py; let mut v_e: Py; 
let t4976 = (t4975).items()?;
if t4976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__p = t4976[0].clone();
v_e = t4976[1].clone();
out.push(v_e.clone()); } Py::list(out) }, None)?; if py::eq(&t4973, &t4974) { true } else { false } }) } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_gcd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_xs = { let mut out = vec![]; for t1955 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("numbers"))?))? { let mut v_v: Py; 
v_v = t1955;
out.push(py::int(&v_v.clone())?); } Py::list(out) };
    let t1956 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("gcd"))?, py::getitem(&v_o.clone(), &Py::str("lcm"))?])).items()?;
    if t1956.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1956[0].clone();
    v_l = t1956[1].clone();
    if (py::not(&{ let t1964 = f__iv(v_g.clone())?; if !t1964.truthy() { t1964 } else { { let t1963 = f__iv(v_l.clone())?; if !t1963.truthy() { t1963 } else { { let t1962 = { let t1958 = v_g.clone(); Py::Bool({ let t1959 = Py::Int(1i64); if py::ge(&t1958, &t1959)? { true } else { false } }) }; if !t1962.truthy() { t1962 } else { { let t1960 = v_l.clone(); Py::Bool({ let t1961 = Py::Int(1i64); if py::ge(&t1960, &t1961)? { true } else { false } }) } } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t1979 = Py::Bool(({ let mut out = vec![]; for t1965 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1965;
out.push({ let t1966 = py::modulo(&v_x.clone(), &v_g.clone())?; Py::Bool({ let t1967 = Py::Int(0i64); if py::eq(&t1966, &t1967) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1979.truthy() { t1979 } else { { let t1978 = { let t1968 = py::reduce(&py::func(|a: &[Py]| -> R { py::gcd_lcm("gcd", a) }), &{ let mut out = vec![]; for t1970 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1970;
out.push(py::floordiv(&v_x.clone(), &v_g.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t1969 = Py::Int(1i64); if py::eq(&t1968, &t1969) { true } else { false } }) }; if !t1978.truthy() { t1978 } else { { let t1977 = Py::Bool(({ let mut out = vec![]; for t1971 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1971;
out.push({ let t1972 = py::modulo(&v_l.clone(), &v_x.clone())?; Py::Bool({ let t1973 = Py::Int(0i64); if py::eq(&t1972, &t1973) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1977.truthy() { t1977 } else { { let t1974 = py::reduce(&py::func(|a: &[Py]| -> R { py::gcd_lcm("gcd", a) }), &{ let mut out = vec![]; for t1976 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1976;
out.push(py::floordiv(&v_l.clone(), &v_x.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t1975 = Py::Int(1i64); if py::eq(&t1974, &t1975) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_girth(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_g = py::getitem(&v_o.clone(), &Py::str("girth"))?;
    v_nodes = f__sorted(v_adj.clone())?;
    for t2025 in py::range_iter(&[(Py::Int(3i64)).i()?, (py::add(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?).i()?]) {
        v_L = t2025;
        for t2026 in py::iter(&(v_nodes.clone()))? {
            v_s = t2026;
            v_stack = Py::list(vec![Py::tuple(vec![v_s.clone(), Py::list(vec![v_s.clone()])])]);
            while (v_stack.clone()).truthy() {
                let t2027 = (py::method(&v_stack.clone(), "pop", &[])?).items()?;
                if t2027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_u = t2027[0].clone();
                v_path = t2027[1].clone();
                if ({ let t2028 = py::len(&v_path.clone())?; Py::Bool({ let t2029 = v_L.clone(); if py::eq(&t2028, &t2029) { true } else { false } }) }).truthy() {
                    if ({ let t2030 = v_s.clone(); Py::Bool({ let t2031 = py::getitem(&v_adj.clone(), &v_u.clone())?; if py::contains(&t2031, &t2030)? { true } else { false } }) }).truthy() {
                        return Ok({ let t2035 = { let t2032 = v_g.clone(); Py::Bool({ let t2033 = v_L.clone(); if py::eq(&t2032, &t2033) { true } else { false } }) }; if !t2035.truthy() { t2035 } else { f__iv(v_g.clone())? } });
                    }
                    continue;
                }
                for t2036 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                    v_v = t2036;
                    if ({ let t2041 = { let t2037 = v_v.clone(); Py::Bool({ let t2038 = v_path.clone(); if !py::contains(&t2038, &t2037)? { true } else { false } }) }; if !t2041.truthy() { t2041 } else { { let t2039 = f__nkey(v_v.clone())?; Py::Bool({ let t2040 = f__nkey(v_s.clone())?; if py::gt(&t2039, &t2040)? { true } else { false } }) } } }).truthy() {
                        let _ = py::method(&v_stack.clone(), "append", &[Py::tuple(vec![v_v.clone(), py::add(&v_path.clone(), &Py::list(vec![v_v.clone()]))?])])?;
                    }
                }
            }
        }
    }
    return Ok({ let t2042 = v_g.clone(); Py::Bool({ let t2043 = Py::None; if py::is(&t2042, &t2043) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_graphical(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    v_d = py::sorted(&{ let mut out = vec![]; for t2268 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("degrees"))?))? { let mut v_x: Py; 
v_x = t2268;
out.push(py::int(&v_x.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?;
    v_n = py::len(&v_d.clone())?;
    v_ok = { let t2275 = { let t2269 = py::modulo(&py::sum(&v_d.clone(), None)?, &Py::Int(2i64))?; Py::Bool({ let t2270 = Py::Int(0i64); if py::eq(&t2269, &t2270) { true } else { false } }) }; if !t2275.truthy() { t2275 } else { Py::Bool(({ let mut out = vec![]; for t2271 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2271;
out.push({ let t2272 = py::sum(&py::slice(&v_d.clone(), None, Some((v_k.clone()).i()?), None)?, None)?; Py::Bool({ let t2273 = py::add(&py::mul(&v_k.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &py::sum(&{ let mut out = vec![]; for t2274 in py::iter(&(py::slice(&v_d.clone(), Some((v_k.clone()).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t2274;
out.push(py::minmax(&[v_x.clone(), v_k.clone()], false)?); } Py::list(out) }, None)?)?; if py::le(&t2272, &t2273)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } };
    return Ok({ let t2276 = py::getitem(&v_o.clone(), &Py::str("is_graphical"))?; Py::Bool({ let t2277 = v_ok.clone(); if py::is(&t2276, &t2277) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_ham(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_full: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_mask: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_nodes = f__sorted(v_adj.clone())?;
    v_n = py::len(&v_nodes.clone())?;
    if ({ let t2318 = py::getitem(&v_o.clone(), &Py::str("exists"))?; Py::Bool({ let t2319 = Py::Bool(true); if py::is(&t2318, &t2319) { true } else { false } }) }).truthy() {
        v_c = py::getitem(&v_o.clone(), &Py::str("cycle"))?;
        return Ok({ let t2329 = { let t2320 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t2321 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t2320, &t2321) { { let t2322 = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?; if py::eq(&t2321, &t2322) { true } else { false } } } else { false } }) }; if !t2329.truthy() { t2329 } else { { let t2328 = { let t2323 = py::sorted_key(&py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?; Py::Bool({ let t2324 = v_nodes.clone(); if py::eq(&t2323, &t2324) { true } else { false } }) }; if !t2328.truthy() { t2328 } else { Py::Bool(({ let mut out = vec![]; for t2325 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2325;
out.push({ let t2326 = py::getitem(&v_c.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2327 = py::getitem(&v_adj.clone(), &py::getitem(&v_c.clone(), &v_k.clone())?)?; if py::contains(&t2327, &t2326)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } });
    }
    if ({ let t2334 = { let t2330 = py::getitem(&v_o.clone(), &Py::str("exists"))?; Py::Bool({ let t2331 = Py::Bool(false); if !py::is(&t2330, &t2331) { true } else { false } }) }; if t2334.truthy() { t2334 } else { { let t2332 = py::getitem(&v_o.clone(), &Py::str("cycle"))?; Py::Bool({ let t2333 = Py::None; if !py::is(&t2332, &t2333) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t2335 = v_n.clone(); Py::Bool({ let t2336 = Py::Int(3i64); if py::lt(&t2335, &t2336)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(true));
    }
    v_idx = { let mut out = vec![]; for t2337 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t2338 = (t2337).items()?;
if t2338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2338[0].clone();
v_v = t2338[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_dp = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::lshift(&Py::Int(1i64), &v_n.clone())?)?;
    py::setitem(&v_dp.clone(), &Py::Int(1i64), Py::Int(1i64))?;
    for t2339 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) {
        v_mask = t2339;
        if ({ let t2340 = py::not(&py::getitem(&v_dp.clone(), &v_mask.clone())?); if t2340.truthy() { t2340 } else { py::not(&py::bitand(&v_mask.clone(), &Py::Int(1i64))?) } }).truthy() {
            continue;
        }
        for t2341 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_u = t2341;
            if (py::bitand(&py::rshift(&py::getitem(&v_dp.clone(), &v_mask.clone())?, &v_u.clone())?, &Py::Int(1i64))?).truthy() {
                for t2342 in py::iter(&(py::getitem(&v_adj.clone(), &py::getitem(&v_nodes.clone(), &v_u.clone())?)?))? {
                    v_v = t2342;
                    v_j = py::getitem(&v_idx.clone(), &v_v.clone())?;
                    if (py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?)).truthy() {
                        let t2343 = v_dp.clone(); let t2344 = py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_j.clone())?)?;
                        py::setitem(&t2343, &t2344, py::bitor(&py::getitem(&t2343, &t2344)?, &py::lshift(&Py::Int(1i64), &v_j.clone())?)?)?;
                    }
                }
            }
        }
    }
    v_full = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?;
    return Ok(py::not(&Py::Bool(({ let mut out = vec![]; for t2345 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) { let mut v_u: Py; 
v_u = t2345;
out.push({ let t2348 = py::bitand(&py::rshift(&py::getitem(&v_dp.clone(), &v_full.clone())?, &v_u.clone())?, &Py::Int(1i64))?; if !t2348.truthy() { t2348 } else { { let t2346 = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?; Py::Bool({ let t2347 = py::getitem(&v_adj.clone(), &py::getitem(&v_nodes.clone(), &v_u.clone())?)?; if py::contains(&t2347, &t2346)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_incl(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_ds: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_ds = py::sorted(&py::set(&{ let mut out = vec![]; for t2385 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("divisors"))?))? { let mut v_d: Py; 
v_d = t2385;
out.push(py::int(&v_d.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?;
    if ({ let t2386 = v_n.clone(); Py::Bool({ let t2387 = py::mul(&Py::Int(3i64), &py::pow(&Py::Int(10i64), &Py::Int(5i64))?)?; if py::le(&t2386, &t2387)? { true } else { false } }) }).truthy() {
        v_want = py::sum(&{ let mut out = vec![]; for t2388 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_x: Py; 
v_x = t2388;
if (Py::Bool(({ let mut out = vec![]; for t2389 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t2389;
out.push({ let t2390 = py::modulo(&v_x.clone(), &v_d.clone())?; Py::Bool({ let t2391 = Py::Int(0i64); if py::eq(&t2390, &t2391) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    } else {
        v_want = Py::Int(0i64);
        for t2392 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_ds.clone())?, &Py::Int(1i64))?).i()?]) {
            v_r = t2392;
            for t2393 in py::iter(&(py::itertools("combinations", &[v_ds.clone(), v_r.clone()])?))? {
                v_c = t2393;
                v_want = py::iadd(&v_want, &py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::add(&v_r.clone(), &Py::Int(1i64))?)?, &py::floordiv(&v_n.clone(), &py::gcd_lcm("lcm", &({ let mut v: Vec<Py> = vec![]; v.extend((v_c.clone()).items()?); v })[..])?)?)?)?;
            }
        }
    }
    return Ok({ let t2400 = f__iv(py::getitem(&v_o.clone(), &Py::str("divisible_by_any"))?)?; if !t2400.truthy() { t2400 } else { { let t2399 = { let t2395 = py::getitem(&v_o.clone(), &Py::str("divisible_by_any"))?; Py::Bool({ let t2396 = v_want.clone(); if py::eq(&t2395, &t2396) { true } else { false } }) }; if !t2399.truthy() { t2399 } else { { let t2397 = py::getitem(&v_o.clone(), &Py::str("divisible_by_none"))?; Py::Bool({ let t2398 = py::sub(&v_n.clone(), &v_want.clone())?; if py::eq(&t2397, &t2398) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_indep(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_s = py::getitem(&v_o.clone(), &Py::str("independent_set"))?;
    if (py::not(&{ let t2458 = f__iv(py::getitem(&v_o.clone(), &Py::str("independence_number"))?)?; if !t2458.truthy() { t2458 } else { { let t2457 = { let t2447 = py::len(&v_s.clone())?; Py::Bool({ let t2448 = py::getitem(&v_o.clone(), &Py::str("independence_number"))?; if py::eq(&t2447, &t2448) { { let t2449 = py::len(&py::set(&v_s.clone())?)?; if py::eq(&t2448, &t2449) { true } else { false } } } else { false } }) }; if !t2457.truthy() { t2457 } else { { let t2456 = { let t2450 = py::set(&v_s.clone())?; Py::Bool({ let t2451 = py::set(&v_adj.clone())?; if py::set_cmp(&t2450, &t2451, "<=")? { true } else { false } }) }; if !t2456.truthy() { t2456 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t2452 in py::iter(&(v_s.clone()))? { let mut v_a: Py; 
v_a = t2452;
for t2453 in py::iter(&(v_s.clone()))? { let mut v_b: Py; 
v_b = t2453;
out.push({ let t2454 = v_b.clone(); Py::Bool({ let t2455 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t2455, &t2454)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t2459 = py::getitem(&v_o.clone(), &Py::str("independence_number"))?; Py::Bool({ let t2460 = f__brute_max(py::sorted_key(&v_adj.clone(), &{ let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?, { let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_ch: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(Py::Bool(({ let mut out = vec![]; for t2465 in py::iter(&(v_ch.clone()))? { let mut v_u: Py; 
v_u = t2465;
out.push({ let t2466 = v_v.clone(); Py::Bool({ let t2467 = py::getitem(&v_adj.clone(), &v_u.clone())?; if !py::contains(&t2467, &t2466)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?; if py::eq(&t2459, &t2460) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_ipart(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_ways: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_ways = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
    for t2497 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_part = t2497;
        for t2498 in py::range_iter(&[(v_part.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_s = t2498;
            let t2499 = v_ways.clone(); let t2500 = v_s.clone();
            py::setitem(&t2499, &t2500, py::iadd(&py::getitem(&t2499, &t2500)?, &py::getitem(&v_ways.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
        }
    }
    return Ok({ let t2504 = f__iv(py::getitem(&v_o.clone(), &Py::str("partitions"))?)?; if !t2504.truthy() { t2504 } else { { let t2502 = py::getitem(&v_o.clone(), &Py::str("partitions"))?; Py::Bool({ let t2503 = py::getitem(&v_ways.clone(), &v_n.clone())?; if py::eq(&t2502, &t2503) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_isprime(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_p = py::getitem(&v_o.clone(), &Py::str("is_prime"))?;
    if ({ let t2629 = py::not(&Py::Bool(matches!(v_p.clone(), Py::Bool(_)))); if t2629.truthy() { t2629 } else { { let t2612 = v_p.clone(); Py::Bool({ let t2613 = f__is_prime_alt(v_n.clone())?; if py::ne(&t2612, &t2613) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_s = py::getitem(&v_o.clone(), &Py::str("smallest_prime_factor"))?;
    if ({ let t2630 = v_n.clone(); Py::Bool({ let t2631 = Py::Int(2i64); if py::lt(&t2630, &t2631)? { true } else { false } }) }).truthy() {
        return Ok({ let t2632 = v_s.clone(); Py::Bool({ let t2633 = Py::None; if py::is(&t2632, &t2633) { true } else { false } }) });
    }
    return Ok({ let t2643 = f__iv(v_s.clone())?; if !t2643.truthy() { t2643 } else { { let t2642 = { let t2635 = py::modulo(&v_n.clone(), &v_s.clone())?; Py::Bool({ let t2636 = Py::Int(0i64); if py::eq(&t2635, &t2636) { true } else { false } }) }; if !t2642.truthy() { t2642 } else { { let t2641 = f__is_prime_alt(v_s.clone())?; if !t2641.truthy() { t2641 } else { { let t2637 = { let t2639 = v_s.clone(); Py::Bool({ let t2640 = v_n.clone(); if py::eq(&t2639, &t2640) { true } else { false } }) }; Py::Bool({ let t2638 = v_p.clone(); if py::eq(&t2637, &t2638) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_isqrt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2530 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("isqrt"))?])).items()?;
    if t2530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2530[0].clone();
    v_r = t2530[1].clone();
    return Ok({ let t2546 = f__iv(v_r.clone())?; if !t2546.truthy() { t2546 } else { { let t2545 = { let t2532 = v_r.clone(); Py::Bool({ let t2533 = Py::Int(0i64); if py::ge(&t2532, &t2533)? { true } else { false } }) }; if !t2545.truthy() { t2545 } else { { let t2544 = { let t2534 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t2535 = v_n.clone(); if py::le(&t2534, &t2535)? { { let t2536 = py::pow(&py::add(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?; if py::lt(&t2535, &t2536)? { true } else { false } } } else { false } }) }; if !t2544.truthy() { t2544 } else { { let t2543 = { let t2537 = py::getitem(&v_o.clone(), &Py::str("remainder"))?; Py::Bool({ let t2538 = py::sub(&v_n.clone(), &py::mul(&v_r.clone(), &v_r.clone())?)?; if py::eq(&t2537, &t2538) { true } else { false } }) }; if !t2543.truthy() { t2543 } else { { let t2539 = py::getitem(&v_o.clone(), &Py::str("is_square"))?; Py::Bool({ let t2540 = { let t2541 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t2542 = v_n.clone(); if py::eq(&t2541, &t2542) { true } else { false } }) }; if py::is(&t2539, &t2540) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_jacobi(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t2748 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?])).items()?;
    if t2748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2748[0].clone();
    v_n = t2748[1].clone();
    v_val = Py::Int(1i64);
    for t2787 in py::iter(&(py::method(&f__factor(v_n.clone())?, "items", &[])?))? {
        let t2788 = (t2787).items()?;
        if t2788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t2788[0].clone();
        v_e = t2788[1].clone();
        v_r = py::pow_mod(&py::modulo(&v_a.clone(), &v_p.clone())?, &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?;
        v_val = py::mul(&v_val, &py::pow(&py::getitem(&Py::dict(vec![(Py::Int(0i64), Py::Int(0i64)), (Py::Int(1i64), Py::Int(1i64)), (py::sub(&v_p.clone(), &Py::Int(1i64))?, py::neg(&Py::Int(1i64))?)]), &v_r.clone())?, &v_e.clone())?)?;
    }
    return Ok({ let t2792 = f__iv(py::getitem(&v_o.clone(), &Py::str("symbol"))?)?; if !t2792.truthy() { t2792 } else { { let t2790 = py::getitem(&v_o.clone(), &Py::str("symbol"))?; Py::Bool({ let t2791 = v_val.clone(); if py::eq(&t2790, &t2791) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_legendre(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t2885 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?])).items()?;
    if t2885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2885[0].clone();
    v_p = t2885[1].clone();
    v_e = py::pow_mod(&py::modulo(&v_a.clone(), &v_p.clone())?, &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?;
    return Ok({ let t2889 = f__iv(py::getitem(&v_o.clone(), &Py::str("symbol"))?)?; if !t2889.truthy() { t2889 } else { { let t2887 = py::getitem(&v_o.clone(), &Py::str("symbol"))?; Py::Bool({ let t2888 = py::method(&Py::dict(vec![(Py::Int(0i64), Py::Int(0i64)), (Py::Int(1i64), Py::Int(1i64)), (py::sub(&v_p.clone(), &Py::Int(1i64))?, py::neg(&Py::Int(1i64))?)]), "get", &[v_e.clone(), Py::Int(99i64)])?; if py::eq(&t2887, &t2888) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_lincong(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t2923 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?])).items()?;
    if t2923.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2923[0].clone();
    v_b = t2923[1].clone();
    v_m = t2923[2].clone();
    v_g = py::gcd(&v_a.clone(), &v_m.clone())?;
    if ({ let t2924 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2925 = (if ({ let t2926 = py::modulo(&v_b.clone(), &v_g.clone())?; Py::Bool({ let t2927 = Py::Int(0i64); if py::eq(&t2926, &t2927) { true } else { false } }) }).truthy() { v_g.clone() } else { Py::Int(0i64) }); if py::ne(&t2924, &t2925) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t2928 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2929 = Py::Int(0i64); if py::eq(&t2928, &t2929) { true } else { false } }) }).truthy() {
        return Ok({ let t2934 = { let t2930 = py::getitem(&v_o.clone(), &Py::str("smallest"))?; Py::Bool({ let t2931 = Py::None; if py::is(&t2930, &t2931) { true } else { false } }) }; if !t2934.truthy() { t2934 } else { { let t2932 = py::getitem(&v_o.clone(), &Py::str("step"))?; Py::Bool({ let t2933 = Py::None; if py::is(&t2932, &t2933) { true } else { false } }) } } });
    }
    return Ok({ let t2945 = f__iv(py::getitem(&v_o.clone(), &Py::str("smallest"))?)?; if !t2945.truthy() { t2945 } else { { let t2944 = { let t2936 = py::getitem(&v_o.clone(), &Py::str("step"))?; Py::Bool({ let t2937 = py::floordiv(&v_m.clone(), &v_g.clone())?; if py::eq(&t2936, &t2937) { true } else { false } }) }; if !t2944.truthy() { t2944 } else { { let t2943 = { let t2938 = Py::Int(0i64); Py::Bool({ let t2939 = py::getitem(&v_o.clone(), &Py::str("smallest"))?; if py::le(&t2938, &t2939)? { { let t2940 = py::getitem(&v_o.clone(), &Py::str("step"))?; if py::lt(&t2939, &t2940)? { true } else { false } } } else { false } }) }; if !t2943.truthy() { t2943 } else { { let t2941 = py::modulo(&py::sub(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("smallest"))?)?, &v_b.clone())?, &v_m.clone())?; Py::Bool({ let t2942 = Py::Int(0i64); if py::eq(&t2941, &t2942) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_mobius(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sq: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_f = (if ({ let t4081 = v_n.clone(); Py::Bool({ let t4082 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::lt(&t4081, &t4082)? { true } else { false } }) }).truthy() { f__trial_factor(v_n.clone())? } else { f__factor(v_n.clone())? });
    v_sq = Py::Bool(({ let mut out = vec![]; for t4092 in py::iter(&(py::method(&v_f.clone(), "values", &[])?))? { let mut v_e: Py; 
v_e = t4092;
out.push({ let t4093 = v_e.clone(); Py::Bool({ let t4094 = Py::Int(1i64); if py::eq(&t4093, &t4094) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()));
    return Ok({ let t4101 = f__iv(py::getitem(&v_o.clone(), &Py::str("mu"))?)?; if !t4101.truthy() { t4101 } else { { let t4100 = { let t4096 = py::getitem(&v_o.clone(), &Py::str("squarefree"))?; Py::Bool({ let t4097 = v_sq.clone(); if py::is(&t4096, &t4097) { true } else { false } }) }; if !t4100.truthy() { t4100 } else { { let t4098 = py::getitem(&v_o.clone(), &Py::str("mu"))?; Py::Bool({ let t4099 = (if (py::not(&v_sq.clone())).truthy() { Py::Int(0i64) } else { py::pow(&py::neg(&Py::Int(1i64))?, &py::len(&v_f.clone())?)? }); if py::eq(&t4098, &t4099) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_modred(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4197 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?])).items()?;
    if t4197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4197[0].clone();
    v_m = t4197[1].clone();
    return Ok({ let t4206 = f__iv(py::getitem(&v_o.clone(), &Py::str("remainder"))?)?; if !t4206.truthy() { t4206 } else { { let t4205 = f__iv(py::getitem(&v_o.clone(), &Py::str("quotient"))?)?; if !t4205.truthy() { t4205 } else { { let t4204 = { let t4199 = Py::Int(0i64); Py::Bool({ let t4200 = py::getitem(&v_o.clone(), &Py::str("remainder"))?; if py::le(&t4199, &t4200)? { { let t4201 = v_m.clone(); if py::lt(&t4200, &t4201)? { true } else { false } } } else { false } }) }; if !t4204.truthy() { t4204 } else { { let t4202 = py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("quotient"))?, &v_m.clone())?, &py::getitem(&v_o.clone(), &Py::str("remainder"))?)?; Py::Bool({ let t4203 = v_a.clone(); if py::eq(&t4202, &t4203) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_neck(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ref: Py = Py::None;
    let mut v_rot: Py = Py::None;
    let t4456 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("beads"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("colours"))?)?])).items()?;
    if t4456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4456[0].clone();
    v_k = t4456[1].clone();
    v_rot = py::sum(&{ let mut out = vec![]; for t4457 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4457;
out.push(py::pow(&v_k.clone(), &py::gcd(&v_j.clone(), &v_n.clone())?)?); } Py::list(out) }, None)?;
    v_ref = (if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::mul(&v_n.clone(), &py::pow(&v_k.clone(), &py::floordiv(&py::add(&v_n.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?)? } else { py::mul(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &py::add(&py::pow(&v_k.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)?, &py::pow(&v_k.clone(), &py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?)? });
    return Ok({ let t4465 = f__iv(py::getitem(&v_o.clone(), &Py::str("necklaces"))?)?; if !t4465.truthy() { t4465 } else { { let t4464 = f__iv(py::getitem(&v_o.clone(), &Py::str("bracelets"))?)?; if !t4464.truthy() { t4464 } else { { let t4463 = { let t4459 = py::mul(&py::getitem(&v_o.clone(), &Py::str("necklaces"))?, &v_n.clone())?; Py::Bool({ let t4460 = v_rot.clone(); if py::eq(&t4459, &t4460) { true } else { false } }) }; if !t4463.truthy() { t4463 } else { { let t4461 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("bracelets"))?, &Py::Int(2i64))?, &v_n.clone())?; Py::Bool({ let t4462 = py::add(&v_rot.clone(), &v_ref.clone())?; if py::eq(&t4461, &t4462) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_next(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4511 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("prime"))?])).items()?;
    if t4511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4511[0].clone();
    v_p = t4511[1].clone();
    return Ok({ let t4533 = f__iv(v_p.clone())?; if !t4533.truthy() { t4533 } else { { let t4532 = { let t4513 = v_p.clone(); Py::Bool({ let t4514 = v_n.clone(); if py::gt(&t4513, &t4514)? { true } else { false } }) }; if !t4532.truthy() { t4532 } else { { let t4531 = f__is_prime_alt(v_p.clone())?; if !t4531.truthy() { t4531 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4530 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?, (v_p.clone()).i()?]) { let mut v_k: Py; 
v_k = t4530;
out.push(f__is_prime_alt(v_k.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_order(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4334 = (Py::tuple(vec![py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("order"))?])).items()?;
    if t4334.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4334[0].clone();
    v_n = t4334[1].clone();
    v_k = t4334[2].clone();
    return Ok({ let t4354 = f__iv(v_k.clone())?; if !t4354.truthy() { t4354 } else { { let t4353 = { let t4336 = v_k.clone(); Py::Bool({ let t4337 = Py::Int(1i64); if py::ge(&t4336, &t4337)? { true } else { false } }) }; if !t4353.truthy() { t4353 } else { { let t4352 = { let t4338 = py::pow_mod(&v_a.clone(), &v_k.clone(), &v_n.clone())?; Py::Bool({ let t4339 = Py::Int(1i64); if py::eq(&t4338, &t4339) { true } else { false } }) }; if !t4352.truthy() { t4352 } else { Py::Bool(({ let mut out = vec![]; for t4349 in py::iter(&(f__trial_factor(v_k.clone())?))? { let mut v_p: Py; 
v_p = t4349;
out.push({ let t4350 = py::pow_mod(&v_a.clone(), &py::floordiv(&v_k.clone(), &v_p.clone())?, &v_n.clone())?; Py::Bool({ let t4351 = Py::Int(1i64); if py::ne(&t4350, &t4351) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_w = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
    for t4591 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?, (Py::Int(2i64)).i()?]) {
        v_part = t4591;
        for t4592 in py::range_iter(&[(v_part.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_s = t4592;
            let t4593 = v_w.clone(); let t4594 = v_s.clone();
            py::setitem(&t4593, &t4594, py::iadd(&py::getitem(&t4593, &t4594)?, &py::getitem(&v_w.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
        }
    }
    return Ok({ let t4598 = f__iv(py::getitem(&v_o.clone(), &Py::str("partitions"))?)?; if !t4598.truthy() { t4598 } else { { let t4596 = py::getitem(&v_o.clone(), &Py::str("partitions"))?; Py::Bool({ let t4597 = py::getitem(&v_w.clone(), &v_n.clone())?; if py::eq(&t4596, &t4597) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pdecode(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_seq: Py = Py::None;
    v_seq = { let mut out = vec![]; for t5120 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("sequence"))?))? { let mut v_s: Py; 
v_s = t5120;
out.push(py::int(&v_s.clone())?); } Py::list(out) };
    v_n = py::add(&py::len(&v_seq.clone())?, &Py::Int(2i64))?;
    if ({ let t5128 = { let t5121 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t5122 = v_n.clone(); if py::ne(&t5121, &t5122) { true } else { false } }) }; if t5128.truthy() { t5128 } else { { let t5127 = { let t5123 = py::len(&py::getitem(&v_o.clone(), &Py::str("edges"))?)?; Py::Bool({ let t5124 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t5123, &t5124) { true } else { false } }) }; if t5127.truthy() { t5127 } else { { let t5125 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t5126 = py::sorted(&py::getitem(&v_o.clone(), &Py::str("edges"))?, (Py::Bool(false)).truthy())?; if py::ne(&t5125, &t5126) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t5129 = py::getitem(&f_prufer_encode({ let mut out = vec![]; for t5182 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("edges"))?))? { let mut v_u: Py; let mut v_v: Py; 
let t5183 = (t5182).items()?;
if t5183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5183[0].clone();
v_v = t5183[1].clone();
out.push(Py::tuple(vec![Py::str(&py::to_str(&v_u.clone())), Py::str(&py::to_str(&v_v.clone()))])); } Py::list(out) }, Py::Bool(false))?, &Py::str("sequence"))?; Py::Bool({ let t5130 = v_seq.clone(); if py::eq(&t5129, &t5130) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pell(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_h1: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t4677 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?])).items()?;
    if t4677.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4677[0].clone();
    v_x = t4677[1].clone();
    v_y = t4677[2].clone();
    if (py::not(&{ let t4685 = f__iv(v_x.clone())?; if !t4685.truthy() { t4685 } else { { let t4684 = f__iv(v_y.clone())?; if !t4684.truthy() { t4684 } else { { let t4683 = { let t4679 = v_y.clone(); Py::Bool({ let t4680 = Py::Int(1i64); if py::ge(&t4679, &t4680)? { true } else { false } }) }; if !t4683.truthy() { t4683 } else { { let t4681 = py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&py::mul(&v_d.clone(), &v_y.clone())?, &v_y.clone())?)?; Py::Bool({ let t4682 = Py::Int(1i64); if py::eq(&t4681, &t4682) { true } else { false } }) } } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_a0 = py::isqrt(&v_d.clone())?;
    let t4686 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), v_a0.clone()])).items()?;
    if t4686.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4686[0].clone();
    v_den = t4686[1].clone();
    v_a = t4686[2].clone();
    let t4687 = (Py::tuple(vec![Py::Int(1i64), v_a0.clone(), Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t4687.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t4687[0].clone();
    v_h1 = t4687[1].clone();
    v_k0 = t4687[2].clone();
    v_k1 = t4687[3].clone();
    while ({ let t4690 = py::sub(&py::mul(&v_h1.clone(), &v_h1.clone())?, &py::mul(&py::mul(&v_d.clone(), &v_k1.clone())?, &v_k1.clone())?)?; Py::Bool({ let t4691 = Py::Int(1i64); if py::ne(&t4690, &t4691) { true } else { false } }) }).truthy() {
        v_m = py::sub(&py::mul(&v_den.clone(), &v_a.clone())?, &v_m.clone())?;
        v_den = py::floordiv(&py::sub(&v_d.clone(), &py::mul(&v_m.clone(), &v_m.clone())?)?, &v_den.clone())?;
        v_a = py::floordiv(&py::add(&v_a0.clone(), &v_m.clone())?, &v_den.clone())?;
        let t4688 = (Py::tuple(vec![v_h1.clone(), py::add(&py::mul(&v_a.clone(), &v_h1.clone())?, &v_h0.clone())?])).items()?;
        if t4688.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h0 = t4688[0].clone();
        v_h1 = t4688[1].clone();
        let t4689 = (Py::tuple(vec![v_k1.clone(), py::add(&py::mul(&v_a.clone(), &v_k1.clone())?, &v_k0.clone())?])).items()?;
        if t4689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k0 = t4689[0].clone();
        v_k1 = t4689[1].clone();
    }
    return Ok({ let t4692 = Py::tuple(vec![v_x.clone(), v_y.clone()]); Py::Bool({ let t4693 = Py::tuple(vec![v_h1.clone(), v_k1.clone()]); if py::eq(&t4692, &t4693) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pencode(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_nb: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t5244 = (f__tree_labels(py::getitem(&v_i.clone(), &Py::str("edges"))?)?).items()?;
    if t5244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nb = t5244[0].clone();
    v_off = t5244[1].clone();
    v_seq = py::getitem(&v_o.clone(), &Py::str("sequence"))?;
    v_want = py::sorted(&{ let mut out = vec![]; for t5245 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t5245;
out.push(py::sorted(&Py::tuple(vec![py::int(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)?]), (Py::Bool(false)).truthy())?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t5268 = { let t5246 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t5247 = py::len(&v_nb.clone())?; if py::eq(&t5246, &t5247) { true } else { false } }) }; if !t5268.truthy() { t5268 } else { { let t5267 = { let t5248 = py::len(&v_seq.clone())?; Py::Bool({ let t5249 = py::sub(&py::len(&v_nb.clone())?, &Py::Int(2i64))?; if py::eq(&t5248, &t5249) { true } else { false } }) }; if !t5267.truthy() { t5267 } else { { let t5266 = Py::Bool(({ let mut out = vec![]; for t5250 in py::iter(&(v_seq.clone()))? { let mut v_s: Py; 
v_s = t5250;
out.push(f__iv(v_s.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t5266.truthy() { t5266 } else { { let t5252 = f__decode(v_seq.clone(), py::len(&v_nb.clone())?, v_off.clone())?; Py::Bool({ let t5253 = v_want.clone(); if py::eq(&t5252, &t5253) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_phi(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    if (py::not(&f__iv(py::getitem(&v_o.clone(), &Py::str("phi"))?)?)).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t1775 = v_n.clone(); Py::Bool({ let t1776 = Py::Int(3000i64); if py::le(&t1775, &t1776)? { true } else { false } }) }).truthy() {
        return Ok({ let t1777 = py::getitem(&v_o.clone(), &Py::str("phi"))?; Py::Bool({ let t1778 = py::sum(&{ let mut out = vec![]; for t1779 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1779;
if ({ let t1780 = py::gcd(&v_k.clone(), &v_n.clone())?; Py::Bool({ let t1781 = Py::Int(1i64); if py::eq(&t1780, &t1781) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t1777, &t1778) { true } else { false } }) });
    }
    return Ok({ let t1782 = py::getitem(&v_o.clone(), &Py::str("phi"))?; Py::Bool({ let t1783 = py::prod(&{ let mut out = vec![]; for t1784 in py::iter(&(py::method(&f__factor(v_n.clone())?, "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t1785 = (t1784).items()?;
if t1785.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t1785[0].clone();
v_e = t1785[1].clone();
out.push(py::mul(&py::pow(&v_p.clone(), &py::sub(&v_e.clone(), &Py::Int(1i64))?)?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)?); } Py::list(out) })?; if py::eq(&t1782, &t1783) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pk(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_at_most: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t4630 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t4630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4630[0].clone();
    v_k = t4630[1].clone();
    v_at_most = { let v_at_most = v_at_most.clone(); let v_i = v_i.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_part: Py = Py::None;
            let mut v_s: Py = Py::None;
            let mut v_w: Py = Py::None;
            v_w = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
            for t4631 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) {
                v_part = t4631;
                for t4632 in py::range_iter(&[(v_part.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_s = t4632;
                    let t4633 = v_w.clone(); let t4634 = v_s.clone();
                    py::setitem(&t4633, &t4634, py::iadd(&py::getitem(&t4633, &t4634)?, &py::getitem(&v_w.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
                }
            }
            return Ok(py::getitem(&v_w.clone(), &v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_want = (if ({ let t4635 = v_k.clone(); Py::Bool({ let t4636 = Py::Int(0i64); if py::eq(&t4635, &t4636) { true } else { false } }) }).truthy() { (if ({ let t4637 = v_n.clone(); Py::Bool({ let t4638 = Py::Int(0i64); if py::eq(&t4637, &t4638) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) } else { py::sub(&py::call(&v_at_most.clone(), &[v_k.clone()])?, &py::call(&v_at_most.clone(), &[py::sub(&v_k.clone(), &Py::Int(1i64))?])?)? });
    return Ok({ let t4642 = f__iv(py::getitem(&v_o.clone(), &Py::str("partitions"))?)?; if !t4642.truthy() { t4642 } else { { let t4640 = py::getitem(&v_o.clone(), &Py::str("partitions"))?; Py::Bool({ let t4641 = v_want.clone(); if py::eq(&t4640, &t4641) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pos(mut v_n: Py, mut v_on: Py, mut v_dc: Py, mut v_names: Py, mut v_o: Py) -> R {
    let mut v_cl: Py = Py::None;
    let mut v_cubes: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_lits: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_e = py::getitem(&v_o.clone(), &Py::str("expression"))?;
    v_off = { let mut out = vec![]; for t3640 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3640;
if ({ let t3645 = { let t3641 = v_m.clone(); Py::Bool({ let t3642 = py::set(&v_on.clone())?; if !py::contains(&t3642, &t3641)? { true } else { false } }) }; if !t3645.truthy() { t3645 } else { { let t3643 = v_m.clone(); Py::Bool({ let t3644 = py::set(&v_dc.clone())?; if !py::contains(&t3644, &t3643)? { true } else { false } }) } } }).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    if ({ let t3646 = v_e.clone(); Py::Bool({ let t3647 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if py::contains(&t3647, &t3646)? { true } else { false } }) }).truthy() {
        v_want = (if ({ let t3648 = v_e.clone(); Py::Bool({ let t3649 = Py::str("1"); if py::eq(&t3648, &t3649) { true } else { false } }) }).truthy() { py::not(&v_off.clone()) } else { Py::Bool(({ let mut out = vec![]; for t3650 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3650;
out.push({ let t3651 = v_m.clone(); Py::Bool({ let t3652 = py::bitor(&py::set(&v_off.clone())?, &py::set(&v_dc.clone())?)?; if py::contains(&t3652, &t3651)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) });
        return Ok({ let t3660 = v_want.clone(); if !t3660.truthy() { t3660 } else { { let t3659 = { let t3653 = py::getitem(&v_o.clone(), &Py::str("clauses"))?; Py::Bool({ let t3654 = (if ({ let t3655 = v_e.clone(); Py::Bool({ let t3656 = Py::str("1"); if py::eq(&t3655, &t3656) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { Py::Int(1i64) }); if py::eq(&t3653, &t3654) { true } else { false } }) }; if !t3659.truthy() { t3659 } else { { let t3657 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3658 = Py::Int(0i64); if py::eq(&t3657, &t3658) { true } else { false } }) } } } } });
    }
    v_cubes = Py::list(vec![]);
    for t3661 in py::iter(&(py::method(&v_e.clone(), "split", &[Py::str(" & ")])?))? {
        v_cl = t3661;
        v_lits = { let mut out = vec![]; for t3662 in py::iter(&(py::method(&py::method(&v_cl.clone(), "strip", &[Py::str("()")])?, "split", &[Py::str("|")])?))? { let mut v_x: Py; 
v_x = t3662;
out.push(py::method(&v_x.clone(), "strip", &[])?); } Py::list(out) };
        let _ = py::method(&v_cubes.clone(), "append", &[f__cube_of(py::method(&Py::str(" & "), "join", &[{ let mut out = vec![]; for t3666 in py::iter(&(v_lits.clone()))? { let mut v_x: Py; 
v_x = t3666;
out.push((if (py::method(&v_x.clone(), "startswith", &[Py::str("~")])?).truthy() { py::slice(&v_x.clone(), Some((Py::Int(1i64)).i()?), None, None)? } else { py::add(&Py::str("~"), &v_x.clone())? })); } Py::list(out) }])?, v_names.clone())?])?;
    }
    return Ok({ let t3699 = { let t3667 = Py::None; Py::Bool({ let t3668 = v_cubes.clone(); if !py::contains(&t3668, &t3667)? { true } else { false } }) }; if !t3699.truthy() { t3699 } else { { let t3698 = f__check_cover(v_n.clone(), v_off.clone(), v_dc.clone(), v_cubes.clone())?; if !t3698.truthy() { t3698 } else { { let t3697 = f__iv(py::getitem(&v_o.clone(), &Py::str("clauses"))?)?; if !t3697.truthy() { t3697 } else { { let t3696 = { let t3691 = py::getitem(&v_o.clone(), &Py::str("clauses"))?; Py::Bool({ let t3692 = py::len(&v_cubes.clone())?; if py::eq(&t3691, &t3692) { true } else { false } }) }; if !t3696.truthy() { t3696 } else { { let t3693 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3694 = py::sum(&{ let mut out = vec![]; for t3695 in py::iter(&(v_cubes.clone()))? { let mut v_c: Py; 
v_c = t3695;
out.push(py::sub(&v_n.clone(), &py::method(&py::int_text_base(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, "bin")?, "count", &[Py::str("1")])?)?); } Py::list(out) }, None)?; if py::eq(&t3693, &t3694) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_prev(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4831 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("prime"))?])).items()?;
    if t4831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4831[0].clone();
    v_p = t4831[1].clone();
    return Ok({ let t4853 = f__iv(v_p.clone())?; if !t4853.truthy() { t4853 } else { { let t4852 = { let t4833 = v_p.clone(); Py::Bool({ let t4834 = v_n.clone(); if py::lt(&t4833, &t4834)? { true } else { false } }) }; if !t4852.truthy() { t4852 } else { { let t4851 = f__is_prime_alt(v_p.clone())?; if !t4851.truthy() { t4851 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4850 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4850;
out.push(f__is_prime_alt(v_k.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_proot(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_prim: Py = Py::None;
    let mut v_qs: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t5048 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_o.clone(), &Py::str("root"))?])).items()?;
    if t5048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5048[0].clone();
    v_r = t5048[1].clone();
    if (py::not(&f__iv(v_r.clone())?)).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t5050 = v_p.clone(); Py::Bool({ let t5051 = Py::Int(2i64); if py::eq(&t5050, &t5051) { true } else { false } }) }).truthy() {
        return Ok({ let t5052 = v_r.clone(); Py::Bool({ let t5053 = Py::Int(1i64); if py::eq(&t5052, &t5053) { true } else { false } }) });
    }
    v_qs = (if ({ let t5054 = v_p.clone(); Py::Bool({ let t5055 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::lt(&t5054, &t5055)? { true } else { false } }) }).truthy() { Py::list((f__trial_factor(py::sub(&v_p.clone(), &Py::Int(1i64))?)?).items()?) } else { Py::list((f__factor(py::sub(&v_p.clone(), &Py::Int(1i64))?)?).items()?) });
    v_prim = { let v_i = v_i.clone(); let v_o = v_o.clone(); let v_p = v_p.clone(); let v_qs = v_qs.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_g: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::Bool(({ let mut out = vec![]; for t5065 in py::iter(&(v_qs.clone()))? { let mut v_q: Py; 
v_q = t5065;
out.push({ let t5066 = py::pow_mod(&v_g.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &v_q.clone())?, &v_p.clone())?; Py::Bool({ let t5067 = Py::Int(1i64); if py::ne(&t5066, &t5067) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok({ let t5073 = { let t5068 = Py::Int(2i64); Py::Bool({ let t5069 = v_r.clone(); if py::le(&t5068, &t5069)? { { let t5070 = v_p.clone(); if py::lt(&t5069, &t5070)? { true } else { false } } } else { false } }) }; if !t5073.truthy() { t5073 } else { { let t5072 = py::call(&v_prim.clone(), &[v_r.clone()])?; if !t5072.truthy() { t5072 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t5071 in py::range_iter(&[(Py::Int(2i64)).i()?, (v_r.clone()).i()?]) { let mut v_g: Py; 
v_g = t5071;
out.push(py::call(&v_prim.clone(), &[v_g.clone()])?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_s1(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_poly: Py = Py::None;
    let t5762 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t5762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5762[0].clone();
    v_k = t5762[1].clone();
    v_poly = Py::list(vec![Py::Int(1i64)]);
    for t5763 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t5763;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&py::len(&v_poly.clone())?, &Py::Int(1i64))?)?;
        for t5764 in py::iter(&(py::enumerate(&v_poly.clone(), 0)?))? {
            let t5769 = (t5764).items()?;
            if t5769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_d = t5769[0].clone();
            v_c = t5769[1].clone();
            let t5765 = v_new.clone(); let t5766 = py::add(&v_d.clone(), &Py::Int(1i64))?;
            py::setitem(&t5765, &t5766, py::iadd(&py::getitem(&t5765, &t5766)?, &v_c.clone())?)?;
            let t5767 = v_new.clone(); let t5768 = v_d.clone();
            py::setitem(&t5767, &t5768, py::iadd(&py::getitem(&t5767, &t5768)?, &py::mul(&v_j.clone(), &v_c.clone())?)?)?;
        }
        v_poly = v_new.clone();
    }
    v_c = (if ({ let t5770 = v_k.clone(); Py::Bool({ let t5771 = py::len(&v_poly.clone())?; if py::lt(&t5770, &t5771)? { true } else { false } }) }).truthy() { py::getitem(&v_poly.clone(), &v_k.clone())? } else { Py::Int(0i64) });
    return Ok({ let t5778 = f__iv(py::getitem(&v_o.clone(), &Py::str("stirling1"))?)?; if !t5778.truthy() { t5778 } else { { let t5777 = { let t5773 = py::getitem(&v_o.clone(), &Py::str("stirling1"))?; Py::Bool({ let t5774 = v_c.clone(); if py::eq(&t5773, &t5774) { true } else { false } }) }; if !t5777.truthy() { t5777 } else { { let t5775 = py::getitem(&v_o.clone(), &Py::str("signed"))?; Py::Bool({ let t5776 = py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::sub(&v_n.clone(), &v_k.clone())?)?, &v_c.clone())?; if py::eq(&t5775, &t5776) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_s2(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sur: Py = Py::None;
    let t5807 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t5807.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5807[0].clone();
    v_k = t5807[1].clone();
    v_sur = py::sum(&{ let mut out = vec![]; for t5808 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t5808;
out.push(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_j.clone())?, &py::comb(&v_k.clone(), &v_j.clone())?)?, &py::pow(&py::sub(&v_k.clone(), &v_j.clone())?, &v_n.clone())?)?); } Py::list(out) }, None)?;
    return Ok({ let t5815 = f__iv(py::getitem(&v_o.clone(), &Py::str("stirling2"))?)?; if !t5815.truthy() { t5815 } else { { let t5814 = { let t5810 = py::getitem(&v_o.clone(), &Py::str("surjections"))?; Py::Bool({ let t5811 = v_sur.clone(); if py::eq(&t5810, &t5811) { true } else { false } }) }; if !t5814.truthy() { t5814 } else { { let t5812 = py::mul(&py::getitem(&v_o.clone(), &Py::str("stirling2"))?, &py::factorial(&v_k.clone())?)?; Py::Bool({ let t5813 = v_sur.clone(); if py::eq(&t5812, &t5813) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_sat(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3360 = (f__table(py::getitem(&v_i.clone(), &Py::str("formula"))?, Py::None)?).items()?;
    if t3360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3360[0].clone();
    v_names = t3360[1].clone();
    v_first = (match ('g3361: { let mut out = vec![]; for t3362 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3363 = (t3362).items()?;
if t3363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3363[0].clone();
v_v = t3363[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); break 'g3361 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok({ let t3373 = { let t3364 = py::getitem(&v_o.clone(), &Py::str("satisfiable"))?; Py::Bool({ let t3365 = Py::Bool((v_vals.clone()).items()?.iter().any(|x| x.truthy())); if py::is(&t3364, &t3365) { true } else { false } }) }; if !t3373.truthy() { t3373 } else { { let t3372 = { let t3366 = py::getitem(&v_o.clone(), &Py::str("models"))?; Py::Bool({ let t3367 = py::sum(&v_vals.clone(), None)?; if py::eq(&t3366, &t3367) { true } else { false } }) }; if !t3372.truthy() { t3372 } else { { let t3368 = py::getitem(&v_o.clone(), &Py::str("first_model"))?; Py::Bool({ let t3369 = (if ({ let t3370 = v_first.clone(); Py::Bool({ let t3371 = Py::None; if py::is(&t3370, &t3371) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }); if py::eq(&t3368, &t3369) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_scc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_comps: Py = Py::None;
    let mut v_reach: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true), Py::Bool(false))?;
    v_reach = Py::dict(vec![]);
    for t5861 in py::iter(&(v_adj.clone()))? {
        v_s = t5861;
        let t5862 = (Py::tuple(vec![py::set(&Py::list(vec![v_s.clone()]))?, Py::list(vec![v_s.clone()])])).items()?;
        if t5862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_seen = t5862[0].clone();
        v_st = t5862[1].clone();
        while (v_st.clone()).truthy() {
            v_u = py::method(&v_st.clone(), "pop", &[])?;
            for t5863 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t5863;
                if ({ let t5864 = v_v.clone(); Py::Bool({ let t5865 = v_seen.clone(); if !py::contains(&t5865, &t5864)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[v_v.clone()])?;
                    let _ = py::method(&v_st.clone(), "append", &[v_v.clone()])?;
                }
            }
        }
        py::setitem(&v_reach.clone(), &v_s.clone(), v_seen.clone())?;
    }
    v_comps = py::sorted_key(&py::set(&{ let mut out = vec![]; for t5866 in py::iter(&(v_adj.clone()))? { let mut v_s: Py; 
v_s = t5866;
out.push(Py::tuple((f__sorted({ let mut out = vec![]; for t5867 in py::iter(&(py::getitem(&v_reach.clone(), &v_s.clone())?))? { let mut v_v: Py; 
v_v = t5867;
if ({ let t5868 = v_s.clone(); Py::Bool({ let t5869 = py::getitem(&v_reach.clone(), &v_v.clone())?; if py::contains(&t5869, &t5868)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?).items()?)); } Py::list(out) })?, &{ let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_reach = v_reach.clone(); let v_s = v_s.clone(); let v_seen = v_seen.clone(); let v_st = v_st.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__nkey(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_comps = { let mut out = vec![]; for t5870 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t5870;
out.push(Py::list((v_c.clone()).items()?)); } Py::list(out) };
    return Ok({ let t5881 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t5881.truthy() { t5881 } else { { let t5880 = { let t5872 = py::getitem(&v_o.clone(), &Py::str("components"))?; Py::Bool({ let t5873 = v_comps.clone(); if py::eq(&t5872, &t5873) { true } else { false } }) }; if !t5880.truthy() { t5880 } else { { let t5879 = { let t5874 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t5875 = py::len(&v_comps.clone())?; if py::eq(&t5874, &t5875) { true } else { false } }) }; if !t5879.truthy() { t5879 } else { { let t5876 = py::getitem(&v_o.clone(), &Py::str("sizes"))?; Py::Bool({ let t5877 = py::sorted(&{ let mut out = vec![]; for t5878 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t5878;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?; if py::eq(&t5876, &t5877) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_sop(mut v_n: Py, mut v_on: Py, mut v_dc: Py, mut v_names: Py, mut v_o: Py) -> R {
    let mut v_cubes: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_e = py::getitem(&v_o.clone(), &Py::str("expression"))?;
    if ({ let t3962 = v_e.clone(); Py::Bool({ let t3963 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if py::contains(&t3963, &t3962)? { true } else { false } }) }).truthy() {
        v_want = (if ({ let t3964 = v_e.clone(); Py::Bool({ let t3965 = Py::str("0"); if py::eq(&t3964, &t3965) { true } else { false } }) }).truthy() { py::not(&v_on.clone()) } else { Py::Bool(({ let mut out = vec![]; for t3966 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3966;
out.push({ let t3967 = v_m.clone(); Py::Bool({ let t3968 = py::bitor(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?; if py::contains(&t3968, &t3967)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) });
        return Ok({ let t3976 = v_want.clone(); if !t3976.truthy() { t3976 } else { { let t3975 = { let t3969 = py::getitem(&v_o.clone(), &Py::str("terms"))?; Py::Bool({ let t3970 = (if ({ let t3971 = v_e.clone(); Py::Bool({ let t3972 = Py::str("0"); if py::eq(&t3971, &t3972) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { Py::Int(1i64) }); if py::eq(&t3969, &t3970) { true } else { false } }) }; if !t3975.truthy() { t3975 } else { { let t3973 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3974 = Py::Int(0i64); if py::eq(&t3973, &t3974) { true } else { false } }) } } } } });
    }
    v_cubes = { let mut out = vec![]; for t3977 in py::iter(&(py::method(&v_e.clone(), "split", &[Py::str(" | ")])?))? { let mut v_t: Py; 
v_t = t3977;
out.push(f__cube_of(v_t.clone(), v_names.clone())?); } Py::list(out) };
    return Ok({ let t4013 = { let t3981 = Py::None; Py::Bool({ let t3982 = v_cubes.clone(); if !py::contains(&t3982, &t3981)? { true } else { false } }) }; if !t4013.truthy() { t4013 } else { { let t4012 = f__check_cover(v_n.clone(), v_on.clone(), v_dc.clone(), v_cubes.clone())?; if !t4012.truthy() { t4012 } else { { let t4011 = f__iv(py::getitem(&v_o.clone(), &Py::str("terms"))?)?; if !t4011.truthy() { t4011 } else { { let t4010 = { let t4005 = py::getitem(&v_o.clone(), &Py::str("terms"))?; Py::Bool({ let t4006 = py::len(&v_cubes.clone())?; if py::eq(&t4005, &t4006) { true } else { false } }) }; if !t4010.truthy() { t4010 } else { { let t4007 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t4008 = py::sum(&{ let mut out = vec![]; for t4009 in py::iter(&(v_cubes.clone()))? { let mut v_c: Py; 
v_c = t4009;
out.push(py::sub(&v_n.clone(), &py::method(&py::int_text_base(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, "bin")?, "count", &[Py::str("1")])?)?); } Py::list(out) }, None)?; if py::eq(&t4007, &t4008) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_sqrtmod(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t5656 = (Py::tuple(vec![py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?])).items()?;
    if t5656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5656[0].clone();
    v_p = t5656[1].clone();
    v_rs = py::getitem(&v_o.clone(), &Py::str("roots"))?;
    v_want = (if ({ let t5661 = { let t5657 = v_a.clone(); Py::Bool({ let t5658 = Py::Int(0i64); if py::eq(&t5657, &t5658) { true } else { false } }) }; if t5661.truthy() { t5661 } else { { let t5659 = v_p.clone(); Py::Bool({ let t5660 = Py::Int(2i64); if py::eq(&t5659, &t5660) { true } else { false } }) } } }).truthy() { Py::Int(1i64) } else { (if ({ let t5662 = py::pow_mod(&v_a.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?; Py::Bool({ let t5663 = Py::Int(1i64); if py::eq(&t5662, &t5663) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(0i64) }) });
    return Ok({ let t5684 = { let t5664 = v_rs.clone(); Py::Bool({ let t5665 = py::sorted(&py::set(&v_rs.clone())?, (Py::Bool(false)).truthy())?; if py::eq(&t5664, &t5665) { true } else { false } }) }; if !t5684.truthy() { t5684 } else { { let t5683 = Py::Bool(({ let mut out = vec![]; for t5666 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t5666;
out.push({ let t5674 = f__iv(v_r.clone())?; if !t5674.truthy() { t5674 } else { { let t5673 = { let t5668 = Py::Int(0i64); Py::Bool({ let t5669 = v_r.clone(); if py::le(&t5668, &t5669)? { { let t5670 = v_p.clone(); if py::lt(&t5669, &t5670)? { true } else { false } } } else { false } }) }; if !t5673.truthy() { t5673 } else { { let t5671 = py::modulo(&py::mul(&v_r.clone(), &v_r.clone())?, &v_p.clone())?; Py::Bool({ let t5672 = v_a.clone(); if py::eq(&t5671, &t5672) { true } else { false } }) } } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t5683.truthy() { t5683 } else { { let t5682 = { let t5675 = py::len(&v_rs.clone())?; Py::Bool({ let t5676 = v_want.clone(); if py::eq(&t5675, &t5676) { true } else { false } }) }; if !t5682.truthy() { t5682 } else { { let t5681 = { let t5677 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t5678 = v_want.clone(); if py::eq(&t5677, &t5678) { true } else { false } }) }; if !t5681.truthy() { t5681 } else { { let t5679 = py::getitem(&v_o.clone(), &Py::str("smallest"))?; Py::Bool({ let t5680 = (if (v_rs.clone()).truthy() { py::getitem(&v_rs.clone(), &Py::Int(0i64))? } else { Py::None }); if py::eq(&t5679, &t5680) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_stars(mut v_i: Py, mut v_o: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_ways: Py = Py::None;
    let t5721 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("total"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("variables"))?)?, py::int(&{ let t5720 = py::method(&v_i.clone(), "get", &[Py::str("lower")])?; if t5720.truthy() { t5720 } else { Py::Int(0i64) } })?])).items()?;
    if t5721.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5721[0].clone();
    v_k = t5721[1].clone();
    v_lo = t5721[2].clone();
    v_hi = (if ({ let t5722 = py::method(&v_i.clone(), "get", &[Py::str("upper")])?; Py::Bool({ let t5723 = Py::None; if !py::is(&t5722, &t5723) { true } else { false } }) }).truthy() { py::int(&py::getitem(&v_i.clone(), &Py::str("upper"))?)? } else { v_n.clone() });
    v_ways = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
    for t5724 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t5724;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
        for t5725 in py::iter(&(py::enumerate(&v_ways.clone(), 0)?))? {
            let t5729 = (t5725).items()?;
            if t5729.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s = t5729[0].clone();
            v_w = t5729[1].clone();
            if (v_w.clone()).truthy() {
                for t5726 in py::range_iter(&[(v_lo.clone()).i()?, (py::add(&py::minmax(&[v_hi.clone(), py::sub(&v_n.clone(), &v_s.clone())?], false)?, &Py::Int(1i64))?).i()?]) {
                    v_v = t5726;
                    let t5727 = v_new.clone(); let t5728 = py::add(&v_s.clone(), &v_v.clone())?;
                    py::setitem(&t5727, &t5728, py::iadd(&py::getitem(&t5727, &t5728)?, &v_w.clone())?)?;
                }
            }
        }
        v_ways = v_new.clone();
    }
    return Ok({ let t5733 = f__iv(py::getitem(&v_o.clone(), &Py::str("solutions"))?)?; if !t5733.truthy() { t5733 } else { { let t5731 = py::getitem(&v_o.clone(), &Py::str("solutions"))?; Py::Bool({ let t5732 = py::getitem(&v_ways.clone(), &v_n.clone())?; if py::eq(&t5731, &t5732) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_taut(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3457 = (f__table(py::getitem(&v_i.clone(), &Py::str("formula"))?, Py::None)?).items()?;
    if t3457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3457[0].clone();
    v_names = t3457[1].clone();
    v_first = (match ('g3458: { let mut out = vec![]; for t3459 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3460 = (t3459).items()?;
if t3460.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3460[0].clone();
v_v = t3460[1].clone();
if (py::not(&v_v.clone())).truthy() { out.push(v_r.clone()); break 'g3458 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok({ let t3470 = { let t3461 = py::getitem(&v_o.clone(), &Py::str("tautology"))?; Py::Bool({ let t3462 = Py::Bool((v_vals.clone()).items()?.iter().all(|x| x.truthy())); if py::is(&t3461, &t3462) { true } else { false } }) }; if !t3470.truthy() { t3470 } else { { let t3469 = { let t3463 = py::getitem(&v_o.clone(), &Py::str("false_rows"))?; Py::Bool({ let t3464 = py::sub(&py::len(&v_vals.clone())?, &py::sum(&v_vals.clone(), None)?)?; if py::eq(&t3463, &t3464) { true } else { false } }) }; if !t3469.truthy() { t3469 } else { { let t3465 = py::getitem(&v_o.clone(), &Py::str("counterexample"))?; Py::Bool({ let t3466 = (if ({ let t3467 = v_first.clone(); Py::Bool({ let t3468 = Py::None; if py::is(&t3467, &t3468) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }); if py::eq(&t3465, &t3466) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_topo(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_avail: Py = Py::None;
    let mut v_indeg: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true), Py::Bool(false))?;
    v_order = py::getitem(&v_o.clone(), &Py::str("order"))?;
    if ({ let t5929 = { let t5925 = py::sorted_key(&v_order.clone(), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?; Py::Bool({ let t5926 = f__sorted(v_adj.clone())?; if py::ne(&t5925, &t5926) { true } else { false } }) }; if t5929.truthy() { t5929 } else { { let t5927 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t5928 = py::len(&v_order.clone())?; if py::ne(&t5927, &t5928) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_indeg = { let mut out = vec![]; for t5930 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5930;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    for t5931 in py::iter(&(v_adj.clone()))? {
        v_u = t5931;
        for t5932 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5932;
            let t5933 = v_indeg.clone(); let t5934 = v_v.clone();
            py::setitem(&t5933, &t5934, py::iadd(&py::getitem(&t5933, &t5934)?, &Py::Int(1i64))?)?;
        }
    }
    for t5935 in py::iter(&(v_order.clone()))? {
        v_u = t5935;
        v_avail = { let mut out = vec![]; for t5936 in py::iter(&(py::method(&v_indeg.clone(), "items", &[])?))? { let mut v_d: Py; let mut v_v: Py; 
let t5937 = (t5936).items()?;
if t5937.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t5937[0].clone();
v_d = t5937[1].clone();
if ({ let t5938 = v_d.clone(); Py::Bool({ let t5939 = Py::Int(0i64); if py::eq(&t5938, &t5939) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
        if ({ let t5944 = { let t5940 = v_u.clone(); Py::Bool({ let t5941 = v_avail.clone(); if !py::contains(&t5941, &t5940)? { true } else { false } }) }; if t5944.truthy() { t5944 } else { { let t5942 = py::minmax_key(&[v_avail.clone()], &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), false)?; Py::Bool({ let t5943 = v_u.clone(); if py::ne(&t5942, &t5943) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Bool(false));
        }
        py::delitem(&v_indeg.clone(), &v_u.clone())?;
        for t5945 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5945;
            let t5946 = v_indeg.clone(); let t5947 = v_v.clone();
            py::setitem(&t5946, &t5947, py::sub(&py::getitem(&t5946, &t5947)?, &Py::Int(1i64))?)?;
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_tt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t6027 = (f__table(py::getitem(&v_i.clone(), &Py::str("formula"))?, Py::None)?).items()?;
    if t6027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t6027[0].clone();
    v_names = t6027[1].clone();
    return Ok({ let t6043 = { let t6028 = py::getitem(&v_o.clone(), &Py::str("variables"))?; Py::Bool({ let t6029 = v_names.clone(); if py::eq(&t6028, &t6029) { true } else { false } }) }; if !t6043.truthy() { t6043 } else { { let t6042 = { let t6030 = py::getitem(&v_o.clone(), &Py::str("values"))?; Py::Bool({ let t6031 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6032 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t6032;
out.push((if (v_v.clone()).truthy() { Py::str("1") } else { Py::str("0") })); } Py::list(out) }])?; if py::eq(&t6030, &t6031) { true } else { false } }) }; if !t6042.truthy() { t6042 } else { { let t6041 = f__iv(py::getitem(&v_o.clone(), &Py::str("true_rows"))?)?; if !t6041.truthy() { t6041 } else { { let t6040 = { let t6034 = py::getitem(&v_o.clone(), &Py::str("true_rows"))?; Py::Bool({ let t6035 = py::sum(&v_vals.clone(), None)?; if py::eq(&t6034, &t6035) { true } else { false } }) }; if !t6040.truthy() { t6040 } else { { let t6036 = py::getitem(&v_o.clone(), &Py::str("minterms"))?; Py::Bool({ let t6037 = { let mut out = vec![]; for t6038 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t6039 = (t6038).items()?;
if t6039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t6039[0].clone();
v_v = t6039[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); }} Py::list(out) }; if py::eq(&t6036, &t6037) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_twos(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t6189 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("bits"))?)?, py::getitem(&v_o.clone(), &Py::str("bits_text"))?])).items()?;
    if t6189.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6189[0].clone();
    v_b = t6189[1].clone();
    v_s = t6189[2].clone();
    if (py::not(&{ let t6196 = f__iv(py::getitem(&v_o.clone(), &Py::str("unsigned"))?)?; if !t6196.truthy() { t6196 } else { { let t6195 = { let t6191 = py::len(&v_s.clone())?; Py::Bool({ let t6192 = v_b.clone(); if py::eq(&t6191, &t6192) { true } else { false } }) }; if !t6195.truthy() { t6195 } else { { let t6193 = py::set(&v_s.clone())?; Py::Bool({ let t6194 = py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?; if py::set_cmp(&t6193, &t6194, "<=")? { true } else { false } }) } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_u = py::int_base(&v_s.clone(), &Py::Int(2i64))?;
    return Ok({ let t6203 = { let t6197 = v_u.clone(); Py::Bool({ let t6198 = py::getitem(&v_o.clone(), &Py::str("unsigned"))?; if py::eq(&t6197, &t6198) { true } else { false } }) }; if !t6203.truthy() { t6203 } else { { let t6199 = (if ({ let t6201 = py::getitem(&v_s.clone(), &Py::Int(0i64))?; Py::Bool({ let t6202 = Py::str("1"); if py::eq(&t6201, &t6202) { true } else { false } }) }).truthy() { py::sub(&v_u.clone(), &py::lshift(&Py::Int(1i64), &v_b.clone())?)? } else { v_u.clone() }); Py::Bool({ let t6200 = v_n.clone(); if py::eq(&t6199, &t6200) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_twosq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_reps: Py = Py::None;
    let t6123 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("representations"))?])).items()?;
    if t6123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6123[0].clone();
    v_reps = t6123[1].clone();
    v_ok = { let t6137 = Py::Bool(({ let mut out = vec![]; for t6124 in py::iter(&(v_reps.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t6125 = (t6124).items()?;
if t6125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t6125[0].clone();
v_y = t6125[1].clone();
out.push({ let t6134 = f__iv(v_x.clone())?; if !t6134.truthy() { t6134 } else { { let t6133 = f__iv(v_y.clone())?; if !t6133.truthy() { t6133 } else { { let t6132 = { let t6127 = Py::Int(0i64); Py::Bool({ let t6128 = v_x.clone(); if py::le(&t6127, &t6128)? { { let t6129 = v_y.clone(); if py::le(&t6128, &t6129)? { true } else { false } } } else { false } }) }; if !t6132.truthy() { t6132 } else { { let t6130 = py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&v_y.clone(), &v_y.clone())?)?; Py::Bool({ let t6131 = v_n.clone(); if py::eq(&t6130, &t6131) { true } else { false } }) } } } } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t6137.truthy() { t6137 } else { { let t6135 = v_reps.clone(); Py::Bool({ let t6136 = py::sorted(&v_reps.clone(), (Py::Bool(false)).truthy())?; if py::eq(&t6135, &t6136) { true } else { false } }) } } };
    return Ok({ let t6156 = v_ok.clone(); if !t6156.truthy() { t6156 } else { { let t6155 = { let t6138 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t6139 = py::len(&v_reps.clone())?; if py::eq(&t6138, &t6139) { true } else { false } }) }; if !t6155.truthy() { t6155 } else { { let t6154 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t6154.truthy() { t6154 } else { { let t6140 = py::getitem(&v_o.clone(), &Py::str("r2"))?; Py::Bool({ let t6141 = py::sum(&{ let mut out = vec![]; for t6142 in py::iter(&(v_reps.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t6143 = (t6142).items()?;
if t6143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t6143[0].clone();
v_y = t6143[1].clone();
out.push(f__mult(v_x.clone(), v_y.clone())?); } Py::list(out) }, None)?; if py::eq(&t6140, &t6141) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_venn2(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_ab: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let t6311 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("both"))?)?])).items()?;
    if t6311.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6311[0].clone();
    v_b = t6311[1].clone();
    v_ab = t6311[2].clone();
    for t6312 in py::iter(&(Py::tuple(vec![Py::str("union"), Py::str("only_a"), Py::str("only_b")])))? {
        v_k = t6312;
        if ({ let t6316 = py::not(&f__iv(py::getitem(&v_o.clone(), &v_k.clone())?)?); if t6316.truthy() { t6316 } else { { let t6314 = py::getitem(&v_o.clone(), &v_k.clone())?; Py::Bool({ let t6315 = Py::Int(0i64); if py::lt(&t6314, &t6315)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Bool(false));
        }
    }
    if ({ let t6324 = { let t6317 = py::add(&py::getitem(&v_o.clone(), &Py::str("only_a"))?, &v_ab.clone())?; Py::Bool({ let t6318 = v_a.clone(); if py::ne(&t6317, &t6318) { true } else { false } }) }; if t6324.truthy() { t6324 } else { { let t6323 = { let t6319 = py::add(&py::getitem(&v_o.clone(), &Py::str("only_b"))?, &v_ab.clone())?; Py::Bool({ let t6320 = v_b.clone(); if py::ne(&t6319, &t6320) { true } else { false } }) }; if t6323.truthy() { t6323 } else { { let t6321 = py::getitem(&v_o.clone(), &Py::str("union"))?; Py::Bool({ let t6322 = py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("only_a"))?, &py::getitem(&v_o.clone(), &Py::str("only_b"))?)?, &v_ab.clone())?; if py::ne(&t6321, &t6322) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t6325 = py::method(&v_i.clone(), "get", &[Py::str("universe")])?; Py::Bool({ let t6326 = Py::None; if py::is(&t6325, &t6326) { true } else { false } }) }).truthy() {
        return Ok({ let t6327 = py::getitem(&v_o.clone(), &Py::str("neither"))?; Py::Bool({ let t6328 = Py::None; if py::is(&t6327, &t6328) { true } else { false } }) });
    }
    return Ok({ let t6334 = f__iv(py::getitem(&v_o.clone(), &Py::str("neither"))?)?; if !t6334.truthy() { t6334 } else { { let t6333 = { let t6329 = py::getitem(&v_o.clone(), &Py::str("neither"))?; Py::Bool({ let t6330 = Py::Int(0i64); if py::ge(&t6329, &t6330)? { true } else { false } }) }; if !t6333.truthy() { t6333 } else { { let t6331 = py::add(&py::getitem(&v_o.clone(), &Py::str("neither"))?, &py::getitem(&v_o.clone(), &Py::str("union"))?)?; Py::Bool({ let t6332 = py::int(&py::getitem(&v_i.clone(), &Py::str("universe"))?)?; if py::eq(&t6331, &t6332) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_venn3(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_abc: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ra: Py = Py::None;
    let mut v_rab: Py = Py::None;
    let mut v_rac: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_rbc: Py = Py::None;
    let mut v_rc: Py = Py::None;
    for t6237 in py::iter(&(py::add(&c__VENN3_OUT()?, &Py::list(vec![Py::str("union"), Py::str("exactly_one"), Py::str("exactly_two")]))?))? {
        v_k = t6237;
        if ({ let t6241 = py::not(&f__iv(py::getitem(&v_o.clone(), &v_k.clone())?)?); if t6241.truthy() { t6241 } else { { let t6239 = py::getitem(&v_o.clone(), &v_k.clone())?; Py::Bool({ let t6240 = Py::Int(0i64); if py::lt(&t6239, &t6240)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Bool(false));
        }
    }
    v_abc = py::int(&py::getitem(&v_i.clone(), &Py::str("abc"))?)?;
    let t6242 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("only_a"))?, py::getitem(&v_o.clone(), &Py::str("only_b"))?, py::getitem(&v_o.clone(), &Py::str("only_c"))?])).items()?;
    if t6242.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ra = t6242[0].clone();
    v_rb = t6242[1].clone();
    v_rc = t6242[2].clone();
    let t6243 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("only_a_and_b"))?, py::getitem(&v_o.clone(), &Py::str("only_a_and_c"))?, py::getitem(&v_o.clone(), &Py::str("only_b_and_c"))?])).items()?;
    if t6243.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_rab = t6243[0].clone();
    v_rac = t6243[1].clone();
    v_rbc = t6243[2].clone();
    if ({ let t6251 = { let t6244 = py::add(&py::add(&py::add(&v_ra.clone(), &v_rab.clone())?, &v_rac.clone())?, &v_abc.clone())?; Py::Bool({ let t6245 = py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?; if py::ne(&t6244, &t6245) { true } else { false } }) }; if t6251.truthy() { t6251 } else { { let t6250 = { let t6246 = py::add(&py::add(&py::add(&v_rb.clone(), &v_rab.clone())?, &v_rbc.clone())?, &v_abc.clone())?; Py::Bool({ let t6247 = py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?; if py::ne(&t6246, &t6247) { true } else { false } }) }; if t6250.truthy() { t6250 } else { { let t6248 = py::add(&py::add(&py::add(&v_rc.clone(), &v_rac.clone())?, &v_rbc.clone())?, &v_abc.clone())?; Py::Bool({ let t6249 = py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?; if py::ne(&t6248, &t6249) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t6259 = { let t6252 = py::add(&v_rab.clone(), &v_abc.clone())?; Py::Bool({ let t6253 = py::int(&py::getitem(&v_i.clone(), &Py::str("ab"))?)?; if py::ne(&t6252, &t6253) { true } else { false } }) }; if t6259.truthy() { t6259 } else { { let t6258 = { let t6254 = py::add(&v_rac.clone(), &v_abc.clone())?; Py::Bool({ let t6255 = py::int(&py::getitem(&v_i.clone(), &Py::str("ac"))?)?; if py::ne(&t6254, &t6255) { true } else { false } }) }; if t6258.truthy() { t6258 } else { { let t6256 = py::add(&v_rbc.clone(), &v_abc.clone())?; Py::Bool({ let t6257 = py::int(&py::getitem(&v_i.clone(), &Py::str("bc"))?)?; if py::ne(&t6256, &t6257) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t6267 = { let t6260 = py::getitem(&v_o.clone(), &Py::str("union"))?; Py::Bool({ let t6261 = py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&v_ra.clone(), &v_rb.clone())?, &v_rc.clone())?, &v_rab.clone())?, &v_rac.clone())?, &v_rbc.clone())?, &v_abc.clone())?; if py::ne(&t6260, &t6261) { true } else { false } }) }; if t6267.truthy() { t6267 } else { { let t6266 = { let t6262 = py::getitem(&v_o.clone(), &Py::str("exactly_one"))?; Py::Bool({ let t6263 = py::add(&py::add(&v_ra.clone(), &v_rb.clone())?, &v_rc.clone())?; if py::ne(&t6262, &t6263) { true } else { false } }) }; if t6266.truthy() { t6266 } else { { let t6264 = py::getitem(&v_o.clone(), &Py::str("exactly_two"))?; Py::Bool({ let t6265 = py::add(&py::add(&v_rab.clone(), &v_rac.clone())?, &v_rbc.clone())?; if py::ne(&t6264, &t6265) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t6268 = py::method(&v_i.clone(), "get", &[Py::str("universe")])?; Py::Bool({ let t6269 = Py::None; if py::is(&t6268, &t6269) { true } else { false } }) }).truthy() {
        return Ok({ let t6270 = py::getitem(&v_o.clone(), &Py::str("none"))?; Py::Bool({ let t6271 = Py::None; if py::is(&t6270, &t6271) { true } else { false } }) });
    }
    return Ok({ let t6277 = f__iv(py::getitem(&v_o.clone(), &Py::str("none"))?)?; if !t6277.truthy() { t6277 } else { { let t6276 = { let t6272 = py::getitem(&v_o.clone(), &Py::str("none"))?; Py::Bool({ let t6273 = Py::Int(0i64); if py::ge(&t6272, &t6273)? { true } else { false } }) }; if !t6276.truthy() { t6276 } else { { let t6274 = py::add(&py::getitem(&v_o.clone(), &Py::str("none"))?, &py::getitem(&v_o.clone(), &Py::str("union"))?)?; Py::Bool({ let t6275 = py::int(&py::getitem(&v_i.clone(), &Py::str("universe"))?)?; if py::eq(&t6274, &t6275) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_vote(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_above: Py = Py::None;
    let mut v_all_won: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_borda: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_condorcet: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_groups: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_points: Py = Py::None;
    let mut v_total: Py = Py::None;
    let t6453 = (f__vt_ballots(py::getitem(&v_i.clone(), &Py::str("ballots"))?)?).items()?;
    if t6453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_groups = t6453[0].clone();
    v_cands = t6453[1].clone();
    v_n = py::len(&v_cands.clone())?;
    v_total = Py::Int(0i64);
    for t6454 in py::iter(&(v_groups.clone()))? {
        v_g = t6454;
        v_total = py::iadd(&v_total, &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?;
    }
    if ({ let t6459 = { let t6455 = py::getitem(&v_o.clone(), &Py::str("candidates"))?; Py::Bool({ let t6456 = py::method(&Py::str(", "), "join", &[v_cands.clone()])?; if py::ne(&t6455, &t6456) { true } else { false } }) }; if t6459.truthy() { t6459 } else { { let t6457 = py::getitem(&v_o.clone(), &Py::str("voters"))?; Py::Bool({ let t6458 = v_total.clone(); if py::ne(&t6457, &t6458) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    let t6460 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("first_place_votes"))?, py::getitem(&v_o.clone(), &Py::str("borda_points"))?, py::getitem(&v_o.clone(), &Py::str("pairwise_points"))?])).items()?;
    if t6460.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_first = t6460[0].clone();
    v_borda = t6460[1].clone();
    v_points = t6460[2].clone();
    if ({ let t6468 = { let t6461 = py::len(&v_first.clone())?; Py::Bool({ let t6462 = v_n.clone(); if py::ne(&t6461, &t6462) { true } else { false } }) }; if t6468.truthy() { t6468 } else { { let t6467 = { let t6463 = py::len(&v_borda.clone())?; Py::Bool({ let t6464 = v_n.clone(); if py::ne(&t6463, &t6464) { true } else { false } }) }; if t6467.truthy() { t6467 } else { { let t6465 = py::len(&v_points.clone())?; Py::Bool({ let t6466 = v_n.clone(); if py::ne(&t6465, &t6466) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_condorcet = Py::str("none");
    for t6469 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t6469;
        v_f = Py::Int(0i64);
        for t6470 in py::iter(&(v_groups.clone()))? {
            v_g = t6470;
            if ({ let t6471 = py::getitem(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?; Py::Bool({ let t6472 = py::getitem(&v_cands.clone(), &v_k.clone())?; if py::eq(&t6471, &t6472) { true } else { false } }) }).truthy() {
                v_f = py::iadd(&v_f, &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?;
            }
        }
        v_b = v_total.clone();
        v_p = Py::Float(0.0);
        v_all_won = Py::Bool(true);
        for t6473 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t6473;
            if ({ let t6474 = v_j.clone(); Py::Bool({ let t6475 = v_k.clone(); if py::eq(&t6474, &t6475) { true } else { false } }) }).truthy() {
                continue;
            }
            v_above = Py::Int(0i64);
            for t6476 in py::iter(&(v_groups.clone()))? {
                v_g = t6476;
                if ({ let t6477 = py::method(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, "index", &[py::getitem(&v_cands.clone(), &v_k.clone())?])?; Py::Bool({ let t6478 = py::method(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, "index", &[py::getitem(&v_cands.clone(), &v_j.clone())?])?; if py::lt(&t6477, &t6478)? { true } else { false } }) }).truthy() {
                    v_above = py::iadd(&v_above, &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?;
                }
            }
            v_b = py::iadd(&v_b, &v_above.clone())?;
            if ({ let t6479 = py::mul(&Py::Int(2i64), &v_above.clone())?; Py::Bool({ let t6480 = v_total.clone(); if py::gt(&t6479, &t6480)? { true } else { false } }) }).truthy() {
                v_p = py::iadd(&v_p, &Py::Float(1.0))?;
            } else {
                v_all_won = Py::Bool(false);
                if ({ let t6481 = py::mul(&Py::Int(2i64), &v_above.clone())?; Py::Bool({ let t6482 = v_total.clone(); if py::eq(&t6481, &t6482) { true } else { false } }) }).truthy() {
                    v_p = py::iadd(&v_p, &Py::Float(0.5))?;
                }
            }
        }
        if ({ let t6490 = { let t6483 = py::getitem(&v_first.clone(), &v_k.clone())?; Py::Bool({ let t6484 = v_f.clone(); if py::ne(&t6483, &t6484) { true } else { false } }) }; if t6490.truthy() { t6490 } else { { let t6489 = { let t6485 = py::getitem(&v_borda.clone(), &v_k.clone())?; Py::Bool({ let t6486 = v_b.clone(); if py::ne(&t6485, &t6486) { true } else { false } }) }; if t6489.truthy() { t6489 } else { { let t6487 = py::getitem(&v_points.clone(), &v_k.clone())?; Py::Bool({ let t6488 = v_p.clone(); if py::ne(&t6487, &t6488) { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Bool(false));
        }
        if (v_all_won.clone()).truthy() {
            v_condorcet = py::getitem(&v_cands.clone(), &v_k.clone())?;
        }
    }
    return Ok({ let t6504 = { let t6491 = py::getitem(&v_o.clone(), &Py::str("condorcet_winner"))?; Py::Bool({ let t6492 = v_condorcet.clone(); if py::eq(&t6491, &t6492) { true } else { false } }) }; if !t6504.truthy() { t6504 } else { { let t6503 = { let t6493 = py::getitem(&v_o.clone(), &Py::str("plurality_winner"))?; Py::Bool({ let t6494 = f__vt_winner(v_cands.clone(), v_first.clone())?; if py::eq(&t6493, &t6494) { true } else { false } }) }; if !t6503.truthy() { t6503 } else { { let t6502 = { let t6495 = py::getitem(&v_o.clone(), &Py::str("borda_winner"))?; Py::Bool({ let t6496 = f__vt_winner(v_cands.clone(), v_borda.clone())?; if py::eq(&t6495, &t6496) { true } else { false } }) }; if !t6502.truthy() { t6502 } else { { let t6501 = { let t6497 = py::getitem(&v_o.clone(), &Py::str("pairwise_winner"))?; Py::Bool({ let t6498 = f__vt_winner(v_cands.clone(), v_points.clone())?; if py::eq(&t6497, &t6498) { true } else { false } }) }; if !t6501.truthy() { t6501 } else { { let t6499 = py::getitem(&v_o.clone(), &Py::str("runoff_winner"))?; Py::Bool({ let t6500 = f__vt_runoff(v_groups.clone(), v_cands.clone(), v_total.clone())?; if py::eq(&t6499, &t6500) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_walks(mut v_i: Py, mut v_o: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_vec: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool((py::method(&v_i.clone(), "get", &[Py::str("directed")])?).truthy()), Py::Bool(false))?;
    let t6554 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?, py::int(&py::getitem(&v_i.clone(), &Py::str("length"))?)?])).items()?;
    if t6554.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t6554[0].clone();
    v_t = t6554[1].clone();
    v_k = t6554[2].clone();
    v_vec = Py::dict(vec![(v_t.clone(), Py::Int(1i64))]);
    for t6555 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t6555;
        v_vec = { let mut out = vec![]; for t6556 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t6556;
out.push((v_u.clone(), py::sum(&{ let mut out = vec![]; for t6557 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t6557;
out.push(py::method(&v_vec.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)); } Py::dict(out) };
    }
    return Ok({ let t6561 = f__iv(py::getitem(&v_o.clone(), &Py::str("walks"))?)?; if !t6561.truthy() { t6561 } else { { let t6559 = py::getitem(&v_o.clone(), &Py::str("walks"))?; Py::Bool({ let t6560 = py::method(&v_vec.clone(), "get", &[v_s.clone(), Py::Int(0i64)])?; if py::eq(&t6559, &t6560) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_word(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cnt: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("word"))?)), "strip", &[])?, "upper", &[])?;
    v_cnt = { let mut out = vec![]; for t6578 in py::iter(&(py::sorted(&py::set(&v_w.clone())?, (Py::Bool(false)).truthy())?))? { let mut v_c: Py; 
v_c = t6578;
out.push((v_c.clone(), py::method(&v_w.clone(), "count", &[v_c.clone()])?)); } Py::dict(out) };
    return Ok({ let t6589 = f__iv(py::getitem(&v_o.clone(), &Py::str("arrangements"))?)?; if !t6589.truthy() { t6589 } else { { let t6588 = { let t6580 = py::getitem(&v_o.clone(), &Py::str("letter_counts"))?; Py::Bool({ let t6581 = v_cnt.clone(); if py::eq(&t6580, &t6581) { true } else { false } }) }; if !t6588.truthy() { t6588 } else { { let t6587 = { let t6582 = py::getitem(&v_o.clone(), &Py::str("length"))?; Py::Bool({ let t6583 = py::len(&v_w.clone())?; if py::eq(&t6582, &t6583) { true } else { false } }) }; if !t6587.truthy() { t6587 } else { { let t6584 = py::mul(&py::getitem(&v_o.clone(), &Py::str("arrangements"))?, &py::prod(&{ let mut out = vec![]; for t6586 in py::iter(&(py::method(&v_cnt.clone(), "values", &[])?))? { let mut v_c: Py; 
v_c = t6586;
out.push(py::factorial(&v_c.clone())?); } Py::list(out) })?)?; Py::Bool({ let t6585 = py::factorial(&py::len(&v_w.clone())?)?; if py::eq(&t6584, &t6585) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__venn_whole(mut v_v: Py, mut v_name: Py) -> R {
    return Ok(f__whole(v_v.clone(), v_name.clone(), Py::Int(0i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vt_ballots(mut v_text: Py) -> R {
    let mut v_bits: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_cnt: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_groups: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_groups = Py::list(vec![]);
    for t6375 in py::iter(&(py::method(&py::method(&Py::str(&py::to_str(&v_text.clone())), "replace", &[Py::str("\n"), Py::str(";")])?, "split", &[Py::str(";")])?))? {
        v_part = t6375;
        v_part = py::method(&v_part.clone(), "strip", &[])?;
        if ({ let t6376 = v_part.clone(); Py::Bool({ let t6377 = Py::str(""); if py::eq(&t6376, &t6377) { true } else { false } }) }).truthy() {
            continue;
        }
        v_bits = py::method(&v_part.clone(), "split", &[Py::str(":")])?;
        if ({ let t6378 = py::len(&v_bits.clone())?; Py::Bool({ let t6379 = Py::Int(2i64); if py::ne(&t6378, &t6379) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each group of ballots is written 'count: A > B > C'"))));
        }
        v_cnt = py::method(&py::getitem(&v_bits.clone(), &Py::Int(0i64))?, "strip", &[])?;
        if ({ let t6385 = py::not(&py::method(&v_cnt.clone(), "isdigit", &[])?); if t6385.truthy() { t6385 } else { { let t6384 = { let t6380 = py::int(&v_cnt.clone())?; Py::Bool({ let t6381 = Py::Int(1i64); if py::lt(&t6380, &t6381)? { true } else { false } }) }; if t6384.truthy() { t6384 } else { { let t6382 = py::int(&v_cnt.clone())?; Py::Bool({ let t6383 = py::pow(&Py::Int(10i64), &Py::Int(9i64))?; if py::gt(&t6382, &t6383)? { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the number of voters of a ballot is a whole number >= 1"))));
        }
        v_names = Py::list(vec![]);
        for t6386 in py::iter(&(py::method(&py::getitem(&v_bits.clone(), &Py::Int(1i64))?, "split", &[Py::str(">")])?))? {
            v_w = t6386;
            v_w = py::method(&v_w.clone(), "strip", &[])?;
            if ({ let t6387 = v_w.clone(); Py::Bool({ let t6388 = Py::str(""); if py::eq(&t6387, &t6388) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a candidate name is missing"))));
            }
            if ({ let t6389 = v_w.clone(); Py::Bool({ let t6390 = v_names.clone(); if py::contains(&t6390, &t6389)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a candidate is ranked twice on one ballot"))));
            }
            let _ = py::method(&v_names.clone(), "append", &[v_w.clone()])?;
        }
        let _ = py::method(&v_groups.clone(), "append", &[Py::list(vec![py::int(&v_cnt.clone())?, v_names.clone()])])?;
    }
    if ({ let t6395 = { let t6391 = py::len(&v_groups.clone())?; Py::Bool({ let t6392 = Py::Int(0i64); if py::eq(&t6391, &t6392) { true } else { false } }) }; if t6395.truthy() { t6395 } else { { let t6393 = py::len(&v_groups.clone())?; Py::Bool({ let t6394 = Py::Int(200i64); if py::gt(&t6393, &t6394)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("1 to 200 groups of ballots"))));
    }
    v_cands = Py::list((py::getitem(&py::getitem(&v_groups.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?).items()?);
    let _ = py::method_sort(&v_cands.clone(), None, false)?;
    if ({ let t6400 = { let t6396 = py::len(&v_cands.clone())?; Py::Bool({ let t6397 = Py::Int(2i64); if py::lt(&t6396, &t6397)? { true } else { false } }) }; if t6400.truthy() { t6400 } else { { let t6398 = py::len(&v_cands.clone())?; Py::Bool({ let t6399 = Py::Int(12i64); if py::gt(&t6398, &t6399)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("2 to 12 candidates"))));
    }
    for t6401 in py::iter(&(v_groups.clone()))? {
        v_g = t6401;
        v_s = Py::list((py::getitem(&v_g.clone(), &Py::Int(1i64))?).items()?);
        let _ = py::method_sort(&v_s.clone(), None, false)?;
        if ({ let t6402 = v_s.clone(); Py::Bool({ let t6403 = v_cands.clone(); if py::ne(&t6402, &t6403) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every ballot must rank all the candidates, each once"))));
        }
    }
    return Ok(Py::tuple(vec![v_groups.clone(), v_cands.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vt_runoff(mut v_groups: Py, mut v_cands: Py, mut v_total: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_low: Py = Py::None;
    let mut v_lows: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_remaining: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_remaining = Py::list((v_cands.clone()).items()?);
    while (Py::Bool(true)).truthy() {
        v_counts = Py::list(vec![]);
        for t6434 in py::iter(&(v_remaining.clone()))? {
            v__ = t6434;
            let _ = py::method(&v_counts.clone(), "append", &[Py::Int(0i64)])?;
        }
        for t6435 in py::iter(&(v_groups.clone()))? {
            v_g = t6435;
            for t6436 in py::iter(&(py::getitem(&v_g.clone(), &Py::Int(1i64))?))? {
                v_name = t6436;
                if ({ let t6437 = v_name.clone(); Py::Bool({ let t6438 = v_remaining.clone(); if py::contains(&t6438, &t6437)? { true } else { false } }) }).truthy() {
                    let t6439 = v_counts.clone(); let t6440 = py::method(&v_remaining.clone(), "index", &[v_name.clone()])?;
                    py::setitem(&t6439, &t6440, py::iadd(&py::getitem(&t6439, &t6440)?, &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?)?;
                    break;
                }
            }
        }
        let t6441 = (Py::tuple(vec![py::getitem(&v_counts.clone(), &Py::Int(0i64))?, py::getitem(&v_counts.clone(), &Py::Int(0i64))?])).items()?;
        if t6441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_top = t6441[0].clone();
        v_low = t6441[1].clone();
        for t6442 in py::iter(&(v_counts.clone()))? {
            v_v = t6442;
            v_top = py::minmax(&[v_top.clone(), v_v.clone()], true)?;
            v_low = py::minmax(&[v_low.clone(), v_v.clone()], false)?;
        }
        if ({ let t6443 = py::mul(&Py::Int(2i64), &v_top.clone())?; Py::Bool({ let t6444 = v_total.clone(); if py::gt(&t6443, &t6444)? { true } else { false } }) }).truthy() {
            return Ok(py::getitem(&v_remaining.clone(), &py::method(&v_counts.clone(), "index", &[v_top.clone()])?)?);
        }
        if ({ let t6445 = py::len(&v_remaining.clone())?; Py::Bool({ let t6446 = Py::Int(2i64); if py::eq(&t6445, &t6446) { true } else { false } }) }).truthy() {
            return Ok(py::add(&py::add(&py::add(&Py::str("tie: "), &py::getitem(&v_remaining.clone(), &Py::Int(0i64))?)?, &Py::str(", "))?, &py::getitem(&v_remaining.clone(), &Py::Int(1i64))?)?);
        }
        v_lows = Py::list(vec![]);
        for t6447 in py::range_iter(&[(py::len(&v_remaining.clone())?).i()?]) {
            v_k = t6447;
            if ({ let t6448 = py::getitem(&v_counts.clone(), &v_k.clone())?; Py::Bool({ let t6449 = v_low.clone(); if py::eq(&t6448, &t6449) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_lows.clone(), "append", &[py::getitem(&v_remaining.clone(), &v_k.clone())?])?;
            }
        }
        if ({ let t6450 = py::len(&v_lows.clone())?; Py::Bool({ let t6451 = Py::Int(1i64); if py::gt(&t6450, &t6451)? { true } else { false } }) }).truthy() {
            return Ok(py::add(&Py::str("undecided: tie for elimination between "), &py::method(&Py::str(", "), "join", &[v_lows.clone()])?)?);
        }
        let _ = py::method(&v_remaining.clone(), "remove", &[py::getitem(&v_lows.clone(), &Py::Int(0i64))?])?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vt_winner(mut v_cands: Py, mut v_scores: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_best = py::getitem(&v_scores.clone(), &Py::Int(0i64))?;
    for t6426 in py::iter(&(v_scores.clone()))? {
        v_v = t6426;
        if ({ let t6427 = v_v.clone(); Py::Bool({ let t6428 = v_best.clone(); if py::gt(&t6427, &t6428)? { true } else { false } }) }).truthy() {
            v_best = v_v.clone();
        }
    }
    v_top = Py::list(vec![]);
    for t6429 in py::range_iter(&[(py::len(&v_cands.clone())?).i()?]) {
        v_k = t6429;
        if ({ let t6430 = py::getitem(&v_scores.clone(), &v_k.clone())?; Py::Bool({ let t6431 = v_best.clone(); if py::eq(&t6430, &t6431) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_top.clone(), "append", &[py::getitem(&v_cands.clone(), &v_k.clone())?])?;
        }
    }
    if ({ let t6432 = py::len(&v_top.clone())?; Py::Bool({ let t6433 = Py::Int(1i64); if py::eq(&t6432, &t6433) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_top.clone(), &Py::Int(0i64))?);
    }
    return Ok(py::add(&Py::str("tie: "), &py::method(&Py::str(", "), "join", &[v_top.clone()])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if (Py::Bool(matches!(v_v.clone(), Py::Bool(_)))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        if ({ let t6533 = py::not(&py::math1("isfinite", &v_v.clone())?); if t6533.truthy() { t6533 } else { { let t6531 = v_v.clone(); Py::Bool({ let t6532 = py::int(&v_v.clone())?; if py::ne(&t6531, &t6532) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
        }
        v_v = py::int(&v_v.clone())?;
    } else {
        if (py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))))).truthy() {
            let t6534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                    v_v = py::int(&py::method(&Py::str(&py::to_str(&v_v.clone())), "strip", &[])?)?;
                    #[allow(unreachable_code)]
                    Ok(None)
                })();
            match t6534 {
                Ok(Some(r)) => { return Ok(r); }
                Ok(None) => {}
                Err(e) if (matches!(e, PyErr::Value(_))) => {
                    return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
                }
                Err(e) => return Err(e),
            }
        }
    }
    if ({ let t6539 = { let t6535 = v_lo.clone(); Py::Bool({ let t6536 = Py::None; if !py::is(&t6535, &t6536) { true } else { false } }) }; if !t6539.truthy() { t6539 } else { { let t6537 = v_v.clone(); Py::Bool({ let t6538 = v_lo.clone(); if py::lt(&t6537, &t6538)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    if ({ let t6544 = { let t6540 = v_hi.clone(); Py::Bool({ let t6541 = Py::None; if !py::is(&t6540, &t6541) { true } else { false } }) }; if !t6544.truthy() { t6544 } else { { let t6542 = py::abs(&v_v.clone())?; Py::Bool({ let t6543 = v_hi.clone(); if py::gt(&t6542, &t6543)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?, " in size".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wlist(mut v_xs: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py, mut v_n_min: Py, mut v_n_max: Py) -> R {
    let mut v_out: Py = Py::None;
    if (Py::Bool(matches!(v_xs.clone(), Py::Str(_)))).truthy() {
        v_xs = { let mut out = vec![]; for t5082 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[,\\s;]+"), &[py::method(&v_xs.clone(), "strip", &[Py::str("[] ")])?, Py::Int(0)], 0)?))? { let mut v_t: Py; 
v_t = t5082;
if (v_t.clone()).truthy() { out.push(v_t.clone()); }} Py::list(out) };
    }
    v_out = { let mut out = vec![]; for t5083 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t5083;
out.push(f__whole(v_x.clone(), v_name.clone(), v_lo.clone(), v_hi.clone())?); } Py::list(out) };
    if ({ let t5098 = py::len(&v_out.clone())?; Py::Bool({ let t5099 = v_n_min.clone(); if py::lt(&t5098, &t5099)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " entries".to_string()].concat()))));
    }
    if ({ let t5104 = { let t5100 = v_n_max.clone(); Py::Bool({ let t5101 = Py::None; if !py::is(&t5100, &t5101) { true } else { false } }) }; if !t5104.truthy() { t5104 } else { { let t5102 = py::len(&v_out.clone())?; Py::Bool({ let t5103 = v_n_max.clone(); if py::gt(&t5102, &t5103)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " allows at most ".to_string(), py::format(&v_n_max.clone(), "")?, " entries".to_string()].concat()))));
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_affine_cipher(mut v_message: Py, mut v_a: Py, mut v_b: Py, mut v_mode: Py) -> R {
    let mut v_ai: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let t15 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?])).items()?;
    if t15.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t15[0].clone();
    v_b = t15[1].clone();
    if ({ let t16 = py::gcd(&v_a.clone(), &Py::Int(26i64))?; Py::Bool({ let t17 = Py::Int(1i64); if py::ne(&t16, &t17) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must be coprime to 26"))));
    }
    v_xs = f__letters(v_message.clone())?;
    if (py::not(&v_xs.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message has no letters"))));
    }
    if ({ let t22 = v_mode.clone(); Py::Bool({ let t23 = Py::str("encrypt"); if py::eq(&t22, &t23) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("result"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t24 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t24;
out.push(py::chr(&py::add(&Py::Int(65i64), &py::modulo(&py::add(&py::mul(&v_a.clone(), &v_x.clone())?, &v_b.clone())?, &Py::Int(26i64))?)?)?); } Py::list(out) }])?)]));
    }
    v_ai = f__inv(v_a.clone(), Py::Int(26i64))?;
    return Ok(Py::dict(vec![(Py::str("result"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t32 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t32;
out.push(py::chr(&py::add(&Py::Int(65i64), &py::modulo(&py::mul(&v_ai.clone(), &py::sub(&v_x.clone(), &v_b.clone())?)?, &Py::Int(26i64))?)?)?); } Py::list(out) }])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_apportionment(mut v_populations: Py, mut v_seats: Py, mut v_method: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pops: Py = Py::None;
    let mut v_quotas: Py = Py::None;
    let mut v_rems: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tie: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_pops = f__wlist(v_populations.clone(), Py::str("populations"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?, Py::Int(2i64), Py::Int(50i64))?;
    v_h = f__whole(v_seats.clone(), Py::str("seats"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(5i64))?)?;
    if ({ let t77 = v_method.clone(); Py::Bool({ let t78 = c__APP_METHODS()?; if !py::contains(&t78, &t77)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the method is hamilton, jefferson, webster, adams or huntington-hill"))));
    }
    v_n = py::len(&v_pops.clone())?;
    v_total = Py::Int(0i64);
    for t79 in py::iter(&(v_pops.clone()))? {
        v_p = t79;
        v_total = py::iadd(&v_total, &v_p.clone())?;
    }
    v_quotas = Py::list(vec![]);
    for t80 in py::iter(&(v_pops.clone()))? {
        v_p = t80;
        let _ = py::method(&v_quotas.clone(), "append", &[py::truediv(&py::mul(&v_p.clone(), &v_h.clone())?, &v_total.clone())?])?;
    }
    v_s = Py::list(vec![]);
    if ({ let t81 = v_method.clone(); Py::Bool({ let t82 = Py::str("hamilton"); if py::eq(&t81, &t82) { true } else { false } }) }).truthy() {
        v_rems = Py::list(vec![]);
        v_given = Py::Int(0i64);
        for t83 in py::iter(&(v_pops.clone()))? {
            v_p = t83;
            let _ = py::method(&v_s.clone(), "append", &[py::floordiv(&py::mul(&v_p.clone(), &v_h.clone())?, &v_total.clone())?])?;
            let _ = py::method(&v_rems.clone(), "append", &[py::modulo(&py::mul(&v_p.clone(), &v_h.clone())?, &v_total.clone())?])?;
            v_given = py::iadd(&v_given, &py::floordiv(&py::mul(&v_p.clone(), &v_h.clone())?, &v_total.clone())?)?;
        }
        v_left = py::sub(&v_h.clone(), &v_given.clone())?;
        v_order = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
        for t84 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_a = t84;
            for t85 in py::range_iter(&[(py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_a.clone())?).i()?]) {
                v_b = t85;
                if ({ let t86 = py::getitem(&v_rems.clone(), &py::getitem(&v_order.clone(), &v_b.clone())?)?; Py::Bool({ let t87 = py::getitem(&v_rems.clone(), &py::getitem(&v_order.clone(), &py::add(&v_b.clone(), &Py::Int(1i64))?)?)?; if py::lt(&t86, &t87)? { true } else { false } }) }).truthy() {
                    v_t = py::getitem(&v_order.clone(), &v_b.clone())?;
                    py::setitem(&v_order.clone(), &v_b.clone(), py::getitem(&v_order.clone(), &py::add(&v_b.clone(), &Py::Int(1i64))?)?)?;
                    py::setitem(&v_order.clone(), &py::add(&v_b.clone(), &Py::Int(1i64))?, v_t.clone())?;
                }
            }
        }
        if ({ let t95 = { let t88 = v_left.clone(); Py::Bool({ let t89 = Py::Int(0i64); if py::gt(&t88, &t89)? { true } else { false } }) }; if !t95.truthy() { t95 } else { { let t94 = { let t90 = v_left.clone(); Py::Bool({ let t91 = v_n.clone(); if py::lt(&t90, &t91)? { true } else { false } }) }; if !t94.truthy() { t94 } else { { let t92 = py::getitem(&v_rems.clone(), &py::getitem(&v_order.clone(), &py::sub(&v_left.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t93 = py::getitem(&v_rems.clone(), &py::getitem(&v_order.clone(), &v_left.clone())?)?; if py::eq(&t92, &t93) { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("two states tie for the last seat (equal remainders): Hamilton's method does not decide it"))));
        }
        for t96 in py::range_iter(&[(v_left.clone()).i()?]) {
            v_k = t96;
            let t97 = v_s.clone(); let t98 = py::getitem(&v_order.clone(), &v_k.clone())?;
            py::setitem(&t97, &t98, py::iadd(&py::getitem(&t97, &t98)?, &Py::Int(1i64))?)?;
        }
    } else {
        v_k = Py::Int(0i64);
        for t99 in py::iter(&(v_pops.clone()))? {
            v__ = t99;
            let _ = py::method(&v_s.clone(), "append", &[Py::Int(0i64)])?;
        }
        if ({ let t104 = { let t100 = v_method.clone(); Py::Bool({ let t101 = Py::str("adams"); if py::eq(&t100, &t101) { true } else { false } }) }; if t104.truthy() { t104 } else { { let t102 = v_method.clone(); Py::Bool({ let t103 = Py::str("huntington-hill"); if py::eq(&t102, &t103) { true } else { false } }) } } }).truthy() {
            if ({ let t105 = v_h.clone(); Py::Bool({ let t106 = v_n.clone(); if py::lt(&t105, &t106)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("this method gives every state at least one seat: the seats must number at least the states"))));
            }
            for t107 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_j = t107;
                py::setitem(&v_s.clone(), &v_j.clone(), Py::Int(1i64))?;
            }
            v_k = v_n.clone();
        }
        while ({ let t140 = v_k.clone(); Py::Bool({ let t141 = v_h.clone(); if py::lt(&t140, &t141)? { true } else { false } }) }).truthy() {
            v_best = Py::Int(0i64);
            v_tie = Py::Bool(false);
            for t108 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
                v_j = t108;
                if (f__ap_gt(v_method.clone(), py::getitem(&v_pops.clone(), &v_j.clone())?, py::add(&py::getitem(&v_s.clone(), &v_j.clone())?, &Py::Int(1i64))?, py::getitem(&v_pops.clone(), &v_best.clone())?, py::add(&py::getitem(&v_s.clone(), &v_best.clone())?, &Py::Int(1i64))?)?).truthy() {
                    v_best = v_j.clone();
                    v_tie = Py::Bool(false);
                } else {
                    if (py::not(&f__ap_gt(v_method.clone(), py::getitem(&v_pops.clone(), &v_best.clone())?, py::add(&py::getitem(&v_s.clone(), &v_best.clone())?, &Py::Int(1i64))?, py::getitem(&v_pops.clone(), &v_j.clone())?, py::add(&py::getitem(&v_s.clone(), &v_j.clone())?, &Py::Int(1i64))?)?)).truthy() {
                        v_tie = Py::Bool(true);
                    }
                }
            }
            if ({ let t137 = v_tie.clone(); if !t137.truthy() { t137 } else { { let t135 = v_k.clone(); Py::Bool({ let t136 = py::sub(&v_h.clone(), &Py::Int(1i64))?; if py::eq(&t135, &t136) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("two states tie for the last seat: the method does not decide it"))));
            }
            let t138 = v_s.clone(); let t139 = v_best.clone();
            py::setitem(&t138, &t139, py::iadd(&py::getitem(&t138, &t139)?, &Py::Int(1i64))?)?;
            v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("seats"), v_s.clone()), (Py::str("standard_divisor"), py::truediv(&v_total.clone(), &v_h.clone())?), (Py::str("quotas"), v_quotas.clone()), (Py::str("total_seats"), v_h.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_articulation_points(mut v_edges: Py) -> R {
    let mut v__b: Py = Py::None;
    let mut v_aps: Py = Py::None;
    let t234 = (f__lowlink(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?).items()?;
    if t234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_aps = t234[0].clone();
    v__b = t234[1].clone();
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_aps.clone())?), (Py::str("points"), v_aps.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_base_convert(mut v_digits: Py, mut v_from_base: Py, mut v_to_base: Py) -> R {
    let mut v_ch: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_neg: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tb: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t287 = (Py::tuple(vec![f__whole(v_from_base.clone(), Py::str("from_base"), Py::Int(2i64), Py::Int(36i64))?, f__whole(v_to_base.clone(), Py::str("to_base"), Py::Int(2i64), Py::Int(36i64))?])).items()?;
    if t287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fb = t287[0].clone();
    v_tb = t287[1].clone();
    v_s = py::method(&py::method(&py::method(&py::method(&Py::str(&py::to_str(&v_digits.clone())), "strip", &[])?, "upper", &[])?, "replace", &[Py::str("_"), Py::str("")])?, "replace", &[Py::str(" "), Py::str("")])?;
    v_neg = py::method(&v_s.clone(), "startswith", &[Py::str("-")])?;
    v_s = (if (v_neg.clone()).truthy() { py::slice(&v_s.clone(), Some((Py::Int(1i64)).i()?), None, None)? } else { v_s.clone() });
    if ({ let t290 = py::not(&v_s.clone()); if t290.truthy() { t290 } else { { let t288 = py::len(&v_s.clone())?; Py::Bool({ let t289 = Py::Int(200i64); if py::gt(&t288, &t289)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the number's digits (at most 200)"))));
    }
    v_v = Py::Int(0i64);
    for t291 in py::iter(&(v_s.clone()))? {
        v_ch = t291;
        v_d = py::method(&c__DIG()?, "find", &[v_ch.clone()])?;
        if ({ let t296 = { let t292 = v_d.clone(); Py::Bool({ let t293 = Py::Int(0i64); if py::lt(&t292, &t293)? { true } else { false } }) }; if t296.truthy() { t296 } else { { let t294 = v_d.clone(); Py::Bool({ let t295 = v_fb.clone(); if py::ge(&t294, &t295)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["'".to_string(), py::format(&v_ch.clone(), "")?, "' is not a digit in base ".to_string(), py::format(&v_fb.clone(), "")?].concat()))));
        }
        v_v = py::add(&py::mul(&v_v.clone(), &v_fb.clone())?, &v_d.clone())?;
    }
    v_v = (if (v_neg.clone()).truthy() { py::neg(&v_v.clone())? } else { v_v.clone() });
    if ({ let t297 = py::abs(&v_v.clone())?; Py::Bool({ let t298 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t297, &t298)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number is too large"))));
    }
    return Ok(Py::dict(vec![(Py::str("result"), f__to_base(v_v.clone(), v_tb.clone())?), (Py::str("decimal"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bell(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(200i64))?;
    v_row = Py::list(vec![Py::Int(1i64)]);
    for t350 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t350;
        v_new = Py::list(vec![py::getitem(&v_row.clone(), &py::neg(&Py::Int(1i64))?)?]);
        for t351 in py::iter(&(v_row.clone()))? {
            v_x = t351;
            let _ = py::method(&v_new.clone(), "append", &[py::add(&py::getitem(&v_new.clone(), &py::neg(&Py::Int(1i64))?)?, &v_x.clone())?])?;
        }
        v_row = v_new.clone();
    }
    return Ok(Py::dict(vec![(Py::str("bell"), py::getitem(&v_row.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bernoulli(mut v_n: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(200i64))?;
    v_B = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    for t381 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t381;
        let _ = py::method(&v_B.clone(), "append", &[py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t382 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t382;
out.push(py::mul(&py::comb(&py::add(&v_m.clone(), &Py::Int(1i64))?, &v_k.clone())?, &py::getitem(&v_B.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::add(&v_m.clone(), &Py::Int(1i64))?)?])?;
    }
    v_b = py::getitem(&v_B.clone(), &v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("numerator"), py::numerator(&v_b.clone())?), (Py::str("denominator"), py::denominator(&v_b.clone())?), (Py::str("value"), py::float(&v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bfs_order(mut v_edges: Py, mut v_source: Py, mut v_directed: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool((v_directed.clone()).truthy()), Py::Bool(false))?;
    v_s = py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?;
    if ({ let t425 = v_s.clone(); Py::Bool({ let t426 = v_adj.clone(); if !py::contains(&t426, &t425)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_s.clone(), "")?, " is not a vertex of the graph".to_string()].concat()))));
    }
    let t431 = (f__bfs(v_adj.clone(), v_s.clone(), Py::Bool(false))?).items()?;
    if t431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_order = t431[0].clone();
    v_dist = t431[1].clone();
    return Ok(Py::dict(vec![(Py::str("order"), v_order.clone()), (Py::str("reached"), py::len(&v_order.clone())?), (Py::str("distances"), { let mut out = vec![]; for t432 in py::iter(&(v_order.clone()))? { let mut v_v: Py; 
v_v = t432;
out.push((v_v.clone(), py::getitem(&v_dist.clone(), &v_v.clone())?)); } Py::dict(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binomial_term(mut v_a: Py, mut v_b: Py, mut v_n: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let t467 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?])).items()?;
    if t467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t467[0].clone();
    v_b = t467[1].clone();
    let t468 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t468[0].clone();
    v_k = t468[1].clone();
    if ({ let t469 = v_k.clone(); Py::Bool({ let t470 = v_n.clone(); if py::gt(&t469, &t470)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("coefficient"), Py::Int(0i64))]));
    }
    v_c = py::mul(&py::mul(&py::comb(&v_n.clone(), &v_k.clone())?, &py::pow(&v_a.clone(), &v_k.clone())?)?, &py::pow(&v_b.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?;
    if ({ let t471 = py::abs(&v_c.clone())?; Py::Bool({ let t472 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t471, &t472)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficient exceeds 10^300"))));
    }
    return Ok(Py::dict(vec![(Py::str("coefficient"), v_c.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bipartite(mut v_edges: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("is_bipartite"), { let t487 = f__two_colour(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?; Py::Bool({ let t488 = Py::None; if !py::is(&t487, &t488) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bridges(mut v_edges: Py) -> R {
    let mut v__a: Py = Py::None;
    let mut v_brs: Py = Py::None;
    let t660 = (f__lowlink(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?).items()?;
    if t660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__a = t660[0].clone();
    v_brs = t660[1].clone();
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_brs.clone())?), (Py::str("bridges"), v_brs.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_caesar(mut v_message: Py, mut v_shift: Py, mut v_mode: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_s = f__whole(v_shift.clone(), Py::str("shift"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?;
    v_sign = (if ({ let t713 = v_mode.clone(); Py::Bool({ let t714 = Py::str("encrypt"); if py::eq(&t713, &t714) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    v_xs = f__letters(v_message.clone())?;
    if (py::not(&v_xs.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message has no letters"))));
    }
    return Ok(Py::dict(vec![(Py::str("result"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t719 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t719;
out.push(py::chr(&py::add(&Py::Int(65i64), &py::modulo(&py::add(&v_x.clone(), &py::mul(&v_sign.clone(), &v_s.clone())?)?, &Py::Int(26i64))?)?)?); } Py::list(out) }])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_carmichael_lambda(mut v_n: Py) -> R {
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?;
    return Ok(Py::dict(vec![(Py::str("lambda"), (if ({ let t755 = v_n.clone(); Py::Bool({ let t756 = Py::Int(1i64); if py::gt(&t755, &t756)? { true } else { false } }) }).truthy() { f__carmichael_from(f__factor(v_n.clone())?)? } else { Py::Int(1i64) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_catalan(mut v_n: Py) -> R {
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(500i64))?;
    return Ok(Py::dict(vec![(Py::str("catalan"), py::floordiv(&py::comb(&py::mul(&Py::Int(2i64), &v_n.clone())?, &v_n.clone())?, &py::add(&v_n.clone(), &Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cf_rational(mut v_numerator: Py, mut v_denominator: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let t885 = (Py::tuple(vec![f__whole(v_numerator.clone(), Py::str("numerator"), Py::None, c_MAXI()?)?, f__whole(v_denominator.clone(), Py::str("denominator"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t885[0].clone();
    v_q = t885[1].clone();
    v_terms = Py::list(vec![]);
    while (v_q.clone()).truthy() {
        v_a = py::floordiv(&v_p.clone(), &v_q.clone())?;
        let _ = py::method(&v_terms.clone(), "append", &[v_a.clone()])?;
        let t886 = (Py::tuple(vec![v_q.clone(), py::sub(&v_p.clone(), &py::mul(&v_a.clone(), &v_q.clone())?)?])).items()?;
        if t886.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t886[0].clone();
        v_q = t886[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("terms"), v_terms.clone()), (Py::str("length"), py::len(&v_terms.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cf_sqrt(mut v_d: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_period: Py = Py::None;
    v_d = f__whole(v_d.clone(), Py::str("d"), Py::Int(2i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    v_a0 = py::isqrt(&v_d.clone())?;
    if ({ let t4665 = py::mul(&v_a0.clone(), &v_a0.clone())?; Py::Bool({ let t4666 = v_d.clone(); if py::eq(&t4665, &t4666) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("d must not be a perfect square (its square root is an integer)"))));
    }
    let t4667 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), v_a0.clone(), Py::list(vec![])])).items()?;
    if t4667.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4667[0].clone();
    v_den = t4667[1].clone();
    v_a = t4667[2].clone();
    v_period = t4667[3].clone();
    while ({ let t4668 = v_a.clone(); Py::Bool({ let t4669 = py::mul(&Py::Int(2i64), &v_a0.clone())?; if py::ne(&t4668, &t4669) { true } else { false } }) }).truthy() {
        v_m = py::sub(&py::mul(&v_den.clone(), &v_a.clone())?, &v_m.clone())?;
        v_den = py::floordiv(&py::sub(&v_d.clone(), &py::mul(&v_m.clone(), &v_m.clone())?)?, &v_den.clone())?;
        v_a = py::floordiv(&py::add(&v_a0.clone(), &v_m.clone())?, &v_den.clone())?;
        let _ = py::method(&v_period.clone(), "append", &[v_a.clone()])?;
    }
    return Ok(Py::dict(vec![(Py::str("a0"), v_a0.clone()), (Py::str("period"), v_period.clone()), (Py::str("period_length"), py::len(&v_period.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chromatic_number(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    if ({ let t982 = py::len(&v_adj.clone())?; Py::Bool({ let t983 = Py::Int(12i64); if py::gt(&t982, &t983)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 vertices (exact colouring is exponential)"))));
    }
    v_nodes = f__sorted(v_adj.clone())?;
    for t984 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t984;
        v_col = f__colourable(v_nodes.clone(), v_adj.clone(), v_k.clone())?;
        if (v_col.clone()).truthy() {
            return Ok(Py::dict(vec![(Py::str("chromatic_number"), v_k.clone()), (Py::str("colouring"), { let mut out = vec![]; for t994 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t994;
out.push((v_v.clone(), py::getitem(&v_col.clone(), &v_v.clone())?)); } Py::dict(out) })]));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_clique_number(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_sets: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    if ({ let t1042 = py::len(&v_adj.clone())?; Py::Bool({ let t1043 = Py::Int(60i64); if py::gt(&t1042, &t1043)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 vertices"))));
    }
    v_sets = { let mut out = vec![]; for t1044 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t1045 = (t1044).items()?;
if t1045.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1045[0].clone();
v_nb = t1045[1].clone();
out.push((v_u.clone(), py::set(&v_nb.clone())?)); } Py::dict(out) };
    v_c = f__max_clique(v_sets.clone())?;
    return Ok(Py::dict(vec![(Py::str("clique_number"), py::len(&v_c.clone())?), (Py::str("clique"), v_c.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combinations(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let t1105 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t1105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1105[0].clone();
    v_k = t1105[1].clone();
    if ({ let t1106 = v_k.clone(); Py::Bool({ let t1107 = v_n.clone(); if py::gt(&t1106, &t1107)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not exceed n"))));
    }
    v_c = Py::Int(1i64);
    for t1108 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_k.clone(), py::sub(&v_n.clone(), &v_k.clone())?], false)?, &Py::Int(1i64))?).i()?]) {
        v_i = t1108;
        v_c = py::floordiv(&py::mul(&v_c.clone(), &py::add(&py::sub(&v_n.clone(), &py::minmax(&[v_k.clone(), py::sub(&v_n.clone(), &v_k.clone())?], false)?)?, &v_i.clone())?)?, &v_i.clone())?;
    }
    if ({ let t1109 = v_c.clone(); Py::Bool({ let t1110 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t1109, &t1110)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result exceeds 10^300"))));
    }
    return Ok(Py::dict(vec![(Py::str("combinations"), v_c.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_connected_components(mut v_edges: Py) -> R {
    let mut v_comps: Py = Py::None;
    v_comps = py::sorted_key(&f__components(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?, &{ let v_edges = v_edges.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__nkey(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_comps.clone())?), (Py::str("sizes"), py::sorted(&{ let mut out = vec![]; for t1151 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t1151;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?), (Py::str("is_connected"), { let t1152 = py::len(&v_comps.clone())?; Py::Bool({ let t1153 = Py::Int(1i64); if py::eq(&t1152, &t1153) { true } else { false } }) }), (Py::str("components"), v_comps.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crt(mut v_residues: Py, mut v_moduli: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mg: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_rs = f__wlist(v_residues.clone(), Py::str("residues"), Py::None, c_MAXI()?, Py::Int(2i64), Py::Int(12i64))?;
    v_ms = f__wlist(v_moduli.clone(), Py::str("moduli"), Py::Int(1i64), c_MAXI()?, Py::Int(2i64), Py::Int(12i64))?;
    if ({ let t1207 = py::len(&v_rs.clone())?; Py::Bool({ let t1208 = py::len(&v_ms.clone())?; if py::ne(&t1207, &t1208) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one residue for each modulus"))));
    }
    let t1209 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t1209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1209[0].clone();
    v_M = t1209[1].clone();
    for t1210 in py::iter(&(py::zip(&[v_rs.clone(), v_ms.clone()])?))? {
        let t1222 = (t1210).items()?;
        if t1222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t1222[0].clone();
        v_m = t1222[1].clone();
        v_g = py::gcd(&v_M.clone(), &v_m.clone())?;
        if (py::modulo(&py::sub(&v_r.clone(), &v_x.clone())?, &v_g.clone())?).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the congruences are inconsistent: no integer satisfies all of them"))));
        }
        v_mg = py::floordiv(&v_m.clone(), &v_g.clone())?;
        v_t = py::modulo(&py::mul(&py::floordiv(&py::sub(&v_r.clone(), &v_x.clone())?, &v_g.clone())?, &(if ({ let t1211 = v_mg.clone(); Py::Bool({ let t1212 = Py::Int(1i64); if py::gt(&t1211, &t1212)? { true } else { false } }) }).truthy() { f__inv(py::modulo(&py::floordiv(&v_M.clone(), &v_g.clone())?, &v_mg.clone())?, v_mg.clone())? } else { Py::Int(0i64) }))?, &(if ({ let t1220 = v_mg.clone(); Py::Bool({ let t1221 = Py::Int(1i64); if py::gt(&t1220, &t1221)? { true } else { false } }) }).truthy() { v_mg.clone() } else { Py::Int(1i64) }))?;
        v_x = py::add(&v_x.clone(), &py::mul(&v_M.clone(), &v_t.clone())?)?;
        v_M = py::mul(&v_M.clone(), &v_mg.clone())?;
        v_x = py::modulo(&v_x, &v_M.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("modulus"), v_M.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cycle_rank(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_c = py::len(&f__components(v_adj.clone())?)?;
    v_m = py::floordiv(&py::sum(&{ let mut out = vec![]; for t1276 in py::iter(&(py::method(&v_adj.clone(), "values", &[])?))? { let mut v_nb: Py; 
v_nb = t1276;
out.push(py::len(&v_nb.clone())?); } Py::list(out) }, None)?, &Py::Int(2i64))?;
    v_r = py::add(&py::sub(&v_m.clone(), &py::len(&v_adj.clone())?)?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("cyclomatic_number"), v_r.clone()), (Py::str("has_cycle"), { let t1277 = v_r.clone(); Py::Bool({ let t1278 = Py::Int(0i64); if py::gt(&t1277, &t1278)? { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_derangements(mut v_n: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c__NCAP()?)?;
    let t1318 = (Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)])).items()?;
    if t1318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1318[0].clone();
    v_b = t1318[1].clone();
    if ({ let t1319 = v_n.clone(); Py::Bool({ let t1320 = Py::Int(0i64); if py::eq(&t1319, &t1320) { true } else { false } }) }).truthy() {
        v_d = Py::Int(1i64);
    } else {
        for t1321 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t1321;
            let t1322 = (Py::tuple(vec![v_b.clone(), py::mul(&py::sub(&v_i.clone(), &Py::Int(1i64))?, &py::add(&v_a.clone(), &v_b.clone())?)?])).items()?;
            if t1322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t1322[0].clone();
            v_b = t1322[1].clone();
        }
        v_d = v_b.clone();
    }
    return Ok(Py::dict(vec![(Py::str("derangements"), v_d.clone()), (Py::str("probability"), py::float(&py::fraction(&[v_d.clone(), py::factorial(&v_n.clone())?])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dfs_order(mut v_edges: Py, mut v_source: Py, mut v_directed: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool((v_directed.clone()).truthy()), Py::Bool(false))?;
    v_s = py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?;
    if ({ let t1358 = v_s.clone(); Py::Bool({ let t1359 = v_adj.clone(); if !py::contains(&t1359, &t1358)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_s.clone(), "")?, " is not a vertex of the graph".to_string()].concat()))));
    }
    v_order = ax_engine::pysym::helper("discrete_math:_dfs", &[v_adj.clone(), v_s.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("order"), v_order.clone()), (Py::str("reached"), py::len(&v_order.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffie_hellman(mut v_p: Py, mut v_g: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let t1395 = (Py::tuple(vec![f__whole(v_p.clone(), Py::str("p"), Py::Int(3i64), c_MAXI()?)?, f__whole(v_g.clone(), Py::str("g"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_a.clone(), Py::str("a"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t1395.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1395[0].clone();
    v_g = t1395[1].clone();
    v_a = t1395[2].clone();
    v_b = t1395[3].clone();
    let _ = f__need_prime(v_p.clone(), Py::str("p"))?;
    if ({ let t1418 = v_g.clone(); Py::Bool({ let t1419 = v_p.clone(); if py::ge(&t1418, &t1419)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the generator g must satisfy 1 < g < p"))));
    }
    let t1420 = (Py::tuple(vec![py::pow_mod(&v_g.clone(), &v_a.clone(), &v_p.clone())?, py::pow_mod(&v_g.clone(), &v_b.clone(), &v_p.clone())?])).items()?;
    if t1420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1420[0].clone();
    v_B = t1420[1].clone();
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("shared"), py::pow_mod(&v_B.clone(), &v_a.clone(), &v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_directed_cycle(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_indeg: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(true), Py::Bool(false))?;
    v_indeg = { let mut out = vec![]; for t1459 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1459;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    for t1460 in py::iter(&(v_adj.clone()))? {
        v_u = t1460;
        for t1461 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t1461;
            let t1462 = v_indeg.clone(); let t1463 = v_v.clone();
            py::setitem(&t1462, &t1463, py::iadd(&py::getitem(&t1462, &t1463)?, &Py::Int(1i64))?)?;
        }
    }
    v_stack = { let mut out = vec![]; for t1464 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1464;
if ({ let t1465 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t1466 = Py::Int(0i64); if py::eq(&t1465, &t1466) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    v_seen = Py::Int(0i64);
    while (v_stack.clone()).truthy() {
        v_u = py::method(&v_stack.clone(), "pop", &[])?;
        v_seen = py::iadd(&v_seen, &Py::Int(1i64))?;
        for t1467 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t1467;
            let t1468 = v_indeg.clone(); let t1469 = v_v.clone();
            py::setitem(&t1468, &t1469, py::sub(&py::getitem(&t1468, &t1469)?, &Py::Int(1i64))?)?;
            if ({ let t1470 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t1471 = Py::Int(0i64); if py::eq(&t1470, &t1471) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("has_cycle"), { let t1472 = v_seen.clone(); Py::Bool({ let t1473 = py::len(&v_adj.clone())?; if py::ne(&t1472, &t1473) { true } else { false } }) }), (Py::str("is_dag"), { let t1474 = v_seen.clone(); Py::Bool({ let t1475 = py::len(&v_adj.clone())?; if py::eq(&t1474, &t1475) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discrete_log(mut v_base: Py, mut v_target: Py, mut v_modulus: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_factor: Py = Py::None;
    let mut v_gamma: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_table: Py = Py::None;
    let t1514 = (Py::tuple(vec![f__whole(v_base.clone(), Py::str("base"), Py::None, c_MAXI()?)?, f__whole(v_target.clone(), Py::str("target"), Py::None, c_MAXI()?)?, f__whole(v_modulus.clone(), Py::str("modulus"), Py::Int(2i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?])).items()?;
    if t1514.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1514[0].clone();
    v_t = t1514[1].clone();
    v_m = t1514[2].clone();
    if ({ let t1515 = py::gcd(&v_b.clone(), &v_m.clone())?; Py::Bool({ let t1516 = Py::Int(1i64); if py::ne(&t1515, &t1516) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("base and modulus must be coprime"))));
    }
    let t1517 = (Py::tuple(vec![py::modulo(&v_b.clone(), &v_m.clone())?, py::modulo(&v_t.clone(), &v_m.clone())?])).items()?;
    if t1517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1517[0].clone();
    v_t = t1517[1].clone();
    v_s = py::add(&py::isqrt(&v_m.clone())?, &Py::Int(1i64))?;
    let t1518 = (Py::tuple(vec![Py::dict(vec![]), Py::Int(1i64)])).items()?;
    if t1518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_table = t1518[0].clone();
    v_cur = t1518[1].clone();
    for t1519 in py::range_iter(&[(v_s.clone()).i()?]) {
        v_j = t1519;
        let _ = py::method(&v_table.clone(), "setdefault", &[v_cur.clone(), v_j.clone()])?;
        v_cur = py::modulo(&py::mul(&v_cur.clone(), &v_b.clone())?, &v_m.clone())?;
    }
    v_factor = py::pow_mod(&f__inv(v_b.clone(), v_m.clone())?, &v_s.clone(), &v_m.clone())?;
    v_gamma = v_t.clone();
    for t1527 in py::range_iter(&[(py::add(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t1527;
        if ({ let t1528 = v_gamma.clone(); Py::Bool({ let t1529 = v_table.clone(); if py::contains(&t1529, &t1528)? { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("x"), py::add(&py::mul(&v_i.clone(), &v_s.clone())?, &py::getitem(&v_table.clone(), &v_gamma.clone())?)?)]));
        }
        v_gamma = py::modulo(&py::mul(&v_gamma.clone(), &v_factor.clone())?, &v_m.clone())?;
    }
    return Err(py::value_error(py::to_str(&Py::str(&["no exponent x satisfies ".to_string(), py::format(&v_base.clone(), "")?, "^x ≡ ".to_string(), py::format(&v_target.clone(), "")?, " (mod ".to_string(), py::format(&v_modulus.clone(), "")?, ")".to_string()].concat()))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_divisor_functions(mut v_n: Py) -> R {
    let mut v_cls: Py = Py::None;
    let mut v_ds: Py = Py::None;
    let mut v_proper: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    v_ds = f__divisors_from(f__factor(v_n.clone())?)?;
    v_s = py::sum(&v_ds.clone(), None)?;
    v_proper = py::sub(&v_s.clone(), &v_n.clone())?;
    v_cls = (if ({ let t1606 = v_proper.clone(); Py::Bool({ let t1607 = v_n.clone(); if py::eq(&t1606, &t1607) { true } else { false } }) }).truthy() { Py::str("perfect") } else { (if ({ let t1608 = v_proper.clone(); Py::Bool({ let t1609 = v_n.clone(); if py::gt(&t1608, &t1609)? { true } else { false } }) }).truthy() { Py::str("abundant") } else { Py::str("deficient") }) });
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_ds.clone())?), (Py::str("sum"), v_s.clone()), (Py::str("proper_sum"), v_proper.clone()), (Py::str("classification"), v_cls.clone()), (Py::str("divisors"), v_ds.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elgamal_encrypt(mut v_p: Py, mut v_g: Py, mut v_x: Py, mut v_k: Py, mut v_m: Py) -> R {
    let mut v_h: Py = Py::None;
    let t1671 = (Py::tuple(vec![f__whole(v_p.clone(), Py::str("p"), Py::Int(3i64), c_MAXI()?)?, f__whole(v_g.clone(), Py::str("g"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_x.clone(), Py::str("x"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_k.clone(), Py::str("k"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t1671.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1671[0].clone();
    v_g = t1671[1].clone();
    v_x = t1671[2].clone();
    v_k = t1671[3].clone();
    v_m = t1671[4].clone();
    let _ = f__need_prime(v_p.clone(), Py::str("p"))?;
    if ({ let t1698 = { let t1694 = v_g.clone(); Py::Bool({ let t1695 = v_p.clone(); if py::ge(&t1694, &t1695)? { true } else { false } }) }; if t1698.truthy() { t1698 } else { { let t1696 = v_m.clone(); Py::Bool({ let t1697 = v_p.clone(); if py::ge(&t1696, &t1697)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("g and m must be between 1 and p - 1"))));
    }
    v_h = py::pow_mod(&v_g.clone(), &v_x.clone(), &v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("c1"), py::pow_mod(&v_g.clone(), &v_k.clone(), &v_p.clone())?), (Py::str("c2"), py::modulo(&py::mul(&v_m.clone(), &py::pow_mod(&v_h.clone(), &v_k.clone(), &v_p.clone())?)?, &v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_euler_phi(mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_phi: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?;
    v_phi = v_n.clone();
    for t4454 in py::iter(&(f__factor(v_n.clone())?))? {
        v_p = t4454;
        v_phi = py::mul(&py::floordiv(&v_phi.clone(), &v_p.clone())?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)?;
    }
    return Ok(Py::dict(vec![(Py::str("phi"), v_phi.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eulerian(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_conn: Py = Py::None;
    let mut v_odd: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_odd = py::sum(&{ let mut out = vec![]; for t1812 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1812;
if (py::modulo(&py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?, &Py::Int(2i64))?).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_conn = { let t1813 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t1814 = Py::Int(1i64); if py::eq(&t1813, &t1814) { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("odd_vertices"), v_odd.clone()), (Py::str("connected"), v_conn.clone()), (Py::str("has_circuit"), { let t1825 = v_conn.clone(); if !t1825.truthy() { t1825 } else { { let t1823 = v_odd.clone(); Py::Bool({ let t1824 = Py::Int(0i64); if py::eq(&t1823, &t1824) { true } else { false } }) } } }), (Py::str("has_path"), { let t1828 = v_conn.clone(); if !t1828.truthy() { t1828 } else { { let t1826 = v_odd.clone(); Py::Bool({ let t1827 = Py::tuple(vec![Py::Int(0i64), Py::Int(2i64)]); if py::contains(&t1827, &t1826)? { true } else { false } }) } } })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_extended_gcd(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t1864 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t1864.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1864[0].clone();
    v_b = t1864[1].clone();
    let t1869 = (f__ext(v_a.clone(), v_b.clone())?).items()?;
    if t1869.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1869[0].clone();
    v_x = t1869[1].clone();
    v_y = t1869[2].clone();
    v_step = py::floordiv(&v_b.clone(), &v_g.clone())?;
    v_x0 = py::modulo(&v_x.clone(), &v_step.clone())?;
    return Ok(Py::dict(vec![(Py::str("gcd"), v_g.clone()), (Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone()), (Py::str("x_min"), v_x0.clone()), (Py::str("y_min"), py::floordiv(&py::sub(&v_g.clone(), &py::mul(&v_a.clone(), &v_x0.clone())?)?, &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fibonacci(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?;
    let t1908 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t1908.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1908[0].clone();
    v_b = t1908[1].clone();
    for t1909 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t1909;
        let t1910 = (Py::tuple(vec![v_b.clone(), py::add(&v_a.clone(), &v_b.clone())?])).items()?;
        if t1910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1910[0].clone();
        v_b = t1910[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("fibonacci"), v_a.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gcd_lcm(mut v_numbers: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_xs = f__wlist(v_numbers.clone(), Py::str("numbers"), Py::Int(1i64), c_MAXI()?, Py::Int(2i64), Py::Int(20i64))?;
    let t1951 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t1951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1951[0].clone();
    v_l = t1951[1].clone();
    for t1952 in py::iter(&(v_xs.clone()))? {
        v_x = t1952;
        v_g = f__euclid(v_g.clone(), v_x.clone())?;
        v_l = py::mul(&py::floordiv(&v_l.clone(), &f__euclid(v_l.clone(), v_x.clone())?)?, &v_x.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("gcd"), v_g.clone()), (Py::str("lcm"), v_l.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_girth(mut v_edges: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("girth"), f__girth(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?, Py::Bool(true))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graph_degrees(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_deg: Py = Py::None;
    let mut v_seq: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_deg = { let mut out = vec![]; for t2070 in py::iter(&(f__sorted(v_adj.clone())?))? { let mut v_v: Py; 
v_v = t2070;
out.push((v_v.clone(), py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?)); } Py::dict(out) };
    v_seq = py::sorted(&py::method(&v_deg.clone(), "values", &[])?, (Py::Bool(true)).truthy())?;
    return Ok(Py::dict(vec![(Py::str("vertices"), py::len(&v_adj.clone())?), (Py::str("edges"), py::floordiv(&py::sum(&v_seq.clone(), None)?, &Py::Int(2i64))?), (Py::str("degrees"), v_deg.clone()), (Py::str("degree_sequence"), v_seq.clone()), (Py::str("max_degree"), py::getitem(&v_seq.clone(), &Py::Int(0i64))?), (Py::str("min_degree"), py::getitem(&v_seq.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graph_diameter(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ecc: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    if ({ let t2119 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t2120 = Py::Int(1i64); if py::ne(&t2119, &t2120) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph is not connected (the diameter is infinite)"))));
    }
    v_ecc = { let mut out = vec![]; for t2129 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t2129;
out.push((v_v.clone(), py::minmax(&[py::method(&py::getitem(&f__bfs(v_adj.clone(), v_v.clone(), Py::Bool(false))?, &Py::Int(1i64))?, "values", &[])?], true)?)); } Py::dict(out) };
    let t2134 = (Py::tuple(vec![py::minmax(&[py::method(&v_ecc.clone(), "values", &[])?], true)?, py::minmax(&[py::method(&v_ecc.clone(), "values", &[])?], false)?])).items()?;
    if t2134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2134[0].clone();
    v_r = t2134[1].clone();
    return Ok(Py::dict(vec![(Py::str("diameter"), v_d.clone()), (Py::str("radius"), v_r.clone()), (Py::str("center"), f__sorted({ let mut out = vec![]; for t2135 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t2135;
if ({ let t2136 = py::getitem(&v_ecc.clone(), &v_v.clone())?; Py::Bool({ let t2137 = v_r.clone(); if py::eq(&t2136, &t2137) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?), (Py::str("periphery"), f__sorted({ let mut out = vec![]; for t2138 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t2138;
if ({ let t2139 = py::getitem(&v_ecc.clone(), &v_v.clone())?; Py::Bool({ let t2140 = v_d.clone(); if py::eq(&t2139, &t2140) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graph_triangles(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_sets: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_sets = { let mut out = vec![]; for t2204 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t2205 = (t2204).items()?;
if t2205.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2205[0].clone();
v_nb = t2205[1].clone();
out.push((v_u.clone(), py::set(&v_nb.clone())?)); } Py::dict(out) };
    v_t = py::floordiv(&py::sum(&{ let mut out = vec![]; for t2206 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t2206;
for t2207 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t2207;
if ({ let t2208 = f__nkey(v_u.clone())?; Py::Bool({ let t2209 = f__nkey(v_v.clone())?; if py::lt(&t2208, &t2209)? { true } else { false } }) }).truthy() { out.push(py::len(&py::bitand(&py::getitem(&v_sets.clone(), &v_u.clone())?, &py::getitem(&v_sets.clone(), &v_v.clone())?)?)?); }}} Py::list(out) }, None)?, &Py::Int(3i64))?;
    return Ok(Py::dict(vec![(Py::str("triangles"), v_t.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graphical_sequence(mut v_degrees: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_d = py::sorted(&f__wlist(v_degrees.clone(), Py::str("degrees"), Py::Int(0i64), py::pow(&Py::Int(10i64), &Py::Int(4i64))?, Py::Int(1i64), Py::Int(2000i64))?, (Py::Bool(true)).truthy())?;
    while ({ let t2266 = v_d.clone(); if !t2266.truthy() { t2266 } else { { let t2264 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t2265 = Py::Int(0i64); if py::gt(&t2264, &t2265)? { true } else { false } }) } } }).truthy() {
        v_k = py::method(&v_d.clone(), "pop", &[Py::Int(0i64)])?;
        if ({ let t2257 = v_k.clone(); Py::Bool({ let t2258 = py::len(&v_d.clone())?; if py::gt(&t2257, &t2258)? { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("is_graphical"), Py::Bool(false))]));
        }
        for t2259 in py::range_iter(&[(v_k.clone()).i()?]) {
            v_j = t2259;
            let t2260 = v_d.clone(); let t2261 = v_j.clone();
            py::setitem(&t2260, &t2261, py::sub(&py::getitem(&t2260, &t2261)?, &Py::Int(1i64))?)?;
            if ({ let t2262 = py::getitem(&v_d.clone(), &v_j.clone())?; Py::Bool({ let t2263 = Py::Int(0i64); if py::lt(&t2262, &t2263)? { true } else { false } }) }).truthy() {
                return Ok(Py::dict(vec![(Py::str("is_graphical"), Py::Bool(false))]));
            }
        }
        let _ = py::method_sort(&v_d.clone(), None, (Py::Bool(true)).truthy())?;
    }
    return Ok(Py::dict(vec![(Py::str("is_graphical"), Py::Bool(true))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hamiltonian_cycle(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_extend: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_used: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_nodes = f__sorted(v_adj.clone())?;
    if ({ let t2304 = py::len(&v_nodes.clone())?; Py::Bool({ let t2305 = Py::Int(12i64); if py::gt(&t2304, &t2305)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 vertices (the search is exponential)"))));
    }
    if ({ let t2306 = py::len(&v_nodes.clone())?; Py::Bool({ let t2307 = Py::Int(3i64); if py::lt(&t2306, &t2307)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("exists"), Py::Bool(false)), (Py::str("cycle"), Py::None)]));
    }
    v_s = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?;
    let t2308 = (Py::tuple(vec![Py::list(vec![v_s.clone()]), py::set(&Py::list(vec![v_s.clone()]))?])).items()?;
    if t2308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_path = t2308[0].clone();
    v_used = t2308[1].clone();
    let rec2309 = py::rec_cell();
    v_extend = { let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_nodes = v_nodes.clone(); let v_path = v_path.clone(); let v_s = v_s.clone(); let v_used = v_used.clone(); let rec2309 = rec2309.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_extend: Py = py::rec_get(&rec2309)?;
            let mut v_v: Py = Py::None;
            if ({ let t2310 = py::len(&v_path.clone())?; Py::Bool({ let t2311 = py::len(&v_nodes.clone())?; if py::eq(&t2310, &t2311) { true } else { false } }) }).truthy() {
                return Ok({ let t2312 = v_s.clone(); Py::Bool({ let t2313 = py::getitem(&v_adj.clone(), &py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?)?; if py::contains(&t2313, &t2312)? { true } else { false } }) });
            }
            for t2314 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?)?)?))? {
                v_v = t2314;
                if ({ let t2315 = v_v.clone(); Py::Bool({ let t2316 = v_used.clone(); if !py::contains(&t2316, &t2315)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_path.clone(), "append", &[v_v.clone()])?;
                    let _ = py::method(&v_used.clone(), "add", &[v_v.clone()])?;
                    if (py::call(&v_extend.clone(), &[])?).truthy() {
                        return Ok(Py::Bool(true));
                    }
                    let _ = py::method(&v_path.clone(), "pop", &[])?;
                    let _ = py::method(&v_used.clone(), "discard", &[v_v.clone()])?;
                }
            }
            return Ok(Py::Bool(false));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2309, &v_extend);
    return Ok((if (py::call(&v_extend.clone(), &[])?).truthy() { Py::dict(vec![(Py::str("exists"), Py::Bool(true)), (Py::str("cycle"), py::add(&v_path.clone(), &Py::list(vec![v_s.clone()]))?)]) } else { Py::dict(vec![(Py::str("exists"), Py::Bool(false)), (Py::str("cycle"), Py::None)]) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inclusion_exclusion(mut v_n: Py, mut v_divisors: Py) -> R {
    let mut v_ds: Py = Py::None;
    let mut v_walk: Py = Py::None;
    let c_total = py::cell_new();
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?;
    v_ds = py::sorted(&py::set(&f__wlist(v_divisors.clone(), Py::str("divisors"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(9i64))?, Py::Int(1i64), Py::Int(12i64))?)?, (Py::Bool(false)).truthy())?;
    py::cell_set(&c_total, Py::Int(0i64));
    let rec2380 = py::rec_cell();
    v_walk = { let v_divisors = v_divisors.clone(); let v_ds = v_ds.clone(); let v_n = v_n.clone(); let c_total = c_total.clone(); let rec2380 = rec2380.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_start: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_l: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_size: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_walk: Py = py::rec_get(&rec2380)?;
            let mut v_i: Py = Py::None;
            let mut v_l2: Py = Py::None;
            for t2381 in py::range_iter(&[(v_start.clone()).i()?, (py::len(&v_ds.clone())?).i()?]) {
                v_i = t2381;
                v_l2 = py::mul(&py::floordiv(&v_l.clone(), &py::gcd(&v_l.clone(), &py::getitem(&v_ds.clone(), &v_i.clone())?)?)?, &py::getitem(&v_ds.clone(), &v_i.clone())?)?;
                if ({ let t2382 = v_l2.clone(); Py::Bool({ let t2383 = v_n.clone(); if py::gt(&t2382, &t2383)? { true } else { false } }) }).truthy() {
                    continue;
                }
                py::cell_set(&c_total, py::iadd(&py::cell_get(&c_total)?, &py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_size.clone())?, &py::floordiv(&v_n.clone(), &v_l2.clone())?)?)?);
                let _ = py::call(&v_walk.clone(), &[py::add(&v_i.clone(), &Py::Int(1i64))?, v_l2.clone(), py::add(&v_size.clone(), &Py::Int(1i64))?])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2380, &v_walk);
    let _ = py::call(&v_walk.clone(), &[Py::Int(0i64), Py::Int(1i64), Py::Int(0i64)])?;
    return Ok(Py::dict(vec![(Py::str("divisible_by_any"), py::cell_get(&c_total)?), (Py::str("divisible_by_none"), py::sub(&v_n.clone(), &py::cell_get(&c_total)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_independence_number(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_comp: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    if ({ let t2427 = py::len(&v_adj.clone())?; Py::Bool({ let t2428 = Py::Int(60i64); if py::gt(&t2427, &t2428)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 vertices"))));
    }
    v_comp = { let mut out = vec![]; for t2429 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t2429;
out.push((v_u.clone(), py::sub(&py::sub(&py::set(&v_adj.clone())?, &py::set(&py::getitem(&v_adj.clone(), &v_u.clone())?)?)?, &py::set(&Py::list(vec![v_u.clone()]))?)?)); } Py::dict(out) };
    v_s = f__max_clique(v_comp.clone())?;
    return Ok(Py::dict(vec![(Py::str("independence_number"), py::len(&v_s.clone())?), (Py::str("independent_set"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_integer_partitions(mut v_n: Py) -> R {
    let mut v_g1: Py = Py::None;
    let mut v_g2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sign: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(2000i64))?;
    v_p = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
    for t2490 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t2490;
        let t2491 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
        if t2491.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2491[0].clone();
        v_k = t2491[1].clone();
        while (Py::Bool(true)).truthy() {
            v_g1 = py::floordiv(&py::mul(&v_k.clone(), &py::sub(&py::mul(&Py::Int(3i64), &v_k.clone())?, &Py::Int(1i64))?)?, &Py::Int(2i64))?;
            if ({ let t2492 = v_g1.clone(); Py::Bool({ let t2493 = v_m.clone(); if py::gt(&t2492, &t2493)? { true } else { false } }) }).truthy() {
                break;
            }
            v_sign = (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
            v_s = py::iadd(&v_s, &py::mul(&v_sign.clone(), &py::getitem(&v_p.clone(), &py::sub(&v_m.clone(), &v_g1.clone())?)?)?)?;
            v_g2 = py::floordiv(&py::mul(&v_k.clone(), &py::add(&py::mul(&Py::Int(3i64), &v_k.clone())?, &Py::Int(1i64))?)?, &Py::Int(2i64))?;
            if ({ let t2494 = v_g2.clone(); Py::Bool({ let t2495 = v_m.clone(); if py::le(&t2494, &t2495)? { true } else { false } }) }).truthy() {
                v_s = py::iadd(&v_s, &py::mul(&v_sign.clone(), &py::getitem(&v_p.clone(), &py::sub(&v_m.clone(), &v_g2.clone())?)?)?)?;
            }
            v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        }
        py::setitem(&v_p.clone(), &v_m.clone(), v_s.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("partitions"), py::getitem(&v_p.clone(), &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_integer_sqrt(mut v_n: Py) -> R {
    let mut v_r: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c_MAXI()?)?;
    v_r = py::isqrt(&v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("isqrt"), v_r.clone()), (Py::str("remainder"), py::sub(&v_n.clone(), &py::mul(&v_r.clone(), &v_r.clone())?)?), (Py::str("is_square"), { let t2527 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t2528 = v_n.clone(); if py::eq(&t2527, &t2528) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_is_prime(mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_spf: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c_MAXI()?)?;
    if ({ let t2569 = v_n.clone(); Py::Bool({ let t2570 = py::pow(&Py::Int(2i64), &Py::Int(64i64))?; if py::ge(&t2569, &t2570)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be below 2**64"))));
    }
    v_p = f__is_prime(v_n.clone())?;
    v_spf = (if ({ let t2593 = v_n.clone(); Py::Bool({ let t2594 = Py::Int(2i64); if py::lt(&t2593, &t2594)? { true } else { false } }) }).truthy() { Py::None } else { (if (v_p.clone()).truthy() { v_n.clone() } else { py::minmax(&[f__factor(v_n.clone())?], false)? }) });
    return Ok(Py::dict(vec![(Py::str("is_prime"), v_p.clone()), (Py::str("smallest_prime_factor"), v_spf.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_is_tree(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_conn: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_m = py::floordiv(&py::sum(&{ let mut out = vec![]; for t2670 in py::iter(&(py::method(&v_adj.clone(), "values", &[])?))? { let mut v_nb: Py; 
v_nb = t2670;
out.push(py::len(&v_nb.clone())?); } Py::list(out) }, None)?, &Py::Int(2i64))?;
    v_conn = { let t2671 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t2672 = Py::Int(1i64); if py::eq(&t2671, &t2672) { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("is_tree"), { let t2683 = v_conn.clone(); if !t2683.truthy() { t2683 } else { { let t2681 = v_m.clone(); Py::Bool({ let t2682 = py::sub(&py::len(&v_adj.clone())?, &Py::Int(1i64))?; if py::eq(&t2681, &t2682) { true } else { false } }) } } }), (Py::str("vertices"), py::len(&v_adj.clone())?), (Py::str("edges"), v_m.clone()), (Py::str("connected"), v_conn.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jacobi_symbol(mut v_a: Py, mut v_n: Py) -> R {
    let t2732 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t2732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2732[0].clone();
    v_n = t2732[1].clone();
    if ({ let t2733 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t2734 = Py::Int(0i64); if py::eq(&t2733, &t2734) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be odd"))));
    }
    return Ok(Py::dict(vec![(Py::str("symbol"), f__jacobi(v_a.clone(), v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lattice_paths(mut v_right: Py, mut v_up: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t2815 = (Py::tuple(vec![f__whole(v_right.clone(), Py::str("right"), Py::Int(0i64), Py::Int(1000i64))?, f__whole(v_up.clone(), Py::str("up"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t2815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2815[0].clone();
    v_n = t2815[1].clone();
    if ({ let t2816 = py::add(&v_m.clone(), &v_n.clone())?; Py::Bool({ let t2817 = Py::Int(1000i64); if py::gt(&t2816, &t2817)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("right + up must be at most 1000"))));
    }
    return Ok(Py::dict(vec![(Py::str("paths"), py::comb(&py::add(&v_m.clone(), &v_n.clone())?, &v_m.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_legendre_symbol(mut v_a: Py, mut v_p: Py) -> R {
    let t2846 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_p.clone(), Py::str("p"), Py::Int(3i64), c_MAXI()?)?])).items()?;
    if t2846.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2846[0].clone();
    v_p = t2846[1].clone();
    if ({ let t2871 = { let t2847 = py::modulo(&v_p.clone(), &Py::Int(2i64))?; Py::Bool({ let t2848 = Py::Int(0i64); if py::eq(&t2847, &t2848) { true } else { false } }) }; if t2871.truthy() { t2871 } else { py::not(&f__is_prime(v_p.clone())?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p must be an odd prime"))));
    }
    return Ok(Py::dict(vec![(Py::str("symbol"), f__jacobi(v_a.clone(), v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_congruence(mut v_a: Py, mut v_b: Py, mut v_m: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let t2912 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t2912.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2912[0].clone();
    v_b = t2912[1].clone();
    v_m = t2912[2].clone();
    v_g = (if (py::modulo(&v_a.clone(), &v_m.clone())?).truthy() { py::gcd(&py::modulo(&v_a.clone(), &v_m.clone())?, &v_m.clone())? } else { v_m.clone() });
    if (py::modulo(&v_b.clone(), &v_g.clone())?).truthy() {
        return Ok(Py::dict(vec![(Py::str("smallest"), Py::None), (Py::str("count"), Py::Int(0i64)), (Py::str("step"), Py::None)]));
    }
    v_mp = py::floordiv(&v_m.clone(), &v_g.clone())?;
    v_x0 = (if ({ let t2913 = v_mp.clone(); Py::Bool({ let t2914 = Py::Int(1i64); if py::gt(&t2913, &t2914)? { true } else { false } }) }).truthy() { py::modulo(&py::mul(&py::modulo(&py::floordiv(&v_b.clone(), &v_g.clone())?, &v_mp.clone())?, &f__inv(py::modulo(&py::floordiv(&v_a.clone(), &v_g.clone())?, &v_mp.clone())?, v_mp.clone())?)?, &v_mp.clone())? } else { Py::Int(0i64) });
    return Ok(Py::dict(vec![(Py::str("smallest"), v_x0.clone()), (Py::str("count"), v_g.clone()), (Py::str("step"), v_mp.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_diophantine(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let t2968 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, c_MAXI()?)?, f__whole(v_c.clone(), Py::str("c"), Py::None, c_MAXI()?)?])).items()?;
    if t2968.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2968[0].clone();
    v_b = t2968[1].clone();
    v_c = t2968[2].clone();
    if ({ let t2973 = { let t2969 = v_a.clone(); Py::Bool({ let t2970 = Py::Int(0i64); if py::eq(&t2969, &t2970) { true } else { false } }) }; if t2973.truthy() { t2973 } else { { let t2971 = v_b.clone(); Py::Bool({ let t2972 = Py::Int(0i64); if py::eq(&t2971, &t2972) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and b must both be non-zero"))));
    }
    let t2978 = (f__ext(py::abs(&v_a.clone())?, py::abs(&v_b.clone())?)?).items()?;
    if t2978.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2978[0].clone();
    v_s = t2978[1].clone();
    v_t = t2978[2].clone();
    v_step = py::floordiv(&py::abs(&v_b.clone())?, &v_g.clone())?;
    if (py::modulo(&v_c.clone(), &v_g.clone())?).truthy() {
        return Ok(Py::dict(vec![(Py::str("solvable"), Py::Bool(false)), (Py::str("x0"), Py::None), (Py::str("y0"), Py::None), (Py::str("x_step"), py::floordiv(&v_b.clone(), &v_g.clone())?), (Py::str("y_step"), py::neg(&py::floordiv(&v_a.clone(), &v_g.clone())?)?)]));
    }
    v_x = py::mul(&py::mul(&v_s.clone(), &(if ({ let t2979 = v_a.clone(); Py::Bool({ let t2980 = Py::Int(0i64); if py::gt(&t2979, &t2980)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?, &py::floordiv(&v_c.clone(), &v_g.clone())?)?;
    v_x0 = py::modulo(&v_x.clone(), &v_step.clone())?;
    return Ok(Py::dict(vec![(Py::str("solvable"), Py::Bool(true)), (Py::str("x0"), v_x0.clone()), (Py::str("y0"), py::floordiv(&py::sub(&v_c.clone(), &py::mul(&v_a.clone(), &v_x0.clone())?)?, &v_b.clone())?), (Py::str("x_step"), py::floordiv(&v_b.clone(), &v_g.clone())?), (Py::str("y_step"), py::neg(&py::floordiv(&v_a.clone(), &v_g.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logic_contradiction(mut v_formula: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = f_logic_satisfiable(v_formula.clone())?;
    return Ok(Py::dict(vec![(Py::str("contradiction"), py::not(&py::getitem(&v_s.clone(), &Py::str("satisfiable"))?)), (Py::str("true_rows"), py::getitem(&v_s.clone(), &Py::str("models"))?), (Py::str("witness"), py::getitem(&v_s.clone(), &Py::str("first_model"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logic_equivalence(mut v_formula1: Py, mut v_formula2: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v__t1: Py = Py::None;
    let mut v__t2: Py = Py::None;
    let mut v_diff: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let t3152 = (f__parse_formula(v_formula1.clone())?).items()?;
    if t3152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__t1 = t3152[0].clone();
    v_n1 = t3152[1].clone();
    let t3153 = (f__parse_formula(v_formula2.clone())?).items()?;
    if t3153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__t2 = t3153[0].clone();
    v_n2 = t3153[1].clone();
    v_names = py::sorted(&py::bitor(&py::set(&v_n1.clone())?, &py::set(&v_n2.clone())?)?, (Py::Bool(false)).truthy())?;
    if ({ let t3154 = py::len(&v_names.clone())?; Py::Bool({ let t3155 = c_MAX_VARS()?; if py::gt(&t3154, &t3155)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&c_MAX_VARS()?, "")?, " variables in total".to_string()].concat()))));
    }
    let t3177 = (f__table(v_formula1.clone(), v_names.clone())?).items()?;
    if t3177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t3177[0].clone();
    v__ = t3177[1].clone();
    let t3178 = (f__table(v_formula2.clone(), v_names.clone())?).items()?;
    if t3178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t3178[0].clone();
    v__ = t3178[1].clone();
    v_diff = { let mut out = vec![]; for t3179 in py::range_iter(&[(py::len(&v_v1.clone())?).i()?]) { let mut v_r: Py; 
v_r = t3179;
if ({ let t3180 = py::getitem(&v_v1.clone(), &v_r.clone())?; Py::Bool({ let t3181 = py::getitem(&v_v2.clone(), &v_r.clone())?; if py::ne(&t3180, &t3181) { true } else { false } }) }).truthy() { out.push(v_r.clone()); }} Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("equivalent"), py::not(&v_diff.clone())), (Py::str("differing_rows"), py::len(&v_diff.clone())?), (Py::str("counterexample"), (if (v_diff.clone()).truthy() { f__assign(py::getitem(&v_diff.clone(), &Py::Int(0i64))?, v_names.clone())? } else { Py::None }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logic_evaluate(mut v_formula: Py, mut v_assignment: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_extra: Py = Py::None;
    let mut v_missing: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_tree: Py = Py::None;
    let t3242 = (f__parse_formula(v_formula.clone())?).items()?;
    if t3242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tree = t3242[0].clone();
    v_names = t3242[1].clone();
    v_env = f__read_assignment(v_assignment.clone())?;
    v_missing = { let mut out = vec![]; for t3252 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t3252;
if ({ let t3253 = v_v.clone(); Py::Bool({ let t3254 = v_env.clone(); if !py::contains(&t3254, &t3253)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    v_extra = { let mut out = vec![]; for t3255 in py::iter(&(v_env.clone()))? { let mut v_v: Py; 
v_v = t3255;
if ({ let t3256 = v_v.clone(); Py::Bool({ let t3257 = v_names.clone(); if !py::contains(&t3257, &t3256)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    if (v_missing.clone()).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("no truth value given for "), &py::method(&Py::str(", "), "join", &[v_missing.clone()])?)?)));
    }
    if (v_extra.clone()).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&py::method(&Py::str(", "), "join", &[v_extra.clone()])?, &Py::str(" does not occur in the formula"))?)));
    }
    return Ok(Py::dict(vec![(Py::str("value"), f__ev(v_tree.clone(), v_env.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logic_satisfiable(mut v_formula: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3351 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t3351.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3351[0].clone();
    v_names = t3351[1].clone();
    v_first = (match ('g3352: { let mut out = vec![]; for t3353 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3354 = (t3353).items()?;
if t3354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3354[0].clone();
v_v = t3354[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); break 'g3352 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok(Py::dict(vec![(Py::str("satisfiable"), Py::Bool((v_vals.clone()).items()?.iter().any(|x| x.truthy()))), (Py::str("models"), py::sum(&v_vals.clone(), None)?), (Py::str("first_model"), (if ({ let t3355 = v_first.clone(); Py::Bool({ let t3356 = Py::None; if py::is(&t3355, &t3356) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logic_tautology(mut v_formula: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3448 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t3448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3448[0].clone();
    v_names = t3448[1].clone();
    v_first = (match ('g3449: { let mut out = vec![]; for t3450 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3451 = (t3450).items()?;
if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3451[0].clone();
v_v = t3451[1].clone();
if (py::not(&v_v.clone())).truthy() { out.push(v_r.clone()); break 'g3449 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok(Py::dict(vec![(Py::str("tautology"), Py::Bool((v_vals.clone()).items()?.iter().all(|x| x.truthy()))), (Py::str("false_rows"), py::sub(&py::len(&v_vals.clone())?, &py::sum(&v_vals.clone(), None)?)?), (Py::str("counterexample"), (if ({ let t3452 = v_first.clone(); Py::Bool({ let t3453 = Py::None; if py::is(&t3452, &t3453) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lucas(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?;
    let t3493 = (Py::tuple(vec![Py::Int(2i64), Py::Int(1i64)])).items()?;
    if t3493.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3493[0].clone();
    v_b = t3493[1].clone();
    for t3494 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t3494;
        let t3495 = (Py::tuple(vec![v_b.clone(), py::add(&v_a.clone(), &v_b.clone())?])).items()?;
        if t3495.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3495[0].clone();
        v_b = t3495[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("lucas"), v_a.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minimal_cnf(mut v_formula: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_on: Py = Py::None;
    let t3584 = (f__formula_fn(v_formula.clone())?).items()?;
    if t3584.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3584[0].clone();
    v_on = t3584[1].clone();
    v_names = t3584[2].clone();
    return Ok(f__pos(v_n.clone(), v_on.clone(), Py::list(vec![]), v_names.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minimal_dnf(mut v_formula: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_on: Py = Py::None;
    let t3779 = (f__formula_fn(v_formula.clone())?).items()?;
    if t3779.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3779[0].clone();
    v_on = t3779[1].clone();
    v_names = t3779[2].clone();
    return Ok(f__sop(v_n.clone(), v_on.clone(), Py::list(vec![]), v_names.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minimal_sop_minterms(mut v_minterms: Py, mut v_variables: Py, mut v_dont_cares: Py) -> R {
    let mut v_dc: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_on: Py = Py::None;
    v_n = f__whole(v_variables.clone(), Py::str("variables"), Py::Int(1i64), Py::Int(6i64))?;
    v_on = (if ({ let t3900 = v_minterms.clone(); Py::Bool({ let t3901 = Py::tuple(vec![Py::None, Py::str(""), Py::list(vec![])]); if !py::contains(&t3901, &t3900)? { true } else { false } }) }).truthy() { py::sorted(&py::set(&f__wlist(v_minterms.clone(), Py::str("minterms"), Py::Int(0i64), py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?, Py::Int(0i64), Py::Int(64i64))?)?, (Py::Bool(false)).truthy())? } else { Py::list(vec![]) });
    v_dc = (if ({ let t3911 = v_dont_cares.clone(); Py::Bool({ let t3912 = Py::tuple(vec![Py::None, Py::str(""), Py::list(vec![])]); if !py::contains(&t3912, &t3911)? { true } else { false } }) }).truthy() { py::sorted(&py::set(&f__wlist(v_dont_cares.clone(), Py::str("dont_cares"), Py::Int(0i64), py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?, Py::Int(0i64), Py::Int(64i64))?)?, (Py::Bool(false)).truthy())? } else { Py::list(vec![]) });
    for t3913 in py::iter(&(py::add(&v_on.clone(), &v_dc.clone())?))? {
        v_m = t3913;
        if ({ let t3914 = v_m.clone(); Py::Bool({ let t3915 = py::lshift(&Py::Int(1i64), &v_n.clone())?; if py::ge(&t3914, &t3915)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["minterm ".to_string(), py::format(&v_m.clone(), "")?, " needs more than ".to_string(), py::format(&v_n.clone(), "")?, " variables".to_string()].concat()))));
        }
    }
    if (py::bitand(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a row cannot be both a minterm and a don't-care"))));
    }
    return Ok(f__sop(v_n.clone(), v_on.clone(), v_dc.clone(), Py::list((py::slice(&Py::str("ABCDEF"), None, Some((v_n.clone()).i()?), None)?).items()?))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mobius(mut v_n: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_sqfree: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?;
    v_f = f__factor(v_n.clone())?;
    v_sqfree = Py::Bool(({ let mut out = vec![]; for t4077 in py::iter(&(py::method(&v_f.clone(), "values", &[])?))? { let mut v_e: Py; 
v_e = t4077;
out.push({ let t4078 = v_e.clone(); Py::Bool({ let t4079 = Py::Int(1i64); if py::eq(&t4078, &t4079) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()));
    return Ok(Py::dict(vec![(Py::str("mu"), (if (v_sqfree.clone()).truthy() { py::pow(&py::neg(&Py::Int(1i64))?, &py::len(&v_f.clone())?)? } else { Py::Int(0i64) })), (Py::str("squarefree"), v_sqfree.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mod_inverse(mut v_a: Py, mut v_m: Py) -> R {
    let t4124 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t4124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4124[0].clone();
    v_m = t4124[1].clone();
    return Ok(Py::dict(vec![(Py::str("inverse"), f__inv(v_a.clone(), v_m.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mod_pow(mut v_base: Py, mut v_exponent: Py, mut v_modulus: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_result: Py = Py::None;
    let t4164 = (Py::tuple(vec![f__whole(v_base.clone(), Py::str("base"), Py::None, c_MAXI()?)?, f__whole(v_exponent.clone(), Py::str("exponent"), Py::Int(0i64), c_MAXI()?)?, f__whole(v_modulus.clone(), Py::str("modulus"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4164.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4164[0].clone();
    v_e = t4164[1].clone();
    v_m = t4164[2].clone();
    let t4165 = (Py::tuple(vec![py::modulo(&Py::Int(1i64), &v_m.clone())?, py::modulo(&v_a.clone(), &v_m.clone())?])).items()?;
    if t4165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_result = t4165[0].clone();
    v_a = t4165[1].clone();
    while (v_e.clone()).truthy() {
        if (py::bitand(&v_e.clone(), &Py::Int(1i64))?).truthy() {
            v_result = py::modulo(&py::mul(&v_result.clone(), &v_a.clone())?, &v_m.clone())?;
        }
        v_a = py::modulo(&py::mul(&v_a.clone(), &v_a.clone())?, &v_m.clone())?;
        v_e = py::rshift(&v_e, &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("result"), v_result.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mod_reduce(mut v_a: Py, mut v_m: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t4194 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4194[0].clone();
    v_m = t4194[1].clone();
    let t4195 = (Py::tuple(vec![py::floordiv(&v_a.clone(), &v_m.clone())?, py::modulo(&v_a.clone(), &v_m.clone())?])).items()?;
    if t4195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t4195[0].clone();
    v_r = t4195[1].clone();
    return Ok(Py::dict(vec![(Py::str("remainder"), v_r.clone()), (Py::str("quotient"), v_q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multinomial(mut v_counts: Py, mut v_total: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_ks = f__wlist(v_counts.clone(), Py::str("counts"), Py::Int(0i64), c_MAXI()?, Py::Int(1i64), Py::Int(30i64))?;
    v_n = py::sum(&v_ks.clone(), None)?;
    if ({ let t4242 = { let t4238 = v_total.clone(); Py::Bool({ let t4239 = Py::None; if !py::is(&t4238, &t4239) { true } else { false } }) }; if !t4242.truthy() { t4242 } else { { let t4240 = f__whole(v_total.clone(), Py::str("total"), Py::Int(0i64), c_MAXI()?)?; Py::Bool({ let t4241 = v_n.clone(); if py::ne(&t4240, &t4241) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the group sizes add up to ".to_string(), py::format(&v_n.clone(), "")?, ", not to the stated total ".to_string(), py::format(&v_total.clone(), "")?].concat()))));
    }
    if ({ let t4243 = v_n.clone(); Py::Bool({ let t4244 = c__NCAP()?; if py::gt(&t4243, &t4244)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the counts may add up to at most ".to_string(), py::format(&c__NCAP()?, "")?].concat()))));
    }
    v_r = py::factorial(&v_n.clone())?;
    for t4245 in py::iter(&(v_ks.clone()))? {
        v_k = t4245;
        v_r = py::floordiv(&v_r, &py::factorial(&v_k.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("coefficient"), v_r.clone()), (Py::str("total"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multiplicative_order(mut v_a: Py, mut v_n: Py) -> R {
    let t4276 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t4276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4276[0].clone();
    v_n = t4276[1].clone();
    if ({ let t4277 = py::gcd(&v_a.clone(), &v_n.clone())?; Py::Bool({ let t4278 = Py::Int(1i64); if py::ne(&t4277, &t4278) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and n must be coprime"))));
    }
    return Ok(Py::dict(vec![(Py::str("order"), f__order(py::modulo(&v_a.clone(), &v_n.clone())?, v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multisets(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let t4377 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), Py::Int(1000i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t4377.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4377[0].clone();
    v_k = t4377[1].clone();
    v_c = py::comb(&py::sub(&py::add(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?, &v_k.clone())?;
    if ({ let t4378 = v_c.clone(); Py::Bool({ let t4379 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t4378, &t4379)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result exceeds 10^300"))));
    }
    return Ok(Py::dict(vec![(Py::str("multisets"), v_c.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_necklaces(mut v_beads: Py, mut v_colours: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4408 = (Py::tuple(vec![f__whole(v_beads.clone(), Py::str("beads"), Py::Int(1i64), Py::Int(60i64))?, f__whole(v_colours.clone(), Py::str("colours"), Py::Int(1i64), Py::Int(60i64))?])).items()?;
    if t4408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4408[0].clone();
    v_k = t4408[1].clone();
    v_N = f__necklace_count(v_n.clone(), v_k.clone())?;
    v_B = (if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::floordiv(&py::add(&v_N.clone(), &py::pow(&v_k.clone(), &py::floordiv(&py::add(&v_n.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?)?, &Py::Int(2i64))? } else { py::floordiv(&py::add(&v_N.clone(), &py::floordiv(&py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &py::pow(&v_k.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)?)?, &Py::Int(2i64))?)?, &Py::Int(2i64))? });
    return Ok(Py::dict(vec![(Py::str("necklaces"), v_N.clone()), (Py::str("bracelets"), v_B.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_next_prime(mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c_MAXI()?)?;
    v_p = py::add(&v_n.clone(), &Py::Int(1i64))?;
    while (py::not(&f__is_prime(v_p.clone())?)).truthy() {
        v_p = py::iadd(&v_p, &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("prime"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f_nth_prime(v_n: Py) -> R {
    ax_engine::pysym::helper("discrete_math:nth_prime", &[v_n], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partitions_distinct(mut v_n: Py) -> R {
    let mut v_part: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?;
    v_q = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
    for t4586 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_part = t4586;
        for t4587 in py::range_iter(&[(v_n.clone()).i()?, (py::sub(&v_part.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_s = t4587;
            let t4588 = v_q.clone(); let t4589 = v_s.clone();
            py::setitem(&t4588, &t4589, py::iadd(&py::getitem(&t4588, &t4589)?, &py::getitem(&v_q.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("partitions"), py::getitem(&v_q.clone(), &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partitions_k(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t4621 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(300i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(300i64))?])).items()?;
    if t4621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4621[0].clone();
    v_k = t4621[1].clone();
    v_t = { let mut out = vec![]; for t4622 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t4622;
out.push(py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?); } Py::list(out) };
    py::setitem(&py::getitem(&v_t.clone(), &Py::Int(0i64))?, &Py::Int(0i64), Py::Int(1i64))?;
    for t4623 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t4623;
        for t4624 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_i.clone(), v_k.clone()], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t4624;
            py::setitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone(), py::add(&py::getitem(&py::getitem(&v_t.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &(if ({ let t4625 = py::sub(&v_i.clone(), &v_j.clone())?; Py::Bool({ let t4626 = v_j.clone(); if py::ge(&t4625, &t4626)? { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_t.clone(), &py::sub(&v_i.clone(), &v_j.clone())?)?, &v_j.clone())? } else { Py::Int(0i64) }))?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("partitions"), (if ({ let t4627 = v_k.clone(); Py::Bool({ let t4628 = v_n.clone(); if py::le(&t4627, &t4628)? { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_t.clone(), &v_n.clone())?, &v_k.clone())? } else { Py::Int(0i64) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pell(mut v_d: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_terms: Py = Py::None;
    v_r = f_cf_sqrt(f__whole(v_d.clone(), Py::str("d"), Py::Int(2i64), Py::Int(1000i64))?)?;
    v_terms = py::add(&Py::list(vec![py::getitem(&v_r.clone(), &Py::str("a0"))?]), &py::slice(&py::getitem(&v_r.clone(), &Py::str("period"))?, None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?;
    let t4674 = (f__convergent(v_terms.clone())?).items()?;
    if t4674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4674[0].clone();
    v_q = t4674[1].clone();
    if (py::modulo(&py::getitem(&v_r.clone(), &Py::str("period_length"))?, &Py::Int(2i64))?).truthy() {
        let t4675 = (Py::tuple(vec![py::add(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&py::mul(&py::int(&v_d.clone())?, &v_q.clone())?, &v_q.clone())?)?, py::mul(&py::mul(&Py::Int(2i64), &v_p.clone())?, &v_q.clone())?])).items()?;
        if t4675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t4675[0].clone();
        v_q = t4675[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("x"), v_p.clone()), (Py::str("y"), v_q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_permutations(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4716 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c__NCAP()?)?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), c__NCAP()?)?])).items()?;
    if t4716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4716[0].clone();
    v_k = t4716[1].clone();
    if ({ let t4717 = v_k.clone(); Py::Bool({ let t4718 = v_n.clone(); if py::gt(&t4717, &t4718)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not exceed n"))));
    }
    v_p = Py::Int(1i64);
    for t4719 in py::range_iter(&[(py::add(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t4719;
        v_p = py::mul(&v_p, &v_i.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("permutations"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pigeonhole_max(mut v_items: Py, mut v_boxes: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4748 = (Py::tuple(vec![f__whole(v_items.clone(), Py::str("items"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_boxes.clone(), Py::str("boxes"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4748[0].clone();
    v_k = t4748[1].clone();
    return Ok(Py::dict(vec![(Py::str("guaranteed"), py::neg(&py::floordiv(&py::neg(&v_n.clone())?, &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pigeonhole_needed(mut v_boxes: Py, mut v_at_least: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t4778 = (Py::tuple(vec![f__whole(v_boxes.clone(), Py::str("boxes"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_at_least.clone(), Py::str("at_least"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4778.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t4778[0].clone();
    v_r = t4778[1].clone();
    return Ok(Py::dict(vec![(Py::str("needed"), py::add(&py::mul(&v_k.clone(), &py::sub(&v_r.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_previous_prime(mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(3i64), c_MAXI()?)?;
    v_p = py::sub(&v_n.clone(), &Py::Int(1i64))?;
    while (py::not(&f__is_prime(v_p.clone())?)).truthy() {
        v_p = py::sub(&v_p, &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("prime"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f_prime_counting(v_x: Py) -> R {
    ax_engine::pysym::helper("discrete_math:prime_counting", &[v_x], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_prime_factorization(mut v_n: Py) -> R {
    let mut v_f: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?;
    v_f = f__factor(v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("factorization"), { let mut out = vec![]; for t4928 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t4929 = (t4928).items()?;
if t4929.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4929[0].clone();
v_e = t4929[1].clone();
out.push(Py::list(vec![v_p.clone(), v_e.clone()])); } Py::list(out) }), (Py::str("distinct_primes"), py::len(&v_f.clone())?), (Py::str("prime_factors_with_multiplicity"), py::sum(&py::method(&v_f.clone(), "values", &[])?, None)?), (Py::str("largest_prime_factor"), py::minmax(&[v_f.clone()], true)?), (Py::str("smallest_prime_factor"), py::minmax(&[v_f.clone()], false)?), (Py::str("text"), py::method(&Py::str(" × "), "join", &[{ let mut out = vec![]; for t4930 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t4931 = (t4930).items()?;
if t4931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4931[0].clone();
v_e = t4931[1].clone();
out.push((if ({ let t4932 = v_e.clone(); Py::Bool({ let t4933 = Py::Int(1i64); if py::gt(&t4932, &t4933)? { true } else { false } }) }).truthy() { Py::str(&[py::format(&v_p.clone(), "")?, "^".to_string(), py::format(&v_e.clone(), "")?].concat()) } else { Py::str(&py::to_str(&v_p.clone())) })); } Py::list(out) }])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_primitive_root(mut v_p: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_qs: Py = Py::None;
    v_p = f__whole(v_p.clone(), Py::str("p"), Py::Int(2i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    if (py::not(&f__is_prime(v_p.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p must be prime"))));
    }
    if ({ let t5026 = v_p.clone(); Py::Bool({ let t5027 = Py::Int(2i64); if py::eq(&t5026, &t5027) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("root"), Py::Int(1i64))]));
    }
    v_qs = Py::list((f__factor(py::sub(&v_p.clone(), &Py::Int(1i64))?)?).items()?);
    v_g = Py::Int(2i64);
    while (Py::Bool(({ let mut out = vec![]; for t5044 in py::iter(&(v_qs.clone()))? { let mut v_q: Py; 
v_q = t5044;
out.push({ let t5045 = py::pow_mod(&v_g.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &v_q.clone())?, &v_p.clone())?; Py::Bool({ let t5046 = Py::Int(1i64); if py::eq(&t5045, &t5046) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        v_g = py::iadd(&v_g, &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("root"), v_g.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_prufer_decode(mut v_sequence: Py) -> R {
    let mut v_edges: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_seq: Py = Py::None;
    v_seq = f__wlist(v_sequence.clone(), Py::str("sequence"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(4i64))?, Py::Int(1i64), Py::Int(2000i64))?;
    v_n = py::add(&py::len(&v_seq.clone())?, &Py::Int(2i64))?;
    if ({ let t5105 = py::minmax(&[v_seq.clone()], true)?; Py::Bool({ let t5106 = v_n.clone(); if py::gt(&t5105, &t5106)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["every entry must be between 1 and n = ".to_string(), py::format(&v_n.clone(), "")?].concat()))));
    }
    v_edges = f__decode(v_seq.clone(), v_n.clone(), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("edges"), v_edges.clone()), (Py::str("vertices"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_prufer_encode(mut v_edges: Py, mut v_largest: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_leaf: Py = Py::None;
    let mut v_leaves: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let t5233 = (f__tree_labels(v_edges.clone())?).items()?;
    if t5233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nb = t5233[0].clone();
    v_off = t5233[1].clone();
    v_nb = { let mut out = vec![]; for t5234 in py::iter(&(py::method(&v_nb.clone(), "items", &[])?))? { let mut v_u: Py; let mut v_v: Py; 
let t5235 = (t5234).items()?;
if t5235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5235[0].clone();
v_v = t5235[1].clone();
out.push((v_u.clone(), py::set(&v_v.clone())?)); } Py::dict(out) };
    v_leaves = { let mut out = vec![]; for t5236 in py::iter(&(v_nb.clone()))? { let mut v_u: Py; 
v_u = t5236;
if ({ let t5237 = py::len(&py::getitem(&v_nb.clone(), &v_u.clone())?)?; Py::Bool({ let t5238 = Py::Int(1i64); if py::eq(&t5237, &t5238) { true } else { false } }) }).truthy() { out.push((if (v_largest.clone()).truthy() { py::neg(&v_u.clone())? } else { v_u.clone() })); }} Py::list(out) };
    let _ = py::heapq("heapify", &[v_leaves.clone()])?;
    v_seq = Py::list(vec![]);
    for t5239 in py::range_iter(&[(py::sub(&py::len(&v_nb.clone())?, &Py::Int(2i64))?).i()?]) {
        v__ = t5239;
        v_leaf = py::abs(&py::heapq("heappop", &[v_leaves.clone()])?)?;
        let t5240 = (py::getitem(&v_nb.clone(), &v_leaf.clone())?).items()?;
        if t5240.len() != 1 { return Err(PyErr::Value("unpack".into())); }
        v_p = t5240[0].clone();
        let _ = py::method(&v_seq.clone(), "append", &[v_p.clone()])?;
        let _ = py::method(&py::getitem(&v_nb.clone(), &v_p.clone())?, "discard", &[v_leaf.clone()])?;
        if ({ let t5241 = py::len(&py::getitem(&v_nb.clone(), &v_p.clone())?)?; Py::Bool({ let t5242 = Py::Int(1i64); if py::eq(&t5241, &t5242) { true } else { false } }) }).truthy() {
            let _ = py::heapq("heappush", &[v_leaves.clone(), (if (v_largest.clone()).truthy() { py::neg(&v_p.clone())? } else { v_p.clone() })])?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("sequence"), v_seq.clone()), (Py::str("vertices"), py::len(&v_nb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_decrypt(mut v_c: Py, mut v_d: Py, mut v_n: Py) -> R {
    let t5291 = (Py::tuple(vec![f__whole(v_c.clone(), Py::str("c"), Py::Int(0i64), c_MAXI()?)?, f__whole(v_d.clone(), Py::str("d"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5291.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5291[0].clone();
    v_d = t5291[1].clone();
    v_n = t5291[2].clone();
    if ({ let t5292 = v_c.clone(); Py::Bool({ let t5293 = v_n.clone(); if py::ge(&t5292, &t5293)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the ciphertext c must satisfy 0 <= c < n"))));
    }
    return Ok(Py::dict(vec![(Py::str("m"), py::pow_mod(&v_c.clone(), &v_d.clone(), &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_decrypt_pq(mut v_c: Py, mut v_p: Py, mut v_q: Py, mut v_e: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_mq: Py = Py::None;
    v_k = f_rsa_keygen(v_p.clone(), v_q.clone(), v_e.clone())?;
    let t5360 = (Py::tuple(vec![py::int(&v_p.clone())?, py::int(&v_q.clone())?, f__whole(v_c.clone(), Py::str("c"), Py::Int(0i64), c_MAXI()?)?])).items()?;
    if t5360.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5360[0].clone();
    v_q = t5360[1].clone();
    v_c = t5360[2].clone();
    if ({ let t5361 = v_c.clone(); Py::Bool({ let t5362 = py::getitem(&v_k.clone(), &Py::str("n"))?; if py::ge(&t5361, &t5362)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the ciphertext c must satisfy 0 <= c < n"))));
    }
    v_d = py::getitem(&v_k.clone(), &Py::str("d"))?;
    let t5363 = (Py::tuple(vec![py::pow_mod(&v_c.clone(), &py::modulo(&v_d.clone(), &py::sub(&v_p.clone(), &Py::Int(1i64))?)?, &v_p.clone())?, py::pow_mod(&v_c.clone(), &py::modulo(&v_d.clone(), &py::sub(&v_q.clone(), &Py::Int(1i64))?)?, &v_q.clone())?])).items()?;
    if t5363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mp = t5363[0].clone();
    v_mq = t5363[1].clone();
    v_h = py::modulo(&py::mul(&f__inv(v_q.clone(), v_p.clone())?, &py::sub(&v_mp.clone(), &v_mq.clone())?)?, &v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("m"), py::add(&v_mq.clone(), &py::mul(&v_h.clone(), &v_q.clone())?)?), (Py::str("d"), v_d.clone()), (Py::str("n"), py::getitem(&v_k.clone(), &Py::str("n"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_encrypt(mut v_m: Py, mut v_e: Py, mut v_n: Py) -> R {
    let t5402 = (Py::tuple(vec![f__whole(v_m.clone(), Py::str("m"), Py::Int(0i64), c_MAXI()?)?, f__whole(v_e.clone(), Py::str("e"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5402.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5402[0].clone();
    v_e = t5402[1].clone();
    v_n = t5402[2].clone();
    if ({ let t5403 = v_m.clone(); Py::Bool({ let t5404 = v_n.clone(); if py::ge(&t5403, &t5404)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message m must satisfy 0 <= m < n"))));
    }
    return Ok(Py::dict(vec![(Py::str("c"), py::pow_mod(&v_m.clone(), &v_e.clone(), &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_keygen(mut v_p: Py, mut v_q: Py, mut v_e: Py) -> R {
    let mut v_phi: Py = Py::None;
    let t5433 = (Py::tuple(vec![f__whole(v_p.clone(), Py::str("p"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_q.clone(), Py::str("q"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_e.clone(), Py::str("e"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5433.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5433[0].clone();
    v_q = t5433[1].clone();
    v_e = t5433[2].clone();
    let _ = f__need_prime(v_p.clone(), Py::str("p"))?;
    let _ = f__need_prime(v_q.clone(), Py::str("q"))?;
    if ({ let t5456 = v_p.clone(); Py::Bool({ let t5457 = v_q.clone(); if py::eq(&t5456, &t5457) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p and q must be different primes"))));
    }
    v_phi = py::mul(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::sub(&v_q.clone(), &Py::Int(1i64))?)?;
    if ({ let t5463 = py::not(&{ let t5458 = Py::Int(1i64); Py::Bool({ let t5459 = v_e.clone(); if py::lt(&t5458, &t5459)? { { let t5460 = v_phi.clone(); if py::lt(&t5459, &t5460)? { true } else { false } } } else { false } }) }); if t5463.truthy() { t5463 } else { { let t5461 = py::gcd(&v_e.clone(), &v_phi.clone())?; Py::Bool({ let t5462 = Py::Int(1i64); if py::ne(&t5461, &t5462) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("e must satisfy 1 < e < phi(n) and gcd(e, phi(n)) = 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("n"), py::mul(&v_p.clone(), &v_q.clone())?), (Py::str("phi"), v_phi.clone()), (Py::str("d"), f__inv(v_e.clone(), v_phi.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sequences_with_repetition(mut v_n: Py, mut v_k: Py) -> R {
    let t5509 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), py::pow(&Py::Int(10i64), &Py::Int(4i64))?)?])).items()?;
    if t5509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5509[0].clone();
    v_k = t5509[1].clone();
    if ({ let t5510 = py::mul(&v_k.clone(), &py::math1("log10", &v_n.clone())?)?; Py::Bool({ let t5511 = Py::Int(300i64); if py::gt(&t5510, &t5511)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result exceeds 10^300"))));
    }
    return Ok(Py::dict(vec![(Py::str("sequences"), py::pow(&v_n.clone(), &v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spanning_tree_count(mut v_edges: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    if ({ let t5544 = py::len(&v_adj.clone())?; Py::Bool({ let t5545 = Py::Int(60i64); if py::gt(&t5544, &t5545)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 vertices"))));
    }
    v_M = { let mut out = vec![]; for t5552 in py::iter(&(f__laplacian_minor(v_adj.clone())?))? { let mut v_row: Py; 
v_row = t5552;
out.push({ let mut out = vec![]; for t5553 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t5553;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) }); } Py::list(out) };
    let t5554 = (Py::tuple(vec![py::len(&v_M.clone())?, py::fraction(&[Py::Int(1i64)])?])).items()?;
    if t5554.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5554[0].clone();
    v_det = t5554[1].clone();
    for t5555 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t5555;
        v_p = (match ('g5556: { let mut out = vec![]; for t5557 in py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5557;
if ({ let t5558 = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t5559 = Py::Int(0i64); if py::ne(&t5558, &t5559) { true } else { false } }) }).truthy() { out.push(v_r.clone()); break 'g5556 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t5560 = v_p.clone(); Py::Bool({ let t5561 = Py::None; if py::is(&t5560, &t5561) { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("spanning_trees"), Py::Int(0i64))]));
        }
        if ({ let t5562 = v_p.clone(); Py::Bool({ let t5563 = v_c.clone(); if py::ne(&t5562, &t5563) { true } else { false } }) }).truthy() {
            let t5564 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
            if t5564.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_M.clone(), &v_c.clone(), t5564[0].clone())?;
            py::setitem(&v_M.clone(), &v_p.clone(), t5564[1].clone())?;
            v_det = py::neg(&v_det.clone())?;
        }
        v_det = py::mul(&v_det, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
        for t5565 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t5565;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if (v_f.clone()).truthy() {
                py::setitem(&v_M.clone(), &v_r.clone(), { let mut out = vec![]; for t5566 in py::iter(&(py::zip(&[py::getitem(&v_M.clone(), &v_r.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5567 = (t5566).items()?;
if t5567.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5567[0].clone();
v_b = t5567[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("spanning_trees"), py::int(&v_det.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sqrt_mod(mut v_a: Py, mut v_p: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let t5612 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_p.clone(), Py::str("p"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5612[0].clone();
    v_p = t5612[1].clone();
    if (py::not(&f__is_prime(v_p.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p must be prime"))));
    }
    v_a = py::modulo(&v_a, &v_p.clone())?;
    if ({ let t5635 = v_a.clone(); Py::Bool({ let t5636 = Py::Int(0i64); if py::eq(&t5635, &t5636) { true } else { false } }) }).truthy() {
        v_roots = Py::list(vec![Py::Int(0i64)]);
    } else {
        if ({ let t5637 = v_p.clone(); Py::Bool({ let t5638 = Py::Int(2i64); if py::eq(&t5637, &t5638) { true } else { false } }) }).truthy() {
            v_roots = Py::list(vec![Py::Int(1i64)]);
        } else {
            if ({ let t5639 = py::pow_mod(&v_a.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?; Py::Bool({ let t5640 = Py::Int(1i64); if py::ne(&t5639, &t5640) { true } else { false } }) }).truthy() {
                v_roots = Py::list(vec![]);
            } else {
                v_r = f__tonelli(v_a.clone(), v_p.clone())?;
                v_roots = py::sorted(&py::set(&Py::list(vec![v_r.clone(), py::sub(&v_p.clone(), &v_r.clone())?]))?, (Py::Bool(false)).truthy())?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("roots"), v_roots.clone()), (Py::str("count"), py::len(&v_roots.clone())?), (Py::str("smallest"), (if (v_roots.clone()).truthy() { py::getitem(&v_roots.clone(), &Py::Int(0i64))? } else { Py::None }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stars_and_bars(mut v_total: Py, mut v_variables: Py, mut v_lower: Py, mut v_upper: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rest: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t5707 = (Py::tuple(vec![f__whole(v_total.clone(), Py::str("total"), Py::Int(0i64), Py::Int(2000i64))?, f__whole(v_variables.clone(), Py::str("variables"), Py::Int(1i64), Py::Int(50i64))?])).items()?;
    if t5707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5707[0].clone();
    v_k = t5707[1].clone();
    v_lo = f__whole(v_lower.clone(), Py::str("lower"), Py::Int(0i64), Py::Int(2000i64))?;
    v_rest = py::sub(&v_n.clone(), &py::mul(&v_k.clone(), &v_lo.clone())?)?;
    if ({ let t5708 = v_upper.clone(); Py::Bool({ let t5709 = Py::None; if !py::is(&t5708, &t5709) { true } else { false } }) }).truthy() {
        v_hi = f__whole(v_upper.clone(), Py::str("upper"), Py::Int(0i64), Py::Int(2000i64))?;
        if ({ let t5710 = v_hi.clone(); Py::Bool({ let t5711 = v_lo.clone(); if py::lt(&t5710, &t5711)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the upper bound must be at least the lower bound"))));
        }
    }
    if ({ let t5712 = v_rest.clone(); Py::Bool({ let t5713 = Py::Int(0i64); if py::lt(&t5712, &t5713)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("solutions"), Py::Int(0i64))]));
    }
    if ({ let t5714 = v_upper.clone(); Py::Bool({ let t5715 = Py::None; if py::is(&t5714, &t5715) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("solutions"), py::comb(&py::sub(&py::add(&v_rest.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)]));
    }
    v_c = py::add(&py::sub(&v_hi.clone(), &v_lo.clone())?, &Py::Int(1i64))?;
    v_s = Py::Int(0i64);
    for t5716 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t5716;
        v_m = py::sub(&v_rest.clone(), &py::mul(&v_j.clone(), &v_c.clone())?)?;
        if ({ let t5717 = v_m.clone(); Py::Bool({ let t5718 = Py::Int(0i64); if py::lt(&t5717, &t5718)? { true } else { false } }) }).truthy() {
            break;
        }
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_j.clone())?, &py::comb(&v_k.clone(), &v_j.clone())?)?, &py::comb(&py::sub(&py::add(&v_m.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("solutions"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stirling1(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_row: Py = Py::None;
    let t5756 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(100i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(100i64))?])).items()?;
    if t5756.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5756[0].clone();
    v_k = t5756[1].clone();
    if ({ let t5757 = v_k.clone(); Py::Bool({ let t5758 = v_n.clone(); if py::gt(&t5757, &t5758)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("stirling1"), Py::Int(0i64)), (Py::str("signed"), Py::Int(0i64))]));
    }
    v_row = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_k.clone())?)?;
    for t5759 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t5759;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
        for t5760 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_i.clone(), v_k.clone()], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t5760;
            py::setitem(&v_new.clone(), &v_j.clone(), py::add(&py::mul(&py::sub(&v_i.clone(), &Py::Int(1i64))?, &py::getitem(&v_row.clone(), &v_j.clone())?)?, &py::getitem(&v_row.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)?)?;
        }
        v_row = v_new.clone();
    }
    return Ok(Py::dict(vec![(Py::str("stirling1"), py::getitem(&v_row.clone(), &v_k.clone())?), (Py::str("signed"), py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::sub(&v_n.clone(), &v_k.clone())?)?, &py::getitem(&v_row.clone(), &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stirling2(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_row: Py = Py::None;
    let t5801 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(100i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(100i64))?])).items()?;
    if t5801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5801[0].clone();
    v_k = t5801[1].clone();
    if ({ let t5802 = v_k.clone(); Py::Bool({ let t5803 = v_n.clone(); if py::gt(&t5802, &t5803)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("stirling2"), Py::Int(0i64)), (Py::str("surjections"), Py::Int(0i64))]));
    }
    v_row = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_k.clone())?)?;
    for t5804 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t5804;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
        for t5805 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_i.clone(), v_k.clone()], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t5805;
            py::setitem(&v_new.clone(), &v_j.clone(), py::add(&py::mul(&v_j.clone(), &py::getitem(&v_row.clone(), &v_j.clone())?)?, &py::getitem(&v_row.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)?)?;
        }
        v_row = v_new.clone();
    }
    return Ok(Py::dict(vec![(Py::str("stirling2"), py::getitem(&v_row.clone(), &v_k.clone())?), (Py::str("surjections"), py::mul(&py::factorial(&v_k.clone())?, &py::getitem(&v_row.clone(), &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_strongly_connected(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_comps: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_low: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_visit: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = { let _ = py::minmax(&[Py::Int(1000), py::add(&py::mul(&Py::Int(4i64), &py::len(&v_adj.clone())?)?, &Py::Int(100i64))?], true)?; Py::None };
    let t5842 = (Py::tuple(vec![Py::dict(vec![]), Py::dict(vec![]), py::set(&Py::list(vec![]))?, Py::list(vec![]), Py::list(vec![]), Py::list(vec![Py::Int(0i64)])])).items()?;
    if t5842.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_idx = t5842[0].clone();
    v_low = t5842[1].clone();
    v_on = t5842[2].clone();
    v_stack = t5842[3].clone();
    v_comps = t5842[4].clone();
    v_t = t5842[5].clone();
    let rec5843 = py::rec_cell();
    v_visit = { let v_adj = v_adj.clone(); let v_comps = v_comps.clone(); let v_edges = v_edges.clone(); let v_idx = v_idx.clone(); let v_low = v_low.clone(); let v_on = v_on.clone(); let v_stack = v_stack.clone(); let v_t = v_t.clone(); let rec5843 = rec5843.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_visit: Py = py::rec_get(&rec5843)?;
            let mut v_comp: Py = Py::None;
            let mut v_v: Py = Py::None;
            let mut v_w: Py = Py::None;
            let t5844 = py::getitem(&v_t.clone(), &Py::Int(0i64))?;
            py::setitem(&v_idx.clone(), &v_u.clone(), t5844.clone())?;
            py::setitem(&v_low.clone(), &v_u.clone(), t5844.clone())?;
            let t5845 = v_t.clone(); let t5846 = Py::Int(0i64);
            py::setitem(&t5845, &t5846, py::iadd(&py::getitem(&t5845, &t5846)?, &Py::Int(1i64))?)?;
            let _ = py::method(&v_stack.clone(), "append", &[v_u.clone()])?;
            let _ = py::method(&v_on.clone(), "add", &[v_u.clone()])?;
            for t5847 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &v_u.clone())?)?))? {
                v_v = t5847;
                if ({ let t5848 = v_v.clone(); Py::Bool({ let t5849 = v_idx.clone(); if !py::contains(&t5849, &t5848)? { true } else { false } }) }).truthy() {
                    let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
                    py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_low.clone(), &v_v.clone())?], false)?)?;
                } else {
                    if ({ let t5850 = v_v.clone(); Py::Bool({ let t5851 = v_on.clone(); if py::contains(&t5851, &t5850)? { true } else { false } }) }).truthy() {
                        py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_idx.clone(), &v_v.clone())?], false)?)?;
                    }
                }
            }
            if ({ let t5852 = py::getitem(&v_low.clone(), &v_u.clone())?; Py::Bool({ let t5853 = py::getitem(&v_idx.clone(), &v_u.clone())?; if py::eq(&t5852, &t5853) { true } else { false } }) }).truthy() {
                v_comp = Py::list(vec![]);
                while (Py::Bool(true)).truthy() {
                    v_w = py::method(&v_stack.clone(), "pop", &[])?;
                    let _ = py::method(&v_on.clone(), "discard", &[v_w.clone()])?;
                    let _ = py::method(&v_comp.clone(), "append", &[v_w.clone()])?;
                    if ({ let t5854 = v_w.clone(); Py::Bool({ let t5855 = v_u.clone(); if py::eq(&t5854, &t5855) { true } else { false } }) }).truthy() {
                        break;
                    }
                }
                let _ = py::method(&v_comps.clone(), "append", &[f__sorted(v_comp.clone())?])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec5843, &v_visit);
    for t5856 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_v = t5856;
        if ({ let t5857 = v_v.clone(); Py::Bool({ let t5858 = v_idx.clone(); if !py::contains(&t5858, &t5857)? { true } else { false } }) }).truthy() {
            let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
        }
    }
    let _ = py::method_sort(&v_comps.clone(), Some(&{ let v_adj = v_adj.clone(); let v_comps = v_comps.clone(); let v_edges = v_edges.clone(); let v_idx = v_idx.clone(); let v_low = v_low.clone(); let v_on = v_on.clone(); let v_stack = v_stack.clone(); let v_t = v_t.clone(); let v_v = v_v.clone(); let v_visit = v_visit.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__nkey(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), false)?;
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_comps.clone())?), (Py::str("sizes"), py::sorted(&{ let mut out = vec![]; for t5859 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t5859;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?), (Py::str("components"), v_comps.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_topological_sort(mut v_edges: Py) -> R {
    let mut v__k: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_heap: Py = Py::None;
    let mut v_indeg: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(true), Py::Bool(false))?;
    v_indeg = { let mut out = vec![]; for t5908 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5908;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    for t5909 in py::iter(&(v_adj.clone()))? {
        v_u = t5909;
        for t5910 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5910;
            let t5911 = v_indeg.clone(); let t5912 = v_v.clone();
            py::setitem(&t5911, &t5912, py::iadd(&py::getitem(&t5911, &t5912)?, &Py::Int(1i64))?)?;
        }
    }
    v_heap = { let mut out = vec![]; for t5913 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5913;
if ({ let t5914 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t5915 = Py::Int(0i64); if py::eq(&t5914, &t5915) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![f__nkey(v_v.clone())?, v_v.clone()])); }} Py::list(out) };
    let _ = py::heapq("heapify", &[v_heap.clone()])?;
    v_order = Py::list(vec![]);
    while (v_heap.clone()).truthy() {
        let t5916 = (py::heapq("heappop", &[v_heap.clone()])?).items()?;
        if t5916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v__k = t5916[0].clone();
        v_u = t5916[1].clone();
        let _ = py::method(&v_order.clone(), "append", &[v_u.clone()])?;
        for t5917 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5917;
            let t5918 = v_indeg.clone(); let t5919 = v_v.clone();
            py::setitem(&t5918, &t5919, py::sub(&py::getitem(&t5918, &t5919)?, &Py::Int(1i64))?)?;
            if ({ let t5920 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t5921 = Py::Int(0i64); if py::eq(&t5920, &t5921) { true } else { false } }) }).truthy() {
                let _ = py::heapq("heappush", &[v_heap.clone(), Py::tuple(vec![f__nkey(v_v.clone())?, v_v.clone()])])?;
            }
        }
    }
    if ({ let t5922 = py::len(&v_order.clone())?; Py::Bool({ let t5923 = py::len(&v_adj.clone())?; if py::ne(&t5922, &t5923) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph has a directed cycle, so no topological order exists"))));
    }
    return Ok(Py::dict(vec![(Py::str("order"), v_order.clone()), (Py::str("vertices"), py::len(&v_order.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_truth_table(mut v_formula: Py) -> R {
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t6022 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t6022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t6022[0].clone();
    v_names = t6022[1].clone();
    return Ok(Py::dict(vec![(Py::str("variables"), v_names.clone()), (Py::str("values"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6023 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t6023;
out.push((if (v_v.clone()).truthy() { Py::str("1") } else { Py::str("0") })); } Py::list(out) }])?), (Py::str("true_rows"), py::sum(&v_vals.clone(), None)?), (Py::str("minterms"), { let mut out = vec![]; for t6024 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t6025 = (t6024).items()?;
if t6025.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t6025[0].clone();
v_v = t6025[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); }} Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_squares(mut v_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d3: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_reps: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_y2: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?;
    v_reps = Py::list(vec![]);
    v_x = Py::Int(0i64);
    while ({ let t6068 = py::mul(&py::mul(&Py::Int(2i64), &v_x.clone())?, &v_x.clone())?; Py::Bool({ let t6069 = v_n.clone(); if py::le(&t6068, &t6069)? { true } else { false } }) }).truthy() {
        v_y2 = py::sub(&v_n.clone(), &py::mul(&v_x.clone(), &v_x.clone())?)?;
        v_y = py::isqrt(&v_y2.clone())?;
        if ({ let t6066 = py::mul(&v_y.clone(), &v_y.clone())?; Py::Bool({ let t6067 = v_y2.clone(); if py::eq(&t6066, &t6067) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_reps.clone(), "append", &[Py::list(vec![v_x.clone(), v_y.clone()])])?;
        }
        v_x = py::iadd(&v_x, &Py::Int(1i64))?;
    }
    if ({ let t6070 = v_n.clone(); Py::Bool({ let t6071 = Py::Int(0i64); if py::eq(&t6070, &t6071) { true } else { false } }) }).truthy() {
        v_r2 = Py::Int(1i64);
    } else {
        let t6072 = Py::Int(0i64);
        v_d1 = t6072.clone();
        v_d3 = t6072.clone();
        for t6119 in py::iter(&(f__divisors_from({ let mut out = vec![]; for t6115 in py::iter(&(py::method(&f__factor(v_n.clone())?, "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t6116 = (t6115).items()?;
if t6116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t6116[0].clone();
v_e = t6116[1].clone();
if ({ let t6117 = v_p.clone(); Py::Bool({ let t6118 = Py::Int(2i64); if py::ne(&t6117, &t6118) { true } else { false } }) }).truthy() { out.push((v_p.clone(), v_e.clone())); }} Py::dict(out) })?))? {
            v_d = t6119;
            if ({ let t6120 = py::modulo(&v_d.clone(), &Py::Int(4i64))?; Py::Bool({ let t6121 = Py::Int(1i64); if py::eq(&t6120, &t6121) { true } else { false } }) }).truthy() {
                v_d1 = py::iadd(&v_d1, &Py::Int(1i64))?;
            } else {
                v_d3 = py::iadd(&v_d3, &Py::Int(1i64))?;
            }
        }
        v_r2 = py::mul(&Py::Int(4i64), &py::sub(&v_d1.clone(), &v_d3.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("representations"), v_reps.clone()), (Py::str("count"), py::len(&v_reps.clone())?), (Py::str("r2"), v_r2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_twos_complement(mut v_n: Py, mut v_bits: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t6179 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::None, c_MAXI()?)?, f__whole(v_bits.clone(), Py::str("bits"), Py::Int(2i64), Py::Int(64i64))?])).items()?;
    if t6179.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6179[0].clone();
    v_b = t6179[1].clone();
    if (py::not(&{ let t6180 = py::neg(&py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t6181 = v_n.clone(); if py::le(&t6180, &t6181)? { { let t6182 = py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?; if py::lt(&t6181, &t6182)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_n.clone(), "")?, " does not fit in ".to_string(), py::format(&v_b.clone(), "")?, "-bit two's complement (range ".to_string(), py::format(&py::neg(&py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)?, "")?, " to ".to_string(), py::format(&py::sub(&py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?, "")?, ")".to_string()].concat()))));
    }
    if ({ let t6183 = v_n.clone(); Py::Bool({ let t6184 = Py::Int(0i64); if py::ge(&t6183, &t6184)? { true } else { false } }) }).truthy() {
        v_s = py::method(&Py::str(&py::format(&v_n.clone(), &(Py::str("b")).as_str()?)?), "zfill", &[v_b.clone()])?;
    } else {
        v_mag = py::method(&Py::str(&py::format(&py::neg(&v_n.clone())?, &(Py::str("b")).as_str()?)?), "zfill", &[v_b.clone()])?;
        v_inv = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6185 in py::iter(&(v_mag.clone()))? { let mut v_c: Py; 
v_c = t6185;
out.push((if ({ let t6186 = v_c.clone(); Py::Bool({ let t6187 = Py::str("0"); if py::eq(&t6186, &t6187) { true } else { false } }) }).truthy() { Py::str("1") } else { Py::str("0") })); } Py::list(out) }])?;
        v_s = py::slice(&py::method(&Py::str(&py::format(&py::add(&py::int_base(&v_inv.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?, &(Py::str("b")).as_str()?)?), "zfill", &[v_b.clone()])?, Some((py::neg(&v_b.clone())?).i()?), None, None)?;
    }
    return Ok(Py::dict(vec![(Py::str("bits_text"), v_s.clone()), (Py::str("unsigned"), py::int_base(&v_s.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_venn_three_sets(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_ab: Py, mut v_ac: Py, mut v_bc: Py, mut v_abc: Py, mut v_universe: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_r_a: Py = Py::None;
    let mut v_r_ab: Py = Py::None;
    let mut v_r_ac: Py = Py::None;
    let mut v_r_b: Py = Py::None;
    let mut v_r_bc: Py = Py::None;
    let mut v_r_c: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_union: Py = Py::None;
    let t6226 = (Py::tuple(vec![f__venn_whole(v_a.clone(), Py::str("n(A)"))?, f__venn_whole(v_b.clone(), Py::str("n(B)"))?, f__venn_whole(v_c.clone(), Py::str("n(C)"))?])).items()?;
    if t6226.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6226[0].clone();
    v_b = t6226[1].clone();
    v_c = t6226[2].clone();
    let t6227 = (Py::tuple(vec![f__venn_whole(v_ab.clone(), Py::str("n(A and B)"))?, f__venn_whole(v_ac.clone(), Py::str("n(A and C)"))?, f__venn_whole(v_bc.clone(), Py::str("n(B and C)"))?])).items()?;
    if t6227.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ab = t6227[0].clone();
    v_ac = t6227[1].clone();
    v_bc = t6227[2].clone();
    v_abc = f__venn_whole(v_abc.clone(), Py::str("n(A and B and C)"))?;
    let t6228 = (Py::tuple(vec![py::sub(&v_ab.clone(), &v_abc.clone())?, py::sub(&v_ac.clone(), &v_abc.clone())?, py::sub(&v_bc.clone(), &v_abc.clone())?])).items()?;
    if t6228.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r_ab = t6228[0].clone();
    v_r_ac = t6228[1].clone();
    v_r_bc = t6228[2].clone();
    let t6229 = (Py::tuple(vec![py::add(&py::sub(&py::sub(&v_a.clone(), &v_ab.clone())?, &v_ac.clone())?, &v_abc.clone())?, py::add(&py::sub(&py::sub(&v_b.clone(), &v_ab.clone())?, &v_bc.clone())?, &v_abc.clone())?, py::add(&py::sub(&py::sub(&v_c.clone(), &v_ac.clone())?, &v_bc.clone())?, &v_abc.clone())?])).items()?;
    if t6229.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r_a = t6229[0].clone();
    v_r_b = t6229[1].clone();
    v_r_c = t6229[2].clone();
    if ({ let t6230 = py::minmax(&[v_r_a.clone(), v_r_b.clone(), v_r_c.clone(), v_r_ab.clone(), v_r_ac.clone(), v_r_bc.clone()], false)?; Py::Bool({ let t6231 = Py::Int(0i64); if py::lt(&t6230, &t6231)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the counts are inconsistent: a region of the diagram would have a negative count"))));
    }
    v_union = py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&v_r_a.clone(), &v_r_b.clone())?, &v_r_c.clone())?, &v_r_ab.clone())?, &v_r_ac.clone())?, &v_r_bc.clone())?, &v_abc.clone())?;
    v_out = Py::dict(vec![(Py::str("union"), v_union.clone()), (Py::str("only_a"), v_r_a.clone()), (Py::str("only_b"), v_r_b.clone()), (Py::str("only_c"), v_r_c.clone()), (Py::str("only_a_and_b"), v_r_ab.clone()), (Py::str("only_a_and_c"), v_r_ac.clone()), (Py::str("only_b_and_c"), v_r_bc.clone()), (Py::str("exactly_one"), py::add(&py::add(&v_r_a.clone(), &v_r_b.clone())?, &v_r_c.clone())?), (Py::str("exactly_two"), py::add(&py::add(&v_r_ab.clone(), &v_r_ac.clone())?, &v_r_bc.clone())?), (Py::str("none"), Py::None)]);
    if ({ let t6232 = v_universe.clone(); Py::Bool({ let t6233 = Py::None; if !py::is(&t6232, &t6233) { true } else { false } }) }).truthy() {
        v_u = f__venn_whole(v_universe.clone(), Py::str("n(U)"))?;
        if ({ let t6234 = v_u.clone(); Py::Bool({ let t6235 = v_union.clone(); if py::lt(&t6234, &t6235)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n(U) is smaller than n(A or B or C): the counts are inconsistent"))));
        }
        py::setitem(&v_out.clone(), &Py::str("none"), py::sub(&v_u.clone(), &v_union.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_venn_two_sets(mut v_a: Py, mut v_b: Py, mut v_both: Py, mut v_universe: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_union: Py = Py::None;
    let t6300 = (Py::tuple(vec![f__venn_whole(v_a.clone(), Py::str("n(A)"))?, f__venn_whole(v_b.clone(), Py::str("n(B)"))?, f__venn_whole(v_both.clone(), Py::str("n(A and B)"))?])).items()?;
    if t6300.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6300[0].clone();
    v_b = t6300[1].clone();
    v_both = t6300[2].clone();
    if ({ let t6305 = { let t6301 = v_both.clone(); Py::Bool({ let t6302 = v_a.clone(); if py::gt(&t6301, &t6302)? { true } else { false } }) }; if t6305.truthy() { t6305 } else { { let t6303 = v_both.clone(); Py::Bool({ let t6304 = v_b.clone(); if py::gt(&t6303, &t6304)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n(A and B) cannot exceed n(A) or n(B)"))));
    }
    v_union = py::sub(&py::add(&v_a.clone(), &v_b.clone())?, &v_both.clone())?;
    v_out = Py::dict(vec![(Py::str("union"), v_union.clone()), (Py::str("only_a"), py::sub(&v_a.clone(), &v_both.clone())?), (Py::str("only_b"), py::sub(&v_b.clone(), &v_both.clone())?), (Py::str("neither"), Py::None)]);
    if ({ let t6306 = v_universe.clone(); Py::Bool({ let t6307 = Py::None; if !py::is(&t6306, &t6307) { true } else { false } }) }).truthy() {
        v_u = f__venn_whole(v_universe.clone(), Py::str("n(U)"))?;
        if ({ let t6308 = v_u.clone(); Py::Bool({ let t6309 = v_union.clone(); if py::lt(&t6308, &t6309)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n(U) is smaller than n(A or B): the counts are inconsistent"))));
        }
        py::setitem(&v_out.clone(), &Py::str("neither"), py::sub(&v_u.clone(), &v_union.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vigenere(mut v_message: Py, mut v_key: Py, mut v_mode: Py) -> R {
    let mut v_ks: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_ks = f__letters(v_key.clone())?;
    if ({ let t6349 = py::not(&v_ks.clone()); if t6349.truthy() { t6349 } else { { let t6347 = py::len(&v_ks.clone())?; Py::Bool({ let t6348 = py::len(&py::method(&Py::str(&py::to_str(&v_key.clone())), "strip", &[])?)?; if py::ne(&t6347, &t6348) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the key must be letters A-Z only"))));
    }
    v_xs = f__letters(v_message.clone())?;
    if (py::not(&v_xs.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message has no letters"))));
    }
    v_sign = (if ({ let t6350 = v_mode.clone(); Py::Bool({ let t6351 = Py::str("encrypt"); if py::eq(&t6350, &t6351) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    return Ok(Py::dict(vec![(Py::str("result"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6352 in py::iter(&(py::enumerate(&v_xs.clone(), 0)?))? { let mut v_j: Py; let mut v_x: Py; 
let t6353 = (t6352).items()?;
if t6353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t6353[0].clone();
v_x = t6353[1].clone();
out.push(py::chr(&py::add(&Py::Int(65i64), &py::modulo(&py::add(&v_x.clone(), &py::mul(&v_sign.clone(), &py::getitem(&v_ks.clone(), &py::modulo(&v_j.clone(), &py::len(&v_ks.clone())?)?)?)?)?, &Py::Int(26i64))?)?)?); } Py::list(out) }])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_voting_methods(mut v_ballots: Py) -> R {
    let mut v_all_won: Py = Py::None;
    let mut v_beats: Py = Py::None;
    let mut v_borda: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_cnt: Py = Py::None;
    let mut v_condorcet: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_groups: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_points: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_total: Py = Py::None;
    let t6404 = (f__vt_ballots(v_ballots.clone())?).items()?;
    if t6404.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_groups = t6404[0].clone();
    v_cands = t6404[1].clone();
    v_n = py::len(&v_cands.clone())?;
    v_pos = Py::dict(vec![]);
    for t6405 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t6405;
        py::setitem(&v_pos.clone(), &py::getitem(&v_cands.clone(), &v_k.clone())?, v_k.clone())?;
    }
    v_total = Py::Int(0i64);
    let t6406 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t6406.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_first = t6406[0].clone();
    v_borda = t6406[1].clone();
    v_beats = t6406[2].clone();
    for t6407 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t6407;
        let _ = py::method(&v_first.clone(), "append", &[Py::Int(0i64)])?;
        let _ = py::method(&v_borda.clone(), "append", &[Py::Int(0i64)])?;
        let _ = py::method(&v_beats.clone(), "append", &[py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?])?;
    }
    for t6408 in py::iter(&(v_groups.clone()))? {
        v_g = t6408;
        let t6409 = (Py::tuple(vec![py::getitem(&v_g.clone(), &Py::Int(0i64))?, py::getitem(&v_g.clone(), &Py::Int(1i64))?])).items()?;
        if t6409.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cnt = t6409[0].clone();
        v_order = t6409[1].clone();
        v_total = py::iadd(&v_total, &v_cnt.clone())?;
        let t6410 = v_first.clone(); let t6411 = py::getitem(&v_pos.clone(), &py::getitem(&v_order.clone(), &Py::Int(0i64))?)?;
        py::setitem(&t6410, &t6411, py::iadd(&py::getitem(&t6410, &t6411)?, &v_cnt.clone())?)?;
        for t6412 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_r = t6412;
            let t6413 = v_borda.clone(); let t6414 = py::getitem(&v_pos.clone(), &py::getitem(&v_order.clone(), &v_r.clone())?)?;
            py::setitem(&t6413, &t6414, py::iadd(&py::getitem(&t6413, &t6414)?, &py::mul(&v_cnt.clone(), &py::sub(&v_n.clone(), &v_r.clone())?)?)?)?;
            for t6415 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_s = t6415;
                let t6416 = py::getitem(&v_beats.clone(), &py::getitem(&v_pos.clone(), &py::getitem(&v_order.clone(), &v_r.clone())?)?)?; let t6417 = py::getitem(&v_pos.clone(), &py::getitem(&v_order.clone(), &v_s.clone())?)?;
                py::setitem(&t6416, &t6417, py::iadd(&py::getitem(&t6416, &t6417)?, &v_cnt.clone())?)?;
            }
        }
    }
    v_points = Py::list(vec![]);
    v_condorcet = Py::str("none");
    for t6418 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t6418;
        v_p = Py::Float(0.0);
        v_all_won = Py::Bool(true);
        for t6419 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t6419;
            if ({ let t6420 = v_j.clone(); Py::Bool({ let t6421 = v_i.clone(); if py::eq(&t6420, &t6421) { true } else { false } }) }).truthy() {
                continue;
            }
            if ({ let t6422 = py::getitem(&py::getitem(&v_beats.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t6423 = py::getitem(&py::getitem(&v_beats.clone(), &v_j.clone())?, &v_i.clone())?; if py::gt(&t6422, &t6423)? { true } else { false } }) }).truthy() {
                v_p = py::iadd(&v_p, &Py::Float(1.0))?;
            } else {
                v_all_won = Py::Bool(false);
                if ({ let t6424 = py::getitem(&py::getitem(&v_beats.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t6425 = py::getitem(&py::getitem(&v_beats.clone(), &v_j.clone())?, &v_i.clone())?; if py::eq(&t6424, &t6425) { true } else { false } }) }).truthy() {
                    v_p = py::iadd(&v_p, &Py::Float(0.5))?;
                }
            }
        }
        let _ = py::method(&v_points.clone(), "append", &[v_p.clone()])?;
        if (v_all_won.clone()).truthy() {
            v_condorcet = py::getitem(&v_cands.clone(), &v_i.clone())?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("plurality_winner"), f__vt_winner(v_cands.clone(), v_first.clone())?), (Py::str("borda_winner"), f__vt_winner(v_cands.clone(), v_borda.clone())?), (Py::str("runoff_winner"), f__vt_runoff(v_groups.clone(), v_cands.clone(), v_total.clone())?), (Py::str("pairwise_winner"), f__vt_winner(v_cands.clone(), v_points.clone())?), (Py::str("condorcet_winner"), v_condorcet.clone()), (Py::str("candidates"), py::method(&Py::str(", "), "join", &[v_cands.clone()])?), (Py::str("voters"), v_total.clone()), (Py::str("first_place_votes"), v_first.clone()), (Py::str("borda_points"), v_borda.clone()), (Py::str("pairwise_points"), v_points.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_walks_count(mut v_edges: Py, mut v_source: Py, mut v_target: Py, mut v_length: Py, mut v_directed: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vec: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool((v_directed.clone()).truthy()), Py::Bool(false))?;
    let t6545 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?, f__whole(v_length.clone(), Py::str("length"), Py::Int(0i64), Py::Int(200i64))?])).items()?;
    if t6545.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t6545[0].clone();
    v_t = t6545[1].clone();
    v_k = t6545[2].clone();
    for t6546 in py::iter(&(Py::tuple(vec![v_s.clone(), v_t.clone()])))? {
        v_v = t6546;
        if ({ let t6547 = v_v.clone(); Py::Bool({ let t6548 = v_adj.clone(); if !py::contains(&t6548, &t6547)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_v.clone(), "")?, " is not a vertex of the graph".to_string()].concat()))));
        }
    }
    v_vec = Py::dict(vec![(v_s.clone(), Py::Int(1i64))]);
    for t6549 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t6549;
        v_new = Py::dict(vec![]);
        for t6550 in py::iter(&(py::method(&v_vec.clone(), "items", &[])?))? {
            let t6552 = (t6550).items()?;
            if t6552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_u = t6552[0].clone();
            v_c = t6552[1].clone();
            for t6551 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t6551;
                py::setitem(&v_new.clone(), &v_v.clone(), py::add(&py::method(&v_new.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?, &v_c.clone())?)?;
            }
        }
        v_vec = v_new.clone();
    }
    return Ok(Py::dict(vec![(Py::str("walks"), py::method(&v_vec.clone(), "get", &[v_t.clone(), Py::Int(0i64)])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_word_arrangements(mut v_word: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_used: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = py::method(&py::method(&Py::str(&py::to_str(&v_word.clone())), "strip", &[])?, "upper", &[])?;
    if ({ let t6571 = py::not(&v_w.clone()); if t6571.truthy() { t6571 } else { { let t6570 = py::not(&py::method(&v_w.clone(), "isalpha", &[])?); if t6570.truthy() { t6570 } else { py::not(&py::method(&v_w.clone(), "isascii", &[])?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the word must consist of letters A-Z only"))));
    }
    if ({ let t6572 = py::len(&v_w.clone())?; Py::Bool({ let t6573 = c__NCAP()?; if py::gt(&t6572, &t6573)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&c__NCAP()?, "")?, " letters".to_string()].concat()))));
    }
    v_counts = Py::dict(vec![]);
    for t6574 in py::iter(&(v_w.clone()))? {
        v_ch = t6574;
        py::setitem(&v_counts.clone(), &v_ch.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_ch.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
    }
    let t6575 = (Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)])).items()?;
    if t6575.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t6575[0].clone();
    v_used = t6575[1].clone();
    for t6576 in py::iter(&(py::method(&v_counts.clone(), "values", &[])?))? {
        v_c = t6576;
        v_used = py::iadd(&v_used, &v_c.clone())?;
        v_r = py::mul(&v_r, &py::comb(&v_used.clone(), &v_c.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("arrangements"), v_r.clone()), (Py::str("length"), py::len(&v_w.clone())?), (Py::str("letter_counts"), py::dict_from(&py::sorted(&py::method(&v_counts.clone(), "items", &[])?, (Py::Bool(false)).truthy())?, vec![])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1061(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_clique(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1083 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1084 = v_r.clone(); Py::Bool({ let t1085 = Py::Bool(true); if py::is(&t1084, &t1085) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1090 = { let t1086 = v_r.clone(); Py::Bool({ let t1087 = Py::Bool(false); if py::is(&t1086, &t1087) { true } else { false } }) }; if t1090.truthy() { t1090 } else { { let t1088 = v_r.clone(); Py::Bool({ let t1089 = Py::None; if py::is(&t1088, &t1089) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1111(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1117: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1112(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1117 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1118 = v_r.clone(); Py::Bool({ let t1119 = Py::Bool(true); if py::is(&t1118, &t1119) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1124 = { let t1120 = v_r.clone(); Py::Bool({ let t1121 = Py::Bool(false); if py::is(&t1120, &t1121) { true } else { false } }) }; if t1124.truthy() { t1124 } else { { let t1122 = v_r.clone(); Py::Bool({ let t1123 = Py::None; if py::is(&t1122, &t1123) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1112(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t1116 = f__iv(py::getitem(&v_o.clone(), &Py::str("combinations"))?)?; if !t1116.truthy() { t1116 } else { { let t1114 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("combinations"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; Py::Bool({ let t1115 = py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t1114, &t1115) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1154(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1176: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_cc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1176 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1177 = v_r.clone(); Py::Bool({ let t1178 = Py::Bool(true); if py::is(&t1177, &t1178) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1183 = { let t1179 = v_r.clone(); Py::Bool({ let t1180 = Py::Bool(false); if py::is(&t1179, &t1180) { true } else { false } }) }; if t1183.truthy() { t1183 } else { { let t1181 = v_r.clone(); Py::Bool({ let t1182 = Py::None; if py::is(&t1181, &t1182) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1223(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1242: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_crt(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1242 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1243 = v_r.clone(); Py::Bool({ let t1244 = Py::Bool(true); if py::is(&t1243, &t1244) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1249 = { let t1245 = v_r.clone(); Py::Bool({ let t1246 = Py::Bool(false); if py::is(&t1245, &t1246) { true } else { false } }) }; if t1249.truthy() { t1249 } else { { let t1247 = v_r.clone(); Py::Bool({ let t1248 = Py::None; if py::is(&t1247, &t1248) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1279(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1296: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_crank(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1296 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1297 = v_r.clone(); Py::Bool({ let t1298 = Py::Bool(true); if py::is(&t1297, &t1298) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1303 = { let t1299 = v_r.clone(); Py::Bool({ let t1300 = Py::Bool(false); if py::is(&t1299, &t1300) { true } else { false } }) }; if t1303.truthy() { t1303 } else { { let t1301 = v_r.clone(); Py::Bool({ let t1302 = Py::None; if py::is(&t1301, &t1302) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1323(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1332: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_der(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1332 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1333 = v_r.clone(); Py::Bool({ let t1334 = Py::Bool(true); if py::is(&t1333, &t1334) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1339 = { let t1335 = v_r.clone(); Py::Bool({ let t1336 = Py::Bool(false); if py::is(&t1335, &t1336) { true } else { false } }) }; if t1339.truthy() { t1339 } else { { let t1337 = v_r.clone(); Py::Bool({ let t1338 = Py::None; if py::is(&t1337, &t1338) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1360(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1373: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dfs(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1373 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1374 = v_r.clone(); Py::Bool({ let t1375 = Py::Bool(true); if py::is(&t1374, &t1375) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1380 = { let t1376 = v_r.clone(); Py::Bool({ let t1377 = Py::Bool(false); if py::is(&t1376, &t1377) { true } else { false } }) }; if t1380.truthy() { t1380 } else { { let t1378 = v_r.clone(); Py::Bool({ let t1379 = Py::None; if py::is(&t1378, &t1379) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_142(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t183: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_app(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t183 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t184 = v_r.clone(); Py::Bool({ let t185 = Py::Bool(true); if py::is(&t184, &t185) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t190 = { let t186 = v_r.clone(); Py::Bool({ let t187 = Py::Bool(false); if py::is(&t186, &t187) { true } else { false } }) }; if t190.truthy() { t190 } else { { let t188 = v_r.clone(); Py::Bool({ let t189 = Py::None; if py::is(&t188, &t189) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1421(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1433: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1422(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1433 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1434 = v_r.clone(); Py::Bool({ let t1435 = Py::Bool(true); if py::is(&t1434, &t1435) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1440 = { let t1436 = v_r.clone(); Py::Bool({ let t1437 = Py::Bool(false); if py::is(&t1436, &t1437) { true } else { false } }) }; if t1440.truthy() { t1440 } else { { let t1438 = v_r.clone(); Py::Bool({ let t1439 = Py::None; if py::is(&t1438, &t1439) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1422(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_g: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(3).cloned().unwrap_or(Py::None);
            return Ok({ let t1432 = f__iv(py::getitem(&v_o.clone(), &Py::str("shared"))?)?; if !t1432.truthy() { t1432 } else { { let t1431 = { let t1424 = py::getitem(&v_o.clone(), &Py::str("A"))?; Py::Bool({ let t1425 = py::pow_mod(&v_g.clone(), &v_a.clone(), &v_p.clone())?; if py::eq(&t1424, &t1425) { true } else { false } }) }; if !t1431.truthy() { t1431 } else { { let t1430 = { let t1426 = py::getitem(&v_o.clone(), &Py::str("B"))?; Py::Bool({ let t1427 = py::pow_mod(&v_g.clone(), &v_b.clone(), &v_p.clone())?; if py::eq(&t1426, &t1427) { true } else { false } }) }; if !t1430.truthy() { t1430 } else { { let t1428 = py::getitem(&v_o.clone(), &Py::str("shared"))?; Py::Bool({ let t1429 = py::pow_mod(&py::getitem(&v_o.clone(), &Py::str("A"))?, &v_b.clone(), &v_p.clone())?; if py::eq(&t1428, &t1429) { true } else { false } }) } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("g"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1476(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1492: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dcyc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1492 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1493 = v_r.clone(); Py::Bool({ let t1494 = Py::Bool(true); if py::is(&t1493, &t1494) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1499 = { let t1495 = v_r.clone(); Py::Bool({ let t1496 = Py::Bool(false); if py::is(&t1495, &t1496) { true } else { false } }) }; if t1499.truthy() { t1499 } else { { let t1497 = v_r.clone(); Py::Bool({ let t1498 = Py::None; if py::is(&t1497, &t1498) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1530(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1542: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dlog(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1542 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1543 = v_r.clone(); Py::Bool({ let t1544 = Py::Bool(true); if py::is(&t1543, &t1544) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1549 = { let t1545 = v_r.clone(); Py::Bool({ let t1546 = Py::Bool(false); if py::is(&t1545, &t1546) { true } else { false } }) }; if t1549.truthy() { t1549 } else { { let t1547 = v_r.clone(); Py::Bool({ let t1548 = Py::None; if py::is(&t1547, &t1548) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1610(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1649: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_div(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1649 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1650 = v_r.clone(); Py::Bool({ let t1651 = Py::Bool(true); if py::is(&t1650, &t1651) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1656 = { let t1652 = v_r.clone(); Py::Bool({ let t1653 = Py::Bool(false); if py::is(&t1652, &t1653) { true } else { false } }) }; if t1656.truthy() { t1656 } else { { let t1654 = v_r.clone(); Py::Bool({ let t1655 = Py::None; if py::is(&t1654, &t1655) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1699(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1712: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_elg(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1712 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1713 = v_r.clone(); Py::Bool({ let t1714 = Py::Bool(true); if py::is(&t1713, &t1714) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1719 = { let t1715 = v_r.clone(); Py::Bool({ let t1716 = Py::Bool(false); if py::is(&t1715, &t1716) { true } else { false } }) }; if t1719.truthy() { t1719 } else { { let t1717 = v_r.clone(); Py::Bool({ let t1718 = Py::None; if py::is(&t1717, &t1718) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1773(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_phi(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1786 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1787 = v_r.clone(); Py::Bool({ let t1788 = Py::Bool(true); if py::is(&t1787, &t1788) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1793 = { let t1789 = v_r.clone(); Py::Bool({ let t1790 = Py::Bool(false); if py::is(&t1789, &t1790) { true } else { false } }) }; if t1793.truthy() { t1793 } else { { let t1791 = v_r.clone(); Py::Bool({ let t1792 = Py::None; if py::is(&t1791, &t1792) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1829(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1842: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1830(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1842 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1843 = v_r.clone(); Py::Bool({ let t1844 = Py::Bool(true); if py::is(&t1843, &t1844) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1849 = { let t1845 = v_r.clone(); Py::Bool({ let t1846 = Py::Bool(false); if py::is(&t1845, &t1846) { true } else { false } }) }; if t1849.truthy() { t1849 } else { { let t1847 = v_r.clone(); Py::Bool({ let t1848 = Py::None; if py::is(&t1847, &t1848) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1830(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t1841 = { let t1831 = v_o.clone(); Py::Bool({ let t1832 = f_eulerian(py::getitem(&v_i.clone(), &Py::str("edges"))?)?; if py::eq(&t1831, &t1832) { true } else { false } }) }; if !t1841.truthy() { t1841 } else { { let t1840 = f__iv(py::getitem(&v_o.clone(), &Py::str("odd_vertices"))?)?; if !t1840.truthy() { t1840 } else { { let t1834 = py::getitem(&v_o.clone(), &Py::str("odd_vertices"))?; Py::Bool({ let t1835 = py::sum(&{ let mut out = vec![]; for t1838 in py::iter(&(py::method(&py::counter(&{ let mut out = vec![]; for t1836 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t1836;
for t1837 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? { let mut v_x: Py; 
v_x = t1837;
out.push(Py::str(&py::to_str(&v_x.clone()))); }} Py::list(out) })?, "items", &[])?))? { let mut v_d: Py; let mut v_v: Py; 
let t1839 = (t1838).items()?;
if t1839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1839[0].clone();
v_d = t1839[1].clone();
if (py::modulo(&v_d.clone(), &Py::Int(2i64))?).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t1834, &t1835) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1870(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1886: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ext(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1886 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1887 = v_r.clone(); Py::Bool({ let t1888 = Py::Bool(true); if py::is(&t1887, &t1888) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1893 = { let t1889 = v_r.clone(); Py::Bool({ let t1890 = Py::Bool(false); if py::is(&t1889, &t1890) { true } else { false } }) }; if t1893.truthy() { t1893 } else { { let t1891 = v_r.clone(); Py::Bool({ let t1892 = Py::None; if py::is(&t1891, &t1892) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1911(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1920: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1912(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1920 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1921 = v_r.clone(); Py::Bool({ let t1922 = Py::Bool(true); if py::is(&t1921, &t1922) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1927 = { let t1923 = v_r.clone(); Py::Bool({ let t1924 = Py::Bool(false); if py::is(&t1923, &t1924) { true } else { false } }) }; if t1927.truthy() { t1927 } else { { let t1925 = v_r.clone(); Py::Bool({ let t1926 = Py::None; if py::is(&t1925, &t1926) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1912(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t1919 = f__iv(py::getitem(&v_o.clone(), &Py::str("fibonacci"))?)?; if !t1919.truthy() { t1919 } else { { let t1914 = py::getitem(&v_o.clone(), &Py::str("fibonacci"))?; Py::Bool({ let t1915 = py::getitem(&f__fib_pair(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, &Py::Int(0i64))?; if py::eq(&t1914, &t1915) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1954(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1980: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_gcd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1980 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1981 = v_r.clone(); Py::Bool({ let t1982 = Py::Bool(true); if py::is(&t1981, &t1982) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1987 = { let t1983 = v_r.clone(); Py::Bool({ let t1984 = Py::Bool(false); if py::is(&t1983, &t1984) { true } else { false } }) }; if t1987.truthy() { t1987 } else { { let t1985 = v_r.clone(); Py::Bool({ let t1986 = Py::None; if py::is(&t1985, &t1986) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2024(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2044: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_girth(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2044 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2045 = v_r.clone(); Py::Bool({ let t2046 = Py::Bool(true); if py::is(&t2045, &t2046) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2051 = { let t2047 = v_r.clone(); Py::Bool({ let t2048 = Py::Bool(false); if py::is(&t2047, &t2048) { true } else { false } }) }; if t2051.truthy() { t2051 } else { { let t2049 = v_r.clone(); Py::Bool({ let t2050 = Py::None; if py::is(&t2049, &t2050) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2071(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2093: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_deg(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2093 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2094 = v_r.clone(); Py::Bool({ let t2095 = Py::Bool(true); if py::is(&t2094, &t2095) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2100 = { let t2096 = v_r.clone(); Py::Bool({ let t2097 = Py::Bool(false); if py::is(&t2096, &t2097) { true } else { false } }) }; if t2100.truthy() { t2100 } else { { let t2098 = v_r.clone(); Py::Bool({ let t2099 = Py::None; if py::is(&t2098, &t2099) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2141(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2178: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_diam(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2178 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2179 = v_r.clone(); Py::Bool({ let t2180 = Py::Bool(true); if py::is(&t2179, &t2180) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2185 = { let t2181 = v_r.clone(); Py::Bool({ let t2182 = Py::Bool(false); if py::is(&t2181, &t2182) { true } else { false } }) }; if t2185.truthy() { t2185 } else { { let t2183 = v_r.clone(); Py::Bool({ let t2184 = Py::None; if py::is(&t2183, &t2184) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2210(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2226: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2211(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2226 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2227 = v_r.clone(); Py::Bool({ let t2228 = Py::Bool(true); if py::is(&t2227, &t2228) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2233 = { let t2229 = v_r.clone(); Py::Bool({ let t2230 = Py::Bool(false); if py::is(&t2229, &t2230) { true } else { false } }) }; if t2233.truthy() { t2233 } else { { let t2231 = v_r.clone(); Py::Bool({ let t2232 = Py::None; if py::is(&t2231, &t2232) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2211(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t2225 = f__iv(py::getitem(&v_o.clone(), &Py::str("triangles"))?)?; if !t2225.truthy() { t2225 } else { py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_adj: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t2213 = py::getitem(&v_o.clone(), &Py::str("triangles"))?; Py::Bool({ let t2214 = py::sum(&{ let mut out = vec![]; for t2215 in py::iter(&(py::itertools("combinations", &[f__sorted(v_adj.clone())?, Py::Int(3i64)])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t2216 = (t2215).items()?;
if t2216.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t2216[0].clone();
v_b = t2216[1].clone();
v_c = t2216[2].clone();
if ({ let t2224 = { let t2217 = v_b.clone(); Py::Bool({ let t2218 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t2218, &t2217)? { true } else { false } }) }; if !t2224.truthy() { t2224 } else { { let t2223 = { let t2219 = v_c.clone(); Py::Bool({ let t2220 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t2220, &t2219)? { true } else { false } }) }; if !t2223.truthy() { t2223 } else { { let t2221 = v_c.clone(); Py::Bool({ let t2222 = py::getitem(&v_adj.clone(), &v_b.clone())?; if py::contains(&t2222, &t2221)? { true } else { false } }) } } } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t2213, &t2214) { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?])? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2267(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2278: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_graphical(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2278 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2279 = v_r.clone(); Py::Bool({ let t2280 = Py::Bool(true); if py::is(&t2279, &t2280) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2285 = { let t2281 = v_r.clone(); Py::Bool({ let t2282 = Py::Bool(false); if py::is(&t2281, &t2282) { true } else { false } }) }; if t2285.truthy() { t2285 } else { { let t2283 = v_r.clone(); Py::Bool({ let t2284 = Py::None; if py::is(&t2283, &t2284) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2317(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2349: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ham(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2349 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2350 = v_r.clone(); Py::Bool({ let t2351 = Py::Bool(true); if py::is(&t2350, &t2351) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2356 = { let t2352 = v_r.clone(); Py::Bool({ let t2353 = Py::Bool(false); if py::is(&t2352, &t2353) { true } else { false } }) }; if t2356.truthy() { t2356 } else { { let t2354 = v_r.clone(); Py::Bool({ let t2355 = Py::None; if py::is(&t2354, &t2355) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_235(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t265: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ap(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t265 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t266 = v_r.clone(); Py::Bool({ let t267 = Py::Bool(true); if py::is(&t266, &t267) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t272 = { let t268 = v_r.clone(); Py::Bool({ let t269 = Py::Bool(false); if py::is(&t268, &t269) { true } else { false } }) }; if t272.truthy() { t272 } else { { let t270 = v_r.clone(); Py::Bool({ let t271 = Py::None; if py::is(&t270, &t271) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2384(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2401: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_incl(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2401 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2402 = v_r.clone(); Py::Bool({ let t2403 = Py::Bool(true); if py::is(&t2402, &t2403) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2408 = { let t2404 = v_r.clone(); Py::Bool({ let t2405 = Py::Bool(false); if py::is(&t2404, &t2405) { true } else { false } }) }; if t2408.truthy() { t2408 } else { { let t2406 = v_r.clone(); Py::Bool({ let t2407 = Py::None; if py::is(&t2406, &t2407) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2445(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2468: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_indep(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2468 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2469 = v_r.clone(); Py::Bool({ let t2470 = Py::Bool(true); if py::is(&t2469, &t2470) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2475 = { let t2471 = v_r.clone(); Py::Bool({ let t2472 = Py::Bool(false); if py::is(&t2471, &t2472) { true } else { false } }) }; if t2475.truthy() { t2475 } else { { let t2473 = v_r.clone(); Py::Bool({ let t2474 = Py::None; if py::is(&t2473, &t2474) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2496(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2505: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ipart(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2505 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2506 = v_r.clone(); Py::Bool({ let t2507 = Py::Bool(true); if py::is(&t2506, &t2507) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2512 = { let t2508 = v_r.clone(); Py::Bool({ let t2509 = Py::Bool(false); if py::is(&t2508, &t2509) { true } else { false } }) }; if t2512.truthy() { t2512 } else { { let t2510 = v_r.clone(); Py::Bool({ let t2511 = Py::None; if py::is(&t2510, &t2511) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2547: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_isqrt(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2547 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2548 = v_r.clone(); Py::Bool({ let t2549 = Py::Bool(true); if py::is(&t2548, &t2549) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2554 = { let t2550 = v_r.clone(); Py::Bool({ let t2551 = Py::Bool(false); if py::is(&t2550, &t2551) { true } else { false } }) }; if t2554.truthy() { t2554 } else { { let t2552 = v_r.clone(); Py::Bool({ let t2553 = Py::None; if py::is(&t2552, &t2553) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2611(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2644: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_isprime(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2644 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2645 = v_r.clone(); Py::Bool({ let t2646 = Py::Bool(true); if py::is(&t2645, &t2646) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2651 = { let t2647 = v_r.clone(); Py::Bool({ let t2648 = Py::Bool(false); if py::is(&t2647, &t2648) { true } else { false } }) }; if t2651.truthy() { t2651 } else { { let t2649 = v_r.clone(); Py::Bool({ let t2650 = Py::None; if py::is(&t2649, &t2650) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2684(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2710: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2685(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2710 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2711 = v_r.clone(); Py::Bool({ let t2712 = Py::Bool(true); if py::is(&t2711, &t2712) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2717 = { let t2713 = v_r.clone(); Py::Bool({ let t2714 = Py::Bool(false); if py::is(&t2713, &t2714) { true } else { false } }) }; if t2717.truthy() { t2717 } else { { let t2715 = v_r.clone(); Py::Bool({ let t2716 = Py::None; if py::is(&t2715, &t2716) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2685(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t2705 = { let t2686 = py::getitem(&v_o.clone(), &Py::str("connected"))?; Py::Bool({ let t2687 = { let t2688 = v_c.clone(); Py::Bool({ let t2689 = Py::Int(1i64); if py::eq(&t2688, &t2689) { true } else { false } }) }; if py::is(&t2686, &t2687) { true } else { false } }) }; if !t2705.truthy() { t2705 } else { { let t2704 = { let t2690 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t2691 = py::len(&py::set(&{ let mut out = vec![]; for t2692 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t2692;
for t2693 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? { let mut v_x: Py; 
v_x = t2693;
out.push(Py::str(&py::to_str(&v_x.clone()))); }} Py::list(out) })?)?; if py::eq(&t2690, &t2691) { true } else { false } }) }; if !t2704.truthy() { t2704 } else { { let t2703 = { let t2694 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t2695 = py::len(&py::getitem(&v_i.clone(), &Py::str("edges"))?)?; if py::eq(&t2694, &t2695) { true } else { false } }) }; if !t2703.truthy() { t2703 } else { { let t2696 = py::getitem(&v_o.clone(), &Py::str("is_tree"))?; Py::Bool({ let t2697 = { let t2702 = { let t2698 = v_c.clone(); Py::Bool({ let t2699 = Py::Int(1i64); if py::eq(&t2698, &t2699) { true } else { false } }) }; if !t2702.truthy() { t2702 } else { { let t2700 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t2701 = py::sub(&py::getitem(&v_o.clone(), &Py::str("vertices"))?, &Py::Int(1i64))?; if py::eq(&t2700, &t2701) { true } else { false } }) } } }; if py::is(&t2696, &t2697) { true } else { false } }) } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__v_cc_count(py::getitem(&v_i.clone(), &Py::str("edges"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2747(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2793: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_jacobi(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2793 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2794 = v_r.clone(); Py::Bool({ let t2795 = Py::Bool(true); if py::is(&t2794, &t2795) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2800 = { let t2796 = v_r.clone(); Py::Bool({ let t2797 = Py::Bool(false); if py::is(&t2796, &t2797) { true } else { false } }) }; if t2800.truthy() { t2800 } else { { let t2798 = v_r.clone(); Py::Bool({ let t2799 = Py::None; if py::is(&t2798, &t2799) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2818(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2824: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2819(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2824 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2825 = v_r.clone(); Py::Bool({ let t2826 = Py::Bool(true); if py::is(&t2825, &t2826) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2831 = { let t2827 = v_r.clone(); Py::Bool({ let t2828 = Py::Bool(false); if py::is(&t2827, &t2828) { true } else { false } }) }; if t2831.truthy() { t2831 } else { { let t2829 = v_r.clone(); Py::Bool({ let t2830 = Py::None; if py::is(&t2829, &t2830) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2819(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t2823 = f__iv(py::getitem(&v_o.clone(), &Py::str("paths"))?)?; if !t2823.truthy() { t2823 } else { { let t2821 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("paths"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("right"))?)?)?)?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("up"))?)?)?)?; Py::Bool({ let t2822 = py::factorial(&py::add(&py::int(&py::getitem(&v_i.clone(), &Py::str("right"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("up"))?)?)?)?; if py::eq(&t2821, &t2822) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2884(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2890: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_legendre(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2890 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2891 = v_r.clone(); Py::Bool({ let t2892 = Py::Bool(true); if py::is(&t2891, &t2892) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2897 = { let t2893 = v_r.clone(); Py::Bool({ let t2894 = Py::Bool(false); if py::is(&t2893, &t2894) { true } else { false } }) }; if t2897.truthy() { t2897 } else { { let t2895 = v_r.clone(); Py::Bool({ let t2896 = Py::None; if py::is(&t2895, &t2896) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2922(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2946: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_lincong(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2946 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2947 = v_r.clone(); Py::Bool({ let t2948 = Py::Bool(true); if py::is(&t2947, &t2948) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2953 = { let t2949 = v_r.clone(); Py::Bool({ let t2950 = Py::Bool(false); if py::is(&t2949, &t2950) { true } else { false } }) }; if t2953.truthy() { t2953 } else { { let t2951 = v_r.clone(); Py::Bool({ let t2952 = Py::None; if py::is(&t2951, &t2952) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2981(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3007: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dioph(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3007 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3008 = v_r.clone(); Py::Bool({ let t3009 = Py::Bool(true); if py::is(&t3008, &t3009) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3014 = { let t3010 = v_r.clone(); Py::Bool({ let t3011 = Py::Bool(false); if py::is(&t3010, &t3011) { true } else { false } }) }; if t3014.truthy() { t3014 } else { { let t3012 = v_r.clone(); Py::Bool({ let t3013 = Py::None; if py::is(&t3012, &t3013) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_305(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t328: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_base(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t328 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t329 = v_r.clone(); Py::Bool({ let t330 = Py::Bool(true); if py::is(&t329, &t330) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t335 = { let t331 = v_r.clone(); Py::Bool({ let t332 = Py::Bool(false); if py::is(&t331, &t332) { true } else { false } }) }; if t335.truthy() { t335 } else { { let t333 = v_r.clone(); Py::Bool({ let t334 = Py::None; if py::is(&t333, &t334) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3089(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3099: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3090(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3099 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3100 = v_r.clone(); Py::Bool({ let t3101 = Py::Bool(true); if py::is(&t3100, &t3101) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3106 = { let t3102 = v_r.clone(); Py::Bool({ let t3103 = Py::Bool(false); if py::is(&t3102, &t3103) { true } else { false } }) }; if t3106.truthy() { t3106 } else { { let t3104 = v_r.clone(); Py::Bool({ let t3105 = Py::None; if py::is(&t3104, &t3105) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3090(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t3098 = { let t3091 = py::getitem(&v_o.clone(), &Py::str("contradiction"))?; Py::Bool({ let t3092 = py::not(&py::getitem(&v_s.clone(), &Py::str("satisfiable"))?); if py::is(&t3091, &t3092) { true } else { false } }) }; if !t3098.truthy() { t3098 } else { { let t3097 = { let t3093 = py::getitem(&v_o.clone(), &Py::str("true_rows"))?; Py::Bool({ let t3094 = py::getitem(&v_s.clone(), &Py::str("models"))?; if py::eq(&t3093, &t3094) { true } else { false } }) }; if !t3097.truthy() { t3097 } else { { let t3095 = py::getitem(&v_o.clone(), &Py::str("witness"))?; Py::Bool({ let t3096 = py::getitem(&v_s.clone(), &Py::str("first_model"))?; if py::eq(&t3095, &t3096) { true } else { false } }) } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_logic_satisfiable(py::getitem(&v_i.clone(), &Py::str("formula"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3189: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3185(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3189 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3190 = v_r.clone(); Py::Bool({ let t3191 = Py::Bool(true); if py::is(&t3190, &t3191) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3196 = { let t3192 = v_r.clone(); Py::Bool({ let t3193 = Py::Bool(false); if py::is(&t3192, &t3193) { true } else { false } }) }; if t3196.truthy() { t3196 } else { { let t3194 = v_r.clone(); Py::Bool({ let t3195 = Py::None; if py::is(&t3194, &t3195) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3185(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t3188 = { let t3186 = v_o.clone(); Py::Bool({ let t3187 = v_r.clone(); if py::eq(&t3186, &t3187) { true } else { false } }) }; if !t3188.truthy() { t3188 } else { Py::Bool(matches!(py::getitem(&v_o.clone(), &Py::str("equivalent"))?, Py::Bool(_))) } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_logic_equivalence(py::getitem(&v_i.clone(), &Py::str("formula1"))?, py::getitem(&v_i.clone(), &Py::str("formula2"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3272(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3277: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3273(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3277 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3278 = v_r.clone(); Py::Bool({ let t3279 = Py::Bool(true); if py::is(&t3278, &t3279) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3284 = { let t3280 = v_r.clone(); Py::Bool({ let t3281 = Py::Bool(false); if py::is(&t3280, &t3281) { true } else { false } }) }; if t3284.truthy() { t3284 } else { { let t3282 = v_r.clone(); Py::Bool({ let t3283 = Py::None; if py::is(&t3282, &t3283) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3273(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t3276 = Py::Bool(matches!(py::getitem(&v_o.clone(), &Py::str("value"))?, Py::Bool(_))); if !t3276.truthy() { t3276 } else { { let t3274 = py::getitem(&v_o.clone(), &Py::str("value"))?; Py::Bool({ let t3275 = py::getitem(&f_logic_evaluate(py::getitem(&v_i.clone(), &Py::str("formula"))?, py::getitem(&v_i.clone(), &Py::str("assignment"))?)?, &Py::str("value"))?; if py::eq(&t3274, &t3275) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_33(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t46: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_34(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t46 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t47 = v_r.clone(); Py::Bool({ let t48 = Py::Bool(true); if py::is(&t47, &t48) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t53 = { let t49 = v_r.clone(); Py::Bool({ let t50 = Py::Bool(false); if py::is(&t49, &t50) { true } else { false } }) }; if t53.truthy() { t53 } else { { let t51 = v_r.clone(); Py::Bool({ let t52 = Py::None; if py::is(&t51, &t52) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3374: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_sat(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3374 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3375 = v_r.clone(); Py::Bool({ let t3376 = Py::Bool(true); if py::is(&t3375, &t3376) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3381 = { let t3377 = v_r.clone(); Py::Bool({ let t3378 = Py::Bool(false); if py::is(&t3377, &t3378) { true } else { false } }) }; if t3381.truthy() { t3381 } else { { let t3379 = v_r.clone(); Py::Bool({ let t3380 = Py::None; if py::is(&t3379, &t3380) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_34(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t45 = { let t35 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t36 = py::getitem(&f_affine_cipher(py::getitem(&v_i.clone(), &Py::str("message"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("mode"))?)?, &Py::str("result"))?; if py::eq(&t35, &t36) { true } else { false } }) }; if !t45.truthy() { t45 } else { { let t37 = py::getitem(&f_affine_cipher(py::getitem(&v_o.clone(), &Py::str("result"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, (if ({ let t39 = py::getitem(&v_i.clone(), &Py::str("mode"))?; Py::Bool({ let t40 = Py::str("encrypt"); if py::eq(&t39, &t40) { true } else { false } }) }).truthy() { Py::str("decrypt") } else { Py::str("encrypt") }))?, &Py::str("result"))?; Py::Bool({ let t38 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t41 in py::iter(&(py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("message"))?)), "upper", &[])?))? { let mut v_c: Py; 
v_c = t41;
if ({ let t42 = Py::str("A"); Py::Bool({ let t43 = v_c.clone(); if py::le(&t42, &t43)? { { let t44 = Py::str("Z"); if py::le(&t43, &t44)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) }])?; if py::eq(&t37, &t38) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3471: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_taut(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3471 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3472 = v_r.clone(); Py::Bool({ let t3473 = Py::Bool(true); if py::is(&t3472, &t3473) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3478 = { let t3474 = v_r.clone(); Py::Bool({ let t3475 = Py::Bool(false); if py::is(&t3474, &t3475) { true } else { false } }) }; if t3478.truthy() { t3478 } else { { let t3476 = v_r.clone(); Py::Bool({ let t3477 = Py::None; if py::is(&t3476, &t3477) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3496(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3505: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3497(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3505 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3506 = v_r.clone(); Py::Bool({ let t3507 = Py::Bool(true); if py::is(&t3506, &t3507) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3512 = { let t3508 = v_r.clone(); Py::Bool({ let t3509 = Py::Bool(false); if py::is(&t3508, &t3509) { true } else { false } }) }; if t3512.truthy() { t3512 } else { { let t3510 = v_r.clone(); Py::Bool({ let t3511 = Py::None; if py::is(&t3510, &t3511) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3497(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t3504 = f__iv(py::getitem(&v_o.clone(), &Py::str("lucas"))?)?; if !t3504.truthy() { t3504 } else { py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let t3499 = py::getitem(&v_o.clone(), &Py::str("lucas"))?; Py::Bool({ let t3500 = py::sub(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_a.clone())?; if py::eq(&t3499, &t3500) { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f__fib_pair(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?).items()?); v })[..])? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_352(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t359: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bell(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t359 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t360 = v_r.clone(); Py::Bool({ let t361 = Py::Bool(true); if py::is(&t360, &t361) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t366 = { let t362 = v_r.clone(); Py::Bool({ let t363 = Py::Bool(false); if py::is(&t362, &t363) { true } else { false } }) }; if t366.truthy() { t366 } else { { let t364 = v_r.clone(); Py::Bool({ let t365 = Py::None; if py::is(&t364, &t365) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3638(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3639(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3701 = v_r.clone(); Py::Bool({ let t3702 = Py::Bool(true); if py::is(&t3701, &t3702) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3707 = { let t3703 = v_r.clone(); Py::Bool({ let t3704 = Py::Bool(false); if py::is(&t3703, &t3704) { true } else { false } }) }; if t3707.truthy() { t3707 } else { { let t3705 = v_r.clone(); Py::Bool({ let t3706 = Py::None; if py::is(&t3705, &t3706) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3639(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_on: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_names: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__v_pos(v_n.clone(), v_on.clone(), Py::list(vec![]), v_names.clone(), v_o.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f__formula_fn(py::getitem(&v_i.clone(), &Py::str("formula"))?)?).items()?); v })[..])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3824(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3878: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3825(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3878 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3879 = v_r.clone(); Py::Bool({ let t3880 = Py::Bool(true); if py::is(&t3879, &t3880) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3885 = { let t3881 = v_r.clone(); Py::Bool({ let t3882 = Py::Bool(false); if py::is(&t3881, &t3882) { true } else { false } }) }; if t3885.truthy() { t3885 } else { { let t3883 = v_r.clone(); Py::Bool({ let t3884 = Py::None; if py::is(&t3883, &t3884) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3825(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_on: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_names: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__v_sop(v_n.clone(), v_on.clone(), Py::list(vec![]), v_names.clone(), v_o.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f__formula_fn(py::getitem(&v_i.clone(), &Py::str("formula"))?)?).items()?); v })[..])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_383(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t399: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bern(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t399 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t400 = v_r.clone(); Py::Bool({ let t401 = Py::Bool(true); if py::is(&t400, &t401) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t406 = { let t402 = v_r.clone(); Py::Bool({ let t403 = Py::Bool(false); if py::is(&t402, &t403) { true } else { false } }) }; if t406.truthy() { t406 } else { { let t404 = v_r.clone(); Py::Bool({ let t405 = Py::None; if py::is(&t404, &t405) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3960(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4017: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3961(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4017 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4018 = v_r.clone(); Py::Bool({ let t4019 = Py::Bool(true); if py::is(&t4018, &t4019) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4024 = { let t4020 = v_r.clone(); Py::Bool({ let t4021 = Py::Bool(false); if py::is(&t4020, &t4021) { true } else { false } }) }; if t4024.truthy() { t4024 } else { { let t4022 = v_r.clone(); Py::Bool({ let t4023 = Py::None; if py::is(&t4022, &t4023) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3961(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__v_sop(py::int(&py::getitem(&v_i.clone(), &Py::str("variables"))?)?, py::sorted(&py::set(&{ let mut out = vec![]; for t4014 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("minterms"))?))? { let mut v_m: Py; 
v_m = t4014;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?, py::sorted(&py::set(&{ let mut out = vec![]; for t4016 in py::iter(&({ let t4015 = py::method(&v_i.clone(), "get", &[Py::str("dont_cares")])?; if t4015.truthy() { t4015 } else { Py::list(vec![]) } }))? { let mut v_m: Py; 
v_m = t4016;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?, Py::list((py::slice(&Py::str("ABCDEF"), None, Some((py::int(&py::getitem(&v_i.clone(), &Py::str("variables"))?)?).i()?), None)?).items()?), v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4080(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_mobius(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4103 = v_r.clone(); Py::Bool({ let t4104 = Py::Bool(true); if py::is(&t4103, &t4104) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4109 = { let t4105 = v_r.clone(); Py::Bool({ let t4106 = Py::Bool(false); if py::is(&t4105, &t4106) { true } else { false } }) }; if t4109.truthy() { t4109 } else { { let t4107 = v_r.clone(); Py::Bool({ let t4108 = Py::None; if py::is(&t4107, &t4108) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4133(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4142 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4143 = v_r.clone(); Py::Bool({ let t4144 = Py::Bool(true); if py::is(&t4143, &t4144) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4149 = { let t4145 = v_r.clone(); Py::Bool({ let t4146 = Py::Bool(false); if py::is(&t4145, &t4146) { true } else { false } }) }; if t4149.truthy() { t4149 } else { { let t4147 = v_r.clone(); Py::Bool({ let t4148 = Py::None; if py::is(&t4147, &t4148) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4133(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4141 = f__iv(py::getitem(&v_o.clone(), &Py::str("inverse"))?)?; if !t4141.truthy() { t4141 } else { { let t4140 = { let t4135 = Py::Int(0i64); Py::Bool({ let t4136 = py::getitem(&v_o.clone(), &Py::str("inverse"))?; if py::le(&t4135, &t4136)? { { let t4137 = py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?; if py::lt(&t4136, &t4137)? { true } else { false } } } else { false } }) }; if !t4140.truthy() { t4140 } else { { let t4138 = py::modulo(&py::mul(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_o.clone(), &Py::str("inverse"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?)?; Py::Bool({ let t4139 = Py::Int(1i64); if py::eq(&t4138, &t4139) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4166(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4172: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4167(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4172 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4173 = v_r.clone(); Py::Bool({ let t4174 = Py::Bool(true); if py::is(&t4173, &t4174) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4179 = { let t4175 = v_r.clone(); Py::Bool({ let t4176 = Py::Bool(false); if py::is(&t4175, &t4176) { true } else { false } }) }; if t4179.truthy() { t4179 } else { { let t4177 = v_r.clone(); Py::Bool({ let t4178 = Py::None; if py::is(&t4177, &t4178) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4167(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4171 = f__iv(py::getitem(&v_o.clone(), &Py::str("result"))?)?; if !t4171.truthy() { t4171 } else { { let t4169 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t4170 = py::pow_mod(&py::int(&py::getitem(&v_i.clone(), &Py::str("base"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("exponent"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?)?; if py::eq(&t4169, &t4170) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4196(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4207: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_modred(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4207 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4208 = v_r.clone(); Py::Bool({ let t4209 = Py::Bool(true); if py::is(&t4208, &t4209) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4214 = { let t4210 = v_r.clone(); Py::Bool({ let t4211 = Py::Bool(false); if py::is(&t4210, &t4211) { true } else { false } }) }; if t4214.truthy() { t4214 } else { { let t4212 = v_r.clone(); Py::Bool({ let t4213 = Py::None; if py::is(&t4212, &t4213) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4254: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4247(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4254 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4255 = v_r.clone(); Py::Bool({ let t4256 = Py::Bool(true); if py::is(&t4255, &t4256) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4261 = { let t4257 = v_r.clone(); Py::Bool({ let t4258 = Py::Bool(false); if py::is(&t4257, &t4258) { true } else { false } }) }; if t4261.truthy() { t4261 } else { { let t4259 = v_r.clone(); Py::Bool({ let t4260 = Py::None; if py::is(&t4259, &t4260) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4247(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4253 = f__iv(py::getitem(&v_o.clone(), &Py::str("coefficient"))?)?; if !t4253.truthy() { t4253 } else { { let t4249 = py::mul(&py::getitem(&v_o.clone(), &Py::str("coefficient"))?, &py::prod(&{ let mut out = vec![]; for t4251 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("counts"))?))? { let mut v_k: Py; 
v_k = t4251;
out.push(py::factorial(&py::int(&v_k.clone())?)?); } Py::list(out) })?)?; Py::Bool({ let t4250 = py::factorial(&py::sum(&{ let mut out = vec![]; for t4252 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("counts"))?))? { let mut v_k: Py; 
v_k = t4252;
out.push(py::int(&v_k.clone())?); } Py::list(out) }, None)?)?; if py::eq(&t4249, &t4250) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_433(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t445: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bfs(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t445 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t446 = v_r.clone(); Py::Bool({ let t447 = Py::Bool(true); if py::is(&t446, &t447) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t452 = { let t448 = v_r.clone(); Py::Bool({ let t449 = Py::Bool(false); if py::is(&t448, &t449) { true } else { false } }) }; if t452.truthy() { t452 } else { { let t450 = v_r.clone(); Py::Bool({ let t451 = Py::None; if py::is(&t450, &t451) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4333(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4355: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_order(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4355 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4356 = v_r.clone(); Py::Bool({ let t4357 = Py::Bool(true); if py::is(&t4356, &t4357) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4362 = { let t4358 = v_r.clone(); Py::Bool({ let t4359 = Py::Bool(false); if py::is(&t4358, &t4359) { true } else { false } }) }; if t4362.truthy() { t4362 } else { { let t4360 = v_r.clone(); Py::Bool({ let t4361 = Py::None; if py::is(&t4360, &t4361) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4380(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4386: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4381(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4386 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4387 = v_r.clone(); Py::Bool({ let t4388 = Py::Bool(true); if py::is(&t4387, &t4388) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4393 = { let t4389 = v_r.clone(); Py::Bool({ let t4390 = Py::Bool(false); if py::is(&t4389, &t4390) { true } else { false } }) }; if t4393.truthy() { t4393 } else { { let t4391 = v_r.clone(); Py::Bool({ let t4392 = Py::None; if py::is(&t4391, &t4392) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4381(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4385 = f__iv(py::getitem(&v_o.clone(), &Py::str("multisets"))?)?; if !t4385.truthy() { t4385 } else { { let t4383 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("multisets"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &Py::Int(1i64))?)?)?; Py::Bool({ let t4384 = py::factorial(&py::sub(&py::add(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?, &Py::Int(1i64))?)?; if py::eq(&t4383, &t4384) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4455(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4466: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_neck(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4466 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4467 = v_r.clone(); Py::Bool({ let t4468 = Py::Bool(true); if py::is(&t4467, &t4468) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4473 = { let t4469 = v_r.clone(); Py::Bool({ let t4470 = Py::Bool(false); if py::is(&t4469, &t4470) { true } else { false } }) }; if t4473.truthy() { t4473 } else { { let t4471 = v_r.clone(); Py::Bool({ let t4472 = Py::None; if py::is(&t4471, &t4472) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4510(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_next(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4534 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4535 = v_r.clone(); Py::Bool({ let t4536 = Py::Bool(true); if py::is(&t4535, &t4536) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4541 = { let t4537 = v_r.clone(); Py::Bool({ let t4538 = Py::Bool(false); if py::is(&t4537, &t4538) { true } else { false } }) }; if t4541.truthy() { t4541 } else { { let t4539 = v_r.clone(); Py::Bool({ let t4540 = Py::None; if py::is(&t4539, &t4540) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4542(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4564: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4543(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4564 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4565 = v_r.clone(); Py::Bool({ let t4566 = Py::Bool(true); if py::is(&t4565, &t4566) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4571 = { let t4567 = v_r.clone(); Py::Bool({ let t4568 = Py::Bool(false); if py::is(&t4567, &t4568) { true } else { false } }) }; if t4571.truthy() { t4571 } else { { let t4569 = v_r.clone(); Py::Bool({ let t4570 = Py::None; if py::is(&t4569, &t4570) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4543(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4563 = f__iv(py::getitem(&v_o.clone(), &Py::str("prime"))?)?; if !t4563.truthy() { t4563 } else { { let t4562 = f__is_prime_alt(py::getitem(&v_o.clone(), &Py::str("prime"))?)?; if !t4562.truthy() { t4562 } else { { let t4560 = ax_engine::pysym::helper("discrete_math:_odd_sieve_count", &[py::getitem(&v_o.clone(), &Py::str("prime"))?], &[])?; Py::Bool({ let t4561 = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?; if py::eq(&t4560, &t4561) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4590(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4599: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4599 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4600 = v_r.clone(); Py::Bool({ let t4601 = Py::Bool(true); if py::is(&t4600, &t4601) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4606 = { let t4602 = v_r.clone(); Py::Bool({ let t4603 = Py::Bool(false); if py::is(&t4602, &t4603) { true } else { false } }) }; if t4606.truthy() { t4606 } else { { let t4604 = v_r.clone(); Py::Bool({ let t4605 = Py::None; if py::is(&t4604, &t4605) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4629(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4643: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pk(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4643 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4644 = v_r.clone(); Py::Bool({ let t4645 = Py::Bool(true); if py::is(&t4644, &t4645) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4650 = { let t4646 = v_r.clone(); Py::Bool({ let t4647 = Py::Bool(false); if py::is(&t4646, &t4647) { true } else { false } }) }; if t4650.truthy() { t4650 } else { { let t4648 = v_r.clone(); Py::Bool({ let t4649 = Py::None; if py::is(&t4648, &t4649) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4676(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4694: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pell(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4694 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4695 = v_r.clone(); Py::Bool({ let t4696 = Py::Bool(true); if py::is(&t4695, &t4696) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4701 = { let t4697 = v_r.clone(); Py::Bool({ let t4698 = Py::Bool(false); if py::is(&t4697, &t4698) { true } else { false } }) }; if t4701.truthy() { t4701 } else { { let t4699 = v_r.clone(); Py::Bool({ let t4700 = Py::None; if py::is(&t4699, &t4700) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4720(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4726: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4721(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4726 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4727 = v_r.clone(); Py::Bool({ let t4728 = Py::Bool(true); if py::is(&t4727, &t4728) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4733 = { let t4729 = v_r.clone(); Py::Bool({ let t4730 = Py::Bool(false); if py::is(&t4729, &t4730) { true } else { false } }) }; if t4733.truthy() { t4733 } else { { let t4731 = v_r.clone(); Py::Bool({ let t4732 = Py::None; if py::is(&t4731, &t4732) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4721(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4725 = f__iv(py::getitem(&v_o.clone(), &Py::str("permutations"))?)?; if !t4725.truthy() { t4725 } else { { let t4723 = py::mul(&py::getitem(&v_o.clone(), &Py::str("permutations"))?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; Py::Bool({ let t4724 = py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t4723, &t4724) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t479: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_474(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t479 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t480 = v_r.clone(); Py::Bool({ let t481 = Py::Bool(true); if py::is(&t480, &t481) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t486 = { let t482 = v_r.clone(); Py::Bool({ let t483 = Py::Bool(false); if py::is(&t482, &t483) { true } else { false } }) }; if t486.truthy() { t486 } else { { let t484 = v_r.clone(); Py::Bool({ let t485 = Py::None; if py::is(&t484, &t485) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_474(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t478 = f__iv(py::getitem(&v_o.clone(), &Py::str("coefficient"))?)?; if !t478.truthy() { t478 } else { { let t476 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("coefficient"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; Py::Bool({ let t477 = py::mul(&py::mul(&py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, &py::pow(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::pow(&py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; if py::eq(&t476, &t477) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4749(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4756: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4750(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4756 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4757 = v_r.clone(); Py::Bool({ let t4758 = Py::Bool(true); if py::is(&t4757, &t4758) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4763 = { let t4759 = v_r.clone(); Py::Bool({ let t4760 = Py::Bool(false); if py::is(&t4759, &t4760) { true } else { false } }) }; if t4763.truthy() { t4763 } else { { let t4761 = v_r.clone(); Py::Bool({ let t4762 = Py::None; if py::is(&t4761, &t4762) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4750(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4755 = f__iv(py::getitem(&v_o.clone(), &Py::str("guaranteed"))?)?; if !t4755.truthy() { t4755 } else { { let t4752 = py::mul(&py::sub(&py::getitem(&v_o.clone(), &Py::str("guaranteed"))?, &Py::Int(1i64))?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?; Py::Bool({ let t4753 = py::int(&py::getitem(&v_i.clone(), &Py::str("items"))?)?; if py::lt(&t4752, &t4753)? { { let t4754 = py::mul(&py::getitem(&v_o.clone(), &Py::str("guaranteed"))?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?; if py::le(&t4753, &t4754)? { true } else { false } } } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4779(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4780(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4786 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4787 = v_r.clone(); Py::Bool({ let t4788 = Py::Bool(true); if py::is(&t4787, &t4788) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4793 = { let t4789 = v_r.clone(); Py::Bool({ let t4790 = Py::Bool(false); if py::is(&t4789, &t4790) { true } else { false } }) }; if t4793.truthy() { t4793 } else { { let t4791 = v_r.clone(); Py::Bool({ let t4792 = Py::None; if py::is(&t4791, &t4792) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4780(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4785 = f__iv(py::getitem(&v_o.clone(), &Py::str("needed"))?)?; if !t4785.truthy() { t4785 } else { { let t4782 = py::neg(&py::floordiv(&py::neg(&py::getitem(&v_o.clone(), &Py::str("needed"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?)?; Py::Bool({ let t4783 = py::int(&py::getitem(&v_i.clone(), &Py::str("at_least"))?)?; if py::ge(&t4782, &t4783)? { { let t4784 = py::neg(&py::floordiv(&py::neg(&py::sub(&py::getitem(&v_o.clone(), &Py::str("needed"))?, &Py::Int(1i64))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?)?; if py::gt(&t4783, &t4784)? { true } else { false } } } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4830(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4854: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_prev(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4854 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4855 = v_r.clone(); Py::Bool({ let t4856 = Py::Bool(true); if py::is(&t4855, &t4856) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4861 = { let t4857 = v_r.clone(); Py::Bool({ let t4858 = Py::Bool(false); if py::is(&t4857, &t4858) { true } else { false } }) }; if t4861.truthy() { t4861 } else { { let t4859 = v_r.clone(); Py::Bool({ let t4860 = Py::None; if py::is(&t4859, &t4860) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4862(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4868: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4863(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4868 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4869 = v_r.clone(); Py::Bool({ let t4870 = Py::Bool(true); if py::is(&t4869, &t4870) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4875 = { let t4871 = v_r.clone(); Py::Bool({ let t4872 = Py::Bool(false); if py::is(&t4871, &t4872) { true } else { false } }) }; if t4875.truthy() { t4875 } else { { let t4873 = v_r.clone(); Py::Bool({ let t4874 = Py::None; if py::is(&t4873, &t4874) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4863(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4867 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t4867.truthy() { t4867 } else { { let t4865 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t4866 = ax_engine::pysym::helper("discrete_math:_odd_sieve_count", &[py::int(&py::getitem(&v_i.clone(), &Py::str("x"))?)?], &[])?; if py::eq(&t4865, &t4866) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4934(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4982: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_factor(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4982 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4983 = v_r.clone(); Py::Bool({ let t4984 = Py::Bool(true); if py::is(&t4983, &t4984) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4989 = { let t4985 = v_r.clone(); Py::Bool({ let t4986 = Py::Bool(false); if py::is(&t4985, &t4986) { true } else { false } }) }; if t4989.truthy() { t4989 } else { { let t4987 = v_r.clone(); Py::Bool({ let t4988 = Py::None; if py::is(&t4987, &t4988) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5047(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5074: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_proot(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5074 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5075 = v_r.clone(); Py::Bool({ let t5076 = Py::Bool(true); if py::is(&t5075, &t5076) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5081 = { let t5077 = v_r.clone(); Py::Bool({ let t5078 = Py::Bool(false); if py::is(&t5077, &t5078) { true } else { false } }) }; if t5081.truthy() { t5081 } else { { let t5079 = v_r.clone(); Py::Bool({ let t5080 = Py::None; if py::is(&t5079, &t5080) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5119(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pdecode(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5184 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5185 = v_r.clone(); Py::Bool({ let t5186 = Py::Bool(true); if py::is(&t5185, &t5186) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5191 = { let t5187 = v_r.clone(); Py::Bool({ let t5188 = Py::Bool(false); if py::is(&t5187, &t5188) { true } else { false } }) }; if t5191.truthy() { t5191 } else { { let t5189 = v_r.clone(); Py::Bool({ let t5190 = Py::None; if py::is(&t5189, &t5190) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_515(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bip(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t529 = v_r.clone(); Py::Bool({ let t530 = Py::Bool(true); if py::is(&t529, &t530) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t535 = { let t531 = v_r.clone(); Py::Bool({ let t532 = Py::Bool(false); if py::is(&t531, &t532) { true } else { false } }) }; if t535.truthy() { t535 } else { { let t533 = v_r.clone(); Py::Bool({ let t534 = Py::None; if py::is(&t533, &t534) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5243(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5269: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pencode(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5269 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5270 = v_r.clone(); Py::Bool({ let t5271 = Py::Bool(true); if py::is(&t5270, &t5271) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5276 = { let t5272 = v_r.clone(); Py::Bool({ let t5273 = Py::Bool(false); if py::is(&t5272, &t5273) { true } else { false } }) }; if t5276.truthy() { t5276 } else { { let t5274 = v_r.clone(); Py::Bool({ let t5275 = Py::None; if py::is(&t5274, &t5275) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5294(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5300: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5295(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5300 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5301 = v_r.clone(); Py::Bool({ let t5302 = Py::Bool(true); if py::is(&t5301, &t5302) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5307 = { let t5303 = v_r.clone(); Py::Bool({ let t5304 = Py::Bool(false); if py::is(&t5303, &t5304) { true } else { false } }) }; if t5307.truthy() { t5307 } else { { let t5305 = v_r.clone(); Py::Bool({ let t5306 = Py::None; if py::is(&t5305, &t5306) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5295(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5299 = f__iv(py::getitem(&v_o.clone(), &Py::str("m"))?)?; if !t5299.truthy() { t5299 } else { { let t5297 = py::getitem(&v_o.clone(), &Py::str("m"))?; Py::Bool({ let t5298 = py::pow_mod(&py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t5297, &t5298) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_536(mut v_beads: Py, mut v_colours: Py) -> R {
    return Ok(py::call(&{ let v_beads = v_beads.clone(); let v_colours = v_colours.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("bracelets"), py::getitem(&v_r.clone(), &Py::str("bracelets"))?), (Py::str("necklaces"), py::getitem(&v_r.clone(), &Py::str("necklaces"))?)]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_necklaces(v_beads.clone(), v_colours.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5364(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5380: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5365(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5380 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5381 = v_r.clone(); Py::Bool({ let t5382 = Py::Bool(true); if py::is(&t5381, &t5382) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5387 = { let t5383 = v_r.clone(); Py::Bool({ let t5384 = Py::Bool(false); if py::is(&t5383, &t5384) { true } else { false } }) }; if t5387.truthy() { t5387 } else { { let t5385 = v_r.clone(); Py::Bool({ let t5386 = Py::None; if py::is(&t5385, &t5386) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5365(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_q: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_e: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(3).cloned().unwrap_or(Py::None);
            return Ok({ let t5379 = { let t5366 = py::getitem(&v_o.clone(), &Py::str("n"))?; Py::Bool({ let t5367 = py::mul(&v_p.clone(), &v_q.clone())?; if py::eq(&t5366, &t5367) { true } else { false } }) }; if !t5379.truthy() { t5379 } else { { let t5378 = { let t5368 = py::modulo(&py::mul(&v_e.clone(), &py::getitem(&v_o.clone(), &Py::str("d"))?)?, &py::mul(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::sub(&v_q.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t5369 = Py::Int(1i64); if py::eq(&t5368, &t5369) { true } else { false } }) }; if !t5378.truthy() { t5378 } else { { let t5377 = f__iv(py::getitem(&v_o.clone(), &Py::str("m"))?)?; if !t5377.truthy() { t5377 } else { { let t5376 = { let t5371 = Py::Int(0i64); Py::Bool({ let t5372 = py::getitem(&v_o.clone(), &Py::str("m"))?; if py::le(&t5371, &t5372)? { { let t5373 = py::mul(&v_p.clone(), &v_q.clone())?; if py::lt(&t5372, &t5373)? { true } else { false } } } else { false } }) }; if !t5376.truthy() { t5376 } else { { let t5374 = py::pow_mod(&py::getitem(&v_o.clone(), &Py::str("m"))?, &v_e.clone(), &py::mul(&v_p.clone(), &v_q.clone())?)?; Py::Bool({ let t5375 = v_c.clone(); if py::eq(&t5374, &t5375) { true } else { false } }) } } } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("q"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("e"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5411: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5406(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5411 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5412 = v_r.clone(); Py::Bool({ let t5413 = Py::Bool(true); if py::is(&t5412, &t5413) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5418 = { let t5414 = v_r.clone(); Py::Bool({ let t5415 = Py::Bool(false); if py::is(&t5414, &t5415) { true } else { false } }) }; if t5418.truthy() { t5418 } else { { let t5416 = v_r.clone(); Py::Bool({ let t5417 = Py::None; if py::is(&t5416, &t5417) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5406(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5410 = f__iv(py::getitem(&v_o.clone(), &Py::str("c"))?)?; if !t5410.truthy() { t5410 } else { { let t5408 = py::getitem(&v_o.clone(), &Py::str("c"))?; Py::Bool({ let t5409 = py::pow_mod(&py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("e"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t5408, &t5409) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5471(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5487: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5472(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5487 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5488 = v_r.clone(); Py::Bool({ let t5489 = Py::Bool(true); if py::is(&t5488, &t5489) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5494 = { let t5490 = v_r.clone(); Py::Bool({ let t5491 = Py::Bool(false); if py::is(&t5490, &t5491) { true } else { false } }) }; if t5494.truthy() { t5494 } else { { let t5492 = v_r.clone(); Py::Bool({ let t5493 = Py::None; if py::is(&t5492, &t5493) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5472(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_q: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_e: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok({ let t5486 = f__iv(py::getitem(&v_o.clone(), &Py::str("d"))?)?; if !t5486.truthy() { t5486 } else { { let t5485 = { let t5474 = py::getitem(&v_o.clone(), &Py::str("n"))?; Py::Bool({ let t5475 = py::mul(&v_p.clone(), &v_q.clone())?; if py::eq(&t5474, &t5475) { true } else { false } }) }; if !t5485.truthy() { t5485 } else { { let t5484 = { let t5476 = py::getitem(&v_o.clone(), &Py::str("phi"))?; Py::Bool({ let t5477 = py::mul(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::sub(&v_q.clone(), &Py::Int(1i64))?)?; if py::eq(&t5476, &t5477) { true } else { false } }) }; if !t5484.truthy() { t5484 } else { { let t5483 = { let t5478 = Py::Int(0i64); Py::Bool({ let t5479 = py::getitem(&v_o.clone(), &Py::str("d"))?; if py::lt(&t5478, &t5479)? { { let t5480 = py::getitem(&v_o.clone(), &Py::str("phi"))?; if py::lt(&t5479, &t5480)? { true } else { false } } } else { false } }) }; if !t5483.truthy() { t5483 } else { { let t5481 = py::modulo(&py::mul(&v_e.clone(), &py::getitem(&v_o.clone(), &Py::str("d"))?)?, &py::getitem(&v_o.clone(), &Py::str("phi"))?)?; Py::Bool({ let t5482 = Py::Int(1i64); if py::eq(&t5481, &t5482) { true } else { false } }) } } } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("q"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("e"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5512(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5518: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5513(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5518 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5519 = v_r.clone(); Py::Bool({ let t5520 = Py::Bool(true); if py::is(&t5519, &t5520) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5525 = { let t5521 = v_r.clone(); Py::Bool({ let t5522 = Py::Bool(false); if py::is(&t5521, &t5522) { true } else { false } }) }; if t5525.truthy() { t5525 } else { { let t5523 = v_r.clone(); Py::Bool({ let t5524 = Py::None; if py::is(&t5523, &t5524) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5513(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5517 = f__iv(py::getitem(&v_o.clone(), &Py::str("sequences"))?)?; if !t5517.truthy() { t5517 } else { { let t5515 = py::getitem(&v_o.clone(), &Py::str("sequences"))?; Py::Bool({ let t5516 = py::prod(&py::mul(&Py::list(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?]), &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?; if py::eq(&t5515, &t5516) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5568(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5590: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5569(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5590 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5591 = v_r.clone(); Py::Bool({ let t5592 = Py::Bool(true); if py::is(&t5591, &t5592) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5597 = { let t5593 = v_r.clone(); Py::Bool({ let t5594 = Py::Bool(false); if py::is(&t5593, &t5594) { true } else { false } }) }; if t5597.truthy() { t5597 } else { { let t5595 = v_r.clone(); Py::Bool({ let t5596 = Py::None; if py::is(&t5595, &t5596) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5569(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5589 = f__iv(py::getitem(&v_o.clone(), &Py::str("spanning_trees"))?)?; if !t5589.truthy() { t5589 } else { { let t5571 = py::getitem(&v_o.clone(), &Py::str("spanning_trees"))?; Py::Bool({ let t5572 = f__bareiss(f__laplacian_minor(f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?)?)?; if py::eq(&t5571, &t5572) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5655(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5685: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_sqrtmod(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5685 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5686 = v_r.clone(); Py::Bool({ let t5687 = Py::Bool(true); if py::is(&t5686, &t5687) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5692 = { let t5688 = v_r.clone(); Py::Bool({ let t5689 = Py::Bool(false); if py::is(&t5688, &t5689) { true } else { false } }) }; if t5692.truthy() { t5692 } else { { let t5690 = v_r.clone(); Py::Bool({ let t5691 = Py::None; if py::is(&t5690, &t5691) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5734: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_stars(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5734 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5735 = v_r.clone(); Py::Bool({ let t5736 = Py::Bool(true); if py::is(&t5735, &t5736) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5741 = { let t5737 = v_r.clone(); Py::Bool({ let t5738 = Py::Bool(false); if py::is(&t5737, &t5738) { true } else { false } }) }; if t5741.truthy() { t5741 } else { { let t5739 = v_r.clone(); Py::Bool({ let t5740 = Py::None; if py::is(&t5739, &t5740) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5761(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5779: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_s1(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5779 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5780 = v_r.clone(); Py::Bool({ let t5781 = Py::Bool(true); if py::is(&t5780, &t5781) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5786 = { let t5782 = v_r.clone(); Py::Bool({ let t5783 = Py::Bool(false); if py::is(&t5782, &t5783) { true } else { false } }) }; if t5786.truthy() { t5786 } else { { let t5784 = v_r.clone(); Py::Bool({ let t5785 = Py::None; if py::is(&t5784, &t5785) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5816: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_s2(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5816 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5817 = v_r.clone(); Py::Bool({ let t5818 = Py::Bool(true); if py::is(&t5817, &t5818) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5823 = { let t5819 = v_r.clone(); Py::Bool({ let t5820 = Py::Bool(false); if py::is(&t5819, &t5820) { true } else { false } }) }; if t5823.truthy() { t5823 } else { { let t5821 = v_r.clone(); Py::Bool({ let t5822 = Py::None; if py::is(&t5821, &t5822) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5860(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5882: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_scc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5882 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5883 = v_r.clone(); Py::Bool({ let t5884 = Py::Bool(true); if py::is(&t5883, &t5884) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5889 = { let t5885 = v_r.clone(); Py::Bool({ let t5886 = Py::Bool(false); if py::is(&t5885, &t5886) { true } else { false } }) }; if t5889.truthy() { t5889 } else { { let t5887 = v_r.clone(); Py::Bool({ let t5888 = Py::None; if py::is(&t5887, &t5888) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5924(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_topo(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5949 = v_r.clone(); Py::Bool({ let t5950 = Py::Bool(true); if py::is(&t5949, &t5950) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5955 = { let t5951 = v_r.clone(); Py::Bool({ let t5952 = Py::Bool(false); if py::is(&t5951, &t5952) { true } else { false } }) }; if t5955.truthy() { t5955 } else { { let t5953 = v_r.clone(); Py::Bool({ let t5954 = Py::None; if py::is(&t5953, &t5954) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t609: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_neck(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t609 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t610 = v_r.clone(); Py::Bool({ let t611 = Py::Bool(true); if py::is(&t610, &t611) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t616 = { let t612 = v_r.clone(); Py::Bool({ let t613 = Py::Bool(false); if py::is(&t612, &t613) { true } else { false } }) }; if t616.truthy() { t616 } else { { let t614 = v_r.clone(); Py::Bool({ let t615 = Py::None; if py::is(&t614, &t615) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6026(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6044: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_tt(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6044 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6045 = v_r.clone(); Py::Bool({ let t6046 = Py::Bool(true); if py::is(&t6045, &t6046) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6051 = { let t6047 = v_r.clone(); Py::Bool({ let t6048 = Py::Bool(false); if py::is(&t6047, &t6048) { true } else { false } }) }; if t6051.truthy() { t6051 } else { { let t6049 = v_r.clone(); Py::Bool({ let t6050 = Py::None; if py::is(&t6049, &t6050) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6122(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_twosq(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6158 = v_r.clone(); Py::Bool({ let t6159 = Py::Bool(true); if py::is(&t6158, &t6159) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6164 = { let t6160 = v_r.clone(); Py::Bool({ let t6161 = Py::Bool(false); if py::is(&t6160, &t6161) { true } else { false } }) }; if t6164.truthy() { t6164 } else { { let t6162 = v_r.clone(); Py::Bool({ let t6163 = Py::None; if py::is(&t6162, &t6163) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6188(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6204: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_twos(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6204 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6205 = v_r.clone(); Py::Bool({ let t6206 = Py::Bool(true); if py::is(&t6205, &t6206) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6211 = { let t6207 = v_r.clone(); Py::Bool({ let t6208 = Py::Bool(false); if py::is(&t6207, &t6208) { true } else { false } }) }; if t6211.truthy() { t6211 } else { { let t6209 = v_r.clone(); Py::Bool({ let t6210 = Py::None; if py::is(&t6209, &t6210) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6236(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6278: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_venn3(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6278 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6279 = v_r.clone(); Py::Bool({ let t6280 = Py::Bool(true); if py::is(&t6279, &t6280) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6285 = { let t6281 = v_r.clone(); Py::Bool({ let t6282 = Py::Bool(false); if py::is(&t6281, &t6282) { true } else { false } }) }; if t6285.truthy() { t6285 } else { { let t6283 = v_r.clone(); Py::Bool({ let t6284 = Py::None; if py::is(&t6283, &t6284) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6310(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6335: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_venn2(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6335 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6336 = v_r.clone(); Py::Bool({ let t6337 = Py::Bool(true); if py::is(&t6336, &t6337) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6342 = { let t6338 = v_r.clone(); Py::Bool({ let t6339 = Py::Bool(false); if py::is(&t6338, &t6339) { true } else { false } }) }; if t6342.truthy() { t6342 } else { { let t6340 = v_r.clone(); Py::Bool({ let t6341 = Py::None; if py::is(&t6340, &t6341) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6354(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6367: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_6355(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6367 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6368 = v_r.clone(); Py::Bool({ let t6369 = Py::Bool(true); if py::is(&t6368, &t6369) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6374 = { let t6370 = v_r.clone(); Py::Bool({ let t6371 = Py::Bool(false); if py::is(&t6370, &t6371) { true } else { false } }) }; if t6374.truthy() { t6374 } else { { let t6372 = v_r.clone(); Py::Bool({ let t6373 = Py::None; if py::is(&t6372, &t6373) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6355(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t6366 = { let t6356 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t6357 = py::getitem(&f_vigenere(py::getitem(&v_i.clone(), &Py::str("message"))?, py::getitem(&v_i.clone(), &Py::str("key"))?, py::getitem(&v_i.clone(), &Py::str("mode"))?)?, &Py::str("result"))?; if py::eq(&t6356, &t6357) { true } else { false } }) }; if !t6366.truthy() { t6366 } else { { let t6358 = py::getitem(&f_vigenere(py::getitem(&v_o.clone(), &Py::str("result"))?, py::getitem(&v_i.clone(), &Py::str("key"))?, (if ({ let t6360 = py::getitem(&v_i.clone(), &Py::str("mode"))?; Py::Bool({ let t6361 = Py::str("encrypt"); if py::eq(&t6360, &t6361) { true } else { false } }) }).truthy() { Py::str("decrypt") } else { Py::str("encrypt") }))?, &Py::str("result"))?; Py::Bool({ let t6359 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6362 in py::iter(&(py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("message"))?)), "upper", &[])?))? { let mut v_c: Py; 
v_c = t6362;
if ({ let t6363 = Py::str("A"); Py::Bool({ let t6364 = v_c.clone(); if py::le(&t6363, &t6364)? { { let t6365 = Py::str("Z"); if py::le(&t6364, &t6365)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) }])?; if py::eq(&t6358, &t6359) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6452(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6505: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_vote(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6505 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6506 = v_r.clone(); Py::Bool({ let t6507 = Py::Bool(true); if py::is(&t6506, &t6507) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6512 = { let t6508 = v_r.clone(); Py::Bool({ let t6509 = Py::Bool(false); if py::is(&t6508, &t6509) { true } else { false } }) }; if t6512.truthy() { t6512 } else { { let t6510 = v_r.clone(); Py::Bool({ let t6511 = Py::None; if py::is(&t6510, &t6511) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6553(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6562: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_walks(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6562 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6563 = v_r.clone(); Py::Bool({ let t6564 = Py::Bool(true); if py::is(&t6563, &t6564) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6569 = { let t6565 = v_r.clone(); Py::Bool({ let t6566 = Py::Bool(false); if py::is(&t6565, &t6566) { true } else { false } }) }; if t6569.truthy() { t6569 } else { { let t6567 = v_r.clone(); Py::Bool({ let t6568 = Py::None; if py::is(&t6567, &t6568) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6577(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6590: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_word(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6590 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6591 = v_r.clone(); Py::Bool({ let t6592 = Py::Bool(true); if py::is(&t6591, &t6592) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6597 = { let t6593 = v_r.clone(); Py::Bool({ let t6594 = Py::Bool(false); if py::is(&t6593, &t6594) { true } else { false } }) }; if t6597.truthy() { t6597 } else { { let t6595 = v_r.clone(); Py::Bool({ let t6596 = Py::None; if py::is(&t6595, &t6596) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_661(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t691: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_br(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t691 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t692 = v_r.clone(); Py::Bool({ let t693 = Py::Bool(true); if py::is(&t692, &t693) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t698 = { let t694 = v_r.clone(); Py::Bool({ let t695 = Py::Bool(false); if py::is(&t694, &t695) { true } else { false } }) }; if t698.truthy() { t698 } else { { let t696 = v_r.clone(); Py::Bool({ let t697 = Py::None; if py::is(&t696, &t697) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_720(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t733: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_721(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t733 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t734 = v_r.clone(); Py::Bool({ let t735 = Py::Bool(true); if py::is(&t734, &t735) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t740 = { let t736 = v_r.clone(); Py::Bool({ let t737 = Py::Bool(false); if py::is(&t736, &t737) { true } else { false } }) }; if t740.truthy() { t740 } else { { let t738 = v_r.clone(); Py::Bool({ let t739 = Py::None; if py::is(&t738, &t739) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_721(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t732 = { let t722 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t723 = py::getitem(&f_caesar(py::getitem(&v_i.clone(), &Py::str("message"))?, py::getitem(&v_i.clone(), &Py::str("shift"))?, py::getitem(&v_i.clone(), &Py::str("mode"))?)?, &Py::str("result"))?; if py::eq(&t722, &t723) { true } else { false } }) }; if !t732.truthy() { t732 } else { { let t724 = py::getitem(&f_caesar(py::getitem(&v_o.clone(), &Py::str("result"))?, py::getitem(&v_i.clone(), &Py::str("shift"))?, (if ({ let t726 = py::getitem(&v_i.clone(), &Py::str("mode"))?; Py::Bool({ let t727 = Py::str("encrypt"); if py::eq(&t726, &t727) { true } else { false } }) }).truthy() { Py::str("decrypt") } else { Py::str("encrypt") }))?, &Py::str("result"))?; Py::Bool({ let t725 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t728 in py::iter(&(py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("message"))?)), "upper", &[])?))? { let mut v_c: Py; 
v_c = t728;
if ({ let t729 = Py::str("A"); Py::Bool({ let t730 = v_c.clone(); if py::le(&t729, &t730)? { { let t731 = Py::str("Z"); if py::le(&t730, &t731)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) }])?; if py::eq(&t724, &t725) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t835: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_carm(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t835 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t836 = v_r.clone(); Py::Bool({ let t837 = Py::Bool(true); if py::is(&t836, &t837) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t842 = { let t838 = v_r.clone(); Py::Bool({ let t839 = Py::Bool(false); if py::is(&t838, &t839) { true } else { false } }) }; if t842.truthy() { t842 } else { { let t840 = v_r.clone(); Py::Bool({ let t841 = Py::None; if py::is(&t840, &t841) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t863: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_catalan(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t863 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t864 = v_r.clone(); Py::Bool({ let t865 = Py::Bool(true); if py::is(&t864, &t865) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t870 = { let t866 = v_r.clone(); Py::Bool({ let t867 = Py::Bool(false); if py::is(&t866, &t867) { true } else { false } }) }; if t870.truthy() { t870 } else { { let t868 = v_r.clone(); Py::Bool({ let t869 = Py::None; if py::is(&t868, &t869) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_887(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_cfr(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t907 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t908 = v_r.clone(); Py::Bool({ let t909 = Py::Bool(true); if py::is(&t908, &t909) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t914 = { let t910 = v_r.clone(); Py::Bool({ let t911 = Py::Bool(false); if py::is(&t910, &t911) { true } else { false } }) }; if t914.truthy() { t914 } else { { let t912 = v_r.clone(); Py::Bool({ let t913 = Py::None; if py::is(&t912, &t913) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_934(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t956: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_cfs(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t956 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t957 = v_r.clone(); Py::Bool({ let t958 = Py::Bool(true); if py::is(&t957, &t958) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t963 = { let t959 = v_r.clone(); Py::Bool({ let t960 = Py::Bool(false); if py::is(&t959, &t960) { true } else { false } }) }; if t963.truthy() { t963 } else { { let t961 = v_r.clone(); Py::Bool({ let t962 = Py::None; if py::is(&t961, &t962) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_995(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1016: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_chrom(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1016 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1017 = v_r.clone(); Py::Bool({ let t1018 = Py::Bool(true); if py::is(&t1017, &t1018) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1023 = { let t1019 = v_r.clone(); Py::Bool({ let t1020 = Py::Bool(false); if py::is(&t1019, &t1020) { true } else { false } }) }; if t1023.truthy() { t1023 } else { { let t1021 = v_r.clone(); Py::Bool({ let t1022 = Py::None; if py::is(&t1021, &t1022) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
