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
    "vigenere_cipher",
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
        "articulation_points" => {
            let r = f_articulation_points(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_98(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "base_convert" => {
            let r = f_base_convert(match v.get("digits") { Some(x) => py::from_val(x), None => { return None } }, match v.get("from_base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("to_base") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_168(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bell" => {
            let r = f_bell(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_215(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bernoulli" => {
            let r = f_bernoulli(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bfs_order" => {
            let r = f_bfs_order(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_296(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_term" => {
            let r = f_binomial_term(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_336(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bipartite" => {
            let r = f_bipartite(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_378(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bracelets" => {
            let r = s_399(match v.get("beads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("colours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_461(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bridges" => {
            let r = f_bridges(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_524(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "caesar_cipher" => {
            let r = f_caesar(match v.get("message") { Some(x) => py::from_val(x), None => { return None } }, match v.get("shift") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mode") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "carmichael_lambda" => {
            let r = f_carmichael_lambda(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_666(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "catalan" => {
            let r = f_catalan(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_720(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cf_rational" => {
            let r = f_cf_rational(match v.get("numerator") { Some(x) => py::from_val(x), None => { return None } }, match v.get("denominator") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_750(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cf_sqrt" => {
            let r = f_cf_sqrt(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_797(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chromatic_number" => {
            let r = f_chromatic_number(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clique_number" => {
            let r = f_clique_number(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_924(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combinations" => {
            let r = f_combinations(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "connected_components" => {
            let r = f_connected_components(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crt" => {
            let r = f_crt(match v.get("residues") { Some(x) => py::from_val(x), None => { return None } }, match v.get("moduli") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1086(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cycle_rank" => {
            let r = f_cycle_rank(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1142(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "derangements" => {
            let r = f_derangements(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1186(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dfs_order" => {
            let r = f_dfs_order(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1223(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diffie_hellman" => {
            let r = f_diffie_hellman(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1284(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "directed_cycle" => {
            let r = f_directed_cycle(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1339(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "discrete_log" => {
            let r = f_discrete_log(match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("modulus") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1393(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "divisor_functions" => {
            let r = f_divisor_functions(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elgamal_encrypt" => {
            let r = f_elgamal_encrypt(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1562(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "euler_phi" => {
            let r = f_euler_phi(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1636(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eulerian" => {
            let r = f_eulerian(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1692(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "extended_gcd" => {
            let r = f_extended_gcd(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1733(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fibonacci" => {
            let r = f_fibonacci(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1774(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gcd_lcm" => {
            let r = f_gcd_lcm(match v.get("numbers") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "girth" => {
            let r = f_girth(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1887(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_degrees" => {
            let r = f_graph_degrees(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1934(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_diameter" => {
            let r = f_graph_diameter(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2004(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_triangles" => {
            let r = f_graph_triangles(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2073(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graphical_sequence" => {
            let r = f_graphical_sequence(match v.get("degrees") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hamiltonian_cycle" => {
            let r = f_hamiltonian_cycle(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inclusion_exclusion" => {
            let r = f_inclusion_exclusion(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("divisors") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2247(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "independence_number" => {
            let r = f_independence_number(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2308(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "integer_partitions" => {
            let r = f_integer_partitions(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "integer_sqrt" => {
            let r = f_integer_sqrt(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2392(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "is_prime" => {
            let r = f_is_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "is_tree" => {
            let r = f_is_tree(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2547(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "jacobi_symbol" => {
            let r = f_jacobi_symbol(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2610(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lattice_paths" => {
            let r = f_lattice_paths(match v.get("right") { Some(x) => py::from_val(x), None => { return None } }, match v.get("up") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2681(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "legendre_symbol" => {
            let r = f_legendre_symbol(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2747(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_congruence" => {
            let r = f_linear_congruence(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2785(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_diophantine" => {
            let r = f_linear_diophantine(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2844(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_contradiction" => {
            let r = f_logic_contradiction(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2952(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_equivalence" => {
            let r = f_logic_equivalence(match v.get("formula1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3047(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_evaluate" => {
            let r = f_logic_evaluate(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("assignment") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3135(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_satisfiable" => {
            let r = f_logic_satisfiable(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logic_tautology" => {
            let r = f_logic_tautology(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3319(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lucas" => {
            let r = f_lucas(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimal_cnf" => {
            let r = f_minimal_cnf(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimal_dnf" => {
            let r = f_minimal_dnf(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimal_sop_minterms" => {
            let r = f_minimal_sop_minterms(match v.get("minterms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dont_cares") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3823(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mobius" => {
            let r = f_mobius(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3943(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mod_inverse" => {
            let r = f_mod_inverse(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3995(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mod_pow" => {
            let r = f_mod_pow(match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("exponent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("modulus") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4029(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mod_reduce" => {
            let r = f_mod_reduce(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4059(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multinomial" => {
            let r = f_multinomial(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multiplicative_order" => {
            let r = f_multiplicative_order(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4196(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multisets" => {
            let r = f_multisets(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4243(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "necklaces" => {
            let r = f_necklaces(match v.get("beads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("colours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4318(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "next_prime" => {
            let r = f_next_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4373(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nth_prime" => {
            let r = f_nth_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partitions_distinct" => {
            let r = f_partitions_distinct(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4453(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partitions_k" => {
            let r = f_partitions_k(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4492(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pell_equation" => {
            let r = f_pell(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4539(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "permutations" => {
            let r = f_permutations(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pigeonhole_max" => {
            let r = f_pigeonhole_max(match v.get("items") { Some(x) => py::from_val(x), None => { return None } }, match v.get("boxes") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4612(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pigeonhole_needed" => {
            let r = f_pigeonhole_needed(match v.get("boxes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at_least") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4642(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "previous_prime" => {
            let r = f_previous_prime(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4693(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prime_counting" => {
            let r = f_prime_counting(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4725(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prime_factorization" => {
            let r = f_prime_factorization(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4797(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "primitive_root" => {
            let r = f_primitive_root(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4910(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prufer_decode" => {
            let r = f_prufer_decode(match v.get("sequence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4982(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "prufer_encode" => {
            let r = f_prufer_encode(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5106(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_decrypt" => {
            let r = f_rsa_decrypt(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5157(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_decrypt_pq" => {
            let r = f_rsa_decrypt_pq(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5227(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_encrypt" => {
            let r = f_rsa_encrypt(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5268(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rsa_keygen" => {
            let r = f_rsa_keygen(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5334(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sequences_with_repetition" => {
            let r = f_sequences_with_repetition(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5375(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spanning_tree_count" => {
            let r = f_spanning_tree_count(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5431(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sqrt_mod" => {
            let r = f_sqrt_mod(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5518(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stars_and_bars" => {
            let r = f_stars_and_bars(match v.get("total") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lower") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("upper") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stirling1" => {
            let r = f_stirling1(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5624(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stirling2" => {
            let r = f_stirling2(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5669(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "strongly_connected" => {
            let r = f_strongly_connected(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5723(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "topological_sort" => {
            let r = f_topological_sort(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5787(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "truth_table" => {
            let r = f_truth_table(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5889(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_squares" => {
            let r = f_two_squares(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "twos_complement" => {
            let r = f_twos_complement(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6051(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vigenere_cipher" => {
            let r = f_vigenere(match v.get("message") { Some(x) => py::from_val(x), None => { return None } }, match v.get("key") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mode") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6086(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "walks_count" => {
            let r = f_walks_count(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("length") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6147(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "word_arrangements" => {
            let r = f_word_arrangements(match v.get("word") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6171(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__assign(mut v_r: Py, mut v_names: Py) -> R {
    return Ok({ let mut out = vec![]; for t3317 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t3318 = (t3317).items()?;
if t3318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3318[0].clone();
v_v = t3318[1].clone();
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
    v_M = { let mut out = vec![]; for t5436 in py::iter(&(v_M.clone()))? { let mut v_row: Py; 
v_row = t5436;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    let t5437 = (Py::tuple(vec![py::len(&v_M.clone())?, Py::Int(1i64), Py::Int(1i64)])).items()?;
    if t5437.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5437[0].clone();
    v_sign = t5437[1].clone();
    v_prev = t5437[2].clone();
    if ({ let t5438 = v_n.clone(); Py::Bool({ let t5439 = Py::Int(0i64); if py::eq(&t5438, &t5439) { true } else { false } }) }).truthy() {
        return Ok(Py::Int(1i64));
    }
    for t5440 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t5440;
        if ({ let t5441 = py::getitem(&py::getitem(&v_M.clone(), &v_k.clone())?, &v_k.clone())?; Py::Bool({ let t5442 = Py::Int(0i64); if py::eq(&t5441, &t5442) { true } else { false } }) }).truthy() {
            v_sw = (match ('g5443: { let mut out = vec![]; for t5444 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5444;
if ({ let t5445 = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?; Py::Bool({ let t5446 = Py::Int(0i64); if py::ne(&t5445, &t5446) { true } else { false } }) }).truthy() { out.push(v_r.clone()); break 'g5443 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
            if ({ let t5447 = v_sw.clone(); Py::Bool({ let t5448 = Py::None; if py::is(&t5447, &t5448) { true } else { false } }) }).truthy() {
                return Ok(Py::Int(0i64));
            }
            let t5449 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_sw.clone())?, py::getitem(&v_M.clone(), &v_k.clone())?])).items()?;
            if t5449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_M.clone(), &v_k.clone(), t5449[0].clone())?;
            py::setitem(&v_M.clone(), &v_sw.clone(), t5449[1].clone())?;
            v_sign = py::neg(&v_sign.clone())?;
        }
        for t5450 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t5450;
            for t5451 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_j = t5451;
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
    let t1993 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), Py::Int(0i64))]), Py::list(vec![v_s.clone()]), Py::list((Py::list(vec![v_s.clone()])).items()?)])).items()?;
    if t1993.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dist = t1993[0].clone();
    v_order = t1993[1].clone();
    v_dq = t1993[2].clone();
    while (v_dq.clone()).truthy() {
        v_u = py::method(&v_dq.clone(), "popleft", &[])?;
        for t1994 in py::iter(&(py::sorted_key(&py::getitem(&v_adj.clone(), &v_u.clone())?, &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (v_desc.clone()).truthy())?))? {
            v_v = t1994;
            if ({ let t1995 = v_v.clone(); Py::Bool({ let t1996 = v_dist.clone(); if !py::contains(&t1996, &t1995)? { true } else { false } }) }).truthy() {
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
    let rec2324 = py::rec_cell();
    v_go = { let v_nodes = v_nodes.clone(); let v_ok = v_ok.clone(); let c_best = c_best.clone(); let rec2324 = rec2324.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_chosen: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_go: Py = py::rec_get(&rec2324)?;
            let mut v_best: Py = Py::None;
            let mut v_j: Py = Py::None;
            py::cell_set(&c_best, py::minmax(&[py::cell_get(&c_best)?, py::len(&v_chosen.clone())?], true)?);
            if ({ let t2325 = py::sub(&py::add(&py::len(&v_chosen.clone())?, &py::len(&v_nodes.clone())?)?, &v_i.clone())?; Py::Bool({ let t2326 = py::cell_get(&c_best)?; if py::le(&t2325, &t2326)? { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            for t2327 in py::range_iter(&[(v_i.clone()).i()?, (py::len(&v_nodes.clone())?).i()?]) {
                v_j = t2327;
                if (py::call(&v_ok.clone(), &[v_chosen.clone(), py::getitem(&v_nodes.clone(), &v_j.clone())?])?).truthy() {
                    let _ = py::call(&v_go.clone(), &[py::add(&v_j.clone(), &Py::Int(1i64))?, py::add(&v_chosen.clone(), &Py::list(vec![py::getitem(&v_nodes.clone(), &v_j.clone())?]))?])?;
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2324, &v_go);
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
    for t4144 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? {
        let t4151 = (t4144).items()?;
        if t4151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t4151[0].clone();
        v_e = t4151[1].clone();
        v_l = (if ({ let t4145 = v_p.clone(); Py::Bool({ let t4146 = Py::Int(2i64); if py::eq(&t4145, &t4146) { true } else { false } }) }).truthy() { (if ({ let t4147 = v_e.clone(); Py::Bool({ let t4148 = Py::Int(3i64); if py::ge(&t4147, &t4148)? { true } else { false } }) }).truthy() { py::pow(&Py::Int(2i64), &py::sub(&v_e.clone(), &Py::Int(2i64))?)? } else { (if ({ let t4149 = v_e.clone(); Py::Bool({ let t4150 = Py::Int(1i64); if py::eq(&t4149, &t4150) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) }) } else { py::mul(&py::pow(&v_p.clone(), &py::sub(&v_e.clone(), &Py::Int(1i64))?)?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)? });
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
            return Ok({ let mut out = vec![]; for t3846 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3846;
if (f__covers(v_c.clone(), v_m.clone())?).truthy() { out.push(v_m.clone()); }} Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t3847 in py::iter(&(v_cubes.clone()))? {
        v_c = t3847;
        if (py::not(&Py::Bool(({ let mut out = vec![]; for t3848 in py::iter(&(py::call(&v_pts.clone(), &[v_c.clone()])?))? { let mut v_m: Py; 
v_m = t3848;
out.push({ let t3849 = v_m.clone(); Py::Bool({ let t3850 = v_care.clone(); if py::contains(&t3850, &t3849)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
            return Ok(Py::Bool(false));
        }
        for t3851 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_b = t3851;
            v_bit = py::lshift(&Py::Int(1i64), &v_b.clone())?;
            if ({ let t3855 = py::not(&py::bitand(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_bit.clone())?); if !t3855.truthy() { t3855 } else { Py::Bool(({ let mut out = vec![]; for t3852 in py::iter(&(py::call(&v_pts.clone(), &[Py::tuple(vec![py::bitand(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::invert(&v_bit.clone())?)?, py::bitor(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_bit.clone())?])])?))? { let mut v_m: Py; 
v_m = t3852;
out.push({ let t3853 = v_m.clone(); Py::Bool({ let t3854 = v_care.clone(); if py::contains(&t3854, &t3853)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }).truthy() {
                return Ok(Py::Bool(false));
            }
        }
    }
    if (py::sub(&py::set(&v_on.clone())?, &py::set(&{ let mut out = vec![]; for t3856 in py::iter(&(v_cubes.clone()))? { let mut v_c: Py; 
v_c = t3856;
for t3857 in py::iter(&(py::call(&v_pts.clone(), &[v_c.clone()])?))? { let mut v_m: Py; 
v_m = t3857;
out.push(v_m.clone()); }} Py::list(out) })?)?).truthy() {
        return Ok(Py::Bool(false));
    }
    for t3858 in py::iter(&(py::enumerate(&v_cubes.clone(), 0)?))? {
        let t3866 = (t3858).items()?;
        if t3866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3866[0].clone();
        v_c = t3866[1].clone();
        v_others = py::set(&{ let mut out = vec![]; for t3859 in py::iter(&(py::enumerate(&v_cubes.clone(), 0)?))? { let mut v_d: Py; let mut v_j: Py; 
let t3860 = (t3859).items()?;
if t3860.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t3860[0].clone();
v_d = t3860[1].clone();
if ({ let t3861 = v_j.clone(); Py::Bool({ let t3862 = v_k.clone(); if py::ne(&t3861, &t3862) { true } else { false } }) }).truthy() { for t3863 in py::iter(&(py::call(&v_pts.clone(), &[v_d.clone()])?))? { let mut v_m: Py; 
v_m = t3863;
out.push(v_m.clone()); }}} Py::list(out) })?;
        if ({ let t3864 = py::bitand(&py::set(&py::call(&v_pts.clone(), &[v_c.clone()])?)?, &py::set(&v_on.clone())?)?; Py::Bool({ let t3865 = v_others.clone(); if py::le(&t3864, &t3865)? { true } else { false } }) }).truthy() {
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
    let rec848 = py::rec_cell();
    v_place = { let v_adj = v_adj.clone(); let v_col = v_col.clone(); let v_k = v_k.clone(); let v_nodes = v_nodes.clone(); let rec848 = rec848.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_place: Py = py::rec_get(&rec848)?;
            let mut v_c: Py = Py::None;
            let mut v_used: Py = Py::None;
            let mut v_v: Py = Py::None;
            if ({ let t849 = v_i.clone(); Py::Bool({ let t850 = py::len(&v_nodes.clone())?; if py::eq(&t849, &t850) { true } else { false } }) }).truthy() {
                return Ok(Py::Bool(true));
            }
            v_v = py::getitem(&v_nodes.clone(), &v_i.clone())?;
            v_used = py::set(&{ let mut out = vec![]; for t851 in py::iter(&(py::getitem(&v_adj.clone(), &v_v.clone())?))? { let mut v_u: Py; 
v_u = t851;
if ({ let t852 = v_u.clone(); Py::Bool({ let t853 = v_col.clone(); if py::contains(&t853, &t852)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_col.clone(), &v_u.clone())?); }} Py::list(out) })?;
            for t854 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
                v_c = t854;
                if ({ let t855 = v_c.clone(); Py::Bool({ let t856 = v_used.clone(); if !py::contains(&t856, &t855)? { true } else { false } }) }).truthy() {
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
    py::rec_set(&rec848, &v_place);
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
    let t5084 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, Py::list(vec![])])).items()?;
    if t5084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_seen = t5084[0].clone();
    v_comps = t5084[1].clone();
    for t5085 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_s = t5085;
        if ({ let t5086 = v_s.clone(); Py::Bool({ let t5087 = v_seen.clone(); if py::contains(&t5087, &t5086)? { true } else { false } }) }).truthy() {
            continue;
        }
        let t5088 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![v_s.clone()])])).items()?;
        if t5088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_comp = t5088[0].clone();
        v_stack = t5088[1].clone();
        let _ = py::method(&v_seen.clone(), "add", &[v_s.clone()])?;
        while (v_stack.clone()).truthy() {
            v_u = py::method(&v_stack.clone(), "pop", &[])?;
            let _ = py::method(&v_comp.clone(), "append", &[v_u.clone()])?;
            for t5089 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t5089;
                if ({ let t5090 = v_v.clone(); Py::Bool({ let t5091 = v_seen.clone(); if !py::contains(&t5091, &t5090)? { true } else { false } }) }).truthy() {
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
    let t4533 = (Py::tuple(vec![Py::Int(1i64), py::getitem(&v_terms.clone(), &Py::Int(0i64))?, Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t4533.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t4533[0].clone();
    v_h1 = t4533[1].clone();
    v_k0 = t4533[2].clone();
    v_k1 = t4533[3].clone();
    for t4534 in py::iter(&(py::slice(&v_terms.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_a = t4534;
        let t4535 = (Py::tuple(vec![v_h1.clone(), py::add(&py::mul(&v_a.clone(), &v_h1.clone())?, &v_h0.clone())?])).items()?;
        if t4535.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h0 = t4535[0].clone();
        v_h1 = t4535[1].clone();
        let t4536 = (Py::tuple(vec![v_k1.clone(), py::add(&py::mul(&v_a.clone(), &v_k1.clone())?, &v_k0.clone())?])).items()?;
        if t4536.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k0 = t4536[0].clone();
        v_k1 = t4536[1].clone();
    }
    return Ok(Py::tuple(vec![v_h1.clone(), v_k1.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__covers(mut v_p: Py, mut v_m: Py) -> R {
    return Ok({ let t3781 = py::bitand(&v_m.clone(), &py::invert(&py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t3782 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; if py::eq(&t3781, &t3782) { true } else { false } }) });
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
    let t3841 = (Py::tuple(vec![py::len(&v_names.clone())?, Py::Int(0i64), Py::Int(0i64)])).items()?;
    if t3841.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3841[0].clone();
    v_val = t3841[1].clone();
    v_fixed = t3841[2].clone();
    for t3843 in py::iter(&({ let mut out = vec![]; for t3842 in py::iter(&(py::method(&v_lits_text.clone(), "split", &[Py::str("&")])?))? { let mut v_x: Py; 
v_x = t3842;
out.push(py::method(&v_x.clone(), "strip", &[])?); } Py::list(out) }))? {
        v_lit = t3843;
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
    v_deg = { let mut out = vec![]; for t5117 in py::range_iter(&[(v_base.clone()).i()?, (py::add(&v_n.clone(), &v_base.clone())?).i()?]) { let mut v_v: Py; 
v_v = t5117;
out.push((v_v.clone(), Py::Int(1i64))); } Py::dict(out) };
    for t5118 in py::iter(&(v_seq.clone()))? {
        v_s = t5118;
        let t5119 = v_deg.clone(); let t5120 = v_s.clone();
        py::setitem(&t5119, &t5120, py::iadd(&py::getitem(&t5119, &t5120)?, &Py::Int(1i64))?)?;
    }
    v_leaves = { let mut out = vec![]; for t5121 in py::iter(&(v_deg.clone()))? { let mut v_v: Py; 
v_v = t5121;
if ({ let t5122 = py::getitem(&v_deg.clone(), &v_v.clone())?; Py::Bool({ let t5123 = Py::Int(1i64); if py::eq(&t5122, &t5123) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    let _ = py::heapq("heapify", &[v_leaves.clone()])?;
    v_out = Py::list(vec![]);
    for t5124 in py::iter(&(v_seq.clone()))? {
        v_s = t5124;
        v_leaf = py::heapq("heappop", &[v_leaves.clone()])?;
        let _ = py::method(&v_out.clone(), "append", &[py::sorted(&Py::tuple(vec![v_leaf.clone(), v_s.clone()]), (Py::Bool(false)).truthy())?])?;
        let t5125 = v_deg.clone(); let t5126 = v_s.clone();
        py::setitem(&t5125, &t5126, py::sub(&py::getitem(&t5125, &t5126)?, &Py::Int(1i64))?)?;
        if ({ let t5127 = py::getitem(&v_deg.clone(), &v_s.clone())?; Py::Bool({ let t5128 = Py::Int(1i64); if py::eq(&t5127, &t5128) { true } else { false } }) }).truthy() {
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
    for t5936 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? {
        let t5939 = (t5936).items()?;
        if t5939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t5939[0].clone();
        v_e = t5939[1].clone();
        v_ds = { let mut out = vec![]; for t5937 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t5937;
for t5938 in py::range_iter(&[(py::add(&v_e.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t5938;
out.push(py::mul(&v_d.clone(), &py::pow(&v_p.clone(), &v_k.clone())?)?); }} Py::list(out) };
    }
    return Ok(py::sorted(&v_ds.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__euclid(mut v_a: Py, mut v_b: Py) -> R {
    while (v_b.clone()).truthy() {
        let t1816 = (Py::tuple(vec![v_b.clone(), py::modulo(&v_a.clone(), &v_b.clone())?])).items()?;
        if t1816.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1816[0].clone();
        v_b = t1816[1].clone();
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
    if ({ let t5871 = v_op.clone(); Py::Bool({ let t5872 = Py::str("VAR"); if py::eq(&t5871, &t5872) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_env.clone(), &py::getitem(&v_t.clone(), &Py::Int(1i64))?)?);
    }
    if ({ let t5873 = v_op.clone(); Py::Bool({ let t5874 = Py::str("CONST"); if py::eq(&t5873, &t5874) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_t.clone(), &Py::Int(1i64))?);
    }
    if ({ let t5875 = v_op.clone(); Py::Bool({ let t5876 = Py::str("NOT"); if py::eq(&t5875, &t5876) { true } else { false } }) }).truthy() {
        return Ok(py::not(&f__ev(py::getitem(&v_t.clone(), &Py::Int(1i64))?, v_env.clone())?));
    }
    let t5877 = (Py::tuple(vec![f__ev(py::getitem(&v_t.clone(), &Py::Int(1i64))?, v_env.clone())?, f__ev(py::getitem(&v_t.clone(), &Py::Int(2i64))?, v_env.clone())?])).items()?;
    if t5877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5877[0].clone();
    v_b = t5877[1].clone();
    return Ok(py::getitem(&Py::dict(vec![(Py::str("AND"), { let t5878 = v_a.clone(); if !t5878.truthy() { t5878 } else { v_b.clone() } }), (Py::str("OR"), { let t5879 = v_a.clone(); if t5879.truthy() { t5879 } else { v_b.clone() } }), (Py::str("XOR"), { let t5880 = v_a.clone(); Py::Bool({ let t5881 = v_b.clone(); if py::ne(&t5880, &t5881) { true } else { false } }) }), (Py::str("IMP"), { let t5882 = py::not(&v_a.clone()); if t5882.truthy() { t5882 } else { v_b.clone() } }), (Py::str("IFF"), { let t5883 = v_a.clone(); Py::Bool({ let t5884 = v_b.clone(); if py::eq(&t5883, &t5884) { true } else { false } }) })]), &v_op.clone())?);
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
    let t5327 = (Py::tuple(vec![v_a.clone(), v_b.clone(), Py::Int(1i64), Py::Int(0i64), Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t5327.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_old_r = t5327[0].clone();
    v_r = t5327[1].clone();
    v_old_s = t5327[2].clone();
    v_s = t5327[3].clone();
    v_old_t = t5327[4].clone();
    v_t = t5327[5].clone();
    while (v_r.clone()).truthy() {
        v_q = py::floordiv(&v_old_r.clone(), &v_r.clone())?;
        let t5328 = (Py::tuple(vec![v_r.clone(), py::sub(&v_old_r.clone(), &py::mul(&v_q.clone(), &v_r.clone())?)?])).items()?;
        if t5328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_old_r = t5328[0].clone();
        v_r = t5328[1].clone();
        let t5329 = (Py::tuple(vec![v_s.clone(), py::sub(&v_old_s.clone(), &py::mul(&v_q.clone(), &v_s.clone())?)?])).items()?;
        if t5329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_old_s = t5329[0].clone();
        v_s = t5329[1].clone();
        let t5330 = (Py::tuple(vec![v_t.clone(), py::sub(&v_old_t.clone(), &py::mul(&v_q.clone(), &v_t.clone())?)?])).items()?;
        if t5330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_old_t = t5330[0].clone();
        v_t = t5330[1].clone();
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
    for t5940 in py::iter(&(Py::tuple(vec![Py::Int(2i64), Py::Int(3i64), Py::Int(5i64), Py::Int(7i64), Py::Int(11i64), Py::Int(13i64), Py::Int(17i64), Py::Int(19i64), Py::Int(23i64), Py::Int(29i64), Py::Int(31i64), Py::Int(37i64), Py::Int(41i64), Py::Int(43i64), Py::Int(47i64)])))? {
        v_p = t5940;
        while ({ let t5941 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t5942 = Py::Int(0i64); if py::eq(&t5941, &t5942) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &v_p.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_p.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
            v_n = py::floordiv(&v_n, &v_p.clone())?;
        }
    }
    v_stack = (if ({ let t5943 = v_n.clone(); Py::Bool({ let t5944 = Py::Int(1i64); if py::gt(&t5943, &t5944)? { true } else { false } }) }).truthy() { Py::list(vec![v_n.clone()]) } else { Py::list(vec![]) });
    while (v_stack.clone()).truthy() {
        v_m = py::method(&v_stack.clone(), "pop", &[])?;
        if ({ let t5945 = v_m.clone(); Py::Bool({ let t5946 = Py::Int(1i64); if py::eq(&t5945, &t5946) { true } else { false } }) }).truthy() {
            continue;
        }
        if (f__is_prime(v_m.clone())?).truthy() {
            py::setitem(&v_out.clone(), &v_m.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_m.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
            continue;
        }
        v_r = py::isqrt(&v_m.clone())?;
        if ({ let t5969 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t5970 = v_m.clone(); if py::eq(&t5969, &t5970) { true } else { false } }) }).truthy() {
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
    if ({ let t3364 = v_n.clone(); Py::Bool({ let t3365 = Py::Int(0i64); if py::eq(&t3364, &t3365) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]));
    }
    let t3366 = (f__fib_pair(py::rshift(&v_n.clone(), &Py::Int(1i64))?)?).items()?;
    if t3366.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3366[0].clone();
    v_b = t3366[1].clone();
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
    let t3637 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t3637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3637[0].clone();
    v_names = t3637[1].clone();
    if ({ let t3638 = py::len(&v_names.clone())?; Py::Bool({ let t3639 = Py::Int(6i64); if py::gt(&t3638, &t3639)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 6 variables for minimization"))));
    }
    return Ok(Py::tuple(vec![py::len(&v_names.clone())?, { let mut out = vec![]; for t3640 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3641 = (t3640).items()?;
if t3641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3641[0].clone();
v_v = t3641[1].clone();
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
    for t1851 in py::iter(&(v_adj.clone()))? {
        v_s = t1851;
        let t1852 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), Py::Int(0i64))]), Py::dict(vec![(v_s.clone(), Py::None)]), Py::list(vec![v_s.clone()])])).items()?;
        if t1852.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_dist = t1852[0].clone();
        v_parent = t1852[1].clone();
        v_q = t1852[2].clone();
        for t1853 in py::iter(&(v_q.clone()))? {
            v_u = t1853;
            for t1854 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t1854;
                if ({ let t1855 = v_v.clone(); Py::Bool({ let t1856 = v_dist.clone(); if !py::contains(&t1856, &t1855)? { true } else { false } }) }).truthy() {
                    let t1857 = (Py::tuple(vec![py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &Py::Int(1i64))?, v_u.clone()])).items()?;
                    if t1857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&v_dist.clone(), &v_v.clone(), t1857[0].clone())?;
                    py::setitem(&v_parent.clone(), &v_v.clone(), t1857[1].clone())?;
                    let _ = py::method(&v_q.clone(), "append", &[v_v.clone()])?;
                } else {
                    if ({ let t1863 = { let t1858 = py::getitem(&v_parent.clone(), &v_u.clone())?; Py::Bool({ let t1859 = v_v.clone(); if py::ne(&t1858, &t1859) { true } else { false } }) }; if !t1863.truthy() { t1863 } else { { let t1862 = v_odd.clone(); if t1862.truthy() { t1862 } else { { let t1860 = py::getitem(&v_dist.clone(), &v_v.clone())?; Py::Bool({ let t1861 = py::getitem(&v_dist.clone(), &v_u.clone())?; if py::ne(&t1860, &t1861) { true } else { false } }) } } } } }).truthy() {
                        v_L = py::add(&py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &py::getitem(&v_dist.clone(), &v_v.clone())?)?, &Py::Int(1i64))?;
                        if ({ let t1868 = { let t1864 = v_best.clone(); Py::Bool({ let t1865 = Py::None; if py::is(&t1864, &t1865) { true } else { false } }) }; if t1868.truthy() { t1868 } else { { let t1866 = v_L.clone(); Py::Bool({ let t1867 = v_best.clone(); if py::lt(&t1866, &t1867)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t6110 = py::not(&v_weighted.clone()); if !t6110.truthy() { t6110 } else { Py::Bool(({ let mut out = vec![]; for t6107 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t6107;
out.push({ let t6108 = py::len(&v_e.clone())?; Py::Bool({ let t6109 = Py::Int(3i64); if py::eq(&t6108, &t6109) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability takes an unweighted graph: give edges without weights"))));
    }
    v_adj = Py::dict(vec![]);
    for t6111 in py::iter(&(v_edges.clone()))? {
        v_e = t6111;
        let t6112 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)), "strip", &[])?])).items()?;
        if t6112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t6112[0].clone();
        v_v = t6112[1].clone();
        if ({ let t6113 = v_u.clone(); Py::Bool({ let t6114 = v_v.clone(); if py::eq(&t6113, &t6114) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["self-loop at ".to_string(), py::format(&v_u.clone(), "")?, ": simple graphs only".to_string()].concat()))));
        }
        let _ = py::method(&v_adj.clone(), "setdefault", &[v_u.clone(), Py::dict(vec![])])?;
        let _ = py::method(&v_adj.clone(), "setdefault", &[v_v.clone(), Py::dict(vec![])])?;
        if ({ let t6120 = { let t6115 = v_v.clone(); Py::Bool({ let t6116 = py::getitem(&v_adj.clone(), &v_u.clone())?; if py::contains(&t6116, &t6115)? { true } else { false } }) }; if t6120.truthy() { t6120 } else { { let t6119 = py::not(&v_directed.clone()); if !t6119.truthy() { t6119 } else { { let t6117 = v_u.clone(); Py::Bool({ let t6118 = py::getitem(&v_adj.clone(), &v_v.clone())?; if py::contains(&t6118, &t6117)? { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["the edge ".to_string(), py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&v_v.clone(), "")?, " is given twice".to_string()].concat()))));
        }
        py::setitem(&py::getitem(&v_adj.clone(), &v_u.clone())?, &v_v.clone(), (if ({ let t6121 = py::len(&v_e.clone())?; Py::Bool({ let t6122 = Py::Int(3i64); if py::eq(&t6121, &t6122) { true } else { false } }) }).truthy() { py::float(&py::getitem(&v_e.clone(), &Py::Int(2i64))?)? } else { Py::Float(1.0) }))?;
        if (py::not(&v_directed.clone())).truthy() {
            py::setitem(&py::getitem(&v_adj.clone(), &v_v.clone())?, &v_u.clone(), py::getitem(&py::getitem(&v_adj.clone(), &v_u.clone())?, &v_v.clone())?)?;
        }
    }
    if (py::not(&v_adj.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph has no edges"))));
    }
    if ({ let t6123 = py::len(&v_adj.clone())?; Py::Bool({ let t6124 = Py::Int(400i64); if py::gt(&t6123, &t6124)? { true } else { false } }) }).truthy() {
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
    let t5331 = (f__ext(py::modulo(&v_a.clone(), &v_m.clone())?, v_m.clone())?).items()?;
    if t5331.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t5331[0].clone();
    v_x = t5331[1].clone();
    v__ = t5331[2].clone();
    if ({ let t5332 = v_g.clone(); Py::Bool({ let t5333 = Py::Int(1i64); if py::ne(&t5332, &t5333) { true } else { false } }) }).truthy() {
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
    if ({ let t5947 = v_n.clone(); Py::Bool({ let t5948 = Py::Int(2i64); if py::lt(&t5947, &t5948)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    for t5949 in py::iter(&(c__SMALL()?))? {
        v_p = t5949;
        if ({ let t5950 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t5951 = Py::Int(0i64); if py::eq(&t5950, &t5951) { true } else { false } }) }).truthy() {
            return Ok({ let t5952 = v_n.clone(); Py::Bool({ let t5953 = v_p.clone(); if py::eq(&t5952, &t5953) { true } else { false } }) });
        }
    }
    let t5954 = (Py::tuple(vec![py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
    if t5954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t5954[0].clone();
    v_s = t5954[1].clone();
    while ({ let t5955 = py::modulo(&v_d.clone(), &Py::Int(2i64))?; Py::Bool({ let t5956 = Py::Int(0i64); if py::eq(&t5955, &t5956) { true } else { false } }) }).truthy() {
        v_d = py::floordiv(&v_d, &Py::Int(2i64))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    for t5957 in py::iter(&(c__MR_BASES()?))? {
        v_a = t5957;
        v_a = py::modulo(&v_a, &v_n.clone())?;
        if ({ let t5958 = v_a.clone(); Py::Bool({ let t5959 = Py::Int(0i64); if py::eq(&t5958, &t5959) { true } else { false } }) }).truthy() {
            continue;
        }
        v_x = py::pow_mod(&v_a.clone(), &v_d.clone(), &v_n.clone())?;
        if ({ let t5964 = { let t5960 = v_x.clone(); Py::Bool({ let t5961 = Py::Int(1i64); if py::eq(&t5960, &t5961) { true } else { false } }) }; if t5964.truthy() { t5964 } else { { let t5962 = v_x.clone(); Py::Bool({ let t5963 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t5962, &t5963) { true } else { false } }) } } }).truthy() {
            continue;
        }
        let mut t5965 = false;
        for t5966 in py::range_iter(&[(py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
            v__ = t5966;
            v_x = py::modulo(&py::mul(&v_x.clone(), &v_x.clone())?, &v_n.clone())?;
            if ({ let t5967 = v_x.clone(); Py::Bool({ let t5968 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t5967, &t5968) { true } else { false } }) }).truthy() {
                { t5965 = true; break; }
            }
        }
        if !t5965 {
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
    if ({ let t4807 = v_n.clone(); Py::Bool({ let t4808 = Py::Int(2i64); if py::lt(&t4807, &t4808)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    for t4809 in py::iter(&(Py::tuple(vec![Py::Int(2i64), Py::Int(3i64), Py::Int(5i64), Py::Int(7i64), Py::Int(11i64), Py::Int(13i64), Py::Int(17i64), Py::Int(19i64), Py::Int(23i64), Py::Int(29i64), Py::Int(31i64), Py::Int(37i64), Py::Int(41i64), Py::Int(43i64), Py::Int(47i64)])))? {
        v_p = t4809;
        if ({ let t4810 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t4811 = Py::Int(0i64); if py::eq(&t4810, &t4811) { true } else { false } }) }).truthy() {
            return Ok({ let t4812 = v_n.clone(); Py::Bool({ let t4813 = v_p.clone(); if py::eq(&t4812, &t4813) { true } else { false } }) });
        }
    }
    let t4814 = (Py::tuple(vec![py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
    if t4814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4814[0].clone();
    v_s = t4814[1].clone();
    while (py::not(&py::bitand(&v_d.clone(), &Py::Int(1i64))?)).truthy() {
        v_d = py::rshift(&v_d, &Py::Int(1i64))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    for t4815 in py::iter(&(c__SMALL()?))? {
        v_a = t4815;
        v_x = py::pow_mod(&v_a.clone(), &v_d.clone(), &v_n.clone())?;
        if ({ let t4816 = v_x.clone(); Py::Bool({ let t4817 = Py::tuple(vec![Py::Int(1i64), py::sub(&v_n.clone(), &Py::Int(1i64))?]); if py::contains(&t4817, &t4816)? { true } else { false } }) }).truthy() {
            continue;
        }
        let mut t4818 = false;
        for t4819 in py::range_iter(&[(py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
            v__ = t4819;
            v_x = py::pow_mod(&v_x.clone(), &Py::Int(2i64), &v_n.clone())?;
            if ({ let t4820 = v_x.clone(); Py::Bool({ let t4821 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t4820, &t4821) { true } else { false } }) }).truthy() {
                { t4818 = true; break; }
            }
        }
        if !t4818 {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__iv(mut v_x: Py) -> R {
    return Ok({ let t6173 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t6173.truthy() { t6173 } else { py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jacobi(mut v_a: Py, mut v_n: Py) -> R {
    let mut v_result: Py = Py::None;
    v_a = py::modulo(&v_a, &v_n.clone())?;
    v_result = Py::Int(1i64);
    while (v_a.clone()).truthy() {
        while ({ let t2737 = py::modulo(&v_a.clone(), &Py::Int(2i64))?; Py::Bool({ let t2738 = Py::Int(0i64); if py::eq(&t2737, &t2738) { true } else { false } }) }).truthy() {
            v_a = py::floordiv(&v_a, &Py::Int(2i64))?;
            if ({ let t2735 = py::modulo(&v_n.clone(), &Py::Int(8i64))?; Py::Bool({ let t2736 = Py::tuple(vec![Py::Int(3i64), Py::Int(5i64)]); if py::contains(&t2736, &t2735)? { true } else { false } }) }).truthy() {
                v_result = py::neg(&v_result.clone())?;
            }
        }
        let t2739 = (Py::tuple(vec![v_n.clone(), v_a.clone()])).items()?;
        if t2739.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2739[0].clone();
        v_n = t2739[1].clone();
        if ({ let t2744 = { let t2740 = py::modulo(&v_a.clone(), &Py::Int(4i64))?; Py::Bool({ let t2741 = Py::Int(3i64); if py::eq(&t2740, &t2741) { true } else { false } }) }; if !t2744.truthy() { t2744 } else { { let t2742 = py::modulo(&v_n.clone(), &Py::Int(4i64))?; Py::Bool({ let t2743 = Py::Int(3i64); if py::eq(&t2742, &t2743) { true } else { false } }) } } }).truthy() {
            v_result = py::neg(&v_result.clone())?;
        }
        v_a = py::modulo(&v_a, &v_n.clone())?;
    }
    return Ok((if ({ let t2745 = v_n.clone(); Py::Bool({ let t2746 = Py::Int(1i64); if py::eq(&t2745, &t2746) { true } else { false } }) }).truthy() { v_result.clone() } else { Py::Int(0i64) }));
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
    v_idx = { let mut out = vec![]; for t5409 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t5410 = (t5409).items()?;
if t5410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5410[0].clone();
v_v = t5410[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_n = py::len(&v_nodes.clone())?;
    v_L = { let mut out = vec![]; for t5411 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t5411;
out.push(py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?); } Py::list(out) };
    for t5412 in py::iter(&(v_adj.clone()))? {
        v_u = t5412;
        for t5413 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5413;
            py::setitem(&py::getitem(&v_L.clone(), &py::getitem(&v_idx.clone(), &v_u.clone())?)?, &py::getitem(&v_idx.clone(), &v_v.clone())?, py::neg(&Py::Int(1i64))?)?;
        }
        py::setitem(&py::getitem(&v_L.clone(), &py::getitem(&v_idx.clone(), &v_u.clone())?)?, &py::getitem(&v_idx.clone(), &v_u.clone())?, py::len(&py::getitem(&v_adj.clone(), &v_u.clone())?)?)?;
    }
    return Ok({ let mut out = vec![]; for t5414 in py::iter(&(py::slice(&v_L.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_row: Py; 
v_row = t5414;
out.push(py::slice(&v_row.clone(), Some((Py::Int(1i64)).i()?), None, None)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__letters(mut v_text: Py) -> R {
    return Ok({ let mut out = vec![]; for t6075 in py::iter(&(py::method(&Py::str(&py::to_str(&v_text.clone())), "upper", &[])?))? { let mut v_ch: Py; 
v_ch = t6075;
if ({ let t6076 = Py::str("A"); Py::Bool({ let t6077 = v_ch.clone(); if py::le(&t6076, &t6077)? { { let t6078 = Py::str("Z"); if py::le(&t6077, &t6078)? { true } else { false } } } else { false } }) }).truthy() { out.push(py::sub(&py::ord(&v_ch.clone())?, &Py::Int(65i64))?); }} Py::list(out) });
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
    if ({ let t5819 = py::len(&v_s.clone())?; Py::Bool({ let t5820 = Py::Int(400i64); if py::gt(&t5819, &t5820)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the formula is too long (at most 400 characters)"))));
    }
    let t5821 = (Py::tuple(vec![Py::Int(0i64), Py::list(vec![])])).items()?;
    if t5821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pos = t5821[0].clone();
    v_out = t5821[1].clone();
    while ({ let t5828 = v_pos.clone(); Py::Bool({ let t5829 = py::len(&v_s.clone())?; if py::lt(&t5828, &t5829)? { true } else { false } }) }).truthy() {
        if ({ let t5822 = py::method(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), None, None)?, "strip", &[])?; Py::Bool({ let t5823 = Py::str(""); if py::eq(&t5822, &t5823) { true } else { false } }) }).truthy() {
            break;
        }
        v_m = py::method(&c__LTOK()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if (py::not(&v_m.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the formula at '".to_string(), py::format(&py::method(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), Some((py::add(&v_pos.clone(), &Py::Int(10i64))?).i()?), None)?, "strip", &[])?, "")?, "'".to_string()].concat()))));
        }
        v_t = py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?;
        v_pos = py::method(&v_m.clone(), "end", &[])?;
        if ({ let t5824 = v_t.clone(); Py::Bool({ let t5825 = c__LSYM()?; if py::contains(&t5825, &t5824)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::getitem(&c__LSYM()?, &v_t.clone())?])?;
        } else {
            if ({ let t5826 = py::method(&v_t.clone(), "lower", &[])?; Py::Bool({ let t5827 = c__LWORD()?; if py::contains(&t5827, &t5826)? { true } else { false } }) }).truthy() {
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
    let t480 = (Py::tuple(vec![Py::dict(vec![]), Py::dict(vec![]), py::set(&Py::list(vec![]))?, Py::list(vec![]), Py::list(vec![Py::Int(0i64)])])).items()?;
    if t480.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_disc = t480[0].clone();
    v_low = t480[1].clone();
    v_aps = t480[2].clone();
    v_brs = t480[3].clone();
    v_t = t480[4].clone();
    let rec481 = py::rec_cell();
    v_dfs = { let v_adj = v_adj.clone(); let v_aps = v_aps.clone(); let v_brs = v_brs.clone(); let v_disc = v_disc.clone(); let v_low = v_low.clone(); let v_t = v_t.clone(); let rec481 = rec481.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_parent: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_dfs: Py = py::rec_get(&rec481)?;
            let mut v_children: Py = Py::None;
            let mut v_v: Py = Py::None;
            let t482 = py::getitem(&v_t.clone(), &Py::Int(0i64))?;
            py::setitem(&v_disc.clone(), &v_u.clone(), t482.clone())?;
            py::setitem(&v_low.clone(), &v_u.clone(), t482.clone())?;
            let t483 = v_t.clone(); let t484 = Py::Int(0i64);
            py::setitem(&t483, &t484, py::iadd(&py::getitem(&t483, &t484)?, &Py::Int(1i64))?)?;
            v_children = Py::Int(0i64);
            for t485 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &v_u.clone())?)?))? {
                v_v = t485;
                if ({ let t486 = v_v.clone(); Py::Bool({ let t487 = v_disc.clone(); if !py::contains(&t487, &t486)? { true } else { false } }) }).truthy() {
                    v_children = py::iadd(&v_children, &Py::Int(1i64))?;
                    let _ = py::call(&v_dfs.clone(), &[v_v.clone(), v_u.clone()])?;
                    py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_low.clone(), &v_v.clone())?], false)?)?;
                    if ({ let t492 = { let t488 = v_parent.clone(); Py::Bool({ let t489 = Py::None; if !py::is(&t488, &t489) { true } else { false } }) }; if !t492.truthy() { t492 } else { { let t490 = py::getitem(&v_low.clone(), &v_v.clone())?; Py::Bool({ let t491 = py::getitem(&v_disc.clone(), &v_u.clone())?; if py::ge(&t490, &t491)? { true } else { false } }) } } }).truthy() {
                        let _ = py::method(&v_aps.clone(), "add", &[v_u.clone()])?;
                    }
                    if ({ let t493 = py::getitem(&v_low.clone(), &v_v.clone())?; Py::Bool({ let t494 = py::getitem(&v_disc.clone(), &v_u.clone())?; if py::gt(&t493, &t494)? { true } else { false } }) }).truthy() {
                        let _ = py::method(&v_brs.clone(), "append", &[py::sorted_key(&Py::tuple(vec![v_u.clone(), v_v.clone()]), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?])?;
                    }
                } else {
                    if ({ let t495 = v_v.clone(); Py::Bool({ let t496 = v_parent.clone(); if py::ne(&t495, &t496) { true } else { false } }) }).truthy() {
                        py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_disc.clone(), &v_v.clone())?], false)?)?;
                    }
                }
            }
            if ({ let t501 = { let t497 = v_parent.clone(); Py::Bool({ let t498 = Py::None; if py::is(&t497, &t498) { true } else { false } }) }; if !t501.truthy() { t501 } else { { let t499 = v_children.clone(); Py::Bool({ let t500 = Py::Int(1i64); if py::gt(&t499, &t500)? { true } else { false } }) } } }).truthy() {
                let _ = py::method(&v_aps.clone(), "add", &[v_u.clone()])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec481, &v_dfs);
    for t502 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_s = t502;
        if ({ let t503 = v_s.clone(); Py::Bool({ let t504 = v_disc.clone(); if !py::contains(&t504, &t503)? { true } else { false } }) }).truthy() {
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
    let rec2293 = py::rec_cell();
    v_expand = { let v_adj = v_adj.clone(); let c_best = c_best.clone(); let rec2293 = rec2293.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_R: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_P: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_X: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_expand: Py = py::rec_get(&rec2293)?;
            let mut v_best: Py = Py::None;
            let mut v_pivot: Py = Py::None;
            let mut v_v: Py = Py::None;
            let c_P = py::cell_new();
            py::cell_set(&c_P, v_P.clone());
            if ({ let t2294 = py::not(&py::cell_get(&c_P)?); if !t2294.truthy() { t2294 } else { py::not(&v_X.clone()) } }).truthy() {
                if ({ let t2304 = { let t2295 = py::len(&v_R.clone())?; Py::Bool({ let t2296 = py::len(&py::cell_get(&c_best)?)?; if py::gt(&t2295, &t2296)? { true } else { false } }) }; if t2304.truthy() { t2304 } else { { let t2303 = { let t2297 = py::len(&v_R.clone())?; Py::Bool({ let t2298 = py::len(&py::cell_get(&c_best)?)?; if py::eq(&t2297, &t2298) { true } else { false } }) }; if !t2303.truthy() { t2303 } else { { let t2299 = { let mut out = vec![]; for t2301 in py::iter(&(py::sorted_key(&v_R.clone(), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?))? { let mut v_v: Py; 
v_v = t2301;
out.push(f__nkey(v_v.clone())?); } Py::list(out) }; Py::Bool({ let t2300 = { let mut out = vec![]; for t2302 in py::iter(&(py::cell_get(&c_best)?))? { let mut v_v: Py; 
v_v = t2302;
out.push(f__nkey(v_v.clone())?); } Py::list(out) }; if py::lt(&t2299, &t2300)? { true } else { false } }) } } } } }).truthy() {
                    py::cell_set(&c_best, f__sorted(v_R.clone())?);
                }
                return Ok(Py::None);
            }
            if ({ let t2305 = py::add(&py::len(&v_R.clone())?, &py::len(&py::cell_get(&c_P)?)?)?; Py::Bool({ let t2306 = py::len(&py::cell_get(&c_best)?)?; if py::lt(&t2305, &t2306)? { true } else { false } }) }).truthy() {
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
            for t2307 in py::iter(&(f__sorted(py::sub(&py::cell_get(&c_P)?, &py::getitem(&v_adj.clone(), &v_pivot.clone())?)?)?))? {
                v_v = t2307;
                let _ = py::call(&v_expand.clone(), &[py::bitor(&v_R.clone(), &py::set(&Py::list(vec![v_v.clone()]))?)?, py::bitand(&py::cell_get(&c_P)?, &py::getitem(&v_adj.clone(), &v_v.clone())?)?, py::bitand(&v_X.clone(), &py::getitem(&v_adj.clone(), &v_v.clone())?)?])?;
                py::cell_set(&c_P, py::sub(&py::cell_get(&c_P)?, &py::set(&Py::list(vec![v_v.clone()]))?)?);
                v_X = py::bitor(&v_X.clone(), &py::set(&Py::list(vec![v_v.clone()]))?)?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2293, &v_expand);
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
    v_cov = { let mut out = vec![]; for t3779 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t3779;
out.push((v_p.clone(), py::set(&{ let mut out = vec![]; for t3780 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3780;
if (f__covers(v_p.clone(), v_m.clone())?).truthy() { out.push(v_m.clone()); }} Py::list(out) })?)); } Py::dict(out) };
    v_by = { let mut out = vec![]; for t3783 in py::iter(&(v_on.clone()))? { let mut v_m: Py; 
v_m = t3783;
out.push((v_m.clone(), { let mut out = vec![]; for t3784 in py::iter(&(v_primes.clone()))? { let mut v_p: Py; 
v_p = t3784;
if ({ let t3785 = v_m.clone(); Py::Bool({ let t3786 = py::getitem(&v_cov.clone(), &v_p.clone())?; if py::contains(&t3786, &t3785)? { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) })); } Py::dict(out) };
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
    let rec3787 = py::rec_cell();
    v_rec = { let v_best = v_best.clone(); let v_by = v_by.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let rec3787 = rec3787.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_unc: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_chosen: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_cost: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_rec: Py = py::rec_get(&rec3787)?;
            let mut v_m: Py = Py::None;
            let mut v_p: Py = Py::None;
            if ({ let t3792 = { let t3788 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t3789 = Py::None; if !py::is(&t3788, &t3789) { true } else { false } }) }; if !t3792.truthy() { t3792 } else { { let t3790 = v_cost.clone(); Py::Bool({ let t3791 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t3790, &t3791)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::None);
            }
            if (py::not(&v_unc.clone())).truthy() {
                let t3793 = (Py::tuple(vec![v_cost.clone(), Py::list((v_chosen.clone()).items()?)])).items()?;
                if t3793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &Py::Int(0i64), t3793[0].clone())?;
                py::setitem(&v_best.clone(), &Py::Int(1i64), t3793[1].clone())?;
                return Ok(Py::None);
            }
            if ({ let t3798 = { let t3794 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t3795 = Py::None; if !py::is(&t3794, &t3795) { true } else { false } }) }; if !t3798.truthy() { t3798 } else { { let t3796 = Py::tuple(vec![py::add(&py::getitem(&v_cost.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::getitem(&v_cost.clone(), &Py::Int(1i64))?]); Py::Bool({ let t3797 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t3796, &t3797)? { true } else { false } }) } } }).truthy() {
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
            for t3799 in py::iter(&(py::sorted_key(&py::getitem(&v_by.clone(), &v_m.clone())?, &{ let v_best = v_best.clone(); let v_by = v_by.clone(); let v_chosen = v_chosen.clone(); let v_cost = v_cost.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let v_rec = v_rec.clone(); let v_unc = v_unc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::len(&py::bitand(&py::getitem(&v_cov.clone(), &v_p.clone())?, &v_unc.clone())?)?)?, py::call(&v_lits.clone(), &[v_p.clone()])?, v_p.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?))? {
                v_p = t3799;
                let _ = py::method(&v_chosen.clone(), "append", &[v_p.clone()])?;
                let _ = py::call(&v_rec.clone(), &[py::sub(&v_unc.clone(), &py::getitem(&v_cov.clone(), &v_p.clone())?)?, v_chosen.clone(), Py::tuple(vec![py::add(&py::getitem(&v_cost.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::add(&py::getitem(&v_cost.clone(), &Py::Int(1i64))?, &py::call(&v_lits.clone(), &[v_p.clone()])?)?])])?;
                let _ = py::method(&v_chosen.clone(), "pop", &[])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3787, &v_rec);
    let _ = py::call(&v_rec.clone(), &[v_on.clone(), Py::list(vec![]), Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)])])?;
    return Ok(py::sorted_key(&py::getitem(&v_best.clone(), &Py::Int(1i64))?, &{ let v_best = v_best.clone(); let v_by = v_by.clone(); let v_cov = v_cov.clone(); let v_lits = v_lits.clone(); let v_n = v_n.clone(); let v_on = v_on.clone(); let v_primes = v_primes.clone(); let v_rec = v_rec.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t3800 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_b: Py; 
v_b = t3800;
out.push((if ({ let t3801 = py::not(&py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_b.clone())?, &Py::Int(1i64))?); if !t3801.truthy() { t3801 } else { py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_b.clone())?, &Py::Int(1i64))? } }).truthy() { Py::Int(0i64) } else { (if (py::not(&py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_b.clone())?, &Py::Int(1i64))?)).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) })); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mult(mut v_x: Py, mut v_y: Py) -> R {
    if ({ let t6011 = { let t6007 = v_x.clone(); Py::Bool({ let t6008 = Py::Int(0i64); if py::eq(&t6007, &t6008) { true } else { false } }) }; if !t6011.truthy() { t6011 } else { { let t6009 = v_y.clone(); Py::Bool({ let t6010 = Py::Int(0i64); if py::eq(&t6009, &t6010) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Int(1i64));
    }
    if ({ let t6016 = { let t6012 = v_x.clone(); Py::Bool({ let t6013 = Py::Int(0i64); if py::eq(&t6012, &t6013) { true } else { false } }) }; if t6016.truthy() { t6016 } else { { let t6014 = v_x.clone(); Py::Bool({ let t6015 = v_y.clone(); if py::eq(&t6014, &t6015) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Int(4i64));
    }
    return Ok(Py::Int(8i64));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ncomp(mut v_adj: Py, mut v_drop_v: Py, mut v_drop_e: Py) -> R {
    let mut v_sub: Py = Py::None;
    v_sub = { let mut out = vec![]; for t525 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t526 = (t525).items()?;
if t526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t526[0].clone();
v_nb = t526[1].clone();
if ({ let t527 = v_u.clone(); Py::Bool({ let t528 = v_drop_v.clone(); if py::ne(&t527, &t528) { true } else { false } }) }).truthy() { out.push((v_u.clone(), { let mut out = vec![]; for t529 in py::iter(&(v_nb.clone()))? { let mut v_v: Py; 
v_v = t529;
if ({ let t535 = { let t530 = v_v.clone(); Py::Bool({ let t531 = v_drop_v.clone(); if py::ne(&t530, &t531) { true } else { false } }) }; if !t535.truthy() { t535 } else { py::not(&{ let t534 = v_drop_e.clone(); if !t534.truthy() { t534 } else { { let t532 = py::set(&Py::list(vec![v_u.clone(), v_v.clone()]))?; Py::Bool({ let t533 = v_drop_e.clone(); if py::eq(&t532, &t533) { true } else { false } }) } } }) } }).truthy() { out.push(v_v.clone()); }} Py::list(out) })); }} Py::dict(out) };
    return Ok(py::len(&f__components(v_sub.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__necklace_count(mut v_n: Py, mut v_k: Py) -> R {
    return Ok((if ({ let t4272 = v_n.clone(); Py::Bool({ let t4273 = Py::Int(1i64); if py::gt(&t4272, &t4273)? { true } else { false } }) }).truthy() { py::floordiv(&py::sum(&{ let mut out = vec![]; for t4316 in py::iter(&(f__divisors_from(f__factor(v_n.clone())?)?))? { let mut v_d: Py; 
v_d = t4316;
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
    v_k = (if ({ let t4142 = v_n.clone(); Py::Bool({ let t4143 = Py::Int(1i64); if py::gt(&t4142, &t4143)? { true } else { false } }) }).truthy() { f__carmichael_from(f__factor(v_n.clone())?)? } else { Py::Int(1i64) });
    for t4190 in py::iter(&(f__factor(v_k.clone())?))? {
        v_p = t4190;
        while ({ let t4195 = { let t4191 = py::modulo(&v_k.clone(), &v_p.clone())?; Py::Bool({ let t4192 = Py::Int(0i64); if py::eq(&t4191, &t4192) { true } else { false } }) }; if !t4195.truthy() { t4195 } else { { let t4193 = py::pow_mod(&v_a.clone(), &py::floordiv(&v_k.clone(), &v_p.clone())?, &v_n.clone())?; Py::Bool({ let t4194 = Py::Int(1i64); if py::eq(&t4193, &t4194) { true } else { false } }) } } }).truthy() {
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
    let t5830 = (Py::tuple(vec![f__lex(v_text.clone())?, Py::list(vec![Py::Int(0i64)])])).items()?;
    if t5830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_toks = t5830[0].clone();
    v_i = t5830[1].clone();
    v_peek = { let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok((if ({ let t5831 = py::getitem(&v_i.clone(), &Py::Int(0i64))?; Py::Bool({ let t5832 = py::len(&v_toks.clone())?; if py::lt(&t5831, &t5832)? { true } else { false } }) }).truthy() { py::getitem(&v_toks.clone(), &py::getitem(&v_i.clone(), &Py::Int(0i64))?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_take = { let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let t5833 = v_i.clone(); let t5834 = Py::Int(0i64);
            py::setitem(&t5833, &t5834, py::iadd(&py::getitem(&t5833, &t5834)?, &Py::Int(1i64))?)?;
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
            let rec5835 = py::rec_cell();
            v_f = { let v_binary = v_binary.clone(); let v_i = v_i.clone(); let v_op = v_op.clone(); let v_peek = v_peek.clone(); let v_right = v_right.clone(); let v_sub = v_sub.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); let rec5835 = rec5835.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_f: Py = py::rec_get(&rec5835)?;
            let mut v_left: Py = Py::None;
            let mut v_rhs: Py = Py::None;
            v_left = py::call(&v_sub.clone(), &[])?;
            while ({ let t5836 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t5837 = v_op.clone(); if py::eq(&t5836, &t5837) { true } else { false } }) }).truthy() {
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
            py::rec_set(&rec5835, &v_f);
            return Ok(v_f.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec5838 = py::rec_cell();
    v_atom = { let v_binary = v_binary.clone(); let v_i = v_i.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let c_iff = c_iff.clone(); let rec5838 = rec5838.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_atom: Py = py::rec_get(&rec5838)?;
            let mut v_e: Py = Py::None;
            let mut v_t: Py = Py::None;
            v_t = (if ({ let t5839 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t5840 = Py::None; if !py::is(&t5839, &t5840) { true } else { false } }) }).truthy() { py::call(&v_take.clone(), &[])? } else { Py::None });
            if ({ let t5841 = v_t.clone(); Py::Bool({ let t5842 = Py::str("NOT"); if py::eq(&t5841, &t5842) { true } else { false } }) }).truthy() {
                return Ok(Py::tuple(vec![Py::str("NOT"), py::call(&v_atom.clone(), &[])?]));
            }
            if ({ let t5843 = v_t.clone(); Py::Bool({ let t5844 = Py::str("("); if py::eq(&t5843, &t5844) { true } else { false } }) }).truthy() {
                v_e = py::call(&py::cell_get(&c_iff)?, &[])?;
                if ((if ({ let t5845 = py::call(&v_peek.clone(), &[])?; Py::Bool({ let t5846 = Py::None; if !py::is(&t5845, &t5846) { true } else { false } }) }).truthy() { { let t5847 = py::call(&v_take.clone(), &[])?; Py::Bool({ let t5848 = Py::str(")"); if py::ne(&t5847, &t5848) { true } else { false } }) } } else { Py::Bool(true) })).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses in the formula"))));
                }
                return Ok(v_e.clone());
            }
            if (Py::Bool(matches!(v_t.clone(), Py::Tuple(_)))).truthy() {
                return Ok(v_t.clone());
            }
            if ({ let t5853 = { let t5849 = v_t.clone(); Py::Bool({ let t5850 = Py::Bool(true); if py::is(&t5849, &t5850) { true } else { false } }) }; if t5853.truthy() { t5853 } else { { let t5851 = v_t.clone(); Py::Bool({ let t5852 = Py::Bool(false); if py::is(&t5851, &t5852) { true } else { false } }) } } }).truthy() {
                return Ok(Py::tuple(vec![Py::str("CONST"), v_t.clone()]));
            }
            return Err(py::value_error(py::to_str(&Py::str("the formula is incomplete or malformed"))));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec5838, &v_atom);
    v_conj = py::call(&v_binary.clone(), &[Py::str("AND"), v_atom.clone()])?;
    v_xor = py::call(&v_binary.clone(), &[Py::str("XOR"), v_conj.clone()])?;
    v_disj = py::call(&v_binary.clone(), &[Py::str("OR"), v_xor.clone()])?;
    v_imp = py::call(&v_binary.clone(), &[Py::str("IMP"), v_disj.clone(), Py::Bool(true)])?;
    py::cell_set(&c_iff, py::call(&v_binary.clone(), &[Py::str("IFF"), v_imp.clone()])?);
    v_tree = py::call(&py::cell_get(&c_iff)?, &[])?;
    if ({ let t5854 = py::getitem(&v_i.clone(), &Py::Int(0i64))?; Py::Bool({ let t5855 = py::len(&v_toks.clone())?; if py::ne(&t5854, &t5855) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("unexpected symbols at the end of the formula"))));
    }
    v_names = py::set(&Py::list(vec![]))?;
    let rec5856 = py::rec_cell();
    v_collect = { let v_atom = v_atom.clone(); let v_binary = v_binary.clone(); let v_conj = v_conj.clone(); let v_disj = v_disj.clone(); let v_i = v_i.clone(); let v_imp = v_imp.clone(); let v_names = v_names.clone(); let v_peek = v_peek.clone(); let v_take = v_take.clone(); let v_text = v_text.clone(); let v_toks = v_toks.clone(); let v_tree = v_tree.clone(); let v_xor = v_xor.clone(); let c_iff = c_iff.clone(); let rec5856 = rec5856.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_collect: Py = py::rec_get(&rec5856)?;
            let mut v_c: Py = Py::None;
            if ({ let t5857 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t5858 = Py::str("VAR"); if py::eq(&t5857, &t5858) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_names.clone(), "add", &[py::getitem(&v_t.clone(), &Py::Int(1i64))?])?;
            } else {
                if ({ let t5859 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t5860 = Py::str("CONST"); if py::ne(&t5859, &t5860) { true } else { false } }) }).truthy() {
                    for t5861 in py::iter(&(py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
                        v_c = t5861;
                        let _ = py::call(&v_collect.clone(), &[v_c.clone()])?;
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec5856, &v_collect);
    let _ = py::call(&v_collect.clone(), &[v_tree.clone()])?;
    if ({ let t5862 = py::len(&v_names.clone())?; Py::Bool({ let t5863 = c_MAX_VARS()?; if py::gt(&t5862, &t5863)? { true } else { false } }) }).truthy() {
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
    v_off = { let mut out = vec![]; for t3448 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3448;
if ({ let t3453 = { let t3449 = v_m.clone(); Py::Bool({ let t3450 = py::set(&v_on.clone())?; if !py::contains(&t3450, &t3449)? { true } else { false } }) }; if !t3453.truthy() { t3453 } else { { let t3451 = v_m.clone(); Py::Bool({ let t3452 = py::set(&v_dc.clone())?; if !py::contains(&t3452, &t3451)? { true } else { false } }) } } }).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    v_cover = py::sorted_key(&f__min_cover(v_n.clone(), v_off.clone(), f__qm_primes(v_n.clone(), py::bitor(&py::set(&v_off.clone())?, &py::set(&v_dc.clone())?)?)?)?, &{ let v_dc = v_dc.clone(); let v_n = v_n.clone(); let v_names = v_names.clone(); let v_off = v_off.clone(); let v_on = v_on.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t3489 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_b: Py; 
v_b = t3489;
out.push((if (py::not(&py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_b.clone())?, &Py::Int(1i64))?)).truthy() { (if (py::bitand(&py::rshift(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_b.clone())?, &Py::Int(1i64))?).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) } else { Py::Int(2i64) })); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    if (py::not(&v_off.clone())).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("1")), (Py::str("clauses"), Py::Int(0i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3490 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3490;
out.push({ let t3491 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t3492 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t3491, &t3492) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("0")), (Py::str("clauses"), Py::Int(1i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    v_texts = Py::list(vec![]);
    for t3493 in py::iter(&(v_cover.clone()))? {
        v_p = t3493;
        v_ls = f__term_text(v_p.clone(), v_names.clone(), Py::Bool(true))?;
        let _ = py::method(&v_texts.clone(), "append", &[(if ({ let t3498 = py::len(&v_ls.clone())?; Py::Bool({ let t3499 = Py::Int(1i64); if py::eq(&t3498, &t3499) { true } else { false } }) }).truthy() { py::getitem(&v_ls.clone(), &Py::Int(0i64))? } else { py::add(&py::add(&Py::str("("), &py::method(&Py::str(" | "), "join", &[v_ls.clone()])?)?, &Py::str(")"))? })])?;
    }
    return Ok(Py::dict(vec![(Py::str("expression"), py::method(&Py::str(" & "), "join", &[v_texts.clone()])?), (Py::str("clauses"), py::len(&v_cover.clone())?), (Py::str("literals"), py::sum(&{ let mut out = vec![]; for t3500 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3500;
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
    v_groups = py::set(&{ let mut out = vec![]; for t3802 in py::iter(&(v_care.clone()))? { let mut v_m: Py; 
v_m = t3802;
out.push(Py::tuple(vec![v_m.clone(), Py::Int(0i64)])); } Py::list(out) })?;
    v_primes = py::set(&Py::list(vec![]))?;
    while (v_groups.clone()).truthy() {
        let t3803 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, py::set(&Py::list(vec![]))?])).items()?;
        if t3803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_merged = t3803[0].clone();
        v_nxt = t3803[1].clone();
        v_items = py::sorted(&v_groups.clone(), (Py::Bool(false)).truthy())?;
        v_by_mask = Py::dict(vec![]);
        for t3804 in py::iter(&(v_items.clone()))? {
            let t3805 = (t3804).items()?;
            if t3805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t3805[0].clone();
            v_d = t3805[1].clone();
            let _ = py::method(&py::method(&v_by_mask.clone(), "setdefault", &[v_d.clone(), Py::list(vec![])])?, "append", &[v_v.clone()])?;
        }
        for t3806 in py::iter(&(py::method(&v_by_mask.clone(), "items", &[])?))? {
            let t3813 = (t3806).items()?;
            if t3813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_d = t3813[0].clone();
            v_vals = t3813[1].clone();
            v_vs = py::set(&v_vals.clone())?;
            for t3807 in py::iter(&(v_vals.clone()))? {
                v_v = t3807;
                for t3808 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_b = t3808;
                    v_bit = py::lshift(&Py::Int(1i64), &v_b.clone())?;
                    if ({ let t3812 = py::not(&py::bitand(&v_d.clone(), &v_bit.clone())?); if !t3812.truthy() { t3812 } else { { let t3811 = py::not(&py::bitand(&v_v.clone(), &v_bit.clone())?); if !t3811.truthy() { t3811 } else { { let t3809 = py::bitor(&v_v.clone(), &v_bit.clone())?; Py::Bool({ let t3810 = v_vs.clone(); if py::contains(&t3810, &t3809)? { true } else { false } }) } } } } }).truthy() {
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
    for t3106 in py::iter(&(v_pairs.clone()))? {
        let t3114 = (t3106).items()?;
        if t3114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3114[0].clone();
        v_v = t3114[1].clone();
        v_val = (if (Py::Bool(matches!(v_v.clone(), Py::Bool(_)))).truthy() { v_v.clone() } else { { let t3107 = py::method(&Py::str(&py::to_str(&v_v.clone())), "lower", &[])?; Py::Bool({ let t3108 = Py::tuple(vec![Py::str("true"), Py::str("t"), Py::str("1")]); if py::contains(&t3108, &t3107)? { true } else { false } }) } });
        if ({ let t3113 = { let t3109 = v_k.clone(); Py::Bool({ let t3110 = v_env.clone(); if py::contains(&t3110, &t3109)? { true } else { false } }) }; if !t3113.truthy() { t3113 } else { { let t3111 = py::getitem(&v_env.clone(), &v_k.clone())?; Py::Bool({ let t3112 = v_val.clone(); if py::ne(&t3111, &t3112) { true } else { false } }) } } }).truthy() {
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
    if ({ let t5971 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t5972 = Py::Int(0i64); if py::eq(&t5971, &t5972) { true } else { false } }) }).truthy() {
        return Ok(Py::Int(2i64));
    }
    v_c = Py::Int(1i64);
    while (Py::Bool(true)).truthy() {
        let t5973 = Py::Int(2i64);
        v_x = t5973.clone();
        v_y = t5973.clone();
        v_d = Py::Int(1i64);
        while ({ let t5974 = v_d.clone(); Py::Bool({ let t5975 = Py::Int(1i64); if py::eq(&t5974, &t5975) { true } else { false } }) }).truthy() {
            v_x = py::modulo(&py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &v_c.clone())?, &v_n.clone())?;
            v_y = py::modulo(&py::add(&py::mul(&v_y.clone(), &v_y.clone())?, &v_c.clone())?, &v_n.clone())?;
            v_y = py::modulo(&py::add(&py::mul(&v_y.clone(), &v_y.clone())?, &v_c.clone())?, &v_n.clone())?;
            v_d = py::gcd(&py::abs(&py::sub(&v_x.clone(), &v_y.clone())?)?, &v_n.clone())?;
        }
        if ({ let t5976 = v_d.clone(); Py::Bool({ let t5977 = v_n.clone(); if py::ne(&t5976, &t5977) { true } else { false } }) }).truthy() {
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
    for t5866 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) {
        v_r = t5866;
        gen_out.push(Py::tuple(vec![v_r.clone(), { let mut out = vec![]; for t5867 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t5868 = (t5867).items()?;
if t5868.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5868[0].clone();
v_v = t5868[1].clone();
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
    if (Py::Bool(({ let mut out = vec![]; for t3814 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3814;
out.push({ let t3815 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; Py::Bool({ let t3816 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t3815, &t3816) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::dict(vec![(Py::str("expression"), Py::str("1")), (Py::str("terms"), Py::Int(1i64)), (Py::str("literals"), Py::Int(0i64))]));
    }
    v_texts = { let mut out = vec![]; for t3817 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3817;
out.push(py::method(&Py::str(" & "), "join", &[f__term_text(v_p.clone(), v_names.clone(), Py::Bool(false))?])?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("expression"), py::method(&Py::str(" | "), "join", &[v_texts.clone()])?), (Py::str("terms"), py::len(&v_cover.clone())?), (Py::str("literals"), py::sum(&{ let mut out = vec![]; for t3822 in py::iter(&(v_cover.clone()))? { let mut v_p: Py; 
v_p = t3822;
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
    let t5864 = (f__parse_formula(v_text.clone())?).items()?;
    if t5864.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tree = t5864[0].clone();
    v_vs = t5864[1].clone();
    v_names = { let t5865 = v_names.clone(); if t5865.truthy() { t5865 } else { v_vs.clone() } };
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t5869 in py::iter(&(f__rows(v_names.clone())?))? { let mut v__r: Py; let mut v_env: Py; 
let t5870 = (t5869).items()?;
if t5870.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__r = t5870[0].clone();
v_env = t5870[1].clone();
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
    for t3818 in py::iter(&(py::enumerate(&v_names.clone(), 0)?))? {
        let t3821 = (t3818).items()?;
        if t3821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3821[0].clone();
        v_v = t3821[1].clone();
        v_bit = py::lshift(&Py::Int(1i64), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?)?;
        if (py::not(&py::bitand(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_bit.clone())?)).truthy() {
            v_pos = { let t3819 = Py::Bool((py::bitand(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_bit.clone())?).truthy()); Py::Bool({ let t3820 = v_negate.clone(); if py::ne(&t3819, &t3820) { true } else { false } }) };
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
    if ({ let t162 = v_v.clone(); Py::Bool({ let t163 = Py::Int(0i64); if py::eq(&t162, &t163) { true } else { false } }) }).truthy() {
        return Ok(Py::str("0"));
    }
    let t166 = (Py::tuple(vec![{ let t164 = v_v.clone(); Py::Bool({ let t165 = Py::Int(0i64); if py::lt(&t164, &t165)? { true } else { false } }) }, py::abs(&v_v.clone())?, Py::list(vec![])])).items()?;
    if t166.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_neg = t166[0].clone();
    v_v = t166[1].clone();
    v_out = t166[2].clone();
    while (v_v.clone()).truthy() {
        let t167 = (Py::tuple(vec![py::floordiv(&v_v.clone(), &v_b.clone())?, py::modulo(&v_v.clone(), &v_b.clone())?])).items()?;
        if t167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t167[0].clone();
        v_r = t167[1].clone();
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
    if ({ let t5504 = v_p.clone(); Py::Bool({ let t5505 = Py::Int(2i64); if py::eq(&t5504, &t5505) { true } else { false } }) }).truthy() {
        return Ok(py::modulo(&v_a.clone(), &Py::Int(2i64))?);
    }
    let t5506 = (Py::tuple(vec![py::sub(&v_p.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
    if t5506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t5506[0].clone();
    v_s = t5506[1].clone();
    while ({ let t5507 = py::modulo(&v_q.clone(), &Py::Int(2i64))?; Py::Bool({ let t5508 = Py::Int(0i64); if py::eq(&t5507, &t5508) { true } else { false } }) }).truthy() {
        v_q = py::floordiv(&v_q, &Py::Int(2i64))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    v_z = Py::Int(2i64);
    while ({ let t5509 = py::pow_mod(&v_z.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?; Py::Bool({ let t5510 = py::sub(&v_p.clone(), &Py::Int(1i64))?; if py::ne(&t5509, &t5510) { true } else { false } }) }).truthy() {
        v_z = py::iadd(&v_z, &Py::Int(1i64))?;
    }
    let t5511 = (Py::tuple(vec![v_s.clone(), py::pow_mod(&v_z.clone(), &v_q.clone(), &v_p.clone())?, py::pow_mod(&v_a.clone(), &v_q.clone(), &v_p.clone())?, py::pow_mod(&v_a.clone(), &py::floordiv(&py::add(&v_q.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?])).items()?;
    if t5511.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5511[0].clone();
    v_c = t5511[1].clone();
    v_t = t5511[2].clone();
    v_r = t5511[3].clone();
    while ({ let t5516 = v_t.clone(); Py::Bool({ let t5517 = Py::Int(1i64); if py::ne(&t5516, &t5517) { true } else { false } }) }).truthy() {
        let t5512 = (Py::tuple(vec![Py::Int(0i64), v_t.clone()])).items()?;
        if t5512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t5512[0].clone();
        v_t2 = t5512[1].clone();
        while ({ let t5513 = v_t2.clone(); Py::Bool({ let t5514 = Py::Int(1i64); if py::ne(&t5513, &t5514) { true } else { false } }) }).truthy() {
            v_t2 = py::modulo(&py::mul(&v_t2.clone(), &v_t2.clone())?, &v_p.clone())?;
            v_i = py::iadd(&v_i, &Py::Int(1i64))?;
        }
        v_b = py::pow_mod(&v_c.clone(), &py::lshift(&Py::Int(1i64), &py::sub(&py::sub(&v_m.clone(), &v_i.clone())?, &Py::Int(1i64))?)?, &v_p.clone())?;
        let t5515 = (Py::tuple(vec![v_i.clone(), py::modulo(&py::mul(&v_b.clone(), &v_b.clone())?, &v_p.clone())?, py::modulo(&py::mul(&py::mul(&v_t.clone(), &v_b.clone())?, &v_b.clone())?, &v_p.clone())?, py::modulo(&py::mul(&v_r.clone(), &v_b.clone())?, &v_p.clone())?])).items()?;
        if t5515.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_m = t5515[0].clone();
        v_c = t5515[1].clone();
        v_t = t5515[2].clone();
        v_r = t5515[3].clone();
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
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t5073 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5073;
out.push(ax_engine::pyre::module_call("fullmatch", &Py::str("\\d+"), &[v_v.clone()], 0)?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the vertices must be numbered 1..n (or 0..n-1)"))));
    }
    v_labels = py::sorted(&{ let mut out = vec![]; for t5074 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5074;
out.push(py::int(&v_v.clone())?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_labels.clone())?;
    if ({ let t5075 = v_labels.clone(); Py::Bool({ let t5076 = Py::list((Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::eq(&t5075, &t5076) { true } else { false } }) }).truthy() {
        v_off = Py::Int(1i64);
    } else {
        if ({ let t5077 = v_labels.clone(); Py::Bool({ let t5078 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::eq(&t5077, &t5078) { true } else { false } }) }).truthy() {
            v_off = Py::Int(0i64);
        } else {
            return Err(py::value_error(py::to_str(&Py::str("the vertices must be numbered 1..n (or 0..n-1)"))));
        }
    }
    if ({ let t5092 = { let t5079 = py::floordiv(&py::sum(&{ let mut out = vec![]; for t5081 in py::iter(&(py::method(&v_adj.clone(), "values", &[])?))? { let mut v_nb: Py; 
v_nb = t5081;
out.push(py::len(&v_nb.clone())?); } Py::list(out) }, None)?, &Py::Int(2i64))?; Py::Bool({ let t5080 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t5079, &t5080) { true } else { false } }) }; if t5092.truthy() { t5092 } else { { let t5082 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t5083 = Py::Int(1i64); if py::ne(&t5082, &t5083) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph is not a tree"))));
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t5093 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t5094 = (t5093).items()?;
if t5094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5094[0].clone();
v_nb = t5094[1].clone();
out.push((py::int(&v_u.clone())?, py::set(&{ let mut out = vec![]; for t5095 in py::iter(&(v_nb.clone()))? { let mut v_v: Py; 
v_v = t5095;
out.push(py::int(&v_v.clone())?); } Py::list(out) })?)); } Py::dict(out) }, v_off.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trial_factor(mut v_n: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4919 = (Py::tuple(vec![Py::dict(vec![]), Py::Int(2i64)])).items()?;
    if t4919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_out = t4919[0].clone();
    v_p = t4919[1].clone();
    while ({ let t4924 = py::mul(&v_p.clone(), &v_p.clone())?; Py::Bool({ let t4925 = v_n.clone(); if py::le(&t4924, &t4925)? { true } else { false } }) }).truthy() {
        while ({ let t4920 = py::modulo(&v_n.clone(), &v_p.clone())?; Py::Bool({ let t4921 = Py::Int(0i64); if py::eq(&t4920, &t4921) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &v_p.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_p.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
            v_n = py::floordiv(&v_n, &v_p.clone())?;
        }
        v_p = py::iadd(&v_p, &(if ({ let t4922 = v_p.clone(); Py::Bool({ let t4923 = Py::Int(2i64); if py::eq(&t4922, &t4923) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }))?;
    }
    if ({ let t4926 = v_n.clone(); Py::Bool({ let t4927 = Py::Int(1i64); if py::gt(&t4926, &t4927)? { true } else { false } }) }).truthy() {
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
    for t352 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_s = t352;
        if ({ let t353 = v_s.clone(); Py::Bool({ let t354 = v_col.clone(); if py::contains(&t354, &t353)? { true } else { false } }) }).truthy() {
            continue;
        }
        py::setitem(&v_col.clone(), &v_s.clone(), Py::Int(0i64))?;
        v_stack = Py::list(vec![v_s.clone()]);
        while (v_stack.clone()).truthy() {
            v_u = py::method(&v_stack.clone(), "pop", &[])?;
            for t355 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t355;
                if ({ let t356 = v_v.clone(); Py::Bool({ let t357 = v_col.clone(); if !py::contains(&t357, &t356)? { true } else { false } }) }).truthy() {
                    py::setitem(&v_col.clone(), &v_v.clone(), py::sub(&Py::Int(1i64), &py::getitem(&v_col.clone(), &v_u.clone())?)?)?;
                    let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
                } else {
                    if ({ let t358 = py::getitem(&v_col.clone(), &v_v.clone())?; Py::Bool({ let t359 = py::getitem(&v_col.clone(), &v_u.clone())?; if py::eq(&t358, &t359) { true } else { false } }) }).truthy() {
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
    v_want = f__sorted({ let mut out = vec![]; for t118 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t118;
if ({ let t119 = f__ncomp(v_adj.clone(), v_v.clone(), Py::None)?; Py::Bool({ let t120 = v_base.clone(); if py::gt(&t119, &t120)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?;
    return Ok({ let t127 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t127.truthy() { t127 } else { { let t126 = { let t122 = py::getitem(&v_o.clone(), &Py::str("points"))?; Py::Bool({ let t123 = v_want.clone(); if py::eq(&t122, &t123) { true } else { false } }) }; if !t126.truthy() { t126 } else { { let t124 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t125 = py::len(&v_want.clone())?; if py::eq(&t124, &t125) { true } else { false } }) } } } } });
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
    let t169 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("from_base"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("to_base"))?)?])).items()?;
    if t169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fb = t169[0].clone();
    v_tb = t169[1].clone();
    v_neg = py::method(&v_s.clone(), "startswith", &[Py::str("-")])?;
    v_val = py::sum(&{ let mut out = vec![]; for t170 in py::iter(&(py::enumerate(&py::reversed(&py::method(&v_s.clone(), "lstrip", &[Py::str("-")])?)?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t171 = (t170).items()?;
if t171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t171[0].clone();
v_c = t171[1].clone();
out.push(py::mul(&py::method(&c__DIG()?, "index", &[v_c.clone()])?, &py::pow(&v_fb.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?;
    v_val = (if (v_neg.clone()).truthy() { py::neg(&v_val.clone())? } else { v_val.clone() });
    v_r = py::getitem(&v_o.clone(), &Py::str("result"))?;
    v_rn = py::method(&v_r.clone(), "startswith", &[Py::str("-")])?;
    v_back = py::sum(&{ let mut out = vec![]; for t172 in py::iter(&(py::enumerate(&py::reversed(&py::method(&v_r.clone(), "lstrip", &[Py::str("-")])?)?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t173 = (t172).items()?;
if t173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t173[0].clone();
v_c = t173[1].clone();
out.push(py::mul(&py::method(&c__DIG()?, "index", &[v_c.clone()])?, &py::pow(&v_tb.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?;
    v_canon = { let t178 = { let t174 = v_r.clone(); Py::Bool({ let t175 = Py::str("0"); if py::eq(&t174, &t175) { true } else { false } }) }; if t178.truthy() { t178 } else { { let t176 = py::getitem(&py::method(&v_r.clone(), "lstrip", &[Py::str("-")])?, &Py::Int(0i64))?; Py::Bool({ let t177 = Py::str("0"); if py::ne(&t176, &t177) { true } else { false } }) } } };
    return Ok({ let t190 = f__iv(py::getitem(&v_o.clone(), &Py::str("decimal"))?)?; if !t190.truthy() { t190 } else { { let t189 = { let t180 = py::getitem(&v_o.clone(), &Py::str("decimal"))?; Py::Bool({ let t181 = v_val.clone(); if py::eq(&t180, &t181) { true } else { false } }) }; if !t189.truthy() { t189 } else { { let t188 = { let t182 = (if (v_rn.clone()).truthy() { py::neg(&v_back.clone())? } else { v_back.clone() }); Py::Bool({ let t183 = v_val.clone(); if py::eq(&t182, &t183) { true } else { false } }) }; if !t188.truthy() { t188 } else { { let t187 = v_canon.clone(); if !t187.truthy() { t187 } else { Py::Bool(({ let mut out = vec![]; for t184 in py::iter(&(py::method(&v_r.clone(), "lstrip", &[Py::str("-")])?))? { let mut v_c: Py; 
v_c = t184;
out.push({ let t185 = py::method(&c__DIG()?, "index", &[v_c.clone()])?; Py::Bool({ let t186 = v_tb.clone(); if py::lt(&t185, &t186)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } } } });
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
    for t216 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_m = t216;
        let _ = py::method(&v_b.clone(), "append", &[py::sum(&{ let mut out = vec![]; for t217 in py::range_iter(&[(py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t217;
out.push(py::mul(&py::comb(&v_m.clone(), &v_j.clone())?, &py::getitem(&v_b.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?])?;
    }
    return Ok({ let t221 = f__iv(py::getitem(&v_o.clone(), &Py::str("bell"))?)?; if !t221.truthy() { t221 } else { { let t219 = py::getitem(&v_o.clone(), &Py::str("bell"))?; Py::Bool({ let t220 = py::getitem(&v_b.clone(), &v_n.clone())?; if py::eq(&t219, &t220) { true } else { false } }) } } });
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
    for t247 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t247;
        py::setitem(&v_a.clone(), &v_m.clone(), py::fraction(&[Py::Int(1i64), py::add(&v_m.clone(), &Py::Int(1i64))?])?)?;
        for t248 in py::range_iter(&[(v_m.clone()).i()?, (Py::Int(0i64)).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_j = t248;
            py::setitem(&v_a.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?, py::mul(&v_j.clone(), &py::sub(&py::getitem(&v_a.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_a.clone(), &v_j.clone())?)?)?)?;
        }
    }
    v_b = (if ({ let t249 = v_n.clone(); Py::Bool({ let t250 = Py::Int(1i64); if py::eq(&t249, &t250) { true } else { false } }) }).truthy() { py::neg(&py::getitem(&v_a.clone(), &Py::Int(0i64))?)? } else { py::getitem(&v_a.clone(), &Py::Int(0i64))? });
    return Ok({ let t261 = f__iv(py::getitem(&v_o.clone(), &Py::str("numerator"))?)?; if !t261.truthy() { t261 } else { { let t260 = f__iv(py::getitem(&v_o.clone(), &Py::str("denominator"))?)?; if !t260.truthy() { t260 } else { { let t259 = { let t252 = py::fraction(&[py::getitem(&v_o.clone(), &Py::str("numerator"))?, py::getitem(&v_o.clone(), &Py::str("denominator"))?])?; Py::Bool({ let t253 = v_b.clone(); if py::eq(&t252, &t253) { true } else { false } }) }; if !t259.truthy() { t259 } else { { let t258 = { let t254 = py::getitem(&v_o.clone(), &Py::str("denominator"))?; Py::Bool({ let t255 = Py::Int(0i64); if py::gt(&t254, &t255)? { true } else { false } }) }; if !t258.truthy() { t258 } else { { let t256 = py::getitem(&v_o.clone(), &Py::str("value"))?; Py::Bool({ let t257 = py::float(&v_b.clone())?; if py::eq(&t256, &t257) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_bfs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_order: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool((py::method(&v_i.clone(), "get", &[Py::str("directed")])?).truthy()), Py::Bool(false))?;
    let t297 = (f__bfs(v_adj.clone(), py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, Py::Bool(false))?).items()?;
    if t297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_order = t297[0].clone();
    v_dist = t297[1].clone();
    return Ok({ let t307 = { let t298 = py::getitem(&v_o.clone(), &Py::str("order"))?; Py::Bool({ let t299 = v_order.clone(); if py::eq(&t298, &t299) { true } else { false } }) }; if !t307.truthy() { t307 } else { { let t306 = f__iv(py::getitem(&v_o.clone(), &Py::str("reached"))?)?; if !t306.truthy() { t306 } else { { let t305 = { let t301 = py::getitem(&v_o.clone(), &Py::str("reached"))?; Py::Bool({ let t302 = py::len(&v_order.clone())?; if py::eq(&t301, &t302) { true } else { false } }) }; if !t305.truthy() { t305 } else { { let t303 = py::getitem(&v_o.clone(), &Py::str("distances"))?; Py::Bool({ let t304 = v_dist.clone(); if py::eq(&t303, &t304) { true } else { false } }) } } } } } } });
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
    let t379 = (Py::tuple(vec![Py::dict(vec![]), Py::Bool(true)])).items()?;
    if t379.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_side = t379[0].clone();
    v_ok = t379[1].clone();
    for t380 in py::iter(&(v_adj.clone()))? {
        v_s = t380;
        if ({ let t381 = v_s.clone(); Py::Bool({ let t382 = v_side.clone(); if py::contains(&t382, &t381)? { true } else { false } }) }).truthy() {
            continue;
        }
        py::setitem(&v_side.clone(), &v_s.clone(), Py::Int(1i64))?;
        v_dq = Py::list((Py::list(vec![v_s.clone()])).items()?);
        while ({ let t388 = v_dq.clone(); if !t388.truthy() { t388 } else { v_ok.clone() } }).truthy() {
            v_u = py::method(&v_dq.clone(), "popleft", &[])?;
            for t383 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t383;
                if ({ let t384 = v_v.clone(); Py::Bool({ let t385 = v_side.clone(); if !py::contains(&t385, &t384)? { true } else { false } }) }).truthy() {
                    py::setitem(&v_side.clone(), &v_v.clone(), py::neg(&py::getitem(&v_side.clone(), &v_u.clone())?)?)?;
                    let _ = py::method(&v_dq.clone(), "append", &[v_v.clone()])?;
                } else {
                    if ({ let t386 = py::getitem(&v_side.clone(), &v_v.clone())?; Py::Bool({ let t387 = py::getitem(&v_side.clone(), &v_u.clone())?; if py::eq(&t386, &t387) { true } else { false } }) }).truthy() {
                        v_ok = Py::Bool(false);
                    }
                }
            }
        }
    }
    return Ok({ let t389 = py::getitem(&v_o.clone(), &Py::str("is_bipartite"))?; Py::Bool({ let t390 = v_ok.clone(); if py::is(&t389, &t390) { true } else { false } }) });
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
    v_want = py::sorted_key(&{ let mut out = vec![]; for t544 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t544;
if ({ let t545 = f__ncomp(v_adj.clone(), Py::None, py::set(&Py::list(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))]))?)?; Py::Bool({ let t546 = v_base.clone(); if py::gt(&t545, &t546)? { true } else { false } }) }).truthy() { out.push(py::sorted_key(&Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))]), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?); }} Py::list(out) }, &{ let v_adj = v_adj.clone(); let v_base = v_base.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![f__nkey(py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, f__nkey(py::getitem(&v_e.clone(), &Py::Int(1i64))?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t553 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t553.truthy() { t553 } else { { let t552 = { let t548 = py::getitem(&v_o.clone(), &Py::str("bridges"))?; Py::Bool({ let t549 = v_want.clone(); if py::eq(&t548, &t549) { true } else { false } }) }; if !t552.truthy() { t552 } else { { let t550 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t551 = py::len(&v_want.clone())?; if py::eq(&t550, &t551) { true } else { false } }) } } } } });
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
    let t667 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("lambda"))?])).items()?;
    if t667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t667[0].clone();
    v_lam = t667[1].clone();
    if ({ let t671 = py::not(&f__iv(v_lam.clone())?); if t671.truthy() { t671 } else { { let t669 = v_lam.clone(); Py::Bool({ let t670 = Py::Int(1i64); if py::lt(&t669, &t670)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_f = (if ({ let t672 = v_n.clone(); Py::Bool({ let t673 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::lt(&t672, &t673)? { true } else { false } }) }).truthy() { f__trial_factor(v_n.clone())? } else { f__factor(v_n.clone())? });
    v_want = Py::Int(1i64);
    for t683 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? {
        let t689 = (t683).items()?;
        if t689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t689[0].clone();
        v_e = t689[1].clone();
        v_l = py::mul(&py::pow(&v_p.clone(), &py::sub(&v_e.clone(), &Py::Int(1i64))?)?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)?;
        if ({ let t688 = { let t684 = v_p.clone(); Py::Bool({ let t685 = Py::Int(2i64); if py::eq(&t684, &t685) { true } else { false } }) }; if !t688.truthy() { t688 } else { { let t686 = v_e.clone(); Py::Bool({ let t687 = Py::Int(3i64); if py::ge(&t686, &t687)? { true } else { false } }) } } }).truthy() {
            v_l = py::floordiv(&v_l, &Py::Int(2i64))?;
        }
        v_want = py::gcd_lcm("lcm", &[v_want.clone(), v_l.clone()])?;
    }
    return Ok({ let t697 = { let t690 = v_lam.clone(); Py::Bool({ let t691 = v_want.clone(); if py::eq(&t690, &t691) { true } else { false } }) }; if !t697.truthy() { t697 } else { Py::Bool(({ let mut out = vec![]; for t692 in py::range_iter(&[(Py::Int(2i64)).i()?, (Py::Int(30i64)).i()?]) { let mut v_a: Py; 
v_a = t692;
if ({ let t693 = py::gcd(&v_a.clone(), &v_n.clone())?; Py::Bool({ let t694 = Py::Int(1i64); if py::eq(&t693, &t694) { true } else { false } }) }).truthy() { out.push({ let t695 = py::pow_mod(&v_a.clone(), &v_lam.clone(), &v_n.clone())?; Py::Bool({ let t696 = py::modulo(&Py::Int(1i64), &v_n.clone())?; if py::eq(&t695, &t696) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_catalan(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_c = Py::Int(1i64);
    for t721 in py::range_iter(&[(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?).i()?]) {
        v_k = t721;
        v_c = py::floordiv(&py::mul(&py::mul(&v_c.clone(), &Py::Int(2i64))?, &py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?)?, &py::add(&v_k.clone(), &Py::Int(2i64))?)?;
    }
    return Ok({ let t725 = f__iv(py::getitem(&v_o.clone(), &Py::str("catalan"))?)?; if !t725.truthy() { t725 } else { { let t723 = py::getitem(&v_o.clone(), &Py::str("catalan"))?; Py::Bool({ let t724 = v_c.clone(); if py::eq(&t723, &t724) { true } else { false } }) } } });
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
            while ({ let t1018 = py::method(&v_parent.clone(), "setdefault", &[v_x.clone(), v_x.clone()])?; Py::Bool({ let t1019 = v_x.clone(); if py::ne(&t1018, &t1019) { true } else { false } }) }).truthy() {
                py::setitem(&v_parent.clone(), &v_x.clone(), py::getitem(&v_parent.clone(), &py::getitem(&v_parent.clone(), &v_x.clone())?)?)?;
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t1020 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? {
        v_e = t1020;
        py::setitem(&v_parent.clone(), &py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?))])?, py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])?)?;
    }
    v_groups = Py::dict(vec![]);
    for t1021 in py::iter(&(Py::list((v_parent.clone()).items()?)))? {
        v_x = t1021;
        let _ = py::method(&py::method(&v_groups.clone(), "setdefault", &[py::call(&v_find.clone(), &[v_x.clone()])?, Py::list(vec![])])?, "append", &[v_x.clone()])?;
    }
    v_comps = py::sorted_key(&{ let mut out = vec![]; for t1022 in py::iter(&(py::method(&v_groups.clone(), "values", &[])?))? { let mut v_g: Py; 
v_g = t1022;
out.push(f__sorted(v_g.clone())?); } Py::list(out) }, &{ let v_e = v_e.clone(); let v_find = v_find.clone(); let v_groups = v_groups.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_parent = v_parent.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__nkey(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t1038 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t1038.truthy() { t1038 } else { { let t1037 = { let t1024 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1025 = py::len(&v_comps.clone())?; if py::eq(&t1024, &t1025) { true } else { false } }) }; if !t1037.truthy() { t1037 } else { { let t1036 = { let t1026 = py::getitem(&v_o.clone(), &Py::str("components"))?; Py::Bool({ let t1027 = v_comps.clone(); if py::eq(&t1026, &t1027) { true } else { false } }) }; if !t1036.truthy() { t1036 } else { { let t1035 = { let t1028 = py::getitem(&v_o.clone(), &Py::str("is_connected"))?; Py::Bool({ let t1029 = { let t1030 = py::len(&v_comps.clone())?; Py::Bool({ let t1031 = Py::Int(1i64); if py::eq(&t1030, &t1031) { true } else { false } }) }; if py::is(&t1028, &t1029) { true } else { false } }) }; if !t1035.truthy() { t1035 } else { { let t1032 = py::getitem(&v_o.clone(), &Py::str("sizes"))?; Py::Bool({ let t1033 = py::sorted(&{ let mut out = vec![]; for t1034 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t1034;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?; if py::eq(&t1032, &t1033) { true } else { false } }) } } } } } } } } });
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
            while ({ let t2569 = py::method(&v_parent.clone(), "setdefault", &[v_x.clone(), v_x.clone()])?; Py::Bool({ let t2570 = v_x.clone(); if py::ne(&t2569, &t2570) { true } else { false } }) }).truthy() {
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t2571 in py::iter(&(v_edges.clone()))? {
        v_e = t2571;
        py::setitem(&v_parent.clone(), &py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?))])?, py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])?)?;
    }
    return Ok(py::len(&py::set(&{ let mut out = vec![]; for t2572 in py::iter(&(Py::list((v_parent.clone()).items()?)))? { let mut v_x: Py; 
v_x = t2572;
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
    if ({ let t766 = py::not(&v_t.clone()); if t766.truthy() { t766 } else { { let t765 = py::not(&Py::Bool(({ let mut out = vec![]; for t751 in py::iter(&(v_t.clone()))? { let mut v_a: Py; 
v_a = t751;
out.push(f__iv(v_a.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))); if t765.truthy() { t765 } else { { let t764 = Py::Bool(({ let mut out = vec![]; for t753 in py::iter(&(py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_a: Py; 
v_a = t753;
out.push({ let t754 = v_a.clone(); Py::Bool({ let t755 = Py::Int(1i64); if py::lt(&t754, &t755)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t764.truthy() { t764 } else { { let t763 = { let t760 = { let t756 = py::len(&v_t.clone())?; Py::Bool({ let t757 = Py::Int(1i64); if py::gt(&t756, &t757)? { true } else { false } }) }; if !t760.truthy() { t760 } else { { let t758 = py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t759 = Py::Int(2i64); if py::lt(&t758, &t759)? { true } else { false } }) } } }; if t763.truthy() { t763 } else { { let t761 = py::getitem(&v_o.clone(), &Py::str("length"))?; Py::Bool({ let t762 = py::len(&v_t.clone())?; if py::ne(&t761, &t762) { true } else { false } }) } } } } } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_v = py::fraction(&[py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?])?;
    for t767 in py::iter(&(py::reversed(&py::slice(&v_t.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?))? {
        v_a = t767;
        v_v = py::add(&v_a.clone(), &py::truediv(&Py::Int(1i64), &v_v.clone())?)?;
    }
    return Ok({ let t768 = v_v.clone(); Py::Bool({ let t769 = py::fraction(&[py::int(&py::getitem(&v_i.clone(), &Py::str("numerator"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("denominator"))?)?])?; if py::eq(&t768, &t769) { true } else { false } }) });
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
    let t798 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_o.clone(), &Py::str("a0"))?, py::getitem(&v_o.clone(), &Py::str("period"))?])).items()?;
    if t798.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t798[0].clone();
    v_a0 = t798[1].clone();
    v_per = t798[2].clone();
    if (py::not(&{ let t809 = f__iv(v_a0.clone())?; if !t809.truthy() { t809 } else { { let t808 = { let t800 = v_a0.clone(); Py::Bool({ let t801 = py::isqrt(&v_d.clone())?; if py::eq(&t800, &t801) { true } else { false } }) }; if !t808.truthy() { t808 } else { { let t807 = v_per.clone(); if !t807.truthy() { t807 } else { { let t806 = { let t802 = py::getitem(&v_per.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t803 = py::mul(&Py::Int(2i64), &v_a0.clone())?; if py::eq(&t802, &t803) { true } else { false } }) }; if !t806.truthy() { t806 } else { { let t804 = py::getitem(&v_o.clone(), &Py::str("period_length"))?; Py::Bool({ let t805 = py::len(&v_per.clone())?; if py::eq(&t804, &t805) { true } else { false } }) } } } } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t810 = py::slice(&v_per.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?; Py::Bool({ let t811 = py::slice(&py::slice(&v_per.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?; if py::ne(&t810, &t811) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    let t816 = (f__convergent(py::add(&Py::list(vec![v_a0.clone()]), &py::slice(&v_per.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?)?).items()?;
    if t816.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t816[0].clone();
    v_q = t816[1].clone();
    return Ok({ let t817 = py::sub(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&py::mul(&v_d.clone(), &v_q.clone())?, &v_q.clone())?)?; Py::Bool({ let t818 = py::pow(&py::neg(&Py::Int(1i64))?, &py::len(&v_per.clone())?)?; if py::eq(&t817, &t818) { true } else { false } }) });
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
    let t859 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("chromatic_number"))?, py::getitem(&v_o.clone(), &Py::str("colouring"))?])).items()?;
    if t859.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t859[0].clone();
    v_col = t859[1].clone();
    if (py::not(&{ let t873 = f__iv(v_k.clone())?; if !t873.truthy() { t873 } else { { let t872 = { let t861 = py::set(&v_col.clone())?; Py::Bool({ let t862 = py::set(&v_adj.clone())?; if py::eq(&t861, &t862) { true } else { false } }) }; if !t872.truthy() { t872 } else { { let t871 = Py::Bool(({ let mut out = vec![]; for t863 in py::iter(&(py::method(&v_col.clone(), "values", &[])?))? { let mut v_c: Py; 
v_c = t863;
out.push({ let t864 = Py::Int(1i64); Py::Bool({ let t865 = v_c.clone(); if py::le(&t864, &t865)? { { let t866 = v_k.clone(); if py::le(&t865, &t866)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t871.truthy() { t871 } else { Py::Bool(({ let mut out = vec![]; for t867 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t867;
for t868 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t868;
out.push({ let t869 = py::getitem(&v_col.clone(), &v_u.clone())?; Py::Bool({ let t870 = py::getitem(&v_col.clone(), &v_v.clone())?; if py::ne(&t869, &t870) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } })).truthy() {
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
    return Ok({ let t878 = { let t874 = v_k.clone(); Py::Bool({ let t875 = Py::Int(1i64); if py::eq(&t874, &t875) { true } else { false } }) }; if t878.truthy() { t878 } else { { let t876 = f__colourable(v_order.clone(), v_adj.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t877 = Py::None; if py::is(&t876, &t877) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_clique(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_c = py::getitem(&v_o.clone(), &Py::str("clique"))?;
    if (py::not(&{ let t936 = f__iv(py::getitem(&v_o.clone(), &Py::str("clique_number"))?)?; if !t936.truthy() { t936 } else { { let t935 = { let t926 = py::len(&v_c.clone())?; Py::Bool({ let t927 = py::getitem(&v_o.clone(), &Py::str("clique_number"))?; if py::eq(&t926, &t927) { { let t928 = py::len(&py::set(&v_c.clone())?)?; if py::eq(&t927, &t928) { true } else { false } } } else { false } }) }; if !t935.truthy() { t935 } else { Py::Bool(({ let mut out = vec![]; for t929 in py::iter(&(v_c.clone()))? { let mut v_a: Py; 
v_a = t929;
for t930 in py::iter(&(v_c.clone()))? { let mut v_b: Py; 
v_b = t930;
if ({ let t931 = v_a.clone(); Py::Bool({ let t932 = v_b.clone(); if py::ne(&t931, &t932) { true } else { false } }) }).truthy() { out.push({ let t933 = v_b.clone(); Py::Bool({ let t934 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t934, &t933)? { true } else { false } }) }); }}} Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t937 = py::getitem(&v_o.clone(), &Py::str("clique_number"))?; Py::Bool({ let t938 = f__brute_max(py::sorted_key(&v_adj.clone(), &{ let v_adj = v_adj.clone(); let v_c = v_c.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
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
            return Ok(Py::Bool(({ let mut out = vec![]; for t943 in py::iter(&(v_ch.clone()))? { let mut v_u: Py; 
v_u = t943;
out.push({ let t944 = v_v.clone(); Py::Bool({ let t945 = py::getitem(&v_adj.clone(), &v_u.clone())?; if py::contains(&t945, &t944)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?; if py::eq(&t937, &t938) { true } else { false } }) });
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
    let t1143 = (Py::tuple(vec![Py::dict(vec![]), Py::Int(0i64)])).items()?;
    if t1143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_parent = t1143[0].clone();
    v_extra = t1143[1].clone();
    v_find = { let v_extra = v_extra.clone(); let v_find = v_find.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_parent = v_parent.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t1144 = py::method(&v_parent.clone(), "setdefault", &[v_x.clone(), v_x.clone()])?; Py::Bool({ let t1145 = v_x.clone(); if py::ne(&t1144, &t1145) { true } else { false } }) }).truthy() {
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t1146 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? {
        v_e = t1146;
        let t1147 = (Py::tuple(vec![py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?))])?, py::call(&v_find.clone(), &[Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])?])).items()?;
        if t1147.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1147[0].clone();
        v_b = t1147[1].clone();
        if ({ let t1148 = v_a.clone(); Py::Bool({ let t1149 = v_b.clone(); if py::eq(&t1148, &t1149) { true } else { false } }) }).truthy() {
            v_extra = py::iadd(&v_extra, &Py::Int(1i64))?;
        } else {
            py::setitem(&v_parent.clone(), &v_a.clone(), v_b.clone())?;
        }
    }
    return Ok({ let t1158 = f__iv(py::getitem(&v_o.clone(), &Py::str("cyclomatic_number"))?)?; if !t1158.truthy() { t1158 } else { { let t1157 = { let t1151 = py::getitem(&v_o.clone(), &Py::str("cyclomatic_number"))?; Py::Bool({ let t1152 = v_extra.clone(); if py::eq(&t1151, &t1152) { true } else { false } }) }; if !t1157.truthy() { t1157 } else { { let t1153 = py::getitem(&v_o.clone(), &Py::str("has_cycle"))?; Py::Bool({ let t1154 = { let t1155 = v_extra.clone(); Py::Bool({ let t1156 = Py::Int(0i64); if py::gt(&t1155, &t1156)? { true } else { false } }) }; if py::is(&t1153, &t1154) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_crt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1089 = (Py::tuple(vec![{ let mut out = vec![]; for t1087 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("residues"))?))? { let mut v_v: Py; 
v_v = t1087;
out.push(py::int(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1088 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("moduli"))?))? { let mut v_v: Py; 
v_v = t1088;
out.push(py::int(&v_v.clone())?); } Py::list(out) }])).items()?;
    if t1089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rs = t1089[0].clone();
    v_ms = t1089[1].clone();
    let t1090 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("modulus"))?])).items()?;
    if t1090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1090[0].clone();
    v_M = t1090[1].clone();
    return Ok({ let t1104 = f__iv(v_x.clone())?; if !t1104.truthy() { t1104 } else { { let t1103 = f__iv(v_M.clone())?; if !t1103.truthy() { t1103 } else { { let t1102 = { let t1092 = v_M.clone(); Py::Bool({ let t1093 = py::gcd_lcm("lcm", &({ let mut v: Vec<Py> = vec![]; v.extend((v_ms.clone()).items()?); v })[..])?; if py::eq(&t1092, &t1093) { true } else { false } }) }; if !t1102.truthy() { t1102 } else { { let t1101 = { let t1094 = Py::Int(0i64); Py::Bool({ let t1095 = v_x.clone(); if py::le(&t1094, &t1095)? { { let t1096 = v_M.clone(); if py::lt(&t1095, &t1096)? { true } else { false } } } else { false } }) }; if !t1101.truthy() { t1101 } else { Py::Bool(({ let mut out = vec![]; for t1097 in py::iter(&(py::zip(&[v_rs.clone(), v_ms.clone()])?))? { let mut v_m: Py; let mut v_r: Py; 
let t1098 = (t1097).items()?;
if t1098.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1098[0].clone();
v_m = t1098[1].clone();
out.push({ let t1099 = py::modulo(&py::sub(&v_x.clone(), &v_r.clone())?, &v_m.clone())?; Py::Bool({ let t1100 = Py::Int(0i64); if py::eq(&t1099, &t1100) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } } } });
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
    v_color = { let mut out = vec![]; for t1340 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1340;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    py::cell_set(&c_found, Py::Bool(false));
    let rec1341 = py::rec_cell();
    v_visit = { let v_adj = v_adj.clone(); let v_color = v_color.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let c_found = c_found.clone(); let rec1341 = rec1341.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_visit: Py = py::rec_get(&rec1341)?;
            let mut v_found: Py = Py::None;
            let mut v_v: Py = Py::None;
            py::setitem(&v_color.clone(), &v_u.clone(), Py::Int(1i64))?;
            for t1342 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t1342;
                if ({ let t1343 = py::getitem(&v_color.clone(), &v_v.clone())?; Py::Bool({ let t1344 = Py::Int(1i64); if py::eq(&t1343, &t1344) { true } else { false } }) }).truthy() {
                    py::cell_set(&c_found, Py::Bool(true));
                } else {
                    if ({ let t1345 = py::getitem(&v_color.clone(), &v_v.clone())?; Py::Bool({ let t1346 = Py::Int(0i64); if py::eq(&t1345, &t1346) { true } else { false } }) }).truthy() {
                        let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
                    }
                }
            }
            py::setitem(&v_color.clone(), &v_u.clone(), Py::Int(2i64))?;
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1341, &v_visit);
    for t1347 in py::iter(&(v_adj.clone()))? {
        v_v = t1347;
        if ({ let t1348 = py::getitem(&v_color.clone(), &v_v.clone())?; Py::Bool({ let t1349 = Py::Int(0i64); if py::eq(&t1348, &t1349) { true } else { false } }) }).truthy() {
            let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
        }
    }
    return Ok({ let t1354 = { let t1350 = py::getitem(&v_o.clone(), &Py::str("has_cycle"))?; Py::Bool({ let t1351 = py::cell_get(&c_found)?; if py::is(&t1350, &t1351) { true } else { false } }) }; if !t1354.truthy() { t1354 } else { { let t1352 = py::getitem(&v_o.clone(), &Py::str("is_dag"))?; Py::Bool({ let t1353 = py::not(&py::cell_get(&c_found)?); if py::is(&t1352, &t1353) { true } else { false } }) } } });
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
    for t1935 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? {
        v_e = t1935;
        for t1936 in py::iter(&(Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])))? {
            v_x = t1936;
            py::setitem(&v_d.clone(), &v_x.clone(), py::add(&py::method(&v_d.clone(), "get", &[v_x.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
        }
    }
    v_seq = py::sorted(&py::method(&v_d.clone(), "values", &[])?, (Py::Bool(true)).truthy())?;
    return Ok({ let t1955 = { let t1937 = py::getitem(&v_o.clone(), &Py::str("degrees"))?; Py::Bool({ let t1938 = v_d.clone(); if py::eq(&t1937, &t1938) { true } else { false } }) }; if !t1955.truthy() { t1955 } else { { let t1954 = { let t1939 = py::getitem(&v_o.clone(), &Py::str("degree_sequence"))?; Py::Bool({ let t1940 = v_seq.clone(); if py::eq(&t1939, &t1940) { true } else { false } }) }; if !t1954.truthy() { t1954 } else { { let t1953 = f__iv(py::getitem(&v_o.clone(), &Py::str("max_degree"))?)?; if !t1953.truthy() { t1953 } else { { let t1952 = { let t1942 = py::getitem(&v_o.clone(), &Py::str("max_degree"))?; Py::Bool({ let t1943 = py::getitem(&v_seq.clone(), &Py::Int(0i64))?; if py::eq(&t1942, &t1943) { true } else { false } }) }; if !t1952.truthy() { t1952 } else { { let t1951 = { let t1944 = py::getitem(&v_o.clone(), &Py::str("min_degree"))?; Py::Bool({ let t1945 = py::getitem(&v_seq.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t1944, &t1945) { true } else { false } }) }; if !t1951.truthy() { t1951 } else { { let t1950 = { let t1946 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t1947 = py::len(&py::getitem(&v_i.clone(), &Py::str("edges"))?)?; if py::eq(&t1946, &t1947) { true } else { false } }) }; if !t1950.truthy() { t1950 } else { { let t1948 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t1949 = py::len(&v_d.clone())?; if py::eq(&t1948, &t1949) { true } else { false } }) } } } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_der(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_d = py::sum(&{ let mut out = vec![]; for t1187 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1187;
out.push(py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_k.clone())?, &py::floordiv(&py::factorial(&v_n.clone())?, &py::factorial(&v_k.clone())?)?)?); } Py::list(out) }, None)?;
    return Ok({ let t1194 = f__iv(py::getitem(&v_o.clone(), &Py::str("derangements"))?)?; if !t1194.truthy() { t1194 } else { { let t1193 = { let t1189 = py::getitem(&v_o.clone(), &Py::str("derangements"))?; Py::Bool({ let t1190 = v_d.clone(); if py::eq(&t1189, &t1190) { true } else { false } }) }; if !t1193.truthy() { t1193 } else { { let t1191 = py::getitem(&v_o.clone(), &Py::str("probability"))?; Py::Bool({ let t1192 = py::float(&py::fraction(&[v_d.clone(), py::factorial(&v_n.clone())?])?)?; if py::eq(&t1191, &t1192) { true } else { false } }) } } } } });
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
    let t1224 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, Py::list(vec![])])).items()?;
    if t1224.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_seen = t1224[0].clone();
    v_order = t1224[1].clone();
    let rec1225 = py::rec_cell();
    v_visit = { let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_order = v_order.clone(); let v_s = v_s.clone(); let v_seen = v_seen.clone(); let rec1225 = rec1225.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_visit: Py = py::rec_get(&rec1225)?;
            let mut v_v: Py = Py::None;
            let _ = py::method(&v_seen.clone(), "add", &[v_u.clone()])?;
            let _ = py::method(&v_order.clone(), "append", &[v_u.clone()])?;
            for t1226 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &v_u.clone())?)?))? {
                v_v = t1226;
                if ({ let t1227 = v_v.clone(); Py::Bool({ let t1228 = v_seen.clone(); if !py::contains(&t1228, &t1227)? { true } else { false } }) }).truthy() {
                    let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1225, &v_visit);
    let _ = py::call(&v_visit.clone(), &[v_s.clone()])?;
    return Ok({ let t1235 = { let t1229 = py::getitem(&v_o.clone(), &Py::str("order"))?; Py::Bool({ let t1230 = v_order.clone(); if py::eq(&t1229, &t1230) { true } else { false } }) }; if !t1235.truthy() { t1235 } else { { let t1234 = f__iv(py::getitem(&v_o.clone(), &Py::str("reached"))?)?; if !t1234.truthy() { t1234 } else { { let t1232 = py::getitem(&v_o.clone(), &Py::str("reached"))?; Py::Bool({ let t1233 = py::len(&v_order.clone())?; if py::eq(&t1232, &t1233) { true } else { false } }) } } } } });
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
    v_idx = { let mut out = vec![]; for t2005 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t2006 = (t2005).items()?;
if t2006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2006[0].clone();
v_v = t2006[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_INF = py::pow(&Py::Int(10i64), &Py::Int(9i64))?;
    v_D = { let mut out = vec![]; for t2007 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) { let mut v_a: Py; 
v_a = t2007;
out.push({ let mut out = vec![]; for t2008 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) { let mut v_b: Py; 
v_b = t2008;
out.push((if ({ let t2009 = v_a.clone(); Py::Bool({ let t2010 = v_b.clone(); if py::eq(&t2009, &t2010) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { (if ({ let t2011 = py::getitem(&v_nodes.clone(), &v_b.clone())?; Py::Bool({ let t2012 = py::getitem(&v_adj.clone(), &py::getitem(&v_nodes.clone(), &v_a.clone())?)?; if py::contains(&t2012, &t2011)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { v_INF.clone() }) })); } Py::list(out) }); } Py::list(out) };
    for t2013 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
        v_k = t2013;
        v_Dk = py::getitem(&v_D.clone(), &v_k.clone())?;
        for t2014 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
            v_a = t2014;
            v_Da = py::getitem(&v_D.clone(), &v_a.clone())?;
            if ({ let t2015 = py::getitem(&v_Da.clone(), &v_k.clone())?; Py::Bool({ let t2016 = v_INF.clone(); if py::lt(&t2015, &t2016)? { true } else { false } }) }).truthy() {
                for t2017 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
                    v_b = t2017;
                    if ({ let t2018 = py::add(&py::getitem(&v_Da.clone(), &v_k.clone())?, &py::getitem(&v_Dk.clone(), &v_b.clone())?)?; Py::Bool({ let t2019 = py::getitem(&v_Da.clone(), &v_b.clone())?; if py::lt(&t2018, &t2019)? { true } else { false } }) }).truthy() {
                        py::setitem(&v_Da.clone(), &v_b.clone(), py::add(&py::getitem(&v_Da.clone(), &v_k.clone())?, &py::getitem(&v_Dk.clone(), &v_b.clone())?)?)?;
                    }
                }
            }
        }
    }
    v_ecc = { let mut out = vec![]; for t2020 in py::iter(&(v_D.clone()))? { let mut v_row: Py; 
v_row = t2020;
out.push(py::minmax(&[v_row.clone()], true)?); } Py::list(out) };
    let t2021 = (Py::tuple(vec![py::minmax(&[v_ecc.clone()], true)?, py::minmax(&[v_ecc.clone()], false)?])).items()?;
    if t2021.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2021[0].clone();
    v_r = t2021[1].clone();
    return Ok({ let t2040 = f__iv(py::getitem(&v_o.clone(), &Py::str("diameter"))?)?; if !t2040.truthy() { t2040 } else { { let t2039 = { let t2023 = py::getitem(&v_o.clone(), &Py::str("diameter"))?; Py::Bool({ let t2024 = v_d.clone(); if py::eq(&t2023, &t2024) { true } else { false } }) }; if !t2039.truthy() { t2039 } else { { let t2038 = { let t2025 = py::getitem(&v_o.clone(), &Py::str("radius"))?; Py::Bool({ let t2026 = v_r.clone(); if py::eq(&t2025, &t2026) { true } else { false } }) }; if !t2038.truthy() { t2038 } else { { let t2037 = { let t2027 = py::getitem(&v_o.clone(), &Py::str("center"))?; Py::Bool({ let t2028 = { let mut out = vec![]; for t2029 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2029;
if ({ let t2030 = py::getitem(&v_ecc.clone(), &py::getitem(&v_idx.clone(), &v_v.clone())?)?; Py::Bool({ let t2031 = v_r.clone(); if py::eq(&t2030, &t2031) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }; if py::eq(&t2027, &t2028) { true } else { false } }) }; if !t2037.truthy() { t2037 } else { { let t2032 = py::getitem(&v_o.clone(), &Py::str("periphery"))?; Py::Bool({ let t2033 = { let mut out = vec![]; for t2034 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2034;
if ({ let t2035 = py::getitem(&v_ecc.clone(), &py::getitem(&v_idx.clone(), &v_v.clone())?)?; Py::Bool({ let t2036 = v_d.clone(); if py::eq(&t2035, &t2036) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }; if py::eq(&t2032, &t2033) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_dioph(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t2845 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])).items()?;
    if t2845.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2845[0].clone();
    v_b = t2845[1].clone();
    v_c = t2845[2].clone();
    v_g = py::gcd(&v_a.clone(), &v_b.clone())?;
    if ({ let t2855 = { let t2846 = py::getitem(&v_o.clone(), &Py::str("solvable"))?; Py::Bool({ let t2847 = { let t2848 = py::modulo(&v_c.clone(), &v_g.clone())?; Py::Bool({ let t2849 = Py::Int(0i64); if py::eq(&t2848, &t2849) { true } else { false } }) }; if !py::is(&t2846, &t2847) { true } else { false } }) }; if t2855.truthy() { t2855 } else { { let t2854 = { let t2850 = py::getitem(&v_o.clone(), &Py::str("x_step"))?; Py::Bool({ let t2851 = py::floordiv(&v_b.clone(), &v_g.clone())?; if py::ne(&t2850, &t2851) { true } else { false } }) }; if t2854.truthy() { t2854 } else { { let t2852 = py::getitem(&v_o.clone(), &Py::str("y_step"))?; Py::Bool({ let t2853 = py::neg(&py::floordiv(&v_a.clone(), &v_g.clone())?)?; if py::ne(&t2852, &t2853) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if (py::not(&py::getitem(&v_o.clone(), &Py::str("solvable"))?)).truthy() {
        return Ok({ let t2860 = { let t2856 = py::getitem(&v_o.clone(), &Py::str("x0"))?; Py::Bool({ let t2857 = Py::None; if py::is(&t2856, &t2857) { true } else { false } }) }; if !t2860.truthy() { t2860 } else { { let t2858 = py::getitem(&v_o.clone(), &Py::str("y0"))?; Py::Bool({ let t2859 = Py::None; if py::is(&t2858, &t2859) { true } else { false } }) } } });
    }
    return Ok({ let t2869 = f__iv(py::getitem(&v_o.clone(), &Py::str("x0"))?)?; if !t2869.truthy() { t2869 } else { { let t2868 = f__iv(py::getitem(&v_o.clone(), &Py::str("y0"))?)?; if !t2868.truthy() { t2868 } else { { let t2867 = { let t2862 = py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("x0"))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("y0"))?)?)?; Py::Bool({ let t2863 = v_c.clone(); if py::eq(&t2862, &t2863) { true } else { false } }) }; if !t2867.truthy() { t2867 } else { { let t2864 = Py::Int(0i64); Py::Bool({ let t2865 = py::getitem(&v_o.clone(), &Py::str("x0"))?; if py::le(&t2864, &t2865)? { { let t2866 = py::floordiv(&py::abs(&v_b.clone())?, &v_g.clone())?; if py::lt(&t2865, &t2866)? { true } else { false } } } else { false } }) } } } } } } });
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
    v_tau = (if ({ let t1474 = v_n.clone(); Py::Bool({ let t1475 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::le(&t1474, &t1475)? { true } else { false } }) }).truthy() { py::prod(&{ let mut out = vec![]; for t1485 in py::iter(&(py::method(&f__trial_factor(v_n.clone())?, "values", &[])?))? { let mut v_e: Py; 
v_e = t1485;
out.push(py::add(&v_e.clone(), &Py::Int(1i64))?); } Py::list(out) })? } else { py::prod(&{ let mut out = vec![]; for t1486 in py::iter(&(py::method(&f__factor(v_n.clone())?, "values", &[])?))? { let mut v_e: Py; 
v_e = t1486;
out.push(py::add(&v_e.clone(), &Py::Int(1i64))?); } Py::list(out) })? });
    v_proper = py::sub(&py::getitem(&v_o.clone(), &Py::str("sum"))?, &v_n.clone())?;
    v_cls = (if ({ let t1487 = v_proper.clone(); Py::Bool({ let t1488 = v_n.clone(); if py::eq(&t1487, &t1488) { true } else { false } }) }).truthy() { Py::str("perfect") } else { (if ({ let t1489 = v_proper.clone(); Py::Bool({ let t1490 = v_n.clone(); if py::gt(&t1489, &t1490)? { true } else { false } }) }).truthy() { Py::str("abundant") } else { Py::str("deficient") }) });
    return Ok({ let t1511 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t1511.truthy() { t1511 } else { { let t1510 = { let t1492 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t1493 = v_tau.clone(); if py::eq(&t1492, &t1493) { { let t1494 = py::len(&v_ds.clone())?; if py::eq(&t1493, &t1494) { true } else { false } } } else { false } }) }; if !t1510.truthy() { t1510 } else { { let t1509 = { let t1495 = v_ds.clone(); Py::Bool({ let t1496 = py::sorted(&py::set(&v_ds.clone())?, (Py::Bool(false)).truthy())?; if py::eq(&t1495, &t1496) { true } else { false } }) }; if !t1509.truthy() { t1509 } else { { let t1508 = Py::Bool(({ let mut out = vec![]; for t1497 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t1497;
out.push({ let t1498 = py::modulo(&v_n.clone(), &v_d.clone())?; Py::Bool({ let t1499 = Py::Int(0i64); if py::eq(&t1498, &t1499) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1508.truthy() { t1508 } else { { let t1507 = { let t1500 = py::getitem(&v_o.clone(), &Py::str("sum"))?; Py::Bool({ let t1501 = py::sum(&v_ds.clone(), None)?; if py::eq(&t1500, &t1501) { true } else { false } }) }; if !t1507.truthy() { t1507 } else { { let t1506 = { let t1502 = py::getitem(&v_o.clone(), &Py::str("proper_sum"))?; Py::Bool({ let t1503 = v_proper.clone(); if py::eq(&t1502, &t1503) { true } else { false } }) }; if !t1506.truthy() { t1506 } else { { let t1504 = py::getitem(&v_o.clone(), &Py::str("classification"))?; Py::Bool({ let t1505 = v_cls.clone(); if py::eq(&t1504, &t1505) { true } else { false } }) } } } } } } } } } } } } });
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
    let t1394 = (Py::tuple(vec![py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("base"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?)?, py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("target"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?])).items()?;
    if t1394.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1394[0].clone();
    v_t = t1394[1].clone();
    v_m = t1394[2].clone();
    v_x = py::getitem(&v_o.clone(), &Py::str("x"))?;
    if (py::not(&{ let t1401 = f__iv(v_x.clone())?; if !t1401.truthy() { t1401 } else { { let t1400 = { let t1396 = v_x.clone(); Py::Bool({ let t1397 = Py::Int(0i64); if py::ge(&t1396, &t1397)? { true } else { false } }) }; if !t1400.truthy() { t1400 } else { { let t1398 = py::pow_mod(&v_b.clone(), &v_x.clone(), &v_m.clone())?; Py::Bool({ let t1399 = v_t.clone(); if py::eq(&t1398, &t1399) { true } else { false } }) } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_cur = py::modulo(&Py::Int(1i64), &v_m.clone())?;
    for t1402 in py::range_iter(&[(v_x.clone()).i()?]) {
        v__ = t1402;
        if ({ let t1403 = v_cur.clone(); Py::Bool({ let t1404 = v_t.clone(); if py::eq(&t1403, &t1404) { true } else { false } }) }).truthy() {
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
    let t1564 = ({ let mut out = vec![]; for t1563 in py::iter(&(Py::tuple(vec![Py::str("p"), Py::str("g"), Py::str("x"), Py::str("k"), Py::str("m")])))? { let mut v_s: Py; 
v_s = t1563;
out.push(py::int(&py::getitem(&v_i.clone(), &v_s.clone())?)?); } Py::list(out) }).items()?;
    if t1564.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1564[0].clone();
    v_g = t1564[1].clone();
    v_x = t1564[2].clone();
    v_k = t1564[3].clone();
    v_m = t1564[4].clone();
    return Ok({ let t1574 = f__iv(py::getitem(&v_o.clone(), &Py::str("c2"))?)?; if !t1574.truthy() { t1574 } else { { let t1573 = { let t1566 = py::getitem(&v_o.clone(), &Py::str("h"))?; Py::Bool({ let t1567 = py::pow_mod(&v_g.clone(), &v_x.clone(), &v_p.clone())?; if py::eq(&t1566, &t1567) { true } else { false } }) }; if !t1573.truthy() { t1573 } else { { let t1572 = { let t1568 = py::getitem(&v_o.clone(), &Py::str("c1"))?; Py::Bool({ let t1569 = py::pow_mod(&v_g.clone(), &v_k.clone(), &v_p.clone())?; if py::eq(&t1568, &t1569) { true } else { false } }) }; if !t1572.truthy() { t1572 } else { { let t1570 = py::modulo(&py::mul(&py::getitem(&v_o.clone(), &Py::str("c2"))?, &py::pow_mod(&py::getitem(&v_o.clone(), &Py::str("c1"))?, &py::sub(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::modulo(&v_x.clone(), &py::sub(&v_p.clone(), &Py::Int(1i64))?)?)?, &v_p.clone())?)?, &v_p.clone())?; Py::Bool({ let t1571 = v_m.clone(); if py::eq(&t1570, &t1571) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_ext(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t1734 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?])).items()?;
    if t1734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1734[0].clone();
    v_b = t1734[1].clone();
    v_g = py::getitem(&v_o.clone(), &Py::str("gcd"))?;
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t1735 in py::iter(&(Py::tuple(vec![Py::str("gcd"), Py::str("x"), Py::str("y"), Py::str("x_min"), Py::str("y_min")])))? { let mut v_k: Py; 
v_k = t1735;
out.push(f__iv(py::getitem(&v_o.clone(), &v_k.clone())?)?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t1748 = { let t1737 = v_g.clone(); Py::Bool({ let t1738 = py::gcd(&v_a.clone(), &v_b.clone())?; if py::eq(&t1737, &t1738) { true } else { false } }) }; if !t1748.truthy() { t1748 } else { { let t1747 = { let t1739 = py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("y"))?)?)?; Py::Bool({ let t1740 = v_g.clone(); if py::eq(&t1739, &t1740) { true } else { false } }) }; if !t1747.truthy() { t1747 } else { { let t1746 = { let t1741 = py::add(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("x_min"))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("y_min"))?)?)?; Py::Bool({ let t1742 = v_g.clone(); if py::eq(&t1741, &t1742) { true } else { false } }) }; if !t1746.truthy() { t1746 } else { { let t1743 = Py::Int(0i64); Py::Bool({ let t1744 = py::getitem(&v_o.clone(), &Py::str("x_min"))?; if py::le(&t1743, &t1744)? { { let t1745 = py::floordiv(&v_b.clone(), &v_g.clone())?; if py::lt(&t1744, &t1745)? { true } else { false } } } else { false } }) } } } } } } });
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
    v_ps = { let mut out = vec![]; for t4798 in py::iter(&(v_fs.clone()))? { let mut v__e: Py; let mut v_p: Py; 
let t4799 = (t4798).items()?;
if t4799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4799[0].clone();
v__e = t4799[1].clone();
out.push(v_p.clone()); } Py::list(out) };
    if ({ let t4825 = { let t4800 = v_ps.clone(); Py::Bool({ let t4801 = py::sorted(&py::set(&v_ps.clone())?, (Py::Bool(false)).truthy())?; if py::ne(&t4800, &t4801) { true } else { false } }) }; if t4825.truthy() { t4825 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4802 in py::iter(&(v_fs.clone()))? { let mut v_e: Py; let mut v_p: Py; 
let t4803 = (t4802).items()?;
if t4803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4803[0].clone();
v_e = t4803[1].clone();
out.push({ let t4824 = f__iv(v_p.clone())?; if !t4824.truthy() { t4824 } else { { let t4823 = f__iv(v_e.clone())?; if !t4823.truthy() { t4823 } else { { let t4822 = { let t4805 = v_e.clone(); Py::Bool({ let t4806 = Py::Int(1i64); if py::ge(&t4805, &t4806)? { true } else { false } }) }; if !t4822.truthy() { t4822 } else { f__is_prime_alt(v_p.clone())? } } } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t4844 = { let t4826 = py::prod(&{ let mut out = vec![]; for t4828 in py::iter(&(v_fs.clone()))? { let mut v_e: Py; let mut v_p: Py; 
let t4829 = (t4828).items()?;
if t4829.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4829[0].clone();
v_e = t4829[1].clone();
out.push(py::pow(&v_p.clone(), &v_e.clone())?); } Py::list(out) })?; Py::Bool({ let t4827 = v_n.clone(); if py::eq(&t4826, &t4827) { true } else { false } }) }; if !t4844.truthy() { t4844 } else { { let t4843 = { let t4830 = py::getitem(&v_o.clone(), &Py::str("largest_prime_factor"))?; Py::Bool({ let t4831 = py::getitem(&v_ps.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t4830, &t4831) { true } else { false } }) }; if !t4843.truthy() { t4843 } else { { let t4842 = f__iv(py::getitem(&v_o.clone(), &Py::str("largest_prime_factor"))?)?; if !t4842.truthy() { t4842 } else { { let t4841 = { let t4832 = py::getitem(&v_o.clone(), &Py::str("smallest_prime_factor"))?; Py::Bool({ let t4833 = py::getitem(&v_ps.clone(), &Py::Int(0i64))?; if py::eq(&t4832, &t4833) { true } else { false } }) }; if !t4841.truthy() { t4841 } else { { let t4840 = { let t4834 = py::getitem(&v_o.clone(), &Py::str("distinct_primes"))?; Py::Bool({ let t4835 = py::len(&v_ps.clone())?; if py::eq(&t4834, &t4835) { true } else { false } }) }; if !t4840.truthy() { t4840 } else { { let t4836 = py::getitem(&v_o.clone(), &Py::str("prime_factors_with_multiplicity"))?; Py::Bool({ let t4837 = py::sum(&{ let mut out = vec![]; for t4838 in py::iter(&(v_fs.clone()))? { let mut v__p: Py; let mut v_e: Py; 
let t4839 = (t4838).items()?;
if t4839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__p = t4839[0].clone();
v_e = t4839[1].clone();
out.push(v_e.clone()); } Py::list(out) }, None)?; if py::eq(&t4836, &t4837) { true } else { false } }) } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_gcd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_xs = { let mut out = vec![]; for t1818 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("numbers"))?))? { let mut v_v: Py; 
v_v = t1818;
out.push(py::int(&v_v.clone())?); } Py::list(out) };
    let t1819 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("gcd"))?, py::getitem(&v_o.clone(), &Py::str("lcm"))?])).items()?;
    if t1819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1819[0].clone();
    v_l = t1819[1].clone();
    if (py::not(&{ let t1827 = f__iv(v_g.clone())?; if !t1827.truthy() { t1827 } else { { let t1826 = f__iv(v_l.clone())?; if !t1826.truthy() { t1826 } else { { let t1825 = { let t1821 = v_g.clone(); Py::Bool({ let t1822 = Py::Int(1i64); if py::ge(&t1821, &t1822)? { true } else { false } }) }; if !t1825.truthy() { t1825 } else { { let t1823 = v_l.clone(); Py::Bool({ let t1824 = Py::Int(1i64); if py::ge(&t1823, &t1824)? { true } else { false } }) } } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t1842 = Py::Bool(({ let mut out = vec![]; for t1828 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1828;
out.push({ let t1829 = py::modulo(&v_x.clone(), &v_g.clone())?; Py::Bool({ let t1830 = Py::Int(0i64); if py::eq(&t1829, &t1830) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1842.truthy() { t1842 } else { { let t1841 = { let t1831 = py::reduce(&py::func(|a: &[Py]| -> R { py::gcd_lcm("gcd", a) }), &{ let mut out = vec![]; for t1833 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1833;
out.push(py::floordiv(&v_x.clone(), &v_g.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t1832 = Py::Int(1i64); if py::eq(&t1831, &t1832) { true } else { false } }) }; if !t1841.truthy() { t1841 } else { { let t1840 = Py::Bool(({ let mut out = vec![]; for t1834 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1834;
out.push({ let t1835 = py::modulo(&v_l.clone(), &v_x.clone())?; Py::Bool({ let t1836 = Py::Int(0i64); if py::eq(&t1835, &t1836) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1840.truthy() { t1840 } else { { let t1837 = py::reduce(&py::func(|a: &[Py]| -> R { py::gcd_lcm("gcd", a) }), &{ let mut out = vec![]; for t1839 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1839;
out.push(py::floordiv(&v_l.clone(), &v_x.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t1838 = Py::Int(1i64); if py::eq(&t1837, &t1838) { true } else { false } }) } } } } } } });
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
    for t1888 in py::range_iter(&[(Py::Int(3i64)).i()?, (py::add(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?).i()?]) {
        v_L = t1888;
        for t1889 in py::iter(&(v_nodes.clone()))? {
            v_s = t1889;
            v_stack = Py::list(vec![Py::tuple(vec![v_s.clone(), Py::list(vec![v_s.clone()])])]);
            while (v_stack.clone()).truthy() {
                let t1890 = (py::method(&v_stack.clone(), "pop", &[])?).items()?;
                if t1890.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_u = t1890[0].clone();
                v_path = t1890[1].clone();
                if ({ let t1891 = py::len(&v_path.clone())?; Py::Bool({ let t1892 = v_L.clone(); if py::eq(&t1891, &t1892) { true } else { false } }) }).truthy() {
                    if ({ let t1893 = v_s.clone(); Py::Bool({ let t1894 = py::getitem(&v_adj.clone(), &v_u.clone())?; if py::contains(&t1894, &t1893)? { true } else { false } }) }).truthy() {
                        return Ok({ let t1898 = { let t1895 = v_g.clone(); Py::Bool({ let t1896 = v_L.clone(); if py::eq(&t1895, &t1896) { true } else { false } }) }; if !t1898.truthy() { t1898 } else { f__iv(v_g.clone())? } });
                    }
                    continue;
                }
                for t1899 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                    v_v = t1899;
                    if ({ let t1904 = { let t1900 = v_v.clone(); Py::Bool({ let t1901 = v_path.clone(); if !py::contains(&t1901, &t1900)? { true } else { false } }) }; if !t1904.truthy() { t1904 } else { { let t1902 = f__nkey(v_v.clone())?; Py::Bool({ let t1903 = f__nkey(v_s.clone())?; if py::gt(&t1902, &t1903)? { true } else { false } }) } } }).truthy() {
                        let _ = py::method(&v_stack.clone(), "append", &[Py::tuple(vec![v_v.clone(), py::add(&v_path.clone(), &Py::list(vec![v_v.clone()]))?])])?;
                    }
                }
            }
        }
    }
    return Ok({ let t1905 = v_g.clone(); Py::Bool({ let t1906 = Py::None; if py::is(&t1905, &t1906) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_graphical(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    v_d = py::sorted(&{ let mut out = vec![]; for t2131 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("degrees"))?))? { let mut v_x: Py; 
v_x = t2131;
out.push(py::int(&v_x.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?;
    v_n = py::len(&v_d.clone())?;
    v_ok = { let t2138 = { let t2132 = py::modulo(&py::sum(&v_d.clone(), None)?, &Py::Int(2i64))?; Py::Bool({ let t2133 = Py::Int(0i64); if py::eq(&t2132, &t2133) { true } else { false } }) }; if !t2138.truthy() { t2138 } else { Py::Bool(({ let mut out = vec![]; for t2134 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2134;
out.push({ let t2135 = py::sum(&py::slice(&v_d.clone(), None, Some((v_k.clone()).i()?), None)?, None)?; Py::Bool({ let t2136 = py::add(&py::mul(&v_k.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &py::sum(&{ let mut out = vec![]; for t2137 in py::iter(&(py::slice(&v_d.clone(), Some((v_k.clone()).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t2137;
out.push(py::minmax(&[v_x.clone(), v_k.clone()], false)?); } Py::list(out) }, None)?)?; if py::le(&t2135, &t2136)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } };
    return Ok({ let t2139 = py::getitem(&v_o.clone(), &Py::str("is_graphical"))?; Py::Bool({ let t2140 = v_ok.clone(); if py::is(&t2139, &t2140) { true } else { false } }) });
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
    if ({ let t2181 = py::getitem(&v_o.clone(), &Py::str("exists"))?; Py::Bool({ let t2182 = Py::Bool(true); if py::is(&t2181, &t2182) { true } else { false } }) }).truthy() {
        v_c = py::getitem(&v_o.clone(), &Py::str("cycle"))?;
        return Ok({ let t2192 = { let t2183 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t2184 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t2183, &t2184) { { let t2185 = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?; if py::eq(&t2184, &t2185) { true } else { false } } } else { false } }) }; if !t2192.truthy() { t2192 } else { { let t2191 = { let t2186 = py::sorted_key(&py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?; Py::Bool({ let t2187 = v_nodes.clone(); if py::eq(&t2186, &t2187) { true } else { false } }) }; if !t2191.truthy() { t2191 } else { Py::Bool(({ let mut out = vec![]; for t2188 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2188;
out.push({ let t2189 = py::getitem(&v_c.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2190 = py::getitem(&v_adj.clone(), &py::getitem(&v_c.clone(), &v_k.clone())?)?; if py::contains(&t2190, &t2189)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } });
    }
    if ({ let t2197 = { let t2193 = py::getitem(&v_o.clone(), &Py::str("exists"))?; Py::Bool({ let t2194 = Py::Bool(false); if !py::is(&t2193, &t2194) { true } else { false } }) }; if t2197.truthy() { t2197 } else { { let t2195 = py::getitem(&v_o.clone(), &Py::str("cycle"))?; Py::Bool({ let t2196 = Py::None; if !py::is(&t2195, &t2196) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t2198 = v_n.clone(); Py::Bool({ let t2199 = Py::Int(3i64); if py::lt(&t2198, &t2199)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(true));
    }
    v_idx = { let mut out = vec![]; for t2200 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t2201 = (t2200).items()?;
if t2201.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2201[0].clone();
v_v = t2201[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_dp = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::lshift(&Py::Int(1i64), &v_n.clone())?)?;
    py::setitem(&v_dp.clone(), &Py::Int(1i64), Py::Int(1i64))?;
    for t2202 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) {
        v_mask = t2202;
        if ({ let t2203 = py::not(&py::getitem(&v_dp.clone(), &v_mask.clone())?); if t2203.truthy() { t2203 } else { py::not(&py::bitand(&v_mask.clone(), &Py::Int(1i64))?) } }).truthy() {
            continue;
        }
        for t2204 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_u = t2204;
            if (py::bitand(&py::rshift(&py::getitem(&v_dp.clone(), &v_mask.clone())?, &v_u.clone())?, &Py::Int(1i64))?).truthy() {
                for t2205 in py::iter(&(py::getitem(&v_adj.clone(), &py::getitem(&v_nodes.clone(), &v_u.clone())?)?))? {
                    v_v = t2205;
                    v_j = py::getitem(&v_idx.clone(), &v_v.clone())?;
                    if (py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?)).truthy() {
                        let t2206 = v_dp.clone(); let t2207 = py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_j.clone())?)?;
                        py::setitem(&t2206, &t2207, py::bitor(&py::getitem(&t2206, &t2207)?, &py::lshift(&Py::Int(1i64), &v_j.clone())?)?)?;
                    }
                }
            }
        }
    }
    v_full = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?;
    return Ok(py::not(&Py::Bool(({ let mut out = vec![]; for t2208 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) { let mut v_u: Py; 
v_u = t2208;
out.push({ let t2211 = py::bitand(&py::rshift(&py::getitem(&v_dp.clone(), &v_full.clone())?, &v_u.clone())?, &Py::Int(1i64))?; if !t2211.truthy() { t2211 } else { { let t2209 = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?; Py::Bool({ let t2210 = py::getitem(&v_adj.clone(), &py::getitem(&v_nodes.clone(), &v_u.clone())?)?; if py::contains(&t2210, &t2209)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))));
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
    v_ds = py::sorted(&py::set(&{ let mut out = vec![]; for t2248 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("divisors"))?))? { let mut v_d: Py; 
v_d = t2248;
out.push(py::int(&v_d.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?;
    if ({ let t2249 = v_n.clone(); Py::Bool({ let t2250 = py::mul(&Py::Int(3i64), &py::pow(&Py::Int(10i64), &Py::Int(5i64))?)?; if py::le(&t2249, &t2250)? { true } else { false } }) }).truthy() {
        v_want = py::sum(&{ let mut out = vec![]; for t2251 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_x: Py; 
v_x = t2251;
if (Py::Bool(({ let mut out = vec![]; for t2252 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t2252;
out.push({ let t2253 = py::modulo(&v_x.clone(), &v_d.clone())?; Py::Bool({ let t2254 = Py::Int(0i64); if py::eq(&t2253, &t2254) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    } else {
        v_want = Py::Int(0i64);
        for t2255 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_ds.clone())?, &Py::Int(1i64))?).i()?]) {
            v_r = t2255;
            for t2256 in py::iter(&(py::itertools("combinations", &[v_ds.clone(), v_r.clone()])?))? {
                v_c = t2256;
                v_want = py::iadd(&v_want, &py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::add(&v_r.clone(), &Py::Int(1i64))?)?, &py::floordiv(&v_n.clone(), &py::gcd_lcm("lcm", &({ let mut v: Vec<Py> = vec![]; v.extend((v_c.clone()).items()?); v })[..])?)?)?)?;
            }
        }
    }
    return Ok({ let t2263 = f__iv(py::getitem(&v_o.clone(), &Py::str("divisible_by_any"))?)?; if !t2263.truthy() { t2263 } else { { let t2262 = { let t2258 = py::getitem(&v_o.clone(), &Py::str("divisible_by_any"))?; Py::Bool({ let t2259 = v_want.clone(); if py::eq(&t2258, &t2259) { true } else { false } }) }; if !t2262.truthy() { t2262 } else { { let t2260 = py::getitem(&v_o.clone(), &Py::str("divisible_by_none"))?; Py::Bool({ let t2261 = py::sub(&v_n.clone(), &v_want.clone())?; if py::eq(&t2260, &t2261) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_indep(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_adj = f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?;
    v_s = py::getitem(&v_o.clone(), &Py::str("independent_set"))?;
    if (py::not(&{ let t2321 = f__iv(py::getitem(&v_o.clone(), &Py::str("independence_number"))?)?; if !t2321.truthy() { t2321 } else { { let t2320 = { let t2310 = py::len(&v_s.clone())?; Py::Bool({ let t2311 = py::getitem(&v_o.clone(), &Py::str("independence_number"))?; if py::eq(&t2310, &t2311) { { let t2312 = py::len(&py::set(&v_s.clone())?)?; if py::eq(&t2311, &t2312) { true } else { false } } } else { false } }) }; if !t2320.truthy() { t2320 } else { { let t2319 = { let t2313 = py::set(&v_s.clone())?; Py::Bool({ let t2314 = py::set(&v_adj.clone())?; if py::set_cmp(&t2313, &t2314, "<=")? { true } else { false } }) }; if !t2319.truthy() { t2319 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t2315 in py::iter(&(v_s.clone()))? { let mut v_a: Py; 
v_a = t2315;
for t2316 in py::iter(&(v_s.clone()))? { let mut v_b: Py; 
v_b = t2316;
out.push({ let t2317 = v_b.clone(); Py::Bool({ let t2318 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t2318, &t2317)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t2322 = py::getitem(&v_o.clone(), &Py::str("independence_number"))?; Py::Bool({ let t2323 = f__brute_max(py::sorted_key(&v_adj.clone(), &{ let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
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
            return Ok(Py::Bool(({ let mut out = vec![]; for t2328 in py::iter(&(v_ch.clone()))? { let mut v_u: Py; 
v_u = t2328;
out.push({ let t2329 = v_v.clone(); Py::Bool({ let t2330 = py::getitem(&v_adj.clone(), &v_u.clone())?; if !py::contains(&t2330, &t2329)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?; if py::eq(&t2322, &t2323) { true } else { false } }) });
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
    for t2360 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_part = t2360;
        for t2361 in py::range_iter(&[(v_part.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_s = t2361;
            let t2362 = v_ways.clone(); let t2363 = v_s.clone();
            py::setitem(&t2362, &t2363, py::iadd(&py::getitem(&t2362, &t2363)?, &py::getitem(&v_ways.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
        }
    }
    return Ok({ let t2367 = f__iv(py::getitem(&v_o.clone(), &Py::str("partitions"))?)?; if !t2367.truthy() { t2367 } else { { let t2365 = py::getitem(&v_o.clone(), &Py::str("partitions"))?; Py::Bool({ let t2366 = py::getitem(&v_ways.clone(), &v_n.clone())?; if py::eq(&t2365, &t2366) { true } else { false } }) } } });
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
    if ({ let t2492 = py::not(&Py::Bool(matches!(v_p.clone(), Py::Bool(_)))); if t2492.truthy() { t2492 } else { { let t2475 = v_p.clone(); Py::Bool({ let t2476 = f__is_prime_alt(v_n.clone())?; if py::ne(&t2475, &t2476) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_s = py::getitem(&v_o.clone(), &Py::str("smallest_prime_factor"))?;
    if ({ let t2493 = v_n.clone(); Py::Bool({ let t2494 = Py::Int(2i64); if py::lt(&t2493, &t2494)? { true } else { false } }) }).truthy() {
        return Ok({ let t2495 = v_s.clone(); Py::Bool({ let t2496 = Py::None; if py::is(&t2495, &t2496) { true } else { false } }) });
    }
    return Ok({ let t2506 = f__iv(v_s.clone())?; if !t2506.truthy() { t2506 } else { { let t2505 = { let t2498 = py::modulo(&v_n.clone(), &v_s.clone())?; Py::Bool({ let t2499 = Py::Int(0i64); if py::eq(&t2498, &t2499) { true } else { false } }) }; if !t2505.truthy() { t2505 } else { { let t2504 = f__is_prime_alt(v_s.clone())?; if !t2504.truthy() { t2504 } else { { let t2500 = { let t2502 = v_s.clone(); Py::Bool({ let t2503 = v_n.clone(); if py::eq(&t2502, &t2503) { true } else { false } }) }; Py::Bool({ let t2501 = v_p.clone(); if py::eq(&t2500, &t2501) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_isqrt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2393 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("isqrt"))?])).items()?;
    if t2393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2393[0].clone();
    v_r = t2393[1].clone();
    return Ok({ let t2409 = f__iv(v_r.clone())?; if !t2409.truthy() { t2409 } else { { let t2408 = { let t2395 = v_r.clone(); Py::Bool({ let t2396 = Py::Int(0i64); if py::ge(&t2395, &t2396)? { true } else { false } }) }; if !t2408.truthy() { t2408 } else { { let t2407 = { let t2397 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t2398 = v_n.clone(); if py::le(&t2397, &t2398)? { { let t2399 = py::pow(&py::add(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?; if py::lt(&t2398, &t2399)? { true } else { false } } } else { false } }) }; if !t2407.truthy() { t2407 } else { { let t2406 = { let t2400 = py::getitem(&v_o.clone(), &Py::str("remainder"))?; Py::Bool({ let t2401 = py::sub(&v_n.clone(), &py::mul(&v_r.clone(), &v_r.clone())?)?; if py::eq(&t2400, &t2401) { true } else { false } }) }; if !t2406.truthy() { t2406 } else { { let t2402 = py::getitem(&v_o.clone(), &Py::str("is_square"))?; Py::Bool({ let t2403 = { let t2404 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t2405 = v_n.clone(); if py::eq(&t2404, &t2405) { true } else { false } }) }; if py::is(&t2402, &t2403) { true } else { false } }) } } } } } } } } });
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
    let t2611 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?])).items()?;
    if t2611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2611[0].clone();
    v_n = t2611[1].clone();
    v_val = Py::Int(1i64);
    for t2650 in py::iter(&(py::method(&f__factor(v_n.clone())?, "items", &[])?))? {
        let t2651 = (t2650).items()?;
        if t2651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t2651[0].clone();
        v_e = t2651[1].clone();
        v_r = py::pow_mod(&py::modulo(&v_a.clone(), &v_p.clone())?, &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?;
        v_val = py::mul(&v_val, &py::pow(&py::getitem(&Py::dict(vec![(Py::Int(0i64), Py::Int(0i64)), (Py::Int(1i64), Py::Int(1i64)), (py::sub(&v_p.clone(), &Py::Int(1i64))?, py::neg(&Py::Int(1i64))?)]), &v_r.clone())?, &v_e.clone())?)?;
    }
    return Ok({ let t2655 = f__iv(py::getitem(&v_o.clone(), &Py::str("symbol"))?)?; if !t2655.truthy() { t2655 } else { { let t2653 = py::getitem(&v_o.clone(), &Py::str("symbol"))?; Py::Bool({ let t2654 = v_val.clone(); if py::eq(&t2653, &t2654) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_legendre(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t2748 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?])).items()?;
    if t2748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2748[0].clone();
    v_p = t2748[1].clone();
    v_e = py::pow_mod(&py::modulo(&v_a.clone(), &v_p.clone())?, &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?;
    return Ok({ let t2752 = f__iv(py::getitem(&v_o.clone(), &Py::str("symbol"))?)?; if !t2752.truthy() { t2752 } else { { let t2750 = py::getitem(&v_o.clone(), &Py::str("symbol"))?; Py::Bool({ let t2751 = py::method(&Py::dict(vec![(Py::Int(0i64), Py::Int(0i64)), (Py::Int(1i64), Py::Int(1i64)), (py::sub(&v_p.clone(), &Py::Int(1i64))?, py::neg(&Py::Int(1i64))?)]), "get", &[v_e.clone(), Py::Int(99i64)])?; if py::eq(&t2750, &t2751) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_lincong(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t2786 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?])).items()?;
    if t2786.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2786[0].clone();
    v_b = t2786[1].clone();
    v_m = t2786[2].clone();
    v_g = py::gcd(&v_a.clone(), &v_m.clone())?;
    if ({ let t2787 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2788 = (if ({ let t2789 = py::modulo(&v_b.clone(), &v_g.clone())?; Py::Bool({ let t2790 = Py::Int(0i64); if py::eq(&t2789, &t2790) { true } else { false } }) }).truthy() { v_g.clone() } else { Py::Int(0i64) }); if py::ne(&t2787, &t2788) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t2791 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2792 = Py::Int(0i64); if py::eq(&t2791, &t2792) { true } else { false } }) }).truthy() {
        return Ok({ let t2797 = { let t2793 = py::getitem(&v_o.clone(), &Py::str("smallest"))?; Py::Bool({ let t2794 = Py::None; if py::is(&t2793, &t2794) { true } else { false } }) }; if !t2797.truthy() { t2797 } else { { let t2795 = py::getitem(&v_o.clone(), &Py::str("step"))?; Py::Bool({ let t2796 = Py::None; if py::is(&t2795, &t2796) { true } else { false } }) } } });
    }
    return Ok({ let t2808 = f__iv(py::getitem(&v_o.clone(), &Py::str("smallest"))?)?; if !t2808.truthy() { t2808 } else { { let t2807 = { let t2799 = py::getitem(&v_o.clone(), &Py::str("step"))?; Py::Bool({ let t2800 = py::floordiv(&v_m.clone(), &v_g.clone())?; if py::eq(&t2799, &t2800) { true } else { false } }) }; if !t2807.truthy() { t2807 } else { { let t2806 = { let t2801 = Py::Int(0i64); Py::Bool({ let t2802 = py::getitem(&v_o.clone(), &Py::str("smallest"))?; if py::le(&t2801, &t2802)? { { let t2803 = py::getitem(&v_o.clone(), &Py::str("step"))?; if py::lt(&t2802, &t2803)? { true } else { false } } } else { false } }) }; if !t2806.truthy() { t2806 } else { { let t2804 = py::modulo(&py::sub(&py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("smallest"))?)?, &v_b.clone())?, &v_m.clone())?; Py::Bool({ let t2805 = Py::Int(0i64); if py::eq(&t2804, &t2805) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_mobius(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sq: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    v_f = (if ({ let t3944 = v_n.clone(); Py::Bool({ let t3945 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::lt(&t3944, &t3945)? { true } else { false } }) }).truthy() { f__trial_factor(v_n.clone())? } else { f__factor(v_n.clone())? });
    v_sq = Py::Bool(({ let mut out = vec![]; for t3955 in py::iter(&(py::method(&v_f.clone(), "values", &[])?))? { let mut v_e: Py; 
v_e = t3955;
out.push({ let t3956 = v_e.clone(); Py::Bool({ let t3957 = Py::Int(1i64); if py::eq(&t3956, &t3957) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()));
    return Ok({ let t3964 = f__iv(py::getitem(&v_o.clone(), &Py::str("mu"))?)?; if !t3964.truthy() { t3964 } else { { let t3963 = { let t3959 = py::getitem(&v_o.clone(), &Py::str("squarefree"))?; Py::Bool({ let t3960 = v_sq.clone(); if py::is(&t3959, &t3960) { true } else { false } }) }; if !t3963.truthy() { t3963 } else { { let t3961 = py::getitem(&v_o.clone(), &Py::str("mu"))?; Py::Bool({ let t3962 = (if (py::not(&v_sq.clone())).truthy() { Py::Int(0i64) } else { py::pow(&py::neg(&Py::Int(1i64))?, &py::len(&v_f.clone())?)? }); if py::eq(&t3961, &t3962) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_modred(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4060 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?])).items()?;
    if t4060.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4060[0].clone();
    v_m = t4060[1].clone();
    return Ok({ let t4069 = f__iv(py::getitem(&v_o.clone(), &Py::str("remainder"))?)?; if !t4069.truthy() { t4069 } else { { let t4068 = f__iv(py::getitem(&v_o.clone(), &Py::str("quotient"))?)?; if !t4068.truthy() { t4068 } else { { let t4067 = { let t4062 = Py::Int(0i64); Py::Bool({ let t4063 = py::getitem(&v_o.clone(), &Py::str("remainder"))?; if py::le(&t4062, &t4063)? { { let t4064 = v_m.clone(); if py::lt(&t4063, &t4064)? { true } else { false } } } else { false } }) }; if !t4067.truthy() { t4067 } else { { let t4065 = py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("quotient"))?, &v_m.clone())?, &py::getitem(&v_o.clone(), &Py::str("remainder"))?)?; Py::Bool({ let t4066 = v_a.clone(); if py::eq(&t4065, &t4066) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_neck(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ref: Py = Py::None;
    let mut v_rot: Py = Py::None;
    let t4319 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("beads"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("colours"))?)?])).items()?;
    if t4319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4319[0].clone();
    v_k = t4319[1].clone();
    v_rot = py::sum(&{ let mut out = vec![]; for t4320 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4320;
out.push(py::pow(&v_k.clone(), &py::gcd(&v_j.clone(), &v_n.clone())?)?); } Py::list(out) }, None)?;
    v_ref = (if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::mul(&v_n.clone(), &py::pow(&v_k.clone(), &py::floordiv(&py::add(&v_n.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?)? } else { py::mul(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &py::add(&py::pow(&v_k.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)?, &py::pow(&v_k.clone(), &py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?)? });
    return Ok({ let t4328 = f__iv(py::getitem(&v_o.clone(), &Py::str("necklaces"))?)?; if !t4328.truthy() { t4328 } else { { let t4327 = f__iv(py::getitem(&v_o.clone(), &Py::str("bracelets"))?)?; if !t4327.truthy() { t4327 } else { { let t4326 = { let t4322 = py::mul(&py::getitem(&v_o.clone(), &Py::str("necklaces"))?, &v_n.clone())?; Py::Bool({ let t4323 = v_rot.clone(); if py::eq(&t4322, &t4323) { true } else { false } }) }; if !t4326.truthy() { t4326 } else { { let t4324 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("bracelets"))?, &Py::Int(2i64))?, &v_n.clone())?; Py::Bool({ let t4325 = py::add(&v_rot.clone(), &v_ref.clone())?; if py::eq(&t4324, &t4325) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_next(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4374 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("prime"))?])).items()?;
    if t4374.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4374[0].clone();
    v_p = t4374[1].clone();
    return Ok({ let t4396 = f__iv(v_p.clone())?; if !t4396.truthy() { t4396 } else { { let t4395 = { let t4376 = v_p.clone(); Py::Bool({ let t4377 = v_n.clone(); if py::gt(&t4376, &t4377)? { true } else { false } }) }; if !t4395.truthy() { t4395 } else { { let t4394 = f__is_prime_alt(v_p.clone())?; if !t4394.truthy() { t4394 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4393 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?, (v_p.clone()).i()?]) { let mut v_k: Py; 
v_k = t4393;
out.push(f__is_prime_alt(v_k.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_order(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4197 = (Py::tuple(vec![py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("order"))?])).items()?;
    if t4197.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4197[0].clone();
    v_n = t4197[1].clone();
    v_k = t4197[2].clone();
    return Ok({ let t4217 = f__iv(v_k.clone())?; if !t4217.truthy() { t4217 } else { { let t4216 = { let t4199 = v_k.clone(); Py::Bool({ let t4200 = Py::Int(1i64); if py::ge(&t4199, &t4200)? { true } else { false } }) }; if !t4216.truthy() { t4216 } else { { let t4215 = { let t4201 = py::pow_mod(&v_a.clone(), &v_k.clone(), &v_n.clone())?; Py::Bool({ let t4202 = Py::Int(1i64); if py::eq(&t4201, &t4202) { true } else { false } }) }; if !t4215.truthy() { t4215 } else { Py::Bool(({ let mut out = vec![]; for t4212 in py::iter(&(f__trial_factor(v_k.clone())?))? { let mut v_p: Py; 
v_p = t4212;
out.push({ let t4213 = py::pow_mod(&v_a.clone(), &py::floordiv(&v_k.clone(), &v_p.clone())?, &v_n.clone())?; Py::Bool({ let t4214 = Py::Int(1i64); if py::ne(&t4213, &t4214) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } } } });
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
    for t4454 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?, (Py::Int(2i64)).i()?]) {
        v_part = t4454;
        for t4455 in py::range_iter(&[(v_part.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_s = t4455;
            let t4456 = v_w.clone(); let t4457 = v_s.clone();
            py::setitem(&t4456, &t4457, py::iadd(&py::getitem(&t4456, &t4457)?, &py::getitem(&v_w.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
        }
    }
    return Ok({ let t4461 = f__iv(py::getitem(&v_o.clone(), &Py::str("partitions"))?)?; if !t4461.truthy() { t4461 } else { { let t4459 = py::getitem(&v_o.clone(), &Py::str("partitions"))?; Py::Bool({ let t4460 = py::getitem(&v_w.clone(), &v_n.clone())?; if py::eq(&t4459, &t4460) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pdecode(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_seq: Py = Py::None;
    v_seq = { let mut out = vec![]; for t4983 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("sequence"))?))? { let mut v_s: Py; 
v_s = t4983;
out.push(py::int(&v_s.clone())?); } Py::list(out) };
    v_n = py::add(&py::len(&v_seq.clone())?, &Py::Int(2i64))?;
    if ({ let t4991 = { let t4984 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t4985 = v_n.clone(); if py::ne(&t4984, &t4985) { true } else { false } }) }; if t4991.truthy() { t4991 } else { { let t4990 = { let t4986 = py::len(&py::getitem(&v_o.clone(), &Py::str("edges"))?)?; Py::Bool({ let t4987 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t4986, &t4987) { true } else { false } }) }; if t4990.truthy() { t4990 } else { { let t4988 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t4989 = py::sorted(&py::getitem(&v_o.clone(), &Py::str("edges"))?, (Py::Bool(false)).truthy())?; if py::ne(&t4988, &t4989) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t4992 = py::getitem(&f_prufer_encode({ let mut out = vec![]; for t5045 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("edges"))?))? { let mut v_u: Py; let mut v_v: Py; 
let t5046 = (t5045).items()?;
if t5046.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5046[0].clone();
v_v = t5046[1].clone();
out.push(Py::tuple(vec![Py::str(&py::to_str(&v_u.clone())), Py::str(&py::to_str(&v_v.clone()))])); } Py::list(out) }, Py::Bool(false))?, &Py::str("sequence"))?; Py::Bool({ let t4993 = v_seq.clone(); if py::eq(&t4992, &t4993) { true } else { false } }) });
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
    let t4540 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?])).items()?;
    if t4540.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t4540[0].clone();
    v_x = t4540[1].clone();
    v_y = t4540[2].clone();
    if (py::not(&{ let t4548 = f__iv(v_x.clone())?; if !t4548.truthy() { t4548 } else { { let t4547 = f__iv(v_y.clone())?; if !t4547.truthy() { t4547 } else { { let t4546 = { let t4542 = v_y.clone(); Py::Bool({ let t4543 = Py::Int(1i64); if py::ge(&t4542, &t4543)? { true } else { false } }) }; if !t4546.truthy() { t4546 } else { { let t4544 = py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&py::mul(&v_d.clone(), &v_y.clone())?, &v_y.clone())?)?; Py::Bool({ let t4545 = Py::Int(1i64); if py::eq(&t4544, &t4545) { true } else { false } }) } } } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_a0 = py::isqrt(&v_d.clone())?;
    let t4549 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), v_a0.clone()])).items()?;
    if t4549.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4549[0].clone();
    v_den = t4549[1].clone();
    v_a = t4549[2].clone();
    let t4550 = (Py::tuple(vec![Py::Int(1i64), v_a0.clone(), Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t4550.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t4550[0].clone();
    v_h1 = t4550[1].clone();
    v_k0 = t4550[2].clone();
    v_k1 = t4550[3].clone();
    while ({ let t4553 = py::sub(&py::mul(&v_h1.clone(), &v_h1.clone())?, &py::mul(&py::mul(&v_d.clone(), &v_k1.clone())?, &v_k1.clone())?)?; Py::Bool({ let t4554 = Py::Int(1i64); if py::ne(&t4553, &t4554) { true } else { false } }) }).truthy() {
        v_m = py::sub(&py::mul(&v_den.clone(), &v_a.clone())?, &v_m.clone())?;
        v_den = py::floordiv(&py::sub(&v_d.clone(), &py::mul(&v_m.clone(), &v_m.clone())?)?, &v_den.clone())?;
        v_a = py::floordiv(&py::add(&v_a0.clone(), &v_m.clone())?, &v_den.clone())?;
        let t4551 = (Py::tuple(vec![v_h1.clone(), py::add(&py::mul(&v_a.clone(), &v_h1.clone())?, &v_h0.clone())?])).items()?;
        if t4551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h0 = t4551[0].clone();
        v_h1 = t4551[1].clone();
        let t4552 = (Py::tuple(vec![v_k1.clone(), py::add(&py::mul(&v_a.clone(), &v_k1.clone())?, &v_k0.clone())?])).items()?;
        if t4552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k0 = t4552[0].clone();
        v_k1 = t4552[1].clone();
    }
    return Ok({ let t4555 = Py::tuple(vec![v_x.clone(), v_y.clone()]); Py::Bool({ let t4556 = Py::tuple(vec![v_h1.clone(), v_k1.clone()]); if py::eq(&t4555, &t4556) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pencode(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_nb: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t5107 = (f__tree_labels(py::getitem(&v_i.clone(), &Py::str("edges"))?)?).items()?;
    if t5107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nb = t5107[0].clone();
    v_off = t5107[1].clone();
    v_seq = py::getitem(&v_o.clone(), &Py::str("sequence"))?;
    v_want = py::sorted(&{ let mut out = vec![]; for t5108 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t5108;
out.push(py::sorted(&Py::tuple(vec![py::int(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)?]), (Py::Bool(false)).truthy())?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
    return Ok({ let t5131 = { let t5109 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t5110 = py::len(&v_nb.clone())?; if py::eq(&t5109, &t5110) { true } else { false } }) }; if !t5131.truthy() { t5131 } else { { let t5130 = { let t5111 = py::len(&v_seq.clone())?; Py::Bool({ let t5112 = py::sub(&py::len(&v_nb.clone())?, &Py::Int(2i64))?; if py::eq(&t5111, &t5112) { true } else { false } }) }; if !t5130.truthy() { t5130 } else { { let t5129 = Py::Bool(({ let mut out = vec![]; for t5113 in py::iter(&(v_seq.clone()))? { let mut v_s: Py; 
v_s = t5113;
out.push(f__iv(v_s.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t5129.truthy() { t5129 } else { { let t5115 = f__decode(v_seq.clone(), py::len(&v_nb.clone())?, v_off.clone())?; Py::Bool({ let t5116 = v_want.clone(); if py::eq(&t5115, &t5116) { true } else { false } }) } } } } } } });
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
    if ({ let t1638 = v_n.clone(); Py::Bool({ let t1639 = Py::Int(3000i64); if py::le(&t1638, &t1639)? { true } else { false } }) }).truthy() {
        return Ok({ let t1640 = py::getitem(&v_o.clone(), &Py::str("phi"))?; Py::Bool({ let t1641 = py::sum(&{ let mut out = vec![]; for t1642 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1642;
if ({ let t1643 = py::gcd(&v_k.clone(), &v_n.clone())?; Py::Bool({ let t1644 = Py::Int(1i64); if py::eq(&t1643, &t1644) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t1640, &t1641) { true } else { false } }) });
    }
    return Ok({ let t1645 = py::getitem(&v_o.clone(), &Py::str("phi"))?; Py::Bool({ let t1646 = py::prod(&{ let mut out = vec![]; for t1647 in py::iter(&(py::method(&f__factor(v_n.clone())?, "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t1648 = (t1647).items()?;
if t1648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t1648[0].clone();
v_e = t1648[1].clone();
out.push(py::mul(&py::pow(&v_p.clone(), &py::sub(&v_e.clone(), &Py::Int(1i64))?)?, &py::sub(&v_p.clone(), &Py::Int(1i64))?)?); } Py::list(out) })?; if py::eq(&t1645, &t1646) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_pk(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_at_most: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t4493 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t4493.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4493[0].clone();
    v_k = t4493[1].clone();
    v_at_most = { let v_at_most = v_at_most.clone(); let v_i = v_i.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_part: Py = Py::None;
            let mut v_s: Py = Py::None;
            let mut v_w: Py = Py::None;
            v_w = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
            for t4494 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) {
                v_part = t4494;
                for t4495 in py::range_iter(&[(v_part.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_s = t4495;
                    let t4496 = v_w.clone(); let t4497 = v_s.clone();
                    py::setitem(&t4496, &t4497, py::iadd(&py::getitem(&t4496, &t4497)?, &py::getitem(&v_w.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
                }
            }
            return Ok(py::getitem(&v_w.clone(), &v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_want = (if ({ let t4498 = v_k.clone(); Py::Bool({ let t4499 = Py::Int(0i64); if py::eq(&t4498, &t4499) { true } else { false } }) }).truthy() { (if ({ let t4500 = v_n.clone(); Py::Bool({ let t4501 = Py::Int(0i64); if py::eq(&t4500, &t4501) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }) } else { py::sub(&py::call(&v_at_most.clone(), &[v_k.clone()])?, &py::call(&v_at_most.clone(), &[py::sub(&v_k.clone(), &Py::Int(1i64))?])?)? });
    return Ok({ let t4505 = f__iv(py::getitem(&v_o.clone(), &Py::str("partitions"))?)?; if !t4505.truthy() { t4505 } else { { let t4503 = py::getitem(&v_o.clone(), &Py::str("partitions"))?; Py::Bool({ let t4504 = v_want.clone(); if py::eq(&t4503, &t4504) { true } else { false } }) } } });
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
    v_off = { let mut out = vec![]; for t3503 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3503;
if ({ let t3508 = { let t3504 = v_m.clone(); Py::Bool({ let t3505 = py::set(&v_on.clone())?; if !py::contains(&t3505, &t3504)? { true } else { false } }) }; if !t3508.truthy() { t3508 } else { { let t3506 = v_m.clone(); Py::Bool({ let t3507 = py::set(&v_dc.clone())?; if !py::contains(&t3507, &t3506)? { true } else { false } }) } } }).truthy() { out.push(v_m.clone()); }} Py::list(out) };
    if ({ let t3509 = v_e.clone(); Py::Bool({ let t3510 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if py::contains(&t3510, &t3509)? { true } else { false } }) }).truthy() {
        v_want = (if ({ let t3511 = v_e.clone(); Py::Bool({ let t3512 = Py::str("1"); if py::eq(&t3511, &t3512) { true } else { false } }) }).truthy() { py::not(&v_off.clone()) } else { Py::Bool(({ let mut out = vec![]; for t3513 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3513;
out.push({ let t3514 = v_m.clone(); Py::Bool({ let t3515 = py::bitor(&py::set(&v_off.clone())?, &py::set(&v_dc.clone())?)?; if py::contains(&t3515, &t3514)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) });
        return Ok({ let t3523 = v_want.clone(); if !t3523.truthy() { t3523 } else { { let t3522 = { let t3516 = py::getitem(&v_o.clone(), &Py::str("clauses"))?; Py::Bool({ let t3517 = (if ({ let t3518 = v_e.clone(); Py::Bool({ let t3519 = Py::str("1"); if py::eq(&t3518, &t3519) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { Py::Int(1i64) }); if py::eq(&t3516, &t3517) { true } else { false } }) }; if !t3522.truthy() { t3522 } else { { let t3520 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3521 = Py::Int(0i64); if py::eq(&t3520, &t3521) { true } else { false } }) } } } } });
    }
    v_cubes = Py::list(vec![]);
    for t3524 in py::iter(&(py::method(&v_e.clone(), "split", &[Py::str(" & ")])?))? {
        v_cl = t3524;
        v_lits = { let mut out = vec![]; for t3525 in py::iter(&(py::method(&py::method(&v_cl.clone(), "strip", &[Py::str("()")])?, "split", &[Py::str("|")])?))? { let mut v_x: Py; 
v_x = t3525;
out.push(py::method(&v_x.clone(), "strip", &[])?); } Py::list(out) };
        let _ = py::method(&v_cubes.clone(), "append", &[f__cube_of(py::method(&Py::str(" & "), "join", &[{ let mut out = vec![]; for t3529 in py::iter(&(v_lits.clone()))? { let mut v_x: Py; 
v_x = t3529;
out.push((if (py::method(&v_x.clone(), "startswith", &[Py::str("~")])?).truthy() { py::slice(&v_x.clone(), Some((Py::Int(1i64)).i()?), None, None)? } else { py::add(&Py::str("~"), &v_x.clone())? })); } Py::list(out) }])?, v_names.clone())?])?;
    }
    return Ok({ let t3562 = { let t3530 = Py::None; Py::Bool({ let t3531 = v_cubes.clone(); if !py::contains(&t3531, &t3530)? { true } else { false } }) }; if !t3562.truthy() { t3562 } else { { let t3561 = f__check_cover(v_n.clone(), v_off.clone(), v_dc.clone(), v_cubes.clone())?; if !t3561.truthy() { t3561 } else { { let t3560 = f__iv(py::getitem(&v_o.clone(), &Py::str("clauses"))?)?; if !t3560.truthy() { t3560 } else { { let t3559 = { let t3554 = py::getitem(&v_o.clone(), &Py::str("clauses"))?; Py::Bool({ let t3555 = py::len(&v_cubes.clone())?; if py::eq(&t3554, &t3555) { true } else { false } }) }; if !t3559.truthy() { t3559 } else { { let t3556 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3557 = py::sum(&{ let mut out = vec![]; for t3558 in py::iter(&(v_cubes.clone()))? { let mut v_c: Py; 
v_c = t3558;
out.push(py::sub(&v_n.clone(), &py::method(&py::int_text_base(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, "bin")?, "count", &[Py::str("1")])?)?); } Py::list(out) }, None)?; if py::eq(&t3556, &t3557) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_prev(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4694 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("prime"))?])).items()?;
    if t4694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4694[0].clone();
    v_p = t4694[1].clone();
    return Ok({ let t4716 = f__iv(v_p.clone())?; if !t4716.truthy() { t4716 } else { { let t4715 = { let t4696 = v_p.clone(); Py::Bool({ let t4697 = v_n.clone(); if py::lt(&t4696, &t4697)? { true } else { false } }) }; if !t4715.truthy() { t4715 } else { { let t4714 = f__is_prime_alt(v_p.clone())?; if !t4714.truthy() { t4714 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4713 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4713;
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
    let t4911 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_o.clone(), &Py::str("root"))?])).items()?;
    if t4911.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4911[0].clone();
    v_r = t4911[1].clone();
    if (py::not(&f__iv(v_r.clone())?)).truthy() {
        return Ok(Py::Bool(false));
    }
    if ({ let t4913 = v_p.clone(); Py::Bool({ let t4914 = Py::Int(2i64); if py::eq(&t4913, &t4914) { true } else { false } }) }).truthy() {
        return Ok({ let t4915 = v_r.clone(); Py::Bool({ let t4916 = Py::Int(1i64); if py::eq(&t4915, &t4916) { true } else { false } }) });
    }
    v_qs = (if ({ let t4917 = v_p.clone(); Py::Bool({ let t4918 = py::pow(&Py::Int(10i64), &Py::Int(10i64))?; if py::lt(&t4917, &t4918)? { true } else { false } }) }).truthy() { Py::list((f__trial_factor(py::sub(&v_p.clone(), &Py::Int(1i64))?)?).items()?) } else { Py::list((f__factor(py::sub(&v_p.clone(), &Py::Int(1i64))?)?).items()?) });
    v_prim = { let v_i = v_i.clone(); let v_o = v_o.clone(); let v_p = v_p.clone(); let v_qs = v_qs.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_g: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::Bool(({ let mut out = vec![]; for t4928 in py::iter(&(v_qs.clone()))? { let mut v_q: Py; 
v_q = t4928;
out.push({ let t4929 = py::pow_mod(&v_g.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &v_q.clone())?, &v_p.clone())?; Py::Bool({ let t4930 = Py::Int(1i64); if py::ne(&t4929, &t4930) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok({ let t4936 = { let t4931 = Py::Int(2i64); Py::Bool({ let t4932 = v_r.clone(); if py::le(&t4931, &t4932)? { { let t4933 = v_p.clone(); if py::lt(&t4932, &t4933)? { true } else { false } } } else { false } }) }; if !t4936.truthy() { t4936 } else { { let t4935 = py::call(&v_prim.clone(), &[v_r.clone()])?; if !t4935.truthy() { t4935 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4934 in py::range_iter(&[(Py::Int(2i64)).i()?, (v_r.clone()).i()?]) { let mut v_g: Py; 
v_g = t4934;
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
    let t5625 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t5625.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5625[0].clone();
    v_k = t5625[1].clone();
    v_poly = Py::list(vec![Py::Int(1i64)]);
    for t5626 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t5626;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&py::len(&v_poly.clone())?, &Py::Int(1i64))?)?;
        for t5627 in py::iter(&(py::enumerate(&v_poly.clone(), 0)?))? {
            let t5632 = (t5627).items()?;
            if t5632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_d = t5632[0].clone();
            v_c = t5632[1].clone();
            let t5628 = v_new.clone(); let t5629 = py::add(&v_d.clone(), &Py::Int(1i64))?;
            py::setitem(&t5628, &t5629, py::iadd(&py::getitem(&t5628, &t5629)?, &v_c.clone())?)?;
            let t5630 = v_new.clone(); let t5631 = v_d.clone();
            py::setitem(&t5630, &t5631, py::iadd(&py::getitem(&t5630, &t5631)?, &py::mul(&v_j.clone(), &v_c.clone())?)?)?;
        }
        v_poly = v_new.clone();
    }
    v_c = (if ({ let t5633 = v_k.clone(); Py::Bool({ let t5634 = py::len(&v_poly.clone())?; if py::lt(&t5633, &t5634)? { true } else { false } }) }).truthy() { py::getitem(&v_poly.clone(), &v_k.clone())? } else { Py::Int(0i64) });
    return Ok({ let t5641 = f__iv(py::getitem(&v_o.clone(), &Py::str("stirling1"))?)?; if !t5641.truthy() { t5641 } else { { let t5640 = { let t5636 = py::getitem(&v_o.clone(), &Py::str("stirling1"))?; Py::Bool({ let t5637 = v_c.clone(); if py::eq(&t5636, &t5637) { true } else { false } }) }; if !t5640.truthy() { t5640 } else { { let t5638 = py::getitem(&v_o.clone(), &Py::str("signed"))?; Py::Bool({ let t5639 = py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::sub(&v_n.clone(), &v_k.clone())?)?, &v_c.clone())?; if py::eq(&t5638, &t5639) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_s2(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sur: Py = Py::None;
    let t5670 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t5670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5670[0].clone();
    v_k = t5670[1].clone();
    v_sur = py::sum(&{ let mut out = vec![]; for t5671 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t5671;
out.push(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_j.clone())?, &py::comb(&v_k.clone(), &v_j.clone())?)?, &py::pow(&py::sub(&v_k.clone(), &v_j.clone())?, &v_n.clone())?)?); } Py::list(out) }, None)?;
    return Ok({ let t5678 = f__iv(py::getitem(&v_o.clone(), &Py::str("stirling2"))?)?; if !t5678.truthy() { t5678 } else { { let t5677 = { let t5673 = py::getitem(&v_o.clone(), &Py::str("surjections"))?; Py::Bool({ let t5674 = v_sur.clone(); if py::eq(&t5673, &t5674) { true } else { false } }) }; if !t5677.truthy() { t5677 } else { { let t5675 = py::mul(&py::getitem(&v_o.clone(), &Py::str("stirling2"))?, &py::factorial(&v_k.clone())?)?; Py::Bool({ let t5676 = v_sur.clone(); if py::eq(&t5675, &t5676) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_sat(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3223 = (f__table(py::getitem(&v_i.clone(), &Py::str("formula"))?, Py::None)?).items()?;
    if t3223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3223[0].clone();
    v_names = t3223[1].clone();
    v_first = (match ('g3224: { let mut out = vec![]; for t3225 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3226 = (t3225).items()?;
if t3226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3226[0].clone();
v_v = t3226[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); break 'g3224 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok({ let t3236 = { let t3227 = py::getitem(&v_o.clone(), &Py::str("satisfiable"))?; Py::Bool({ let t3228 = Py::Bool((v_vals.clone()).items()?.iter().any(|x| x.truthy())); if py::is(&t3227, &t3228) { true } else { false } }) }; if !t3236.truthy() { t3236 } else { { let t3235 = { let t3229 = py::getitem(&v_o.clone(), &Py::str("models"))?; Py::Bool({ let t3230 = py::sum(&v_vals.clone(), None)?; if py::eq(&t3229, &t3230) { true } else { false } }) }; if !t3235.truthy() { t3235 } else { { let t3231 = py::getitem(&v_o.clone(), &Py::str("first_model"))?; Py::Bool({ let t3232 = (if ({ let t3233 = v_first.clone(); Py::Bool({ let t3234 = Py::None; if py::is(&t3233, &t3234) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }); if py::eq(&t3231, &t3232) { true } else { false } }) } } } } });
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
    for t5724 in py::iter(&(v_adj.clone()))? {
        v_s = t5724;
        let t5725 = (Py::tuple(vec![py::set(&Py::list(vec![v_s.clone()]))?, Py::list(vec![v_s.clone()])])).items()?;
        if t5725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_seen = t5725[0].clone();
        v_st = t5725[1].clone();
        while (v_st.clone()).truthy() {
            v_u = py::method(&v_st.clone(), "pop", &[])?;
            for t5726 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t5726;
                if ({ let t5727 = v_v.clone(); Py::Bool({ let t5728 = v_seen.clone(); if !py::contains(&t5728, &t5727)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[v_v.clone()])?;
                    let _ = py::method(&v_st.clone(), "append", &[v_v.clone()])?;
                }
            }
        }
        py::setitem(&v_reach.clone(), &v_s.clone(), v_seen.clone())?;
    }
    v_comps = py::sorted_key(&py::set(&{ let mut out = vec![]; for t5729 in py::iter(&(v_adj.clone()))? { let mut v_s: Py; 
v_s = t5729;
out.push(Py::tuple((f__sorted({ let mut out = vec![]; for t5730 in py::iter(&(py::getitem(&v_reach.clone(), &v_s.clone())?))? { let mut v_v: Py; 
v_v = t5730;
if ({ let t5731 = v_s.clone(); Py::Bool({ let t5732 = py::getitem(&v_reach.clone(), &v_v.clone())?; if py::contains(&t5732, &t5731)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?).items()?)); } Py::list(out) })?, &{ let v_adj = v_adj.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_reach = v_reach.clone(); let v_s = v_s.clone(); let v_seen = v_seen.clone(); let v_st = v_st.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__nkey(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_comps = { let mut out = vec![]; for t5733 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t5733;
out.push(Py::list((v_c.clone()).items()?)); } Py::list(out) };
    return Ok({ let t5744 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t5744.truthy() { t5744 } else { { let t5743 = { let t5735 = py::getitem(&v_o.clone(), &Py::str("components"))?; Py::Bool({ let t5736 = v_comps.clone(); if py::eq(&t5735, &t5736) { true } else { false } }) }; if !t5743.truthy() { t5743 } else { { let t5742 = { let t5737 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t5738 = py::len(&v_comps.clone())?; if py::eq(&t5737, &t5738) { true } else { false } }) }; if !t5742.truthy() { t5742 } else { { let t5739 = py::getitem(&v_o.clone(), &Py::str("sizes"))?; Py::Bool({ let t5740 = py::sorted(&{ let mut out = vec![]; for t5741 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t5741;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?; if py::eq(&t5739, &t5740) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_sop(mut v_n: Py, mut v_on: Py, mut v_dc: Py, mut v_names: Py, mut v_o: Py) -> R {
    let mut v_cubes: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_e = py::getitem(&v_o.clone(), &Py::str("expression"))?;
    if ({ let t3825 = v_e.clone(); Py::Bool({ let t3826 = Py::tuple(vec![Py::str("0"), Py::str("1")]); if py::contains(&t3826, &t3825)? { true } else { false } }) }).truthy() {
        v_want = (if ({ let t3827 = v_e.clone(); Py::Bool({ let t3828 = Py::str("0"); if py::eq(&t3827, &t3828) { true } else { false } }) }).truthy() { py::not(&v_on.clone()) } else { Py::Bool(({ let mut out = vec![]; for t3829 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_n.clone())?).i()?]) { let mut v_m: Py; 
v_m = t3829;
out.push({ let t3830 = v_m.clone(); Py::Bool({ let t3831 = py::bitor(&py::set(&v_on.clone())?, &py::set(&v_dc.clone())?)?; if py::contains(&t3831, &t3830)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) });
        return Ok({ let t3839 = v_want.clone(); if !t3839.truthy() { t3839 } else { { let t3838 = { let t3832 = py::getitem(&v_o.clone(), &Py::str("terms"))?; Py::Bool({ let t3833 = (if ({ let t3834 = v_e.clone(); Py::Bool({ let t3835 = Py::str("0"); if py::eq(&t3834, &t3835) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { Py::Int(1i64) }); if py::eq(&t3832, &t3833) { true } else { false } }) }; if !t3838.truthy() { t3838 } else { { let t3836 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3837 = Py::Int(0i64); if py::eq(&t3836, &t3837) { true } else { false } }) } } } } });
    }
    v_cubes = { let mut out = vec![]; for t3840 in py::iter(&(py::method(&v_e.clone(), "split", &[Py::str(" | ")])?))? { let mut v_t: Py; 
v_t = t3840;
out.push(f__cube_of(v_t.clone(), v_names.clone())?); } Py::list(out) };
    return Ok({ let t3876 = { let t3844 = Py::None; Py::Bool({ let t3845 = v_cubes.clone(); if !py::contains(&t3845, &t3844)? { true } else { false } }) }; if !t3876.truthy() { t3876 } else { { let t3875 = f__check_cover(v_n.clone(), v_on.clone(), v_dc.clone(), v_cubes.clone())?; if !t3875.truthy() { t3875 } else { { let t3874 = f__iv(py::getitem(&v_o.clone(), &Py::str("terms"))?)?; if !t3874.truthy() { t3874 } else { { let t3873 = { let t3868 = py::getitem(&v_o.clone(), &Py::str("terms"))?; Py::Bool({ let t3869 = py::len(&v_cubes.clone())?; if py::eq(&t3868, &t3869) { true } else { false } }) }; if !t3873.truthy() { t3873 } else { { let t3870 = py::getitem(&v_o.clone(), &Py::str("literals"))?; Py::Bool({ let t3871 = py::sum(&{ let mut out = vec![]; for t3872 in py::iter(&(v_cubes.clone()))? { let mut v_c: Py; 
v_c = t3872;
out.push(py::sub(&v_n.clone(), &py::method(&py::int_text_base(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, "bin")?, "count", &[Py::str("1")])?)?); } Py::list(out) }, None)?; if py::eq(&t3870, &t3871) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_sqrtmod(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t5519 = (Py::tuple(vec![py::modulo(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?])).items()?;
    if t5519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5519[0].clone();
    v_p = t5519[1].clone();
    v_rs = py::getitem(&v_o.clone(), &Py::str("roots"))?;
    v_want = (if ({ let t5524 = { let t5520 = v_a.clone(); Py::Bool({ let t5521 = Py::Int(0i64); if py::eq(&t5520, &t5521) { true } else { false } }) }; if t5524.truthy() { t5524 } else { { let t5522 = v_p.clone(); Py::Bool({ let t5523 = Py::Int(2i64); if py::eq(&t5522, &t5523) { true } else { false } }) } } }).truthy() { Py::Int(1i64) } else { (if ({ let t5525 = py::pow_mod(&v_a.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?; Py::Bool({ let t5526 = Py::Int(1i64); if py::eq(&t5525, &t5526) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(0i64) }) });
    return Ok({ let t5547 = { let t5527 = v_rs.clone(); Py::Bool({ let t5528 = py::sorted(&py::set(&v_rs.clone())?, (Py::Bool(false)).truthy())?; if py::eq(&t5527, &t5528) { true } else { false } }) }; if !t5547.truthy() { t5547 } else { { let t5546 = Py::Bool(({ let mut out = vec![]; for t5529 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t5529;
out.push({ let t5537 = f__iv(v_r.clone())?; if !t5537.truthy() { t5537 } else { { let t5536 = { let t5531 = Py::Int(0i64); Py::Bool({ let t5532 = v_r.clone(); if py::le(&t5531, &t5532)? { { let t5533 = v_p.clone(); if py::lt(&t5532, &t5533)? { true } else { false } } } else { false } }) }; if !t5536.truthy() { t5536 } else { { let t5534 = py::modulo(&py::mul(&v_r.clone(), &v_r.clone())?, &v_p.clone())?; Py::Bool({ let t5535 = v_a.clone(); if py::eq(&t5534, &t5535) { true } else { false } }) } } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t5546.truthy() { t5546 } else { { let t5545 = { let t5538 = py::len(&v_rs.clone())?; Py::Bool({ let t5539 = v_want.clone(); if py::eq(&t5538, &t5539) { true } else { false } }) }; if !t5545.truthy() { t5545 } else { { let t5544 = { let t5540 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t5541 = v_want.clone(); if py::eq(&t5540, &t5541) { true } else { false } }) }; if !t5544.truthy() { t5544 } else { { let t5542 = py::getitem(&v_o.clone(), &Py::str("smallest"))?; Py::Bool({ let t5543 = (if (v_rs.clone()).truthy() { py::getitem(&v_rs.clone(), &Py::Int(0i64))? } else { Py::None }); if py::eq(&t5542, &t5543) { true } else { false } }) } } } } } } } } });
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
    let t5584 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("total"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("variables"))?)?, py::int(&{ let t5583 = py::method(&v_i.clone(), "get", &[Py::str("lower")])?; if t5583.truthy() { t5583 } else { Py::Int(0i64) } })?])).items()?;
    if t5584.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5584[0].clone();
    v_k = t5584[1].clone();
    v_lo = t5584[2].clone();
    v_hi = (if ({ let t5585 = py::method(&v_i.clone(), "get", &[Py::str("upper")])?; Py::Bool({ let t5586 = Py::None; if !py::is(&t5585, &t5586) { true } else { false } }) }).truthy() { py::int(&py::getitem(&v_i.clone(), &Py::str("upper"))?)? } else { v_n.clone() });
    v_ways = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?)?;
    for t5587 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t5587;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
        for t5588 in py::iter(&(py::enumerate(&v_ways.clone(), 0)?))? {
            let t5592 = (t5588).items()?;
            if t5592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s = t5592[0].clone();
            v_w = t5592[1].clone();
            if (v_w.clone()).truthy() {
                for t5589 in py::range_iter(&[(v_lo.clone()).i()?, (py::add(&py::minmax(&[v_hi.clone(), py::sub(&v_n.clone(), &v_s.clone())?], false)?, &Py::Int(1i64))?).i()?]) {
                    v_v = t5589;
                    let t5590 = v_new.clone(); let t5591 = py::add(&v_s.clone(), &v_v.clone())?;
                    py::setitem(&t5590, &t5591, py::iadd(&py::getitem(&t5590, &t5591)?, &v_w.clone())?)?;
                }
            }
        }
        v_ways = v_new.clone();
    }
    return Ok({ let t5596 = f__iv(py::getitem(&v_o.clone(), &Py::str("solutions"))?)?; if !t5596.truthy() { t5596 } else { { let t5594 = py::getitem(&v_o.clone(), &Py::str("solutions"))?; Py::Bool({ let t5595 = py::getitem(&v_ways.clone(), &v_n.clone())?; if py::eq(&t5594, &t5595) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_taut(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3320 = (f__table(py::getitem(&v_i.clone(), &Py::str("formula"))?, Py::None)?).items()?;
    if t3320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3320[0].clone();
    v_names = t3320[1].clone();
    v_first = (match ('g3321: { let mut out = vec![]; for t3322 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3323 = (t3322).items()?;
if t3323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3323[0].clone();
v_v = t3323[1].clone();
if (py::not(&v_v.clone())).truthy() { out.push(v_r.clone()); break 'g3321 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok({ let t3333 = { let t3324 = py::getitem(&v_o.clone(), &Py::str("tautology"))?; Py::Bool({ let t3325 = Py::Bool((v_vals.clone()).items()?.iter().all(|x| x.truthy())); if py::is(&t3324, &t3325) { true } else { false } }) }; if !t3333.truthy() { t3333 } else { { let t3332 = { let t3326 = py::getitem(&v_o.clone(), &Py::str("false_rows"))?; Py::Bool({ let t3327 = py::sub(&py::len(&v_vals.clone())?, &py::sum(&v_vals.clone(), None)?)?; if py::eq(&t3326, &t3327) { true } else { false } }) }; if !t3332.truthy() { t3332 } else { { let t3328 = py::getitem(&v_o.clone(), &Py::str("counterexample"))?; Py::Bool({ let t3329 = (if ({ let t3330 = v_first.clone(); Py::Bool({ let t3331 = Py::None; if py::is(&t3330, &t3331) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }); if py::eq(&t3328, &t3329) { true } else { false } }) } } } } });
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
    if ({ let t5792 = { let t5788 = py::sorted_key(&v_order.clone(), &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), (Py::Bool(false)).truthy())?; Py::Bool({ let t5789 = f__sorted(v_adj.clone())?; if py::ne(&t5788, &t5789) { true } else { false } }) }; if t5792.truthy() { t5792 } else { { let t5790 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t5791 = py::len(&v_order.clone())?; if py::ne(&t5790, &t5791) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_indeg = { let mut out = vec![]; for t5793 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5793;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    for t5794 in py::iter(&(v_adj.clone()))? {
        v_u = t5794;
        for t5795 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5795;
            let t5796 = v_indeg.clone(); let t5797 = v_v.clone();
            py::setitem(&t5796, &t5797, py::iadd(&py::getitem(&t5796, &t5797)?, &Py::Int(1i64))?)?;
        }
    }
    for t5798 in py::iter(&(v_order.clone()))? {
        v_u = t5798;
        v_avail = { let mut out = vec![]; for t5799 in py::iter(&(py::method(&v_indeg.clone(), "items", &[])?))? { let mut v_d: Py; let mut v_v: Py; 
let t5800 = (t5799).items()?;
if t5800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t5800[0].clone();
v_d = t5800[1].clone();
if ({ let t5801 = v_d.clone(); Py::Bool({ let t5802 = Py::Int(0i64); if py::eq(&t5801, &t5802) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
        if ({ let t5807 = { let t5803 = v_u.clone(); Py::Bool({ let t5804 = v_avail.clone(); if !py::contains(&t5804, &t5803)? { true } else { false } }) }; if t5807.truthy() { t5807 } else { { let t5805 = py::minmax_key(&[v_avail.clone()], &py::func(|a: &[Py]| -> R { f__nkey(a.get(0).cloned().unwrap_or(Py::None)) }), false)?; Py::Bool({ let t5806 = v_u.clone(); if py::ne(&t5805, &t5806) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Bool(false));
        }
        py::delitem(&v_indeg.clone(), &v_u.clone())?;
        for t5808 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5808;
            let t5809 = v_indeg.clone(); let t5810 = v_v.clone();
            py::setitem(&t5809, &t5810, py::sub(&py::getitem(&t5809, &t5810)?, &Py::Int(1i64))?)?;
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
    let t5890 = (f__table(py::getitem(&v_i.clone(), &Py::str("formula"))?, Py::None)?).items()?;
    if t5890.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t5890[0].clone();
    v_names = t5890[1].clone();
    return Ok({ let t5906 = { let t5891 = py::getitem(&v_o.clone(), &Py::str("variables"))?; Py::Bool({ let t5892 = v_names.clone(); if py::eq(&t5891, &t5892) { true } else { false } }) }; if !t5906.truthy() { t5906 } else { { let t5905 = { let t5893 = py::getitem(&v_o.clone(), &Py::str("values"))?; Py::Bool({ let t5894 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t5895 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t5895;
out.push((if (v_v.clone()).truthy() { Py::str("1") } else { Py::str("0") })); } Py::list(out) }])?; if py::eq(&t5893, &t5894) { true } else { false } }) }; if !t5905.truthy() { t5905 } else { { let t5904 = f__iv(py::getitem(&v_o.clone(), &Py::str("true_rows"))?)?; if !t5904.truthy() { t5904 } else { { let t5903 = { let t5897 = py::getitem(&v_o.clone(), &Py::str("true_rows"))?; Py::Bool({ let t5898 = py::sum(&v_vals.clone(), None)?; if py::eq(&t5897, &t5898) { true } else { false } }) }; if !t5903.truthy() { t5903 } else { { let t5899 = py::getitem(&v_o.clone(), &Py::str("minterms"))?; Py::Bool({ let t5900 = { let mut out = vec![]; for t5901 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t5902 = (t5901).items()?;
if t5902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5902[0].clone();
v_v = t5902[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); }} Py::list(out) }; if py::eq(&t5899, &t5900) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_twos(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t6052 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("bits"))?)?, py::getitem(&v_o.clone(), &Py::str("bits_text"))?])).items()?;
    if t6052.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6052[0].clone();
    v_b = t6052[1].clone();
    v_s = t6052[2].clone();
    if (py::not(&{ let t6059 = f__iv(py::getitem(&v_o.clone(), &Py::str("unsigned"))?)?; if !t6059.truthy() { t6059 } else { { let t6058 = { let t6054 = py::len(&v_s.clone())?; Py::Bool({ let t6055 = v_b.clone(); if py::eq(&t6054, &t6055) { true } else { false } }) }; if !t6058.truthy() { t6058 } else { { let t6056 = py::set(&v_s.clone())?; Py::Bool({ let t6057 = py::set(&Py::list(vec![Py::str("0"), Py::str("1")]))?; if py::set_cmp(&t6056, &t6057, "<=")? { true } else { false } }) } } } } })).truthy() {
        return Ok(Py::Bool(false));
    }
    v_u = py::int_base(&v_s.clone(), &Py::Int(2i64))?;
    return Ok({ let t6066 = { let t6060 = v_u.clone(); Py::Bool({ let t6061 = py::getitem(&v_o.clone(), &Py::str("unsigned"))?; if py::eq(&t6060, &t6061) { true } else { false } }) }; if !t6066.truthy() { t6066 } else { { let t6062 = (if ({ let t6064 = py::getitem(&v_s.clone(), &Py::Int(0i64))?; Py::Bool({ let t6065 = Py::str("1"); if py::eq(&t6064, &t6065) { true } else { false } }) }).truthy() { py::sub(&v_u.clone(), &py::lshift(&Py::Int(1i64), &v_b.clone())?)? } else { v_u.clone() }); Py::Bool({ let t6063 = v_n.clone(); if py::eq(&t6062, &t6063) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_twosq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_reps: Py = Py::None;
    let t5986 = (Py::tuple(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_o.clone(), &Py::str("representations"))?])).items()?;
    if t5986.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5986[0].clone();
    v_reps = t5986[1].clone();
    v_ok = { let t6000 = Py::Bool(({ let mut out = vec![]; for t5987 in py::iter(&(v_reps.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t5988 = (t5987).items()?;
if t5988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5988[0].clone();
v_y = t5988[1].clone();
out.push({ let t5997 = f__iv(v_x.clone())?; if !t5997.truthy() { t5997 } else { { let t5996 = f__iv(v_y.clone())?; if !t5996.truthy() { t5996 } else { { let t5995 = { let t5990 = Py::Int(0i64); Py::Bool({ let t5991 = v_x.clone(); if py::le(&t5990, &t5991)? { { let t5992 = v_y.clone(); if py::le(&t5991, &t5992)? { true } else { false } } } else { false } }) }; if !t5995.truthy() { t5995 } else { { let t5993 = py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&v_y.clone(), &v_y.clone())?)?; Py::Bool({ let t5994 = v_n.clone(); if py::eq(&t5993, &t5994) { true } else { false } }) } } } } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t6000.truthy() { t6000 } else { { let t5998 = v_reps.clone(); Py::Bool({ let t5999 = py::sorted(&v_reps.clone(), (Py::Bool(false)).truthy())?; if py::eq(&t5998, &t5999) { true } else { false } }) } } };
    return Ok({ let t6019 = v_ok.clone(); if !t6019.truthy() { t6019 } else { { let t6018 = { let t6001 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t6002 = py::len(&v_reps.clone())?; if py::eq(&t6001, &t6002) { true } else { false } }) }; if !t6018.truthy() { t6018 } else { { let t6017 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t6017.truthy() { t6017 } else { { let t6003 = py::getitem(&v_o.clone(), &Py::str("r2"))?; Py::Bool({ let t6004 = py::sum(&{ let mut out = vec![]; for t6005 in py::iter(&(v_reps.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t6006 = (t6005).items()?;
if t6006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t6006[0].clone();
v_y = t6006[1].clone();
out.push(f__mult(v_x.clone(), v_y.clone())?); } Py::list(out) }, None)?; if py::eq(&t6003, &t6004) { true } else { false } }) } } } } } } });
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
    let t6148 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?, py::int(&py::getitem(&v_i.clone(), &Py::str("length"))?)?])).items()?;
    if t6148.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t6148[0].clone();
    v_t = t6148[1].clone();
    v_k = t6148[2].clone();
    v_vec = Py::dict(vec![(v_t.clone(), Py::Int(1i64))]);
    for t6149 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t6149;
        v_vec = { let mut out = vec![]; for t6150 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t6150;
out.push((v_u.clone(), py::sum(&{ let mut out = vec![]; for t6151 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t6151;
out.push(py::method(&v_vec.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)); } Py::dict(out) };
    }
    return Ok({ let t6155 = f__iv(py::getitem(&v_o.clone(), &Py::str("walks"))?)?; if !t6155.truthy() { t6155 } else { { let t6153 = py::getitem(&v_o.clone(), &Py::str("walks"))?; Py::Bool({ let t6154 = py::method(&v_vec.clone(), "get", &[v_s.clone(), Py::Int(0i64)])?; if py::eq(&t6153, &t6154) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v_word(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cnt: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("word"))?)), "strip", &[])?, "upper", &[])?;
    v_cnt = { let mut out = vec![]; for t6172 in py::iter(&(py::sorted(&py::set(&v_w.clone())?, (Py::Bool(false)).truthy())?))? { let mut v_c: Py; 
v_c = t6172;
out.push((v_c.clone(), py::method(&v_w.clone(), "count", &[v_c.clone()])?)); } Py::dict(out) };
    return Ok({ let t6183 = f__iv(py::getitem(&v_o.clone(), &Py::str("arrangements"))?)?; if !t6183.truthy() { t6183 } else { { let t6182 = { let t6174 = py::getitem(&v_o.clone(), &Py::str("letter_counts"))?; Py::Bool({ let t6175 = v_cnt.clone(); if py::eq(&t6174, &t6175) { true } else { false } }) }; if !t6182.truthy() { t6182 } else { { let t6181 = { let t6176 = py::getitem(&v_o.clone(), &Py::str("length"))?; Py::Bool({ let t6177 = py::len(&v_w.clone())?; if py::eq(&t6176, &t6177) { true } else { false } }) }; if !t6181.truthy() { t6181 } else { { let t6178 = py::mul(&py::getitem(&v_o.clone(), &Py::str("arrangements"))?, &py::prod(&{ let mut out = vec![]; for t6180 in py::iter(&(py::method(&v_cnt.clone(), "values", &[])?))? { let mut v_c: Py; 
v_c = t6180;
out.push(py::factorial(&v_c.clone())?); } Py::list(out) })?)?; Py::Bool({ let t6179 = py::factorial(&py::len(&v_w.clone())?)?; if py::eq(&t6178, &t6179) { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if (Py::Bool(matches!(v_v.clone(), Py::Bool(_)))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        if ({ let t6127 = py::not(&py::math1("isfinite", &v_v.clone())?); if t6127.truthy() { t6127 } else { { let t6125 = v_v.clone(); Py::Bool({ let t6126 = py::int(&v_v.clone())?; if py::ne(&t6125, &t6126) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
        }
        v_v = py::int(&v_v.clone())?;
    } else {
        if (py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))))).truthy() {
            let t6128: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                    v_v = py::int(&py::method(&Py::str(&py::to_str(&v_v.clone())), "strip", &[])?)?;
                    #[allow(unreachable_code)]
                    Ok(None)
                })();
            match t6128 {
                Ok(Some(r)) => { return Ok(r); }
                Ok(None) => {}
                Err(e) if (matches!(e, PyErr::Value(_))) => {
                    return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
                }
                Err(e) => return Err(e),
            }
        }
    }
    if ({ let t6133 = { let t6129 = v_lo.clone(); Py::Bool({ let t6130 = Py::None; if !py::is(&t6129, &t6130) { true } else { false } }) }; if !t6133.truthy() { t6133 } else { { let t6131 = v_v.clone(); Py::Bool({ let t6132 = v_lo.clone(); if py::lt(&t6131, &t6132)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    if ({ let t6138 = { let t6134 = v_hi.clone(); Py::Bool({ let t6135 = Py::None; if !py::is(&t6134, &t6135) { true } else { false } }) }; if !t6138.truthy() { t6138 } else { { let t6136 = py::abs(&v_v.clone())?; Py::Bool({ let t6137 = v_hi.clone(); if py::gt(&t6136, &t6137)? { true } else { false } }) } } }).truthy() {
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
        v_xs = { let mut out = vec![]; for t4945 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[,\\s;]+"), &[py::method(&v_xs.clone(), "strip", &[Py::str("[] ")])?, Py::Int(0)], 0)?))? { let mut v_t: Py; 
v_t = t4945;
if (v_t.clone()).truthy() { out.push(v_t.clone()); }} Py::list(out) };
    }
    v_out = { let mut out = vec![]; for t4946 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t4946;
out.push(f__whole(v_x.clone(), v_name.clone(), v_lo.clone(), v_hi.clone())?); } Py::list(out) };
    if ({ let t4961 = py::len(&v_out.clone())?; Py::Bool({ let t4962 = v_n_min.clone(); if py::lt(&t4961, &t4962)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " entries".to_string()].concat()))));
    }
    if ({ let t4967 = { let t4963 = v_n_max.clone(); Py::Bool({ let t4964 = Py::None; if !py::is(&t4963, &t4964) { true } else { false } }) }; if !t4967.truthy() { t4967 } else { { let t4965 = py::len(&v_out.clone())?; Py::Bool({ let t4966 = v_n_max.clone(); if py::gt(&t4965, &t4966)? { true } else { false } }) } } }).truthy() {
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
fn f_articulation_points(mut v_edges: Py) -> R {
    let mut v__b: Py = Py::None;
    let mut v_aps: Py = Py::None;
    let t97 = (f__lowlink(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?).items()?;
    if t97.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_aps = t97[0].clone();
    v__b = t97[1].clone();
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
    let t150 = (Py::tuple(vec![f__whole(v_from_base.clone(), Py::str("from_base"), Py::Int(2i64), Py::Int(36i64))?, f__whole(v_to_base.clone(), Py::str("to_base"), Py::Int(2i64), Py::Int(36i64))?])).items()?;
    if t150.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fb = t150[0].clone();
    v_tb = t150[1].clone();
    v_s = py::method(&py::method(&py::method(&py::method(&Py::str(&py::to_str(&v_digits.clone())), "strip", &[])?, "upper", &[])?, "replace", &[Py::str("_"), Py::str("")])?, "replace", &[Py::str(" "), Py::str("")])?;
    v_neg = py::method(&v_s.clone(), "startswith", &[Py::str("-")])?;
    v_s = (if (v_neg.clone()).truthy() { py::slice(&v_s.clone(), Some((Py::Int(1i64)).i()?), None, None)? } else { v_s.clone() });
    if ({ let t153 = py::not(&v_s.clone()); if t153.truthy() { t153 } else { { let t151 = py::len(&v_s.clone())?; Py::Bool({ let t152 = Py::Int(200i64); if py::gt(&t151, &t152)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the number's digits (at most 200)"))));
    }
    v_v = Py::Int(0i64);
    for t154 in py::iter(&(v_s.clone()))? {
        v_ch = t154;
        v_d = py::method(&c__DIG()?, "find", &[v_ch.clone()])?;
        if ({ let t159 = { let t155 = v_d.clone(); Py::Bool({ let t156 = Py::Int(0i64); if py::lt(&t155, &t156)? { true } else { false } }) }; if t159.truthy() { t159 } else { { let t157 = v_d.clone(); Py::Bool({ let t158 = v_fb.clone(); if py::ge(&t157, &t158)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["'".to_string(), py::format(&v_ch.clone(), "")?, "' is not a digit in base ".to_string(), py::format(&v_fb.clone(), "")?].concat()))));
        }
        v_v = py::add(&py::mul(&v_v.clone(), &v_fb.clone())?, &v_d.clone())?;
    }
    v_v = (if (v_neg.clone()).truthy() { py::neg(&v_v.clone())? } else { v_v.clone() });
    if ({ let t160 = py::abs(&v_v.clone())?; Py::Bool({ let t161 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t160, &t161)? { true } else { false } }) }).truthy() {
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
    for t213 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t213;
        v_new = Py::list(vec![py::getitem(&v_row.clone(), &py::neg(&Py::Int(1i64))?)?]);
        for t214 in py::iter(&(v_row.clone()))? {
            v_x = t214;
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
    for t244 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t244;
        let _ = py::method(&v_B.clone(), "append", &[py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t245 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t245;
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
    if ({ let t288 = v_s.clone(); Py::Bool({ let t289 = v_adj.clone(); if !py::contains(&t289, &t288)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_s.clone(), "")?, " is not a vertex of the graph".to_string()].concat()))));
    }
    let t294 = (f__bfs(v_adj.clone(), v_s.clone(), Py::Bool(false))?).items()?;
    if t294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_order = t294[0].clone();
    v_dist = t294[1].clone();
    return Ok(Py::dict(vec![(Py::str("order"), v_order.clone()), (Py::str("reached"), py::len(&v_order.clone())?), (Py::str("distances"), { let mut out = vec![]; for t295 in py::iter(&(v_order.clone()))? { let mut v_v: Py; 
v_v = t295;
out.push((v_v.clone(), py::getitem(&v_dist.clone(), &v_v.clone())?)); } Py::dict(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binomial_term(mut v_a: Py, mut v_b: Py, mut v_n: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let t330 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?])).items()?;
    if t330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t330[0].clone();
    v_b = t330[1].clone();
    let t331 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t331.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t331[0].clone();
    v_k = t331[1].clone();
    if ({ let t332 = v_k.clone(); Py::Bool({ let t333 = v_n.clone(); if py::gt(&t332, &t333)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("coefficient"), Py::Int(0i64))]));
    }
    v_c = py::mul(&py::mul(&py::comb(&v_n.clone(), &v_k.clone())?, &py::pow(&v_a.clone(), &v_k.clone())?)?, &py::pow(&v_b.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?;
    if ({ let t334 = py::abs(&v_c.clone())?; Py::Bool({ let t335 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t334, &t335)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficient exceeds 10^300"))));
    }
    return Ok(Py::dict(vec![(Py::str("coefficient"), v_c.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bipartite(mut v_edges: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("is_bipartite"), { let t350 = f__two_colour(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?; Py::Bool({ let t351 = Py::None; if !py::is(&t350, &t351) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bridges(mut v_edges: Py) -> R {
    let mut v__a: Py = Py::None;
    let mut v_brs: Py = Py::None;
    let t523 = (f__lowlink(f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?)?).items()?;
    if t523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__a = t523[0].clone();
    v_brs = t523[1].clone();
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
    v_sign = (if ({ let t576 = v_mode.clone(); Py::Bool({ let t577 = Py::str("encrypt"); if py::eq(&t576, &t577) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    v_xs = f__letters(v_message.clone())?;
    if (py::not(&v_xs.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message has no letters"))));
    }
    return Ok(Py::dict(vec![(Py::str("result"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t582 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t582;
out.push(py::chr(&py::add(&Py::Int(65i64), &py::modulo(&py::add(&v_x.clone(), &py::mul(&v_sign.clone(), &v_s.clone())?)?, &Py::Int(26i64))?)?)?); } Py::list(out) }])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_carmichael_lambda(mut v_n: Py) -> R {
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?;
    return Ok(Py::dict(vec![(Py::str("lambda"), (if ({ let t618 = v_n.clone(); Py::Bool({ let t619 = Py::Int(1i64); if py::gt(&t618, &t619)? { true } else { false } }) }).truthy() { f__carmichael_from(f__factor(v_n.clone())?)? } else { Py::Int(1i64) }))]));
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
    let t748 = (Py::tuple(vec![f__whole(v_numerator.clone(), Py::str("numerator"), Py::None, c_MAXI()?)?, f__whole(v_denominator.clone(), Py::str("denominator"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t748[0].clone();
    v_q = t748[1].clone();
    v_terms = Py::list(vec![]);
    while (v_q.clone()).truthy() {
        v_a = py::floordiv(&v_p.clone(), &v_q.clone())?;
        let _ = py::method(&v_terms.clone(), "append", &[v_a.clone()])?;
        let t749 = (Py::tuple(vec![v_q.clone(), py::sub(&v_p.clone(), &py::mul(&v_a.clone(), &v_q.clone())?)?])).items()?;
        if t749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t749[0].clone();
        v_q = t749[1].clone();
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
    if ({ let t4528 = py::mul(&v_a0.clone(), &v_a0.clone())?; Py::Bool({ let t4529 = v_d.clone(); if py::eq(&t4528, &t4529) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("d must not be a perfect square (its square root is an integer)"))));
    }
    let t4530 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), v_a0.clone(), Py::list(vec![])])).items()?;
    if t4530.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4530[0].clone();
    v_den = t4530[1].clone();
    v_a = t4530[2].clone();
    v_period = t4530[3].clone();
    while ({ let t4531 = v_a.clone(); Py::Bool({ let t4532 = py::mul(&Py::Int(2i64), &v_a0.clone())?; if py::ne(&t4531, &t4532) { true } else { false } }) }).truthy() {
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
    if ({ let t845 = py::len(&v_adj.clone())?; Py::Bool({ let t846 = Py::Int(12i64); if py::gt(&t845, &t846)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 vertices (exact colouring is exponential)"))));
    }
    v_nodes = f__sorted(v_adj.clone())?;
    for t847 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t847;
        v_col = f__colourable(v_nodes.clone(), v_adj.clone(), v_k.clone())?;
        if (v_col.clone()).truthy() {
            return Ok(Py::dict(vec![(Py::str("chromatic_number"), v_k.clone()), (Py::str("colouring"), { let mut out = vec![]; for t857 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t857;
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
    if ({ let t905 = py::len(&v_adj.clone())?; Py::Bool({ let t906 = Py::Int(60i64); if py::gt(&t905, &t906)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 vertices"))));
    }
    v_sets = { let mut out = vec![]; for t907 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t908 = (t907).items()?;
if t908.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t908[0].clone();
v_nb = t908[1].clone();
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
    let t968 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t968[0].clone();
    v_k = t968[1].clone();
    if ({ let t969 = v_k.clone(); Py::Bool({ let t970 = v_n.clone(); if py::gt(&t969, &t970)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not exceed n"))));
    }
    v_c = Py::Int(1i64);
    for t971 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_k.clone(), py::sub(&v_n.clone(), &v_k.clone())?], false)?, &Py::Int(1i64))?).i()?]) {
        v_i = t971;
        v_c = py::floordiv(&py::mul(&v_c.clone(), &py::add(&py::sub(&v_n.clone(), &py::minmax(&[v_k.clone(), py::sub(&v_n.clone(), &v_k.clone())?], false)?)?, &v_i.clone())?)?, &v_i.clone())?;
    }
    if ({ let t972 = v_c.clone(); Py::Bool({ let t973 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t972, &t973)? { true } else { false } }) }).truthy() {
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
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_comps.clone())?), (Py::str("sizes"), py::sorted(&{ let mut out = vec![]; for t1014 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t1014;
out.push(py::len(&v_c.clone())?); } Py::list(out) }, (Py::Bool(true)).truthy())?), (Py::str("is_connected"), { let t1015 = py::len(&v_comps.clone())?; Py::Bool({ let t1016 = Py::Int(1i64); if py::eq(&t1015, &t1016) { true } else { false } }) }), (Py::str("components"), v_comps.clone())]));
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
    if ({ let t1070 = py::len(&v_rs.clone())?; Py::Bool({ let t1071 = py::len(&v_ms.clone())?; if py::ne(&t1070, &t1071) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one residue for each modulus"))));
    }
    let t1072 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t1072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1072[0].clone();
    v_M = t1072[1].clone();
    for t1073 in py::iter(&(py::zip(&[v_rs.clone(), v_ms.clone()])?))? {
        let t1085 = (t1073).items()?;
        if t1085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t1085[0].clone();
        v_m = t1085[1].clone();
        v_g = py::gcd(&v_M.clone(), &v_m.clone())?;
        if (py::modulo(&py::sub(&v_r.clone(), &v_x.clone())?, &v_g.clone())?).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the congruences are inconsistent: no integer satisfies all of them"))));
        }
        v_mg = py::floordiv(&v_m.clone(), &v_g.clone())?;
        v_t = py::modulo(&py::mul(&py::floordiv(&py::sub(&v_r.clone(), &v_x.clone())?, &v_g.clone())?, &(if ({ let t1074 = v_mg.clone(); Py::Bool({ let t1075 = Py::Int(1i64); if py::gt(&t1074, &t1075)? { true } else { false } }) }).truthy() { f__inv(py::modulo(&py::floordiv(&v_M.clone(), &v_g.clone())?, &v_mg.clone())?, v_mg.clone())? } else { Py::Int(0i64) }))?, &(if ({ let t1083 = v_mg.clone(); Py::Bool({ let t1084 = Py::Int(1i64); if py::gt(&t1083, &t1084)? { true } else { false } }) }).truthy() { v_mg.clone() } else { Py::Int(1i64) }))?;
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
    v_m = py::floordiv(&py::sum(&{ let mut out = vec![]; for t1139 in py::iter(&(py::method(&v_adj.clone(), "values", &[])?))? { let mut v_nb: Py; 
v_nb = t1139;
out.push(py::len(&v_nb.clone())?); } Py::list(out) }, None)?, &Py::Int(2i64))?;
    v_r = py::add(&py::sub(&v_m.clone(), &py::len(&v_adj.clone())?)?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("cyclomatic_number"), v_r.clone()), (Py::str("has_cycle"), { let t1140 = v_r.clone(); Py::Bool({ let t1141 = Py::Int(0i64); if py::gt(&t1140, &t1141)? { true } else { false } }) })]));
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
    let t1181 = (Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)])).items()?;
    if t1181.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1181[0].clone();
    v_b = t1181[1].clone();
    if ({ let t1182 = v_n.clone(); Py::Bool({ let t1183 = Py::Int(0i64); if py::eq(&t1182, &t1183) { true } else { false } }) }).truthy() {
        v_d = Py::Int(1i64);
    } else {
        for t1184 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t1184;
            let t1185 = (Py::tuple(vec![v_b.clone(), py::mul(&py::sub(&v_i.clone(), &Py::Int(1i64))?, &py::add(&v_a.clone(), &v_b.clone())?)?])).items()?;
            if t1185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t1185[0].clone();
            v_b = t1185[1].clone();
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
    if ({ let t1221 = v_s.clone(); Py::Bool({ let t1222 = v_adj.clone(); if !py::contains(&t1222, &t1221)? { true } else { false } }) }).truthy() {
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
    let t1258 = (Py::tuple(vec![f__whole(v_p.clone(), Py::str("p"), Py::Int(3i64), c_MAXI()?)?, f__whole(v_g.clone(), Py::str("g"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_a.clone(), Py::str("a"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t1258.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1258[0].clone();
    v_g = t1258[1].clone();
    v_a = t1258[2].clone();
    v_b = t1258[3].clone();
    let _ = f__need_prime(v_p.clone(), Py::str("p"))?;
    if ({ let t1281 = v_g.clone(); Py::Bool({ let t1282 = v_p.clone(); if py::ge(&t1281, &t1282)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the generator g must satisfy 1 < g < p"))));
    }
    let t1283 = (Py::tuple(vec![py::pow_mod(&v_g.clone(), &v_a.clone(), &v_p.clone())?, py::pow_mod(&v_g.clone(), &v_b.clone(), &v_p.clone())?])).items()?;
    if t1283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1283[0].clone();
    v_B = t1283[1].clone();
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
    v_indeg = { let mut out = vec![]; for t1322 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1322;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    for t1323 in py::iter(&(v_adj.clone()))? {
        v_u = t1323;
        for t1324 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t1324;
            let t1325 = v_indeg.clone(); let t1326 = v_v.clone();
            py::setitem(&t1325, &t1326, py::iadd(&py::getitem(&t1325, &t1326)?, &Py::Int(1i64))?)?;
        }
    }
    v_stack = { let mut out = vec![]; for t1327 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1327;
if ({ let t1328 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t1329 = Py::Int(0i64); if py::eq(&t1328, &t1329) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    v_seen = Py::Int(0i64);
    while (v_stack.clone()).truthy() {
        v_u = py::method(&v_stack.clone(), "pop", &[])?;
        v_seen = py::iadd(&v_seen, &Py::Int(1i64))?;
        for t1330 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t1330;
            let t1331 = v_indeg.clone(); let t1332 = v_v.clone();
            py::setitem(&t1331, &t1332, py::sub(&py::getitem(&t1331, &t1332)?, &Py::Int(1i64))?)?;
            if ({ let t1333 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t1334 = Py::Int(0i64); if py::eq(&t1333, &t1334) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_stack.clone(), "append", &[v_v.clone()])?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("has_cycle"), { let t1335 = v_seen.clone(); Py::Bool({ let t1336 = py::len(&v_adj.clone())?; if py::ne(&t1335, &t1336) { true } else { false } }) }), (Py::str("is_dag"), { let t1337 = v_seen.clone(); Py::Bool({ let t1338 = py::len(&v_adj.clone())?; if py::eq(&t1337, &t1338) { true } else { false } }) })]));
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
    let t1377 = (Py::tuple(vec![f__whole(v_base.clone(), Py::str("base"), Py::None, c_MAXI()?)?, f__whole(v_target.clone(), Py::str("target"), Py::None, c_MAXI()?)?, f__whole(v_modulus.clone(), Py::str("modulus"), Py::Int(2i64), py::pow(&Py::Int(10i64), &Py::Int(12i64))?)?])).items()?;
    if t1377.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1377[0].clone();
    v_t = t1377[1].clone();
    v_m = t1377[2].clone();
    if ({ let t1378 = py::gcd(&v_b.clone(), &v_m.clone())?; Py::Bool({ let t1379 = Py::Int(1i64); if py::ne(&t1378, &t1379) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("base and modulus must be coprime"))));
    }
    let t1380 = (Py::tuple(vec![py::modulo(&v_b.clone(), &v_m.clone())?, py::modulo(&v_t.clone(), &v_m.clone())?])).items()?;
    if t1380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1380[0].clone();
    v_t = t1380[1].clone();
    v_s = py::add(&py::isqrt(&v_m.clone())?, &Py::Int(1i64))?;
    let t1381 = (Py::tuple(vec![Py::dict(vec![]), Py::Int(1i64)])).items()?;
    if t1381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_table = t1381[0].clone();
    v_cur = t1381[1].clone();
    for t1382 in py::range_iter(&[(v_s.clone()).i()?]) {
        v_j = t1382;
        let _ = py::method(&v_table.clone(), "setdefault", &[v_cur.clone(), v_j.clone()])?;
        v_cur = py::modulo(&py::mul(&v_cur.clone(), &v_b.clone())?, &v_m.clone())?;
    }
    v_factor = py::pow_mod(&f__inv(v_b.clone(), v_m.clone())?, &v_s.clone(), &v_m.clone())?;
    v_gamma = v_t.clone();
    for t1390 in py::range_iter(&[(py::add(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t1390;
        if ({ let t1391 = v_gamma.clone(); Py::Bool({ let t1392 = v_table.clone(); if py::contains(&t1392, &t1391)? { true } else { false } }) }).truthy() {
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
    v_cls = (if ({ let t1469 = v_proper.clone(); Py::Bool({ let t1470 = v_n.clone(); if py::eq(&t1469, &t1470) { true } else { false } }) }).truthy() { Py::str("perfect") } else { (if ({ let t1471 = v_proper.clone(); Py::Bool({ let t1472 = v_n.clone(); if py::gt(&t1471, &t1472)? { true } else { false } }) }).truthy() { Py::str("abundant") } else { Py::str("deficient") }) });
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_ds.clone())?), (Py::str("sum"), v_s.clone()), (Py::str("proper_sum"), v_proper.clone()), (Py::str("classification"), v_cls.clone()), (Py::str("divisors"), v_ds.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elgamal_encrypt(mut v_p: Py, mut v_g: Py, mut v_x: Py, mut v_k: Py, mut v_m: Py) -> R {
    let mut v_h: Py = Py::None;
    let t1534 = (Py::tuple(vec![f__whole(v_p.clone(), Py::str("p"), Py::Int(3i64), c_MAXI()?)?, f__whole(v_g.clone(), Py::str("g"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_x.clone(), Py::str("x"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_k.clone(), Py::str("k"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t1534.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1534[0].clone();
    v_g = t1534[1].clone();
    v_x = t1534[2].clone();
    v_k = t1534[3].clone();
    v_m = t1534[4].clone();
    let _ = f__need_prime(v_p.clone(), Py::str("p"))?;
    if ({ let t1561 = { let t1557 = v_g.clone(); Py::Bool({ let t1558 = v_p.clone(); if py::ge(&t1557, &t1558)? { true } else { false } }) }; if t1561.truthy() { t1561 } else { { let t1559 = v_m.clone(); Py::Bool({ let t1560 = v_p.clone(); if py::ge(&t1559, &t1560)? { true } else { false } }) } } }).truthy() {
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
    for t4317 in py::iter(&(f__factor(v_n.clone())?))? {
        v_p = t4317;
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
    v_odd = py::sum(&{ let mut out = vec![]; for t1675 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1675;
if (py::modulo(&py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?, &Py::Int(2i64))?).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_conn = { let t1676 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t1677 = Py::Int(1i64); if py::eq(&t1676, &t1677) { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("odd_vertices"), v_odd.clone()), (Py::str("connected"), v_conn.clone()), (Py::str("has_circuit"), { let t1688 = v_conn.clone(); if !t1688.truthy() { t1688 } else { { let t1686 = v_odd.clone(); Py::Bool({ let t1687 = Py::Int(0i64); if py::eq(&t1686, &t1687) { true } else { false } }) } } }), (Py::str("has_path"), { let t1691 = v_conn.clone(); if !t1691.truthy() { t1691 } else { { let t1689 = v_odd.clone(); Py::Bool({ let t1690 = Py::tuple(vec![Py::Int(0i64), Py::Int(2i64)]); if py::contains(&t1690, &t1689)? { true } else { false } }) } } })]));
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
    let t1727 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t1727.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1727[0].clone();
    v_b = t1727[1].clone();
    let t1732 = (f__ext(v_a.clone(), v_b.clone())?).items()?;
    if t1732.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1732[0].clone();
    v_x = t1732[1].clone();
    v_y = t1732[2].clone();
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
    let t1771 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t1771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1771[0].clone();
    v_b = t1771[1].clone();
    for t1772 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t1772;
        let t1773 = (Py::tuple(vec![v_b.clone(), py::add(&v_a.clone(), &v_b.clone())?])).items()?;
        if t1773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1773[0].clone();
        v_b = t1773[1].clone();
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
    let t1814 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t1814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1814[0].clone();
    v_l = t1814[1].clone();
    for t1815 in py::iter(&(v_xs.clone()))? {
        v_x = t1815;
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
    v_deg = { let mut out = vec![]; for t1933 in py::iter(&(f__sorted(v_adj.clone())?))? { let mut v_v: Py; 
v_v = t1933;
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
    if ({ let t1982 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t1983 = Py::Int(1i64); if py::ne(&t1982, &t1983) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph is not connected (the diameter is infinite)"))));
    }
    v_ecc = { let mut out = vec![]; for t1992 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1992;
out.push((v_v.clone(), py::minmax(&[py::method(&py::getitem(&f__bfs(v_adj.clone(), v_v.clone(), Py::Bool(false))?, &Py::Int(1i64))?, "values", &[])?], true)?)); } Py::dict(out) };
    let t1997 = (Py::tuple(vec![py::minmax(&[py::method(&v_ecc.clone(), "values", &[])?], true)?, py::minmax(&[py::method(&v_ecc.clone(), "values", &[])?], false)?])).items()?;
    if t1997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1997[0].clone();
    v_r = t1997[1].clone();
    return Ok(Py::dict(vec![(Py::str("diameter"), v_d.clone()), (Py::str("radius"), v_r.clone()), (Py::str("center"), f__sorted({ let mut out = vec![]; for t1998 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t1998;
if ({ let t1999 = py::getitem(&v_ecc.clone(), &v_v.clone())?; Py::Bool({ let t2000 = v_r.clone(); if py::eq(&t1999, &t2000) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?), (Py::str("periphery"), f__sorted({ let mut out = vec![]; for t2001 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t2001;
if ({ let t2002 = py::getitem(&v_ecc.clone(), &v_v.clone())?; Py::Bool({ let t2003 = v_d.clone(); if py::eq(&t2002, &t2003) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graph_triangles(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_sets: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_adj = f__graph(v_edges.clone(), Py::Bool(false), Py::Bool(false))?;
    v_sets = { let mut out = vec![]; for t2067 in py::iter(&(py::method(&v_adj.clone(), "items", &[])?))? { let mut v_nb: Py; let mut v_u: Py; 
let t2068 = (t2067).items()?;
if t2068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2068[0].clone();
v_nb = t2068[1].clone();
out.push((v_u.clone(), py::set(&v_nb.clone())?)); } Py::dict(out) };
    v_t = py::floordiv(&py::sum(&{ let mut out = vec![]; for t2069 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t2069;
for t2070 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t2070;
if ({ let t2071 = f__nkey(v_u.clone())?; Py::Bool({ let t2072 = f__nkey(v_v.clone())?; if py::lt(&t2071, &t2072)? { true } else { false } }) }).truthy() { out.push(py::len(&py::bitand(&py::getitem(&v_sets.clone(), &v_u.clone())?, &py::getitem(&v_sets.clone(), &v_v.clone())?)?)?); }}} Py::list(out) }, None)?, &Py::Int(3i64))?;
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
    while ({ let t2129 = v_d.clone(); if !t2129.truthy() { t2129 } else { { let t2127 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t2128 = Py::Int(0i64); if py::gt(&t2127, &t2128)? { true } else { false } }) } } }).truthy() {
        v_k = py::method(&v_d.clone(), "pop", &[Py::Int(0i64)])?;
        if ({ let t2120 = v_k.clone(); Py::Bool({ let t2121 = py::len(&v_d.clone())?; if py::gt(&t2120, &t2121)? { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("is_graphical"), Py::Bool(false))]));
        }
        for t2122 in py::range_iter(&[(v_k.clone()).i()?]) {
            v_j = t2122;
            let t2123 = v_d.clone(); let t2124 = v_j.clone();
            py::setitem(&t2123, &t2124, py::sub(&py::getitem(&t2123, &t2124)?, &Py::Int(1i64))?)?;
            if ({ let t2125 = py::getitem(&v_d.clone(), &v_j.clone())?; Py::Bool({ let t2126 = Py::Int(0i64); if py::lt(&t2125, &t2126)? { true } else { false } }) }).truthy() {
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
    if ({ let t2167 = py::len(&v_nodes.clone())?; Py::Bool({ let t2168 = Py::Int(12i64); if py::gt(&t2167, &t2168)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 vertices (the search is exponential)"))));
    }
    if ({ let t2169 = py::len(&v_nodes.clone())?; Py::Bool({ let t2170 = Py::Int(3i64); if py::lt(&t2169, &t2170)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("exists"), Py::Bool(false)), (Py::str("cycle"), Py::None)]));
    }
    v_s = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?;
    let t2171 = (Py::tuple(vec![Py::list(vec![v_s.clone()]), py::set(&Py::list(vec![v_s.clone()]))?])).items()?;
    if t2171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_path = t2171[0].clone();
    v_used = t2171[1].clone();
    let rec2172 = py::rec_cell();
    v_extend = { let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_nodes = v_nodes.clone(); let v_path = v_path.clone(); let v_s = v_s.clone(); let v_used = v_used.clone(); let rec2172 = rec2172.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let v_extend: Py = py::rec_get(&rec2172)?;
            let mut v_v: Py = Py::None;
            if ({ let t2173 = py::len(&v_path.clone())?; Py::Bool({ let t2174 = py::len(&v_nodes.clone())?; if py::eq(&t2173, &t2174) { true } else { false } }) }).truthy() {
                return Ok({ let t2175 = v_s.clone(); Py::Bool({ let t2176 = py::getitem(&v_adj.clone(), &py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?)?; if py::contains(&t2176, &t2175)? { true } else { false } }) });
            }
            for t2177 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?)?)?))? {
                v_v = t2177;
                if ({ let t2178 = v_v.clone(); Py::Bool({ let t2179 = v_used.clone(); if !py::contains(&t2179, &t2178)? { true } else { false } }) }).truthy() {
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
    py::rec_set(&rec2172, &v_extend);
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
    let rec2243 = py::rec_cell();
    v_walk = { let v_divisors = v_divisors.clone(); let v_ds = v_ds.clone(); let v_n = v_n.clone(); let c_total = c_total.clone(); let rec2243 = rec2243.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_start: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_l: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_size: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_walk: Py = py::rec_get(&rec2243)?;
            let mut v_i: Py = Py::None;
            let mut v_l2: Py = Py::None;
            for t2244 in py::range_iter(&[(v_start.clone()).i()?, (py::len(&v_ds.clone())?).i()?]) {
                v_i = t2244;
                v_l2 = py::mul(&py::floordiv(&v_l.clone(), &py::gcd(&v_l.clone(), &py::getitem(&v_ds.clone(), &v_i.clone())?)?)?, &py::getitem(&v_ds.clone(), &v_i.clone())?)?;
                if ({ let t2245 = v_l2.clone(); Py::Bool({ let t2246 = v_n.clone(); if py::gt(&t2245, &t2246)? { true } else { false } }) }).truthy() {
                    continue;
                }
                py::cell_set(&c_total, py::iadd(&py::cell_get(&c_total)?, &py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_size.clone())?, &py::floordiv(&v_n.clone(), &v_l2.clone())?)?)?);
                let _ = py::call(&v_walk.clone(), &[py::add(&v_i.clone(), &Py::Int(1i64))?, v_l2.clone(), py::add(&v_size.clone(), &Py::Int(1i64))?])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2243, &v_walk);
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
    if ({ let t2290 = py::len(&v_adj.clone())?; Py::Bool({ let t2291 = Py::Int(60i64); if py::gt(&t2290, &t2291)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 vertices"))));
    }
    v_comp = { let mut out = vec![]; for t2292 in py::iter(&(v_adj.clone()))? { let mut v_u: Py; 
v_u = t2292;
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
    for t2353 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t2353;
        let t2354 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
        if t2354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2354[0].clone();
        v_k = t2354[1].clone();
        while (Py::Bool(true)).truthy() {
            v_g1 = py::floordiv(&py::mul(&v_k.clone(), &py::sub(&py::mul(&Py::Int(3i64), &v_k.clone())?, &Py::Int(1i64))?)?, &Py::Int(2i64))?;
            if ({ let t2355 = v_g1.clone(); Py::Bool({ let t2356 = v_m.clone(); if py::gt(&t2355, &t2356)? { true } else { false } }) }).truthy() {
                break;
            }
            v_sign = (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
            v_s = py::iadd(&v_s, &py::mul(&v_sign.clone(), &py::getitem(&v_p.clone(), &py::sub(&v_m.clone(), &v_g1.clone())?)?)?)?;
            v_g2 = py::floordiv(&py::mul(&v_k.clone(), &py::add(&py::mul(&Py::Int(3i64), &v_k.clone())?, &Py::Int(1i64))?)?, &Py::Int(2i64))?;
            if ({ let t2357 = v_g2.clone(); Py::Bool({ let t2358 = v_m.clone(); if py::le(&t2357, &t2358)? { true } else { false } }) }).truthy() {
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
    return Ok(Py::dict(vec![(Py::str("isqrt"), v_r.clone()), (Py::str("remainder"), py::sub(&v_n.clone(), &py::mul(&v_r.clone(), &v_r.clone())?)?), (Py::str("is_square"), { let t2390 = py::mul(&v_r.clone(), &v_r.clone())?; Py::Bool({ let t2391 = v_n.clone(); if py::eq(&t2390, &t2391) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_is_prime(mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_spf: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c_MAXI()?)?;
    if ({ let t2432 = v_n.clone(); Py::Bool({ let t2433 = py::pow(&Py::Int(2i64), &Py::Int(64i64))?; if py::ge(&t2432, &t2433)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be below 2**64"))));
    }
    v_p = f__is_prime(v_n.clone())?;
    v_spf = (if ({ let t2456 = v_n.clone(); Py::Bool({ let t2457 = Py::Int(2i64); if py::lt(&t2456, &t2457)? { true } else { false } }) }).truthy() { Py::None } else { (if (v_p.clone()).truthy() { v_n.clone() } else { py::minmax(&[f__factor(v_n.clone())?], false)? }) });
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
    v_m = py::floordiv(&py::sum(&{ let mut out = vec![]; for t2533 in py::iter(&(py::method(&v_adj.clone(), "values", &[])?))? { let mut v_nb: Py; 
v_nb = t2533;
out.push(py::len(&v_nb.clone())?); } Py::list(out) }, None)?, &Py::Int(2i64))?;
    v_conn = { let t2534 = py::len(&f__components(v_adj.clone())?)?; Py::Bool({ let t2535 = Py::Int(1i64); if py::eq(&t2534, &t2535) { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("is_tree"), { let t2546 = v_conn.clone(); if !t2546.truthy() { t2546 } else { { let t2544 = v_m.clone(); Py::Bool({ let t2545 = py::sub(&py::len(&v_adj.clone())?, &Py::Int(1i64))?; if py::eq(&t2544, &t2545) { true } else { false } }) } } }), (Py::str("vertices"), py::len(&v_adj.clone())?), (Py::str("edges"), v_m.clone()), (Py::str("connected"), v_conn.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jacobi_symbol(mut v_a: Py, mut v_n: Py) -> R {
    let t2595 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t2595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2595[0].clone();
    v_n = t2595[1].clone();
    if ({ let t2596 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t2597 = Py::Int(0i64); if py::eq(&t2596, &t2597) { true } else { false } }) }).truthy() {
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
    let t2678 = (Py::tuple(vec![f__whole(v_right.clone(), Py::str("right"), Py::Int(0i64), Py::Int(1000i64))?, f__whole(v_up.clone(), Py::str("up"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t2678.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2678[0].clone();
    v_n = t2678[1].clone();
    if ({ let t2679 = py::add(&v_m.clone(), &v_n.clone())?; Py::Bool({ let t2680 = Py::Int(1000i64); if py::gt(&t2679, &t2680)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("right + up must be at most 1000"))));
    }
    return Ok(Py::dict(vec![(Py::str("paths"), py::comb(&py::add(&v_m.clone(), &v_n.clone())?, &v_m.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_legendre_symbol(mut v_a: Py, mut v_p: Py) -> R {
    let t2709 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_p.clone(), Py::str("p"), Py::Int(3i64), c_MAXI()?)?])).items()?;
    if t2709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2709[0].clone();
    v_p = t2709[1].clone();
    if ({ let t2734 = { let t2710 = py::modulo(&v_p.clone(), &Py::Int(2i64))?; Py::Bool({ let t2711 = Py::Int(0i64); if py::eq(&t2710, &t2711) { true } else { false } }) }; if t2734.truthy() { t2734 } else { py::not(&f__is_prime(v_p.clone())?) } }).truthy() {
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
    let t2775 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t2775.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2775[0].clone();
    v_b = t2775[1].clone();
    v_m = t2775[2].clone();
    v_g = (if (py::modulo(&v_a.clone(), &v_m.clone())?).truthy() { py::gcd(&py::modulo(&v_a.clone(), &v_m.clone())?, &v_m.clone())? } else { v_m.clone() });
    if (py::modulo(&v_b.clone(), &v_g.clone())?).truthy() {
        return Ok(Py::dict(vec![(Py::str("smallest"), Py::None), (Py::str("count"), Py::Int(0i64)), (Py::str("step"), Py::None)]));
    }
    v_mp = py::floordiv(&v_m.clone(), &v_g.clone())?;
    v_x0 = (if ({ let t2776 = v_mp.clone(); Py::Bool({ let t2777 = Py::Int(1i64); if py::gt(&t2776, &t2777)? { true } else { false } }) }).truthy() { py::modulo(&py::mul(&py::modulo(&py::floordiv(&v_b.clone(), &v_g.clone())?, &v_mp.clone())?, &f__inv(py::modulo(&py::floordiv(&v_a.clone(), &v_g.clone())?, &v_mp.clone())?, v_mp.clone())?)?, &v_mp.clone())? } else { Py::Int(0i64) });
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
    let t2831 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_b.clone(), Py::str("b"), Py::None, c_MAXI()?)?, f__whole(v_c.clone(), Py::str("c"), Py::None, c_MAXI()?)?])).items()?;
    if t2831.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2831[0].clone();
    v_b = t2831[1].clone();
    v_c = t2831[2].clone();
    if ({ let t2836 = { let t2832 = v_a.clone(); Py::Bool({ let t2833 = Py::Int(0i64); if py::eq(&t2832, &t2833) { true } else { false } }) }; if t2836.truthy() { t2836 } else { { let t2834 = v_b.clone(); Py::Bool({ let t2835 = Py::Int(0i64); if py::eq(&t2834, &t2835) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and b must both be non-zero"))));
    }
    let t2841 = (f__ext(py::abs(&v_a.clone())?, py::abs(&v_b.clone())?)?).items()?;
    if t2841.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2841[0].clone();
    v_s = t2841[1].clone();
    v_t = t2841[2].clone();
    v_step = py::floordiv(&py::abs(&v_b.clone())?, &v_g.clone())?;
    if (py::modulo(&v_c.clone(), &v_g.clone())?).truthy() {
        return Ok(Py::dict(vec![(Py::str("solvable"), Py::Bool(false)), (Py::str("x0"), Py::None), (Py::str("y0"), Py::None), (Py::str("x_step"), py::floordiv(&v_b.clone(), &v_g.clone())?), (Py::str("y_step"), py::neg(&py::floordiv(&v_a.clone(), &v_g.clone())?)?)]));
    }
    v_x = py::mul(&py::mul(&v_s.clone(), &(if ({ let t2842 = v_a.clone(); Py::Bool({ let t2843 = Py::Int(0i64); if py::gt(&t2842, &t2843)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?, &py::floordiv(&v_c.clone(), &v_g.clone())?)?;
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
    let t3015 = (f__parse_formula(v_formula1.clone())?).items()?;
    if t3015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__t1 = t3015[0].clone();
    v_n1 = t3015[1].clone();
    let t3016 = (f__parse_formula(v_formula2.clone())?).items()?;
    if t3016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__t2 = t3016[0].clone();
    v_n2 = t3016[1].clone();
    v_names = py::sorted(&py::bitor(&py::set(&v_n1.clone())?, &py::set(&v_n2.clone())?)?, (Py::Bool(false)).truthy())?;
    if ({ let t3017 = py::len(&v_names.clone())?; Py::Bool({ let t3018 = c_MAX_VARS()?; if py::gt(&t3017, &t3018)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&c_MAX_VARS()?, "")?, " variables in total".to_string()].concat()))));
    }
    let t3040 = (f__table(v_formula1.clone(), v_names.clone())?).items()?;
    if t3040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t3040[0].clone();
    v__ = t3040[1].clone();
    let t3041 = (f__table(v_formula2.clone(), v_names.clone())?).items()?;
    if t3041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v2 = t3041[0].clone();
    v__ = t3041[1].clone();
    v_diff = { let mut out = vec![]; for t3042 in py::range_iter(&[(py::len(&v_v1.clone())?).i()?]) { let mut v_r: Py; 
v_r = t3042;
if ({ let t3043 = py::getitem(&v_v1.clone(), &v_r.clone())?; Py::Bool({ let t3044 = py::getitem(&v_v2.clone(), &v_r.clone())?; if py::ne(&t3043, &t3044) { true } else { false } }) }).truthy() { out.push(v_r.clone()); }} Py::list(out) };
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
    let t3105 = (f__parse_formula(v_formula.clone())?).items()?;
    if t3105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tree = t3105[0].clone();
    v_names = t3105[1].clone();
    v_env = f__read_assignment(v_assignment.clone())?;
    v_missing = { let mut out = vec![]; for t3115 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t3115;
if ({ let t3116 = v_v.clone(); Py::Bool({ let t3117 = v_env.clone(); if !py::contains(&t3117, &t3116)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    v_extra = { let mut out = vec![]; for t3118 in py::iter(&(v_env.clone()))? { let mut v_v: Py; 
v_v = t3118;
if ({ let t3119 = v_v.clone(); Py::Bool({ let t3120 = v_names.clone(); if !py::contains(&t3120, &t3119)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
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
    let t3214 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t3214.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3214[0].clone();
    v_names = t3214[1].clone();
    v_first = (match ('g3215: { let mut out = vec![]; for t3216 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3217 = (t3216).items()?;
if t3217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3217[0].clone();
v_v = t3217[1].clone();
if (v_v.clone()).truthy() { out.push(v_r.clone()); break 'g3215 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok(Py::dict(vec![(Py::str("satisfiable"), Py::Bool((v_vals.clone()).items()?.iter().any(|x| x.truthy()))), (Py::str("models"), py::sum(&v_vals.clone(), None)?), (Py::str("first_model"), (if ({ let t3218 = v_first.clone(); Py::Bool({ let t3219 = Py::None; if py::is(&t3218, &t3219) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logic_tautology(mut v_formula: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t3311 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t3311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3311[0].clone();
    v_names = t3311[1].clone();
    v_first = (match ('g3312: { let mut out = vec![]; for t3313 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t3314 = (t3313).items()?;
if t3314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3314[0].clone();
v_v = t3314[1].clone();
if (py::not(&v_v.clone())).truthy() { out.push(v_r.clone()); break 'g3312 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    return Ok(Py::dict(vec![(Py::str("tautology"), Py::Bool((v_vals.clone()).items()?.iter().all(|x| x.truthy()))), (Py::str("false_rows"), py::sub(&py::len(&v_vals.clone())?, &py::sum(&v_vals.clone(), None)?)?), (Py::str("counterexample"), (if ({ let t3315 = v_first.clone(); Py::Bool({ let t3316 = Py::None; if py::is(&t3315, &t3316) { true } else { false } }) }).truthy() { Py::None } else { f__assign(v_first.clone(), v_names.clone())? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lucas(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(1000i64))?;
    let t3356 = (Py::tuple(vec![Py::Int(2i64), Py::Int(1i64)])).items()?;
    if t3356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3356[0].clone();
    v_b = t3356[1].clone();
    for t3357 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t3357;
        let t3358 = (Py::tuple(vec![v_b.clone(), py::add(&v_a.clone(), &v_b.clone())?])).items()?;
        if t3358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3358[0].clone();
        v_b = t3358[1].clone();
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
    let t3447 = (f__formula_fn(v_formula.clone())?).items()?;
    if t3447.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3447[0].clone();
    v_on = t3447[1].clone();
    v_names = t3447[2].clone();
    return Ok(f__pos(v_n.clone(), v_on.clone(), Py::list(vec![]), v_names.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minimal_dnf(mut v_formula: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_on: Py = Py::None;
    let t3642 = (f__formula_fn(v_formula.clone())?).items()?;
    if t3642.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3642[0].clone();
    v_on = t3642[1].clone();
    v_names = t3642[2].clone();
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
    v_on = (if ({ let t3763 = v_minterms.clone(); Py::Bool({ let t3764 = Py::tuple(vec![Py::None, Py::str(""), Py::list(vec![])]); if !py::contains(&t3764, &t3763)? { true } else { false } }) }).truthy() { py::sorted(&py::set(&f__wlist(v_minterms.clone(), Py::str("minterms"), Py::Int(0i64), py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?, Py::Int(0i64), Py::Int(64i64))?)?, (Py::Bool(false)).truthy())? } else { Py::list(vec![]) });
    v_dc = (if ({ let t3774 = v_dont_cares.clone(); Py::Bool({ let t3775 = Py::tuple(vec![Py::None, Py::str(""), Py::list(vec![])]); if !py::contains(&t3775, &t3774)? { true } else { false } }) }).truthy() { py::sorted(&py::set(&f__wlist(v_dont_cares.clone(), Py::str("dont_cares"), Py::Int(0i64), py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?, Py::Int(0i64), Py::Int(64i64))?)?, (Py::Bool(false)).truthy())? } else { Py::list(vec![]) });
    for t3776 in py::iter(&(py::add(&v_on.clone(), &v_dc.clone())?))? {
        v_m = t3776;
        if ({ let t3777 = v_m.clone(); Py::Bool({ let t3778 = py::lshift(&Py::Int(1i64), &v_n.clone())?; if py::ge(&t3777, &t3778)? { true } else { false } }) }).truthy() {
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
    v_sqfree = Py::Bool(({ let mut out = vec![]; for t3940 in py::iter(&(py::method(&v_f.clone(), "values", &[])?))? { let mut v_e: Py; 
v_e = t3940;
out.push({ let t3941 = v_e.clone(); Py::Bool({ let t3942 = Py::Int(1i64); if py::eq(&t3941, &t3942) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()));
    return Ok(Py::dict(vec![(Py::str("mu"), (if (v_sqfree.clone()).truthy() { py::pow(&py::neg(&Py::Int(1i64))?, &py::len(&v_f.clone())?)? } else { Py::Int(0i64) })), (Py::str("squarefree"), v_sqfree.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mod_inverse(mut v_a: Py, mut v_m: Py) -> R {
    let t3987 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t3987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3987[0].clone();
    v_m = t3987[1].clone();
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
    let t4027 = (Py::tuple(vec![f__whole(v_base.clone(), Py::str("base"), Py::None, c_MAXI()?)?, f__whole(v_exponent.clone(), Py::str("exponent"), Py::Int(0i64), c_MAXI()?)?, f__whole(v_modulus.clone(), Py::str("modulus"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4027.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4027[0].clone();
    v_e = t4027[1].clone();
    v_m = t4027[2].clone();
    let t4028 = (Py::tuple(vec![py::modulo(&Py::Int(1i64), &v_m.clone())?, py::modulo(&v_a.clone(), &v_m.clone())?])).items()?;
    if t4028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_result = t4028[0].clone();
    v_a = t4028[1].clone();
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
    let t4057 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_m.clone(), Py::str("m"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4057[0].clone();
    v_m = t4057[1].clone();
    let t4058 = (Py::tuple(vec![py::floordiv(&v_a.clone(), &v_m.clone())?, py::modulo(&v_a.clone(), &v_m.clone())?])).items()?;
    if t4058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t4058[0].clone();
    v_r = t4058[1].clone();
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
    if ({ let t4105 = { let t4101 = v_total.clone(); Py::Bool({ let t4102 = Py::None; if !py::is(&t4101, &t4102) { true } else { false } }) }; if !t4105.truthy() { t4105 } else { { let t4103 = f__whole(v_total.clone(), Py::str("total"), Py::Int(0i64), c_MAXI()?)?; Py::Bool({ let t4104 = v_n.clone(); if py::ne(&t4103, &t4104) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the group sizes add up to ".to_string(), py::format(&v_n.clone(), "")?, ", not to the stated total ".to_string(), py::format(&v_total.clone(), "")?].concat()))));
    }
    if ({ let t4106 = v_n.clone(); Py::Bool({ let t4107 = c__NCAP()?; if py::gt(&t4106, &t4107)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the counts may add up to at most ".to_string(), py::format(&c__NCAP()?, "")?].concat()))));
    }
    v_r = py::factorial(&v_n.clone())?;
    for t4108 in py::iter(&(v_ks.clone()))? {
        v_k = t4108;
        v_r = py::floordiv(&v_r, &py::factorial(&v_k.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("coefficient"), v_r.clone()), (Py::str("total"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multiplicative_order(mut v_a: Py, mut v_n: Py) -> R {
    let t4139 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t4139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4139[0].clone();
    v_n = t4139[1].clone();
    if ({ let t4140 = py::gcd(&v_a.clone(), &v_n.clone())?; Py::Bool({ let t4141 = Py::Int(1i64); if py::ne(&t4140, &t4141) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and n must be coprime"))));
    }
    return Ok(Py::dict(vec![(Py::str("order"), f__order(py::modulo(&v_a.clone(), &v_n.clone())?, v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multisets(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let t4240 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), Py::Int(1000i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(1000i64))?])).items()?;
    if t4240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4240[0].clone();
    v_k = t4240[1].clone();
    v_c = py::comb(&py::sub(&py::add(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?, &v_k.clone())?;
    if ({ let t4241 = v_c.clone(); Py::Bool({ let t4242 = py::pow(&Py::Int(10i64), &Py::Int(300i64))?; if py::gt(&t4241, &t4242)? { true } else { false } }) }).truthy() {
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
    let t4271 = (Py::tuple(vec![f__whole(v_beads.clone(), Py::str("beads"), Py::Int(1i64), Py::Int(60i64))?, f__whole(v_colours.clone(), Py::str("colours"), Py::Int(1i64), Py::Int(60i64))?])).items()?;
    if t4271.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4271[0].clone();
    v_k = t4271[1].clone();
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
    for t4449 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_part = t4449;
        for t4450 in py::range_iter(&[(v_n.clone()).i()?, (py::sub(&v_part.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_s = t4450;
            let t4451 = v_q.clone(); let t4452 = v_s.clone();
            py::setitem(&t4451, &t4452, py::iadd(&py::getitem(&t4451, &t4452)?, &py::getitem(&v_q.clone(), &py::sub(&v_s.clone(), &v_part.clone())?)?)?)?;
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
    let t4484 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(300i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(300i64))?])).items()?;
    if t4484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4484[0].clone();
    v_k = t4484[1].clone();
    v_t = { let mut out = vec![]; for t4485 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t4485;
out.push(py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?); } Py::list(out) };
    py::setitem(&py::getitem(&v_t.clone(), &Py::Int(0i64))?, &Py::Int(0i64), Py::Int(1i64))?;
    for t4486 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t4486;
        for t4487 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_i.clone(), v_k.clone()], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t4487;
            py::setitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone(), py::add(&py::getitem(&py::getitem(&v_t.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &(if ({ let t4488 = py::sub(&v_i.clone(), &v_j.clone())?; Py::Bool({ let t4489 = v_j.clone(); if py::ge(&t4488, &t4489)? { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_t.clone(), &py::sub(&v_i.clone(), &v_j.clone())?)?, &v_j.clone())? } else { Py::Int(0i64) }))?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("partitions"), (if ({ let t4490 = v_k.clone(); Py::Bool({ let t4491 = v_n.clone(); if py::le(&t4490, &t4491)? { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_t.clone(), &v_n.clone())?, &v_k.clone())? } else { Py::Int(0i64) }))]));
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
    let t4537 = (f__convergent(v_terms.clone())?).items()?;
    if t4537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4537[0].clone();
    v_q = t4537[1].clone();
    if (py::modulo(&py::getitem(&v_r.clone(), &Py::str("period_length"))?, &Py::Int(2i64))?).truthy() {
        let t4538 = (Py::tuple(vec![py::add(&py::mul(&v_p.clone(), &v_p.clone())?, &py::mul(&py::mul(&py::int(&v_d.clone())?, &v_q.clone())?, &v_q.clone())?)?, py::mul(&py::mul(&Py::Int(2i64), &v_p.clone())?, &v_q.clone())?])).items()?;
        if t4538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t4538[0].clone();
        v_q = t4538[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("x"), v_p.clone()), (Py::str("y"), v_q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_permutations(mut v_n: Py, mut v_k: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4579 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), c__NCAP()?)?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), c__NCAP()?)?])).items()?;
    if t4579.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4579[0].clone();
    v_k = t4579[1].clone();
    if ({ let t4580 = v_k.clone(); Py::Bool({ let t4581 = v_n.clone(); if py::gt(&t4580, &t4581)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not exceed n"))));
    }
    v_p = Py::Int(1i64);
    for t4582 in py::range_iter(&[(py::add(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t4582;
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
    let t4611 = (Py::tuple(vec![f__whole(v_items.clone(), Py::str("items"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_boxes.clone(), Py::str("boxes"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4611[0].clone();
    v_k = t4611[1].clone();
    return Ok(Py::dict(vec![(Py::str("guaranteed"), py::neg(&py::floordiv(&py::neg(&v_n.clone())?, &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pigeonhole_needed(mut v_boxes: Py, mut v_at_least: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t4641 = (Py::tuple(vec![f__whole(v_boxes.clone(), Py::str("boxes"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_at_least.clone(), Py::str("at_least"), Py::Int(1i64), c_MAXI()?)?])).items()?;
    if t4641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t4641[0].clone();
    v_r = t4641[1].clone();
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
    return Ok(Py::dict(vec![(Py::str("factorization"), { let mut out = vec![]; for t4791 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t4792 = (t4791).items()?;
if t4792.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4792[0].clone();
v_e = t4792[1].clone();
out.push(Py::list(vec![v_p.clone(), v_e.clone()])); } Py::list(out) }), (Py::str("distinct_primes"), py::len(&v_f.clone())?), (Py::str("prime_factors_with_multiplicity"), py::sum(&py::method(&v_f.clone(), "values", &[])?, None)?), (Py::str("largest_prime_factor"), py::minmax(&[v_f.clone()], true)?), (Py::str("smallest_prime_factor"), py::minmax(&[v_f.clone()], false)?), (Py::str("text"), py::method(&Py::str(" × "), "join", &[{ let mut out = vec![]; for t4793 in py::iter(&(py::method(&v_f.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t4794 = (t4793).items()?;
if t4794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t4794[0].clone();
v_e = t4794[1].clone();
out.push((if ({ let t4795 = v_e.clone(); Py::Bool({ let t4796 = Py::Int(1i64); if py::gt(&t4795, &t4796)? { true } else { false } }) }).truthy() { Py::str(&[py::format(&v_p.clone(), "")?, "^".to_string(), py::format(&v_e.clone(), "")?].concat()) } else { Py::str(&py::to_str(&v_p.clone())) })); } Py::list(out) }])?)]));
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
    if ({ let t4889 = v_p.clone(); Py::Bool({ let t4890 = Py::Int(2i64); if py::eq(&t4889, &t4890) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("root"), Py::Int(1i64))]));
    }
    v_qs = Py::list((f__factor(py::sub(&v_p.clone(), &Py::Int(1i64))?)?).items()?);
    v_g = Py::Int(2i64);
    while (Py::Bool(({ let mut out = vec![]; for t4907 in py::iter(&(v_qs.clone()))? { let mut v_q: Py; 
v_q = t4907;
out.push({ let t4908 = py::pow_mod(&v_g.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &v_q.clone())?, &v_p.clone())?; Py::Bool({ let t4909 = Py::Int(1i64); if py::eq(&t4908, &t4909) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
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
    if ({ let t4968 = py::minmax(&[v_seq.clone()], true)?; Py::Bool({ let t4969 = v_n.clone(); if py::gt(&t4968, &t4969)? { true } else { false } }) }).truthy() {
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
    let t5096 = (f__tree_labels(v_edges.clone())?).items()?;
    if t5096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nb = t5096[0].clone();
    v_off = t5096[1].clone();
    v_nb = { let mut out = vec![]; for t5097 in py::iter(&(py::method(&v_nb.clone(), "items", &[])?))? { let mut v_u: Py; let mut v_v: Py; 
let t5098 = (t5097).items()?;
if t5098.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5098[0].clone();
v_v = t5098[1].clone();
out.push((v_u.clone(), py::set(&v_v.clone())?)); } Py::dict(out) };
    v_leaves = { let mut out = vec![]; for t5099 in py::iter(&(v_nb.clone()))? { let mut v_u: Py; 
v_u = t5099;
if ({ let t5100 = py::len(&py::getitem(&v_nb.clone(), &v_u.clone())?)?; Py::Bool({ let t5101 = Py::Int(1i64); if py::eq(&t5100, &t5101) { true } else { false } }) }).truthy() { out.push((if (v_largest.clone()).truthy() { py::neg(&v_u.clone())? } else { v_u.clone() })); }} Py::list(out) };
    let _ = py::heapq("heapify", &[v_leaves.clone()])?;
    v_seq = Py::list(vec![]);
    for t5102 in py::range_iter(&[(py::sub(&py::len(&v_nb.clone())?, &Py::Int(2i64))?).i()?]) {
        v__ = t5102;
        v_leaf = py::abs(&py::heapq("heappop", &[v_leaves.clone()])?)?;
        let t5103 = (py::getitem(&v_nb.clone(), &v_leaf.clone())?).items()?;
        if t5103.len() != 1 { return Err(PyErr::Value("unpack".into())); }
        v_p = t5103[0].clone();
        let _ = py::method(&v_seq.clone(), "append", &[v_p.clone()])?;
        let _ = py::method(&py::getitem(&v_nb.clone(), &v_p.clone())?, "discard", &[v_leaf.clone()])?;
        if ({ let t5104 = py::len(&py::getitem(&v_nb.clone(), &v_p.clone())?)?; Py::Bool({ let t5105 = Py::Int(1i64); if py::eq(&t5104, &t5105) { true } else { false } }) }).truthy() {
            let _ = py::heapq("heappush", &[v_leaves.clone(), (if (v_largest.clone()).truthy() { py::neg(&v_p.clone())? } else { v_p.clone() })])?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("sequence"), v_seq.clone()), (Py::str("vertices"), py::len(&v_nb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_decrypt(mut v_c: Py, mut v_d: Py, mut v_n: Py) -> R {
    let t5154 = (Py::tuple(vec![f__whole(v_c.clone(), Py::str("c"), Py::Int(0i64), c_MAXI()?)?, f__whole(v_d.clone(), Py::str("d"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5154.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5154[0].clone();
    v_d = t5154[1].clone();
    v_n = t5154[2].clone();
    if ({ let t5155 = v_c.clone(); Py::Bool({ let t5156 = v_n.clone(); if py::ge(&t5155, &t5156)? { true } else { false } }) }).truthy() {
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
    let t5223 = (Py::tuple(vec![py::int(&v_p.clone())?, py::int(&v_q.clone())?, f__whole(v_c.clone(), Py::str("c"), Py::Int(0i64), c_MAXI()?)?])).items()?;
    if t5223.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5223[0].clone();
    v_q = t5223[1].clone();
    v_c = t5223[2].clone();
    if ({ let t5224 = v_c.clone(); Py::Bool({ let t5225 = py::getitem(&v_k.clone(), &Py::str("n"))?; if py::ge(&t5224, &t5225)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the ciphertext c must satisfy 0 <= c < n"))));
    }
    v_d = py::getitem(&v_k.clone(), &Py::str("d"))?;
    let t5226 = (Py::tuple(vec![py::pow_mod(&v_c.clone(), &py::modulo(&v_d.clone(), &py::sub(&v_p.clone(), &Py::Int(1i64))?)?, &v_p.clone())?, py::pow_mod(&v_c.clone(), &py::modulo(&v_d.clone(), &py::sub(&v_q.clone(), &Py::Int(1i64))?)?, &v_q.clone())?])).items()?;
    if t5226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mp = t5226[0].clone();
    v_mq = t5226[1].clone();
    v_h = py::modulo(&py::mul(&f__inv(v_q.clone(), v_p.clone())?, &py::sub(&v_mp.clone(), &v_mq.clone())?)?, &v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("m"), py::add(&v_mq.clone(), &py::mul(&v_h.clone(), &v_q.clone())?)?), (Py::str("d"), v_d.clone()), (Py::str("n"), py::getitem(&v_k.clone(), &Py::str("n"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_encrypt(mut v_m: Py, mut v_e: Py, mut v_n: Py) -> R {
    let t5265 = (Py::tuple(vec![f__whole(v_m.clone(), Py::str("m"), Py::Int(0i64), c_MAXI()?)?, f__whole(v_e.clone(), Py::str("e"), Py::Int(1i64), c_MAXI()?)?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5265.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5265[0].clone();
    v_e = t5265[1].clone();
    v_n = t5265[2].clone();
    if ({ let t5266 = v_m.clone(); Py::Bool({ let t5267 = v_n.clone(); if py::ge(&t5266, &t5267)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message m must satisfy 0 <= m < n"))));
    }
    return Ok(Py::dict(vec![(Py::str("c"), py::pow_mod(&v_m.clone(), &v_e.clone(), &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rsa_keygen(mut v_p: Py, mut v_q: Py, mut v_e: Py) -> R {
    let mut v_phi: Py = Py::None;
    let t5296 = (Py::tuple(vec![f__whole(v_p.clone(), Py::str("p"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_q.clone(), Py::str("q"), Py::Int(2i64), c_MAXI()?)?, f__whole(v_e.clone(), Py::str("e"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5296.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t5296[0].clone();
    v_q = t5296[1].clone();
    v_e = t5296[2].clone();
    let _ = f__need_prime(v_p.clone(), Py::str("p"))?;
    let _ = f__need_prime(v_q.clone(), Py::str("q"))?;
    if ({ let t5319 = v_p.clone(); Py::Bool({ let t5320 = v_q.clone(); if py::eq(&t5319, &t5320) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p and q must be different primes"))));
    }
    v_phi = py::mul(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::sub(&v_q.clone(), &Py::Int(1i64))?)?;
    if ({ let t5326 = py::not(&{ let t5321 = Py::Int(1i64); Py::Bool({ let t5322 = v_e.clone(); if py::lt(&t5321, &t5322)? { { let t5323 = v_phi.clone(); if py::lt(&t5322, &t5323)? { true } else { false } } } else { false } }) }); if t5326.truthy() { t5326 } else { { let t5324 = py::gcd(&v_e.clone(), &v_phi.clone())?; Py::Bool({ let t5325 = Py::Int(1i64); if py::ne(&t5324, &t5325) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("e must satisfy 1 < e < phi(n) and gcd(e, phi(n)) = 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("n"), py::mul(&v_p.clone(), &v_q.clone())?), (Py::str("phi"), v_phi.clone()), (Py::str("d"), f__inv(v_e.clone(), v_phi.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sequences_with_repetition(mut v_n: Py, mut v_k: Py) -> R {
    let t5372 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(6i64))?)?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), py::pow(&Py::Int(10i64), &Py::Int(4i64))?)?])).items()?;
    if t5372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5372[0].clone();
    v_k = t5372[1].clone();
    if ({ let t5373 = py::mul(&v_k.clone(), &py::math1("log10", &v_n.clone())?)?; Py::Bool({ let t5374 = Py::Int(300i64); if py::gt(&t5373, &t5374)? { true } else { false } }) }).truthy() {
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
    if ({ let t5407 = py::len(&v_adj.clone())?; Py::Bool({ let t5408 = Py::Int(60i64); if py::gt(&t5407, &t5408)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 vertices"))));
    }
    v_M = { let mut out = vec![]; for t5415 in py::iter(&(f__laplacian_minor(v_adj.clone())?))? { let mut v_row: Py; 
v_row = t5415;
out.push({ let mut out = vec![]; for t5416 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t5416;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) }); } Py::list(out) };
    let t5417 = (Py::tuple(vec![py::len(&v_M.clone())?, py::fraction(&[Py::Int(1i64)])?])).items()?;
    if t5417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5417[0].clone();
    v_det = t5417[1].clone();
    for t5418 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t5418;
        v_p = (match ('g5419: { let mut out = vec![]; for t5420 in py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5420;
if ({ let t5421 = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t5422 = Py::Int(0i64); if py::ne(&t5421, &t5422) { true } else { false } }) }).truthy() { out.push(v_r.clone()); break 'g5419 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t5423 = v_p.clone(); Py::Bool({ let t5424 = Py::None; if py::is(&t5423, &t5424) { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("spanning_trees"), Py::Int(0i64))]));
        }
        if ({ let t5425 = v_p.clone(); Py::Bool({ let t5426 = v_c.clone(); if py::ne(&t5425, &t5426) { true } else { false } }) }).truthy() {
            let t5427 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
            if t5427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_M.clone(), &v_c.clone(), t5427[0].clone())?;
            py::setitem(&v_M.clone(), &v_p.clone(), t5427[1].clone())?;
            v_det = py::neg(&v_det.clone())?;
        }
        v_det = py::mul(&v_det, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
        for t5428 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t5428;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if (v_f.clone()).truthy() {
                py::setitem(&v_M.clone(), &v_r.clone(), { let mut out = vec![]; for t5429 in py::iter(&(py::zip(&[py::getitem(&v_M.clone(), &v_r.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5430 = (t5429).items()?;
if t5430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5430[0].clone();
v_b = t5430[1].clone();
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
    let t5475 = (Py::tuple(vec![f__whole(v_a.clone(), Py::str("a"), Py::None, c_MAXI()?)?, f__whole(v_p.clone(), Py::str("p"), Py::Int(2i64), c_MAXI()?)?])).items()?;
    if t5475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5475[0].clone();
    v_p = t5475[1].clone();
    if (py::not(&f__is_prime(v_p.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p must be prime"))));
    }
    v_a = py::modulo(&v_a, &v_p.clone())?;
    if ({ let t5498 = v_a.clone(); Py::Bool({ let t5499 = Py::Int(0i64); if py::eq(&t5498, &t5499) { true } else { false } }) }).truthy() {
        v_roots = Py::list(vec![Py::Int(0i64)]);
    } else {
        if ({ let t5500 = v_p.clone(); Py::Bool({ let t5501 = Py::Int(2i64); if py::eq(&t5500, &t5501) { true } else { false } }) }).truthy() {
            v_roots = Py::list(vec![Py::Int(1i64)]);
        } else {
            if ({ let t5502 = py::pow_mod(&v_a.clone(), &py::floordiv(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &v_p.clone())?; Py::Bool({ let t5503 = Py::Int(1i64); if py::ne(&t5502, &t5503) { true } else { false } }) }).truthy() {
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
    let t5570 = (Py::tuple(vec![f__whole(v_total.clone(), Py::str("total"), Py::Int(0i64), Py::Int(2000i64))?, f__whole(v_variables.clone(), Py::str("variables"), Py::Int(1i64), Py::Int(50i64))?])).items()?;
    if t5570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5570[0].clone();
    v_k = t5570[1].clone();
    v_lo = f__whole(v_lower.clone(), Py::str("lower"), Py::Int(0i64), Py::Int(2000i64))?;
    v_rest = py::sub(&v_n.clone(), &py::mul(&v_k.clone(), &v_lo.clone())?)?;
    if ({ let t5571 = v_upper.clone(); Py::Bool({ let t5572 = Py::None; if !py::is(&t5571, &t5572) { true } else { false } }) }).truthy() {
        v_hi = f__whole(v_upper.clone(), Py::str("upper"), Py::Int(0i64), Py::Int(2000i64))?;
        if ({ let t5573 = v_hi.clone(); Py::Bool({ let t5574 = v_lo.clone(); if py::lt(&t5573, &t5574)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the upper bound must be at least the lower bound"))));
        }
    }
    if ({ let t5575 = v_rest.clone(); Py::Bool({ let t5576 = Py::Int(0i64); if py::lt(&t5575, &t5576)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("solutions"), Py::Int(0i64))]));
    }
    if ({ let t5577 = v_upper.clone(); Py::Bool({ let t5578 = Py::None; if py::is(&t5577, &t5578) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("solutions"), py::comb(&py::sub(&py::add(&v_rest.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)]));
    }
    v_c = py::add(&py::sub(&v_hi.clone(), &v_lo.clone())?, &Py::Int(1i64))?;
    v_s = Py::Int(0i64);
    for t5579 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t5579;
        v_m = py::sub(&v_rest.clone(), &py::mul(&v_j.clone(), &v_c.clone())?)?;
        if ({ let t5580 = v_m.clone(); Py::Bool({ let t5581 = Py::Int(0i64); if py::lt(&t5580, &t5581)? { true } else { false } }) }).truthy() {
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
    let t5619 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(100i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(100i64))?])).items()?;
    if t5619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5619[0].clone();
    v_k = t5619[1].clone();
    if ({ let t5620 = v_k.clone(); Py::Bool({ let t5621 = v_n.clone(); if py::gt(&t5620, &t5621)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("stirling1"), Py::Int(0i64)), (Py::str("signed"), Py::Int(0i64))]));
    }
    v_row = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_k.clone())?)?;
    for t5622 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t5622;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
        for t5623 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_i.clone(), v_k.clone()], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t5623;
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
    let t5664 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(0i64), Py::Int(100i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64), Py::Int(100i64))?])).items()?;
    if t5664.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5664[0].clone();
    v_k = t5664[1].clone();
    if ({ let t5665 = v_k.clone(); Py::Bool({ let t5666 = v_n.clone(); if py::gt(&t5665, &t5666)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("stirling2"), Py::Int(0i64)), (Py::str("surjections"), Py::Int(0i64))]));
    }
    v_row = py::add(&Py::list(vec![Py::Int(1i64)]), &py::mul(&Py::list(vec![Py::Int(0i64)]), &v_k.clone())?)?;
    for t5667 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t5667;
        v_new = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
        for t5668 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_i.clone(), v_k.clone()], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t5668;
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
    let t5705 = (Py::tuple(vec![Py::dict(vec![]), Py::dict(vec![]), py::set(&Py::list(vec![]))?, Py::list(vec![]), Py::list(vec![]), Py::list(vec![Py::Int(0i64)])])).items()?;
    if t5705.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_idx = t5705[0].clone();
    v_low = t5705[1].clone();
    v_on = t5705[2].clone();
    v_stack = t5705[3].clone();
    v_comps = t5705[4].clone();
    v_t = t5705[5].clone();
    let rec5706 = py::rec_cell();
    v_visit = { let v_adj = v_adj.clone(); let v_comps = v_comps.clone(); let v_edges = v_edges.clone(); let v_idx = v_idx.clone(); let v_low = v_low.clone(); let v_on = v_on.clone(); let v_stack = v_stack.clone(); let v_t = v_t.clone(); let rec5706 = rec5706.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_visit: Py = py::rec_get(&rec5706)?;
            let mut v_comp: Py = Py::None;
            let mut v_v: Py = Py::None;
            let mut v_w: Py = Py::None;
            let t5707 = py::getitem(&v_t.clone(), &Py::Int(0i64))?;
            py::setitem(&v_idx.clone(), &v_u.clone(), t5707.clone())?;
            py::setitem(&v_low.clone(), &v_u.clone(), t5707.clone())?;
            let t5708 = v_t.clone(); let t5709 = Py::Int(0i64);
            py::setitem(&t5708, &t5709, py::iadd(&py::getitem(&t5708, &t5709)?, &Py::Int(1i64))?)?;
            let _ = py::method(&v_stack.clone(), "append", &[v_u.clone()])?;
            let _ = py::method(&v_on.clone(), "add", &[v_u.clone()])?;
            for t5710 in py::iter(&(f__sorted(py::getitem(&v_adj.clone(), &v_u.clone())?)?))? {
                v_v = t5710;
                if ({ let t5711 = v_v.clone(); Py::Bool({ let t5712 = v_idx.clone(); if !py::contains(&t5712, &t5711)? { true } else { false } }) }).truthy() {
                    let _ = py::call(&v_visit.clone(), &[v_v.clone()])?;
                    py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_low.clone(), &v_v.clone())?], false)?)?;
                } else {
                    if ({ let t5713 = v_v.clone(); Py::Bool({ let t5714 = v_on.clone(); if py::contains(&t5714, &t5713)? { true } else { false } }) }).truthy() {
                        py::setitem(&v_low.clone(), &v_u.clone(), py::minmax(&[py::getitem(&v_low.clone(), &v_u.clone())?, py::getitem(&v_idx.clone(), &v_v.clone())?], false)?)?;
                    }
                }
            }
            if ({ let t5715 = py::getitem(&v_low.clone(), &v_u.clone())?; Py::Bool({ let t5716 = py::getitem(&v_idx.clone(), &v_u.clone())?; if py::eq(&t5715, &t5716) { true } else { false } }) }).truthy() {
                v_comp = Py::list(vec![]);
                while (Py::Bool(true)).truthy() {
                    v_w = py::method(&v_stack.clone(), "pop", &[])?;
                    let _ = py::method(&v_on.clone(), "discard", &[v_w.clone()])?;
                    let _ = py::method(&v_comp.clone(), "append", &[v_w.clone()])?;
                    if ({ let t5717 = v_w.clone(); Py::Bool({ let t5718 = v_u.clone(); if py::eq(&t5717, &t5718) { true } else { false } }) }).truthy() {
                        break;
                    }
                }
                let _ = py::method(&v_comps.clone(), "append", &[f__sorted(v_comp.clone())?])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec5706, &v_visit);
    for t5719 in py::iter(&(f__sorted(v_adj.clone())?))? {
        v_v = t5719;
        if ({ let t5720 = v_v.clone(); Py::Bool({ let t5721 = v_idx.clone(); if !py::contains(&t5721, &t5720)? { true } else { false } }) }).truthy() {
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
    return Ok(Py::dict(vec![(Py::str("count"), py::len(&v_comps.clone())?), (Py::str("sizes"), py::sorted(&{ let mut out = vec![]; for t5722 in py::iter(&(v_comps.clone()))? { let mut v_c: Py; 
v_c = t5722;
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
    v_indeg = { let mut out = vec![]; for t5771 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5771;
out.push((v_v.clone(), Py::Int(0i64))); } Py::dict(out) };
    for t5772 in py::iter(&(v_adj.clone()))? {
        v_u = t5772;
        for t5773 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5773;
            let t5774 = v_indeg.clone(); let t5775 = v_v.clone();
            py::setitem(&t5774, &t5775, py::iadd(&py::getitem(&t5774, &t5775)?, &Py::Int(1i64))?)?;
        }
    }
    v_heap = { let mut out = vec![]; for t5776 in py::iter(&(v_adj.clone()))? { let mut v_v: Py; 
v_v = t5776;
if ({ let t5777 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t5778 = Py::Int(0i64); if py::eq(&t5777, &t5778) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![f__nkey(v_v.clone())?, v_v.clone()])); }} Py::list(out) };
    let _ = py::heapq("heapify", &[v_heap.clone()])?;
    v_order = Py::list(vec![]);
    while (v_heap.clone()).truthy() {
        let t5779 = (py::heapq("heappop", &[v_heap.clone()])?).items()?;
        if t5779.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v__k = t5779[0].clone();
        v_u = t5779[1].clone();
        let _ = py::method(&v_order.clone(), "append", &[v_u.clone()])?;
        for t5780 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t5780;
            let t5781 = v_indeg.clone(); let t5782 = v_v.clone();
            py::setitem(&t5781, &t5782, py::sub(&py::getitem(&t5781, &t5782)?, &Py::Int(1i64))?)?;
            if ({ let t5783 = py::getitem(&v_indeg.clone(), &v_v.clone())?; Py::Bool({ let t5784 = Py::Int(0i64); if py::eq(&t5783, &t5784) { true } else { false } }) }).truthy() {
                let _ = py::heapq("heappush", &[v_heap.clone(), Py::tuple(vec![f__nkey(v_v.clone())?, v_v.clone()])])?;
            }
        }
    }
    if ({ let t5785 = py::len(&v_order.clone())?; Py::Bool({ let t5786 = py::len(&v_adj.clone())?; if py::ne(&t5785, &t5786) { true } else { false } }) }).truthy() {
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
    let t5885 = (f__table(v_formula.clone(), Py::None)?).items()?;
    if t5885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t5885[0].clone();
    v_names = t5885[1].clone();
    return Ok(Py::dict(vec![(Py::str("variables"), v_names.clone()), (Py::str("values"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t5886 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t5886;
out.push((if (v_v.clone()).truthy() { Py::str("1") } else { Py::str("0") })); } Py::list(out) }])?), (Py::str("true_rows"), py::sum(&v_vals.clone(), None)?), (Py::str("minterms"), { let mut out = vec![]; for t5887 in py::iter(&(py::enumerate(&v_vals.clone(), 0)?))? { let mut v_r: Py; let mut v_v: Py; 
let t5888 = (t5887).items()?;
if t5888.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5888[0].clone();
v_v = t5888[1].clone();
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
    while ({ let t5931 = py::mul(&py::mul(&Py::Int(2i64), &v_x.clone())?, &v_x.clone())?; Py::Bool({ let t5932 = v_n.clone(); if py::le(&t5931, &t5932)? { true } else { false } }) }).truthy() {
        v_y2 = py::sub(&v_n.clone(), &py::mul(&v_x.clone(), &v_x.clone())?)?;
        v_y = py::isqrt(&v_y2.clone())?;
        if ({ let t5929 = py::mul(&v_y.clone(), &v_y.clone())?; Py::Bool({ let t5930 = v_y2.clone(); if py::eq(&t5929, &t5930) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_reps.clone(), "append", &[Py::list(vec![v_x.clone(), v_y.clone()])])?;
        }
        v_x = py::iadd(&v_x, &Py::Int(1i64))?;
    }
    if ({ let t5933 = v_n.clone(); Py::Bool({ let t5934 = Py::Int(0i64); if py::eq(&t5933, &t5934) { true } else { false } }) }).truthy() {
        v_r2 = Py::Int(1i64);
    } else {
        let t5935 = Py::Int(0i64);
        v_d1 = t5935.clone();
        v_d3 = t5935.clone();
        for t5982 in py::iter(&(f__divisors_from({ let mut out = vec![]; for t5978 in py::iter(&(py::method(&f__factor(v_n.clone())?, "items", &[])?))? { let mut v_e: Py; let mut v_p: Py; 
let t5979 = (t5978).items()?;
if t5979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t5979[0].clone();
v_e = t5979[1].clone();
if ({ let t5980 = v_p.clone(); Py::Bool({ let t5981 = Py::Int(2i64); if py::ne(&t5980, &t5981) { true } else { false } }) }).truthy() { out.push((v_p.clone(), v_e.clone())); }} Py::dict(out) })?))? {
            v_d = t5982;
            if ({ let t5983 = py::modulo(&v_d.clone(), &Py::Int(4i64))?; Py::Bool({ let t5984 = Py::Int(1i64); if py::eq(&t5983, &t5984) { true } else { false } }) }).truthy() {
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
    let t6042 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::None, c_MAXI()?)?, f__whole(v_bits.clone(), Py::str("bits"), Py::Int(2i64), Py::Int(64i64))?])).items()?;
    if t6042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6042[0].clone();
    v_b = t6042[1].clone();
    if (py::not(&{ let t6043 = py::neg(&py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t6044 = v_n.clone(); if py::le(&t6043, &t6044)? { { let t6045 = py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?; if py::lt(&t6044, &t6045)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_n.clone(), "")?, " does not fit in ".to_string(), py::format(&v_b.clone(), "")?, "-bit two's complement (range ".to_string(), py::format(&py::neg(&py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)?, "")?, " to ".to_string(), py::format(&py::sub(&py::lshift(&Py::Int(1i64), &py::sub(&v_b.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?, "")?, ")".to_string()].concat()))));
    }
    if ({ let t6046 = v_n.clone(); Py::Bool({ let t6047 = Py::Int(0i64); if py::ge(&t6046, &t6047)? { true } else { false } }) }).truthy() {
        v_s = py::method(&Py::str(&py::format(&v_n.clone(), &(Py::str("b")).as_str()?)?), "zfill", &[v_b.clone()])?;
    } else {
        v_mag = py::method(&Py::str(&py::format(&py::neg(&v_n.clone())?, &(Py::str("b")).as_str()?)?), "zfill", &[v_b.clone()])?;
        v_inv = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6048 in py::iter(&(v_mag.clone()))? { let mut v_c: Py; 
v_c = t6048;
out.push((if ({ let t6049 = v_c.clone(); Py::Bool({ let t6050 = Py::str("0"); if py::eq(&t6049, &t6050) { true } else { false } }) }).truthy() { Py::str("1") } else { Py::str("0") })); } Py::list(out) }])?;
        v_s = py::slice(&py::method(&Py::str(&py::format(&py::add(&py::int_base(&v_inv.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?, &(Py::str("b")).as_str()?)?), "zfill", &[v_b.clone()])?, Some((py::neg(&v_b.clone())?).i()?), None, None)?;
    }
    return Ok(Py::dict(vec![(Py::str("bits_text"), v_s.clone()), (Py::str("unsigned"), py::int_base(&v_s.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vigenere(mut v_message: Py, mut v_key: Py, mut v_mode: Py) -> R {
    let mut v_ks: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_ks = f__letters(v_key.clone())?;
    if ({ let t6081 = py::not(&v_ks.clone()); if t6081.truthy() { t6081 } else { { let t6079 = py::len(&v_ks.clone())?; Py::Bool({ let t6080 = py::len(&py::method(&Py::str(&py::to_str(&v_key.clone())), "strip", &[])?)?; if py::ne(&t6079, &t6080) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the key must be letters A-Z only"))));
    }
    v_xs = f__letters(v_message.clone())?;
    if (py::not(&v_xs.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the message has no letters"))));
    }
    v_sign = (if ({ let t6082 = v_mode.clone(); Py::Bool({ let t6083 = Py::str("encrypt"); if py::eq(&t6082, &t6083) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    return Ok(Py::dict(vec![(Py::str("result"), py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6084 in py::iter(&(py::enumerate(&v_xs.clone(), 0)?))? { let mut v_j: Py; let mut v_x: Py; 
let t6085 = (t6084).items()?;
if t6085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t6085[0].clone();
v_x = t6085[1].clone();
out.push(py::chr(&py::add(&Py::Int(65i64), &py::modulo(&py::add(&v_x.clone(), &py::mul(&v_sign.clone(), &py::getitem(&v_ks.clone(), &py::modulo(&v_j.clone(), &py::len(&v_ks.clone())?)?)?)?)?, &Py::Int(26i64))?)?)?); } Py::list(out) }])?)]));
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
    let t6139 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?, f__whole(v_length.clone(), Py::str("length"), Py::Int(0i64), Py::Int(200i64))?])).items()?;
    if t6139.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t6139[0].clone();
    v_t = t6139[1].clone();
    v_k = t6139[2].clone();
    for t6140 in py::iter(&(Py::tuple(vec![v_s.clone(), v_t.clone()])))? {
        v_v = t6140;
        if ({ let t6141 = v_v.clone(); Py::Bool({ let t6142 = v_adj.clone(); if !py::contains(&t6142, &t6141)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_v.clone(), "")?, " is not a vertex of the graph".to_string()].concat()))));
        }
    }
    v_vec = Py::dict(vec![(v_s.clone(), Py::Int(1i64))]);
    for t6143 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t6143;
        v_new = Py::dict(vec![]);
        for t6144 in py::iter(&(py::method(&v_vec.clone(), "items", &[])?))? {
            let t6146 = (t6144).items()?;
            if t6146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_u = t6146[0].clone();
            v_c = t6146[1].clone();
            for t6145 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t6145;
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
    if ({ let t6165 = py::not(&v_w.clone()); if t6165.truthy() { t6165 } else { { let t6164 = py::not(&py::method(&v_w.clone(), "isalpha", &[])?); if t6164.truthy() { t6164 } else { py::not(&py::method(&v_w.clone(), "isascii", &[])?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the word must consist of letters A-Z only"))));
    }
    if ({ let t6166 = py::len(&v_w.clone())?; Py::Bool({ let t6167 = c__NCAP()?; if py::gt(&t6166, &t6167)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&c__NCAP()?, "")?, " letters".to_string()].concat()))));
    }
    v_counts = Py::dict(vec![]);
    for t6168 in py::iter(&(v_w.clone()))? {
        v_ch = t6168;
        py::setitem(&v_counts.clone(), &v_ch.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_ch.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
    }
    let t6169 = (Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)])).items()?;
    if t6169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t6169[0].clone();
    v_used = t6169[1].clone();
    for t6170 in py::iter(&(py::method(&v_counts.clone(), "values", &[])?))? {
        v_c = t6170;
        v_used = py::iadd(&v_used, &v_c.clone())?;
        v_r = py::mul(&v_r, &py::comb(&v_used.clone(), &v_c.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("arrangements"), v_r.clone()), (Py::str("length"), py::len(&v_w.clone())?), (Py::str("letter_counts"), py::dict_from(&py::sorted(&py::method(&v_counts.clone(), "items", &[])?, (Py::Bool(false)).truthy())?, vec![])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1039: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_cc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1039 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1040 = v_r.clone(); Py::Bool({ let t1041 = Py::Bool(true); if py::is(&t1040, &t1041) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1046 = { let t1042 = v_r.clone(); Py::Bool({ let t1043 = Py::Bool(false); if py::is(&t1042, &t1043) { true } else { false } }) }; if t1046.truthy() { t1046 } else { { let t1044 = v_r.clone(); Py::Bool({ let t1045 = Py::None; if py::is(&t1044, &t1045) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1086(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_crt(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1106 = v_r.clone(); Py::Bool({ let t1107 = Py::Bool(true); if py::is(&t1106, &t1107) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1112 = { let t1108 = v_r.clone(); Py::Bool({ let t1109 = Py::Bool(false); if py::is(&t1108, &t1109) { true } else { false } }) }; if t1112.truthy() { t1112 } else { { let t1110 = v_r.clone(); Py::Bool({ let t1111 = Py::None; if py::is(&t1110, &t1111) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1142(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1159: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_crank(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1159 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1160 = v_r.clone(); Py::Bool({ let t1161 = Py::Bool(true); if py::is(&t1160, &t1161) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1166 = { let t1162 = v_r.clone(); Py::Bool({ let t1163 = Py::Bool(false); if py::is(&t1162, &t1163) { true } else { false } }) }; if t1166.truthy() { t1166 } else { { let t1164 = v_r.clone(); Py::Bool({ let t1165 = Py::None; if py::is(&t1164, &t1165) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1186(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1195: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_der(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1195 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1196 = v_r.clone(); Py::Bool({ let t1197 = Py::Bool(true); if py::is(&t1196, &t1197) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1202 = { let t1198 = v_r.clone(); Py::Bool({ let t1199 = Py::Bool(false); if py::is(&t1198, &t1199) { true } else { false } }) }; if t1202.truthy() { t1202 } else { { let t1200 = v_r.clone(); Py::Bool({ let t1201 = Py::None; if py::is(&t1200, &t1201) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1223(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1236: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dfs(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1236 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1237 = v_r.clone(); Py::Bool({ let t1238 = Py::Bool(true); if py::is(&t1237, &t1238) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1243 = { let t1239 = v_r.clone(); Py::Bool({ let t1240 = Py::Bool(false); if py::is(&t1239, &t1240) { true } else { false } }) }; if t1243.truthy() { t1243 } else { { let t1241 = v_r.clone(); Py::Bool({ let t1242 = Py::None; if py::is(&t1241, &t1242) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1284(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1296: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1285(v_inputs.clone(), v_outputs.clone())?;
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
fn s_1285(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_g: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(3).cloned().unwrap_or(Py::None);
            return Ok({ let t1295 = f__iv(py::getitem(&v_o.clone(), &Py::str("shared"))?)?; if !t1295.truthy() { t1295 } else { { let t1294 = { let t1287 = py::getitem(&v_o.clone(), &Py::str("A"))?; Py::Bool({ let t1288 = py::pow_mod(&v_g.clone(), &v_a.clone(), &v_p.clone())?; if py::eq(&t1287, &t1288) { true } else { false } }) }; if !t1294.truthy() { t1294 } else { { let t1293 = { let t1289 = py::getitem(&v_o.clone(), &Py::str("B"))?; Py::Bool({ let t1290 = py::pow_mod(&v_g.clone(), &v_b.clone(), &v_p.clone())?; if py::eq(&t1289, &t1290) { true } else { false } }) }; if !t1293.truthy() { t1293 } else { { let t1291 = py::getitem(&v_o.clone(), &Py::str("shared"))?; Py::Bool({ let t1292 = py::pow_mod(&py::getitem(&v_o.clone(), &Py::str("A"))?, &v_b.clone(), &v_p.clone())?; if py::eq(&t1291, &t1292) { true } else { false } }) } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("g"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1339(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1355: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dcyc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1355 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1356 = v_r.clone(); Py::Bool({ let t1357 = Py::Bool(true); if py::is(&t1356, &t1357) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1362 = { let t1358 = v_r.clone(); Py::Bool({ let t1359 = Py::Bool(false); if py::is(&t1358, &t1359) { true } else { false } }) }; if t1362.truthy() { t1362 } else { { let t1360 = v_r.clone(); Py::Bool({ let t1361 = Py::None; if py::is(&t1360, &t1361) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1393(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1405: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dlog(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1405 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1406 = v_r.clone(); Py::Bool({ let t1407 = Py::Bool(true); if py::is(&t1406, &t1407) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1412 = { let t1408 = v_r.clone(); Py::Bool({ let t1409 = Py::Bool(false); if py::is(&t1408, &t1409) { true } else { false } }) }; if t1412.truthy() { t1412 } else { { let t1410 = v_r.clone(); Py::Bool({ let t1411 = Py::None; if py::is(&t1410, &t1411) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1512: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_div(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1512 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1513 = v_r.clone(); Py::Bool({ let t1514 = Py::Bool(true); if py::is(&t1513, &t1514) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1519 = { let t1515 = v_r.clone(); Py::Bool({ let t1516 = Py::Bool(false); if py::is(&t1515, &t1516) { true } else { false } }) }; if t1519.truthy() { t1519 } else { { let t1517 = v_r.clone(); Py::Bool({ let t1518 = Py::None; if py::is(&t1517, &t1518) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1562(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1575: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_elg(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1575 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1576 = v_r.clone(); Py::Bool({ let t1577 = Py::Bool(true); if py::is(&t1576, &t1577) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1582 = { let t1578 = v_r.clone(); Py::Bool({ let t1579 = Py::Bool(false); if py::is(&t1578, &t1579) { true } else { false } }) }; if t1582.truthy() { t1582 } else { { let t1580 = v_r.clone(); Py::Bool({ let t1581 = Py::None; if py::is(&t1580, &t1581) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1636(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1649: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_phi(v_inputs.clone(), v_outputs.clone())?;
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
fn s_168(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t191: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_base(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t191 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t192 = v_r.clone(); Py::Bool({ let t193 = Py::Bool(true); if py::is(&t192, &t193) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t198 = { let t194 = v_r.clone(); Py::Bool({ let t195 = Py::Bool(false); if py::is(&t194, &t195) { true } else { false } }) }; if t198.truthy() { t198 } else { { let t196 = v_r.clone(); Py::Bool({ let t197 = Py::None; if py::is(&t196, &t197) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1692(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1705: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1693(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1705 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1706 = v_r.clone(); Py::Bool({ let t1707 = Py::Bool(true); if py::is(&t1706, &t1707) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1712 = { let t1708 = v_r.clone(); Py::Bool({ let t1709 = Py::Bool(false); if py::is(&t1708, &t1709) { true } else { false } }) }; if t1712.truthy() { t1712 } else { { let t1710 = v_r.clone(); Py::Bool({ let t1711 = Py::None; if py::is(&t1710, &t1711) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1693(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t1704 = { let t1694 = v_o.clone(); Py::Bool({ let t1695 = f_eulerian(py::getitem(&v_i.clone(), &Py::str("edges"))?)?; if py::eq(&t1694, &t1695) { true } else { false } }) }; if !t1704.truthy() { t1704 } else { { let t1703 = f__iv(py::getitem(&v_o.clone(), &Py::str("odd_vertices"))?)?; if !t1703.truthy() { t1703 } else { { let t1697 = py::getitem(&v_o.clone(), &Py::str("odd_vertices"))?; Py::Bool({ let t1698 = py::sum(&{ let mut out = vec![]; for t1701 in py::iter(&(py::method(&py::counter(&{ let mut out = vec![]; for t1699 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t1699;
for t1700 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? { let mut v_x: Py; 
v_x = t1700;
out.push(Py::str(&py::to_str(&v_x.clone()))); }} Py::list(out) })?, "items", &[])?))? { let mut v_d: Py; let mut v_v: Py; 
let t1702 = (t1701).items()?;
if t1702.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1702[0].clone();
v_d = t1702[1].clone();
if (py::modulo(&v_d.clone(), &Py::Int(2i64))?).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t1697, &t1698) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1733(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1749: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ext(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1749 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1750 = v_r.clone(); Py::Bool({ let t1751 = Py::Bool(true); if py::is(&t1750, &t1751) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1756 = { let t1752 = v_r.clone(); Py::Bool({ let t1753 = Py::Bool(false); if py::is(&t1752, &t1753) { true } else { false } }) }; if t1756.truthy() { t1756 } else { { let t1754 = v_r.clone(); Py::Bool({ let t1755 = Py::None; if py::is(&t1754, &t1755) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1774(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1783: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1775(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1783 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1784 = v_r.clone(); Py::Bool({ let t1785 = Py::Bool(true); if py::is(&t1784, &t1785) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1790 = { let t1786 = v_r.clone(); Py::Bool({ let t1787 = Py::Bool(false); if py::is(&t1786, &t1787) { true } else { false } }) }; if t1790.truthy() { t1790 } else { { let t1788 = v_r.clone(); Py::Bool({ let t1789 = Py::None; if py::is(&t1788, &t1789) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1775(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t1782 = f__iv(py::getitem(&v_o.clone(), &Py::str("fibonacci"))?)?; if !t1782.truthy() { t1782 } else { { let t1777 = py::getitem(&v_o.clone(), &Py::str("fibonacci"))?; Py::Bool({ let t1778 = py::getitem(&f__fib_pair(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, &Py::Int(0i64))?; if py::eq(&t1777, &t1778) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1817(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1843: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_gcd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1843 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1844 = v_r.clone(); Py::Bool({ let t1845 = Py::Bool(true); if py::is(&t1844, &t1845) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1850 = { let t1846 = v_r.clone(); Py::Bool({ let t1847 = Py::Bool(false); if py::is(&t1846, &t1847) { true } else { false } }) }; if t1850.truthy() { t1850 } else { { let t1848 = v_r.clone(); Py::Bool({ let t1849 = Py::None; if py::is(&t1848, &t1849) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1887(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_girth(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1907 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1908 = v_r.clone(); Py::Bool({ let t1909 = Py::Bool(true); if py::is(&t1908, &t1909) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1914 = { let t1910 = v_r.clone(); Py::Bool({ let t1911 = Py::Bool(false); if py::is(&t1910, &t1911) { true } else { false } }) }; if t1914.truthy() { t1914 } else { { let t1912 = v_r.clone(); Py::Bool({ let t1913 = Py::None; if py::is(&t1912, &t1913) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1934(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1956: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_deg(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1956 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1957 = v_r.clone(); Py::Bool({ let t1958 = Py::Bool(true); if py::is(&t1957, &t1958) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1963 = { let t1959 = v_r.clone(); Py::Bool({ let t1960 = Py::Bool(false); if py::is(&t1959, &t1960) { true } else { false } }) }; if t1963.truthy() { t1963 } else { { let t1961 = v_r.clone(); Py::Bool({ let t1962 = Py::None; if py::is(&t1961, &t1962) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2004(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2041: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_diam(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2041 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2042 = v_r.clone(); Py::Bool({ let t2043 = Py::Bool(true); if py::is(&t2042, &t2043) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2048 = { let t2044 = v_r.clone(); Py::Bool({ let t2045 = Py::Bool(false); if py::is(&t2044, &t2045) { true } else { false } }) }; if t2048.truthy() { t2048 } else { { let t2046 = v_r.clone(); Py::Bool({ let t2047 = Py::None; if py::is(&t2046, &t2047) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2073(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2089: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2074(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2089 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2090 = v_r.clone(); Py::Bool({ let t2091 = Py::Bool(true); if py::is(&t2090, &t2091) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2096 = { let t2092 = v_r.clone(); Py::Bool({ let t2093 = Py::Bool(false); if py::is(&t2092, &t2093) { true } else { false } }) }; if t2096.truthy() { t2096 } else { { let t2094 = v_r.clone(); Py::Bool({ let t2095 = Py::None; if py::is(&t2094, &t2095) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2074(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t2088 = f__iv(py::getitem(&v_o.clone(), &Py::str("triangles"))?)?; if !t2088.truthy() { t2088 } else { py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_adj: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t2076 = py::getitem(&v_o.clone(), &Py::str("triangles"))?; Py::Bool({ let t2077 = py::sum(&{ let mut out = vec![]; for t2078 in py::iter(&(py::itertools("combinations", &[f__sorted(v_adj.clone())?, Py::Int(3i64)])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t2079 = (t2078).items()?;
if t2079.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t2079[0].clone();
v_b = t2079[1].clone();
v_c = t2079[2].clone();
if ({ let t2087 = { let t2080 = v_b.clone(); Py::Bool({ let t2081 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t2081, &t2080)? { true } else { false } }) }; if !t2087.truthy() { t2087 } else { { let t2086 = { let t2082 = v_c.clone(); Py::Bool({ let t2083 = py::getitem(&v_adj.clone(), &v_a.clone())?; if py::contains(&t2083, &t2082)? { true } else { false } }) }; if !t2086.truthy() { t2086 } else { { let t2084 = v_c.clone(); Py::Bool({ let t2085 = py::getitem(&v_adj.clone(), &v_b.clone())?; if py::contains(&t2085, &t2084)? { true } else { false } }) } } } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t2076, &t2077) { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?])? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2141: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_graphical(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2141 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2142 = v_r.clone(); Py::Bool({ let t2143 = Py::Bool(true); if py::is(&t2142, &t2143) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2148 = { let t2144 = v_r.clone(); Py::Bool({ let t2145 = Py::Bool(false); if py::is(&t2144, &t2145) { true } else { false } }) }; if t2148.truthy() { t2148 } else { { let t2146 = v_r.clone(); Py::Bool({ let t2147 = Py::None; if py::is(&t2146, &t2147) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_215(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t222: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bell(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t222 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t223 = v_r.clone(); Py::Bool({ let t224 = Py::Bool(true); if py::is(&t223, &t224) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t229 = { let t225 = v_r.clone(); Py::Bool({ let t226 = Py::Bool(false); if py::is(&t225, &t226) { true } else { false } }) }; if t229.truthy() { t229 } else { { let t227 = v_r.clone(); Py::Bool({ let t228 = Py::None; if py::is(&t227, &t228) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2212: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ham(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2212 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2213 = v_r.clone(); Py::Bool({ let t2214 = Py::Bool(true); if py::is(&t2213, &t2214) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2219 = { let t2215 = v_r.clone(); Py::Bool({ let t2216 = Py::Bool(false); if py::is(&t2215, &t2216) { true } else { false } }) }; if t2219.truthy() { t2219 } else { { let t2217 = v_r.clone(); Py::Bool({ let t2218 = Py::None; if py::is(&t2217, &t2218) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2247(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2264: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_incl(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2264 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2265 = v_r.clone(); Py::Bool({ let t2266 = Py::Bool(true); if py::is(&t2265, &t2266) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2271 = { let t2267 = v_r.clone(); Py::Bool({ let t2268 = Py::Bool(false); if py::is(&t2267, &t2268) { true } else { false } }) }; if t2271.truthy() { t2271 } else { { let t2269 = v_r.clone(); Py::Bool({ let t2270 = Py::None; if py::is(&t2269, &t2270) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2308(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2331: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_indep(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2331 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2332 = v_r.clone(); Py::Bool({ let t2333 = Py::Bool(true); if py::is(&t2332, &t2333) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2338 = { let t2334 = v_r.clone(); Py::Bool({ let t2335 = Py::Bool(false); if py::is(&t2334, &t2335) { true } else { false } }) }; if t2338.truthy() { t2338 } else { { let t2336 = v_r.clone(); Py::Bool({ let t2337 = Py::None; if py::is(&t2336, &t2337) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2368: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ipart(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2368 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2369 = v_r.clone(); Py::Bool({ let t2370 = Py::Bool(true); if py::is(&t2369, &t2370) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2375 = { let t2371 = v_r.clone(); Py::Bool({ let t2372 = Py::Bool(false); if py::is(&t2371, &t2372) { true } else { false } }) }; if t2375.truthy() { t2375 } else { { let t2373 = v_r.clone(); Py::Bool({ let t2374 = Py::None; if py::is(&t2373, &t2374) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2392(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2410: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_isqrt(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2410 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2411 = v_r.clone(); Py::Bool({ let t2412 = Py::Bool(true); if py::is(&t2411, &t2412) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2417 = { let t2413 = v_r.clone(); Py::Bool({ let t2414 = Py::Bool(false); if py::is(&t2413, &t2414) { true } else { false } }) }; if t2417.truthy() { t2417 } else { { let t2415 = v_r.clone(); Py::Bool({ let t2416 = Py::None; if py::is(&t2415, &t2416) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t262: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bern(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t262 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t263 = v_r.clone(); Py::Bool({ let t264 = Py::Bool(true); if py::is(&t263, &t264) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t269 = { let t265 = v_r.clone(); Py::Bool({ let t266 = Py::Bool(false); if py::is(&t265, &t266) { true } else { false } }) }; if t269.truthy() { t269 } else { { let t267 = v_r.clone(); Py::Bool({ let t268 = Py::None; if py::is(&t267, &t268) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2507: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_isprime(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2507 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2508 = v_r.clone(); Py::Bool({ let t2509 = Py::Bool(true); if py::is(&t2508, &t2509) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2514 = { let t2510 = v_r.clone(); Py::Bool({ let t2511 = Py::Bool(false); if py::is(&t2510, &t2511) { true } else { false } }) }; if t2514.truthy() { t2514 } else { { let t2512 = v_r.clone(); Py::Bool({ let t2513 = Py::None; if py::is(&t2512, &t2513) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2547(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2573: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2548(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2573 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2574 = v_r.clone(); Py::Bool({ let t2575 = Py::Bool(true); if py::is(&t2574, &t2575) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2580 = { let t2576 = v_r.clone(); Py::Bool({ let t2577 = Py::Bool(false); if py::is(&t2576, &t2577) { true } else { false } }) }; if t2580.truthy() { t2580 } else { { let t2578 = v_r.clone(); Py::Bool({ let t2579 = Py::None; if py::is(&t2578, &t2579) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2548(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t2568 = { let t2549 = py::getitem(&v_o.clone(), &Py::str("connected"))?; Py::Bool({ let t2550 = { let t2551 = v_c.clone(); Py::Bool({ let t2552 = Py::Int(1i64); if py::eq(&t2551, &t2552) { true } else { false } }) }; if py::is(&t2549, &t2550) { true } else { false } }) }; if !t2568.truthy() { t2568 } else { { let t2567 = { let t2553 = py::getitem(&v_o.clone(), &Py::str("vertices"))?; Py::Bool({ let t2554 = py::len(&py::set(&{ let mut out = vec![]; for t2555 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("edges"))?))? { let mut v_e: Py; 
v_e = t2555;
for t2556 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? { let mut v_x: Py; 
v_x = t2556;
out.push(Py::str(&py::to_str(&v_x.clone()))); }} Py::list(out) })?)?; if py::eq(&t2553, &t2554) { true } else { false } }) }; if !t2567.truthy() { t2567 } else { { let t2566 = { let t2557 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t2558 = py::len(&py::getitem(&v_i.clone(), &Py::str("edges"))?)?; if py::eq(&t2557, &t2558) { true } else { false } }) }; if !t2566.truthy() { t2566 } else { { let t2559 = py::getitem(&v_o.clone(), &Py::str("is_tree"))?; Py::Bool({ let t2560 = { let t2565 = { let t2561 = v_c.clone(); Py::Bool({ let t2562 = Py::Int(1i64); if py::eq(&t2561, &t2562) { true } else { false } }) }; if !t2565.truthy() { t2565 } else { { let t2563 = py::getitem(&v_o.clone(), &Py::str("edges"))?; Py::Bool({ let t2564 = py::sub(&py::getitem(&v_o.clone(), &Py::str("vertices"))?, &Py::Int(1i64))?; if py::eq(&t2563, &t2564) { true } else { false } }) } } }; if py::is(&t2559, &t2560) { true } else { false } }) } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__v_cc_count(py::getitem(&v_i.clone(), &Py::str("edges"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2610(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2656: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_jacobi(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2656 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2657 = v_r.clone(); Py::Bool({ let t2658 = Py::Bool(true); if py::is(&t2657, &t2658) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2663 = { let t2659 = v_r.clone(); Py::Bool({ let t2660 = Py::Bool(false); if py::is(&t2659, &t2660) { true } else { false } }) }; if t2663.truthy() { t2663 } else { { let t2661 = v_r.clone(); Py::Bool({ let t2662 = Py::None; if py::is(&t2661, &t2662) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2681(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2687: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2682(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2687 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2688 = v_r.clone(); Py::Bool({ let t2689 = Py::Bool(true); if py::is(&t2688, &t2689) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2694 = { let t2690 = v_r.clone(); Py::Bool({ let t2691 = Py::Bool(false); if py::is(&t2690, &t2691) { true } else { false } }) }; if t2694.truthy() { t2694 } else { { let t2692 = v_r.clone(); Py::Bool({ let t2693 = Py::None; if py::is(&t2692, &t2693) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2682(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t2686 = f__iv(py::getitem(&v_o.clone(), &Py::str("paths"))?)?; if !t2686.truthy() { t2686 } else { { let t2684 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("paths"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("right"))?)?)?)?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("up"))?)?)?)?; Py::Bool({ let t2685 = py::factorial(&py::add(&py::int(&py::getitem(&v_i.clone(), &Py::str("right"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("up"))?)?)?)?; if py::eq(&t2684, &t2685) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2747(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2753: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_legendre(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2753 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2754 = v_r.clone(); Py::Bool({ let t2755 = Py::Bool(true); if py::is(&t2754, &t2755) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2760 = { let t2756 = v_r.clone(); Py::Bool({ let t2757 = Py::Bool(false); if py::is(&t2756, &t2757) { true } else { false } }) }; if t2760.truthy() { t2760 } else { { let t2758 = v_r.clone(); Py::Bool({ let t2759 = Py::None; if py::is(&t2758, &t2759) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2785(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2809: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_lincong(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2809 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2810 = v_r.clone(); Py::Bool({ let t2811 = Py::Bool(true); if py::is(&t2810, &t2811) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2816 = { let t2812 = v_r.clone(); Py::Bool({ let t2813 = Py::Bool(false); if py::is(&t2812, &t2813) { true } else { false } }) }; if t2816.truthy() { t2816 } else { { let t2814 = v_r.clone(); Py::Bool({ let t2815 = Py::None; if py::is(&t2814, &t2815) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2844(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2870: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_dioph(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2870 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2871 = v_r.clone(); Py::Bool({ let t2872 = Py::Bool(true); if py::is(&t2871, &t2872) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2877 = { let t2873 = v_r.clone(); Py::Bool({ let t2874 = Py::Bool(false); if py::is(&t2873, &t2874) { true } else { false } }) }; if t2877.truthy() { t2877 } else { { let t2875 = v_r.clone(); Py::Bool({ let t2876 = Py::None; if py::is(&t2875, &t2876) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2952(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2962: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2953(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2962 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2963 = v_r.clone(); Py::Bool({ let t2964 = Py::Bool(true); if py::is(&t2963, &t2964) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2969 = { let t2965 = v_r.clone(); Py::Bool({ let t2966 = Py::Bool(false); if py::is(&t2965, &t2966) { true } else { false } }) }; if t2969.truthy() { t2969 } else { { let t2967 = v_r.clone(); Py::Bool({ let t2968 = Py::None; if py::is(&t2967, &t2968) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2953(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t2961 = { let t2954 = py::getitem(&v_o.clone(), &Py::str("contradiction"))?; Py::Bool({ let t2955 = py::not(&py::getitem(&v_s.clone(), &Py::str("satisfiable"))?); if py::is(&t2954, &t2955) { true } else { false } }) }; if !t2961.truthy() { t2961 } else { { let t2960 = { let t2956 = py::getitem(&v_o.clone(), &Py::str("true_rows"))?; Py::Bool({ let t2957 = py::getitem(&v_s.clone(), &Py::str("models"))?; if py::eq(&t2956, &t2957) { true } else { false } }) }; if !t2960.truthy() { t2960 } else { { let t2958 = py::getitem(&v_o.clone(), &Py::str("witness"))?; Py::Bool({ let t2959 = py::getitem(&v_s.clone(), &Py::str("first_model"))?; if py::eq(&t2958, &t2959) { true } else { false } }) } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_logic_satisfiable(py::getitem(&v_i.clone(), &Py::str("formula"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_296(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t308: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bfs(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t308 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t309 = v_r.clone(); Py::Bool({ let t310 = Py::Bool(true); if py::is(&t309, &t310) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t315 = { let t311 = v_r.clone(); Py::Bool({ let t312 = Py::Bool(false); if py::is(&t311, &t312) { true } else { false } }) }; if t315.truthy() { t315 } else { { let t313 = v_r.clone(); Py::Bool({ let t314 = Py::None; if py::is(&t313, &t314) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3047(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3052: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3048(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3052 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3053 = v_r.clone(); Py::Bool({ let t3054 = Py::Bool(true); if py::is(&t3053, &t3054) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3059 = { let t3055 = v_r.clone(); Py::Bool({ let t3056 = Py::Bool(false); if py::is(&t3055, &t3056) { true } else { false } }) }; if t3059.truthy() { t3059 } else { { let t3057 = v_r.clone(); Py::Bool({ let t3058 = Py::None; if py::is(&t3057, &t3058) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3048(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let t3051 = { let t3049 = v_o.clone(); Py::Bool({ let t3050 = v_r.clone(); if py::eq(&t3049, &t3050) { true } else { false } }) }; if !t3051.truthy() { t3051 } else { Py::Bool(matches!(py::getitem(&v_o.clone(), &Py::str("equivalent"))?, Py::Bool(_))) } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_logic_equivalence(py::getitem(&v_i.clone(), &Py::str("formula1"))?, py::getitem(&v_i.clone(), &Py::str("formula2"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3135(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3140: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3136(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3140 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3141 = v_r.clone(); Py::Bool({ let t3142 = Py::Bool(true); if py::is(&t3141, &t3142) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3147 = { let t3143 = v_r.clone(); Py::Bool({ let t3144 = Py::Bool(false); if py::is(&t3143, &t3144) { true } else { false } }) }; if t3147.truthy() { t3147 } else { { let t3145 = v_r.clone(); Py::Bool({ let t3146 = Py::None; if py::is(&t3145, &t3146) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3136(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t3139 = Py::Bool(matches!(py::getitem(&v_o.clone(), &Py::str("value"))?, Py::Bool(_))); if !t3139.truthy() { t3139 } else { { let t3137 = py::getitem(&v_o.clone(), &Py::str("value"))?; Py::Bool({ let t3138 = py::getitem(&f_logic_evaluate(py::getitem(&v_i.clone(), &Py::str("formula"))?, py::getitem(&v_i.clone(), &Py::str("assignment"))?)?, &Py::str("value"))?; if py::eq(&t3137, &t3138) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3237: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_sat(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3237 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3238 = v_r.clone(); Py::Bool({ let t3239 = Py::Bool(true); if py::is(&t3238, &t3239) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3244 = { let t3240 = v_r.clone(); Py::Bool({ let t3241 = Py::Bool(false); if py::is(&t3240, &t3241) { true } else { false } }) }; if t3244.truthy() { t3244 } else { { let t3242 = v_r.clone(); Py::Bool({ let t3243 = Py::None; if py::is(&t3242, &t3243) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
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
fn s_3319(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3334: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_taut(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3334 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3335 = v_r.clone(); Py::Bool({ let t3336 = Py::Bool(true); if py::is(&t3335, &t3336) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3341 = { let t3337 = v_r.clone(); Py::Bool({ let t3338 = Py::Bool(false); if py::is(&t3337, &t3338) { true } else { false } }) }; if t3341.truthy() { t3341 } else { { let t3339 = v_r.clone(); Py::Bool({ let t3340 = Py::None; if py::is(&t3339, &t3340) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3368: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3360(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3368 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3369 = v_r.clone(); Py::Bool({ let t3370 = Py::Bool(true); if py::is(&t3369, &t3370) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3375 = { let t3371 = v_r.clone(); Py::Bool({ let t3372 = Py::Bool(false); if py::is(&t3371, &t3372) { true } else { false } }) }; if t3375.truthy() { t3375 } else { { let t3373 = v_r.clone(); Py::Bool({ let t3374 = Py::None; if py::is(&t3373, &t3374) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_336(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t342: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_337(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t342 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t343 = v_r.clone(); Py::Bool({ let t344 = Py::Bool(true); if py::is(&t343, &t344) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t349 = { let t345 = v_r.clone(); Py::Bool({ let t346 = Py::Bool(false); if py::is(&t345, &t346) { true } else { false } }) }; if t349.truthy() { t349 } else { { let t347 = v_r.clone(); Py::Bool({ let t348 = Py::None; if py::is(&t347, &t348) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3360(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t3367 = f__iv(py::getitem(&v_o.clone(), &Py::str("lucas"))?)?; if !t3367.truthy() { t3367 } else { py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let t3362 = py::getitem(&v_o.clone(), &Py::str("lucas"))?; Py::Bool({ let t3363 = py::sub(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_a.clone())?; if py::eq(&t3362, &t3363) { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f__fib_pair(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?).items()?); v })[..])? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_337(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t341 = f__iv(py::getitem(&v_o.clone(), &Py::str("coefficient"))?)?; if !t341.truthy() { t341 } else { { let t339 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("coefficient"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; Py::Bool({ let t340 = py::mul(&py::mul(&py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, &py::pow(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::pow(&py::int(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; if py::eq(&t339, &t340) { true } else { false } }) } } });
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
fn s_3501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3563: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3502(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3563 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3564 = v_r.clone(); Py::Bool({ let t3565 = Py::Bool(true); if py::is(&t3564, &t3565) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3570 = { let t3566 = v_r.clone(); Py::Bool({ let t3567 = Py::Bool(false); if py::is(&t3566, &t3567) { true } else { false } }) }; if t3570.truthy() { t3570 } else { { let t3568 = v_r.clone(); Py::Bool({ let t3569 = Py::None; if py::is(&t3568, &t3569) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3502(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3687(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3741: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3688(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3741 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3742 = v_r.clone(); Py::Bool({ let t3743 = Py::Bool(true); if py::is(&t3742, &t3743) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3748 = { let t3744 = v_r.clone(); Py::Bool({ let t3745 = Py::Bool(false); if py::is(&t3744, &t3745) { true } else { false } }) }; if t3748.truthy() { t3748 } else { { let t3746 = v_r.clone(); Py::Bool({ let t3747 = Py::None; if py::is(&t3746, &t3747) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3688(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_378(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t391: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_bip(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t391 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t392 = v_r.clone(); Py::Bool({ let t393 = Py::Bool(true); if py::is(&t392, &t393) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t398 = { let t394 = v_r.clone(); Py::Bool({ let t395 = Py::Bool(false); if py::is(&t394, &t395) { true } else { false } }) }; if t398.truthy() { t398 } else { { let t396 = v_r.clone(); Py::Bool({ let t397 = Py::None; if py::is(&t396, &t397) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3823(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3880: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3824(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3880 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3881 = v_r.clone(); Py::Bool({ let t3882 = Py::Bool(true); if py::is(&t3881, &t3882) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3887 = { let t3883 = v_r.clone(); Py::Bool({ let t3884 = Py::Bool(false); if py::is(&t3883, &t3884) { true } else { false } }) }; if t3887.truthy() { t3887 } else { { let t3885 = v_r.clone(); Py::Bool({ let t3886 = Py::None; if py::is(&t3885, &t3886) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3824(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__v_sop(py::int(&py::getitem(&v_i.clone(), &Py::str("variables"))?)?, py::sorted(&py::set(&{ let mut out = vec![]; for t3877 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("minterms"))?))? { let mut v_m: Py; 
v_m = t3877;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?, py::sorted(&py::set(&{ let mut out = vec![]; for t3879 in py::iter(&({ let t3878 = py::method(&v_i.clone(), "get", &[Py::str("dont_cares")])?; if t3878.truthy() { t3878 } else { Py::list(vec![]) } }))? { let mut v_m: Py; 
v_m = t3879;
out.push(py::int(&v_m.clone())?); } Py::list(out) })?, (Py::Bool(false)).truthy())?, Py::list((py::slice(&Py::str("ABCDEF"), None, Some((py::int(&py::getitem(&v_i.clone(), &Py::str("variables"))?)?).i()?), None)?).items()?), v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3943(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3965: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_mobius(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3965 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3966 = v_r.clone(); Py::Bool({ let t3967 = Py::Bool(true); if py::is(&t3966, &t3967) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3972 = { let t3968 = v_r.clone(); Py::Bool({ let t3969 = Py::Bool(false); if py::is(&t3968, &t3969) { true } else { false } }) }; if t3972.truthy() { t3972 } else { { let t3970 = v_r.clone(); Py::Bool({ let t3971 = Py::None; if py::is(&t3970, &t3971) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_399(mut v_beads: Py, mut v_colours: Py) -> R {
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
fn s_3995(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4005: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3996(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4005 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4006 = v_r.clone(); Py::Bool({ let t4007 = Py::Bool(true); if py::is(&t4006, &t4007) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4012 = { let t4008 = v_r.clone(); Py::Bool({ let t4009 = Py::Bool(false); if py::is(&t4008, &t4009) { true } else { false } }) }; if t4012.truthy() { t4012 } else { { let t4010 = v_r.clone(); Py::Bool({ let t4011 = Py::None; if py::is(&t4010, &t4011) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3996(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4004 = f__iv(py::getitem(&v_o.clone(), &Py::str("inverse"))?)?; if !t4004.truthy() { t4004 } else { { let t4003 = { let t3998 = Py::Int(0i64); Py::Bool({ let t3999 = py::getitem(&v_o.clone(), &Py::str("inverse"))?; if py::le(&t3998, &t3999)? { { let t4000 = py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?; if py::lt(&t3999, &t4000)? { true } else { false } } } else { false } }) }; if !t4003.truthy() { t4003 } else { { let t4001 = py::modulo(&py::mul(&py::int(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_o.clone(), &Py::str("inverse"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?)?; Py::Bool({ let t4002 = Py::Int(1i64); if py::eq(&t4001, &t4002) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4029(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4035: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4030(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4035 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4036 = v_r.clone(); Py::Bool({ let t4037 = Py::Bool(true); if py::is(&t4036, &t4037) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4042 = { let t4038 = v_r.clone(); Py::Bool({ let t4039 = Py::Bool(false); if py::is(&t4038, &t4039) { true } else { false } }) }; if t4042.truthy() { t4042 } else { { let t4040 = v_r.clone(); Py::Bool({ let t4041 = Py::None; if py::is(&t4040, &t4041) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4030(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4034 = f__iv(py::getitem(&v_o.clone(), &Py::str("result"))?)?; if !t4034.truthy() { t4034 } else { { let t4032 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t4033 = py::pow_mod(&py::int(&py::getitem(&v_i.clone(), &Py::str("base"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("exponent"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("modulus"))?)?)?; if py::eq(&t4032, &t4033) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4059(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4070: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_modred(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4070 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4071 = v_r.clone(); Py::Bool({ let t4072 = Py::Bool(true); if py::is(&t4071, &t4072) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4077 = { let t4073 = v_r.clone(); Py::Bool({ let t4074 = Py::Bool(false); if py::is(&t4073, &t4074) { true } else { false } }) }; if t4077.truthy() { t4077 } else { { let t4075 = v_r.clone(); Py::Bool({ let t4076 = Py::None; if py::is(&t4075, &t4076) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4117: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4110(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4117 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4118 = v_r.clone(); Py::Bool({ let t4119 = Py::Bool(true); if py::is(&t4118, &t4119) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4124 = { let t4120 = v_r.clone(); Py::Bool({ let t4121 = Py::Bool(false); if py::is(&t4120, &t4121) { true } else { false } }) }; if t4124.truthy() { t4124 } else { { let t4122 = v_r.clone(); Py::Bool({ let t4123 = Py::None; if py::is(&t4122, &t4123) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4110(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4116 = f__iv(py::getitem(&v_o.clone(), &Py::str("coefficient"))?)?; if !t4116.truthy() { t4116 } else { { let t4112 = py::mul(&py::getitem(&v_o.clone(), &Py::str("coefficient"))?, &py::prod(&{ let mut out = vec![]; for t4114 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("counts"))?))? { let mut v_k: Py; 
v_k = t4114;
out.push(py::factorial(&py::int(&v_k.clone())?)?); } Py::list(out) })?)?; Py::Bool({ let t4113 = py::factorial(&py::sum(&{ let mut out = vec![]; for t4115 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("counts"))?))? { let mut v_k: Py; 
v_k = t4115;
out.push(py::int(&v_k.clone())?); } Py::list(out) }, None)?)?; if py::eq(&t4112, &t4113) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4196(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4218: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_order(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4218 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4219 = v_r.clone(); Py::Bool({ let t4220 = Py::Bool(true); if py::is(&t4219, &t4220) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4225 = { let t4221 = v_r.clone(); Py::Bool({ let t4222 = Py::Bool(false); if py::is(&t4221, &t4222) { true } else { false } }) }; if t4225.truthy() { t4225 } else { { let t4223 = v_r.clone(); Py::Bool({ let t4224 = Py::None; if py::is(&t4223, &t4224) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4243(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4249: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4244(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4249 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4250 = v_r.clone(); Py::Bool({ let t4251 = Py::Bool(true); if py::is(&t4250, &t4251) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4256 = { let t4252 = v_r.clone(); Py::Bool({ let t4253 = Py::Bool(false); if py::is(&t4252, &t4253) { true } else { false } }) }; if t4256.truthy() { t4256 } else { { let t4254 = v_r.clone(); Py::Bool({ let t4255 = Py::None; if py::is(&t4254, &t4255) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4244(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4248 = f__iv(py::getitem(&v_o.clone(), &Py::str("multisets"))?)?; if !t4248.truthy() { t4248 } else { { let t4246 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("multisets"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &Py::Int(1i64))?)?)?; Py::Bool({ let t4247 = py::factorial(&py::sub(&py::add(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?, &Py::Int(1i64))?)?; if py::eq(&t4246, &t4247) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4318(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4329: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_neck(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4329 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4330 = v_r.clone(); Py::Bool({ let t4331 = Py::Bool(true); if py::is(&t4330, &t4331) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4336 = { let t4332 = v_r.clone(); Py::Bool({ let t4333 = Py::Bool(false); if py::is(&t4332, &t4333) { true } else { false } }) }; if t4336.truthy() { t4336 } else { { let t4334 = v_r.clone(); Py::Bool({ let t4335 = Py::None; if py::is(&t4334, &t4335) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4373(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4397: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_next(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4397 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4398 = v_r.clone(); Py::Bool({ let t4399 = Py::Bool(true); if py::is(&t4398, &t4399) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4404 = { let t4400 = v_r.clone(); Py::Bool({ let t4401 = Py::Bool(false); if py::is(&t4400, &t4401) { true } else { false } }) }; if t4404.truthy() { t4404 } else { { let t4402 = v_r.clone(); Py::Bool({ let t4403 = Py::None; if py::is(&t4402, &t4403) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4427: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4406(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4427 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4428 = v_r.clone(); Py::Bool({ let t4429 = Py::Bool(true); if py::is(&t4428, &t4429) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4434 = { let t4430 = v_r.clone(); Py::Bool({ let t4431 = Py::Bool(false); if py::is(&t4430, &t4431) { true } else { false } }) }; if t4434.truthy() { t4434 } else { { let t4432 = v_r.clone(); Py::Bool({ let t4433 = Py::None; if py::is(&t4432, &t4433) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4406(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4426 = f__iv(py::getitem(&v_o.clone(), &Py::str("prime"))?)?; if !t4426.truthy() { t4426 } else { { let t4425 = f__is_prime_alt(py::getitem(&v_o.clone(), &Py::str("prime"))?)?; if !t4425.truthy() { t4425 } else { { let t4423 = ax_engine::pysym::helper("discrete_math:_odd_sieve_count", &[py::getitem(&v_o.clone(), &Py::str("prime"))?], &[])?; Py::Bool({ let t4424 = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?; if py::eq(&t4423, &t4424) { true } else { false } }) } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4453(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4462: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4462 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4463 = v_r.clone(); Py::Bool({ let t4464 = Py::Bool(true); if py::is(&t4463, &t4464) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4469 = { let t4465 = v_r.clone(); Py::Bool({ let t4466 = Py::Bool(false); if py::is(&t4465, &t4466) { true } else { false } }) }; if t4469.truthy() { t4469 } else { { let t4467 = v_r.clone(); Py::Bool({ let t4468 = Py::None; if py::is(&t4467, &t4468) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4492(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4506: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pk(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4506 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4507 = v_r.clone(); Py::Bool({ let t4508 = Py::Bool(true); if py::is(&t4507, &t4508) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4513 = { let t4509 = v_r.clone(); Py::Bool({ let t4510 = Py::Bool(false); if py::is(&t4509, &t4510) { true } else { false } }) }; if t4513.truthy() { t4513 } else { { let t4511 = v_r.clone(); Py::Bool({ let t4512 = Py::None; if py::is(&t4511, &t4512) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4539(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4557: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pell(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4557 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4558 = v_r.clone(); Py::Bool({ let t4559 = Py::Bool(true); if py::is(&t4558, &t4559) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4564 = { let t4560 = v_r.clone(); Py::Bool({ let t4561 = Py::Bool(false); if py::is(&t4560, &t4561) { true } else { false } }) }; if t4564.truthy() { t4564 } else { { let t4562 = v_r.clone(); Py::Bool({ let t4563 = Py::None; if py::is(&t4562, &t4563) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4589: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4584(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4589 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4590 = v_r.clone(); Py::Bool({ let t4591 = Py::Bool(true); if py::is(&t4590, &t4591) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4596 = { let t4592 = v_r.clone(); Py::Bool({ let t4593 = Py::Bool(false); if py::is(&t4592, &t4593) { true } else { false } }) }; if t4596.truthy() { t4596 } else { { let t4594 = v_r.clone(); Py::Bool({ let t4595 = Py::None; if py::is(&t4594, &t4595) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4584(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4588 = f__iv(py::getitem(&v_o.clone(), &Py::str("permutations"))?)?; if !t4588.truthy() { t4588 } else { { let t4586 = py::mul(&py::getitem(&v_o.clone(), &Py::str("permutations"))?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; Py::Bool({ let t4587 = py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t4586, &t4587) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_461(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_neck(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t472 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t473 = v_r.clone(); Py::Bool({ let t474 = Py::Bool(true); if py::is(&t473, &t474) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t479 = { let t475 = v_r.clone(); Py::Bool({ let t476 = Py::Bool(false); if py::is(&t475, &t476) { true } else { false } }) }; if t479.truthy() { t479 } else { { let t477 = v_r.clone(); Py::Bool({ let t478 = Py::None; if py::is(&t477, &t478) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4612(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4619: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4613(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4619 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4620 = v_r.clone(); Py::Bool({ let t4621 = Py::Bool(true); if py::is(&t4620, &t4621) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4626 = { let t4622 = v_r.clone(); Py::Bool({ let t4623 = Py::Bool(false); if py::is(&t4622, &t4623) { true } else { false } }) }; if t4626.truthy() { t4626 } else { { let t4624 = v_r.clone(); Py::Bool({ let t4625 = Py::None; if py::is(&t4624, &t4625) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4613(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4618 = f__iv(py::getitem(&v_o.clone(), &Py::str("guaranteed"))?)?; if !t4618.truthy() { t4618 } else { { let t4615 = py::mul(&py::sub(&py::getitem(&v_o.clone(), &Py::str("guaranteed"))?, &Py::Int(1i64))?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?; Py::Bool({ let t4616 = py::int(&py::getitem(&v_i.clone(), &Py::str("items"))?)?; if py::lt(&t4615, &t4616)? { { let t4617 = py::mul(&py::getitem(&v_o.clone(), &Py::str("guaranteed"))?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?; if py::le(&t4616, &t4617)? { true } else { false } } } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4642(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4649: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4643(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4649 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4650 = v_r.clone(); Py::Bool({ let t4651 = Py::Bool(true); if py::is(&t4650, &t4651) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4656 = { let t4652 = v_r.clone(); Py::Bool({ let t4653 = Py::Bool(false); if py::is(&t4652, &t4653) { true } else { false } }) }; if t4656.truthy() { t4656 } else { { let t4654 = v_r.clone(); Py::Bool({ let t4655 = Py::None; if py::is(&t4654, &t4655) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4643(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4648 = f__iv(py::getitem(&v_o.clone(), &Py::str("needed"))?)?; if !t4648.truthy() { t4648 } else { { let t4645 = py::neg(&py::floordiv(&py::neg(&py::getitem(&v_o.clone(), &Py::str("needed"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?)?; Py::Bool({ let t4646 = py::int(&py::getitem(&v_i.clone(), &Py::str("at_least"))?)?; if py::ge(&t4645, &t4646)? { { let t4647 = py::neg(&py::floordiv(&py::neg(&py::sub(&py::getitem(&v_o.clone(), &Py::str("needed"))?, &Py::Int(1i64))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("boxes"))?)?)?)?; if py::gt(&t4646, &t4647)? { true } else { false } } } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4693(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4717: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_prev(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4717 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4718 = v_r.clone(); Py::Bool({ let t4719 = Py::Bool(true); if py::is(&t4718, &t4719) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4724 = { let t4720 = v_r.clone(); Py::Bool({ let t4721 = Py::Bool(false); if py::is(&t4720, &t4721) { true } else { false } }) }; if t4724.truthy() { t4724 } else { { let t4722 = v_r.clone(); Py::Bool({ let t4723 = Py::None; if py::is(&t4722, &t4723) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4725(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4731: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_4726(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4731 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4732 = v_r.clone(); Py::Bool({ let t4733 = Py::Bool(true); if py::is(&t4732, &t4733) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4738 = { let t4734 = v_r.clone(); Py::Bool({ let t4735 = Py::Bool(false); if py::is(&t4734, &t4735) { true } else { false } }) }; if t4738.truthy() { t4738 } else { { let t4736 = v_r.clone(); Py::Bool({ let t4737 = Py::None; if py::is(&t4736, &t4737) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4726(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t4730 = f__iv(py::getitem(&v_o.clone(), &Py::str("count"))?)?; if !t4730.truthy() { t4730 } else { { let t4728 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t4729 = ax_engine::pysym::helper("discrete_math:_odd_sieve_count", &[py::int(&py::getitem(&v_i.clone(), &Py::str("x"))?)?], &[])?; if py::eq(&t4728, &t4729) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4797(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4845: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_factor(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4845 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4846 = v_r.clone(); Py::Bool({ let t4847 = Py::Bool(true); if py::is(&t4846, &t4847) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4852 = { let t4848 = v_r.clone(); Py::Bool({ let t4849 = Py::Bool(false); if py::is(&t4848, &t4849) { true } else { false } }) }; if t4852.truthy() { t4852 } else { { let t4850 = v_r.clone(); Py::Bool({ let t4851 = Py::None; if py::is(&t4850, &t4851) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4910(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4937: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_proot(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4937 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4938 = v_r.clone(); Py::Bool({ let t4939 = Py::Bool(true); if py::is(&t4938, &t4939) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4944 = { let t4940 = v_r.clone(); Py::Bool({ let t4941 = Py::Bool(false); if py::is(&t4940, &t4941) { true } else { false } }) }; if t4944.truthy() { t4944 } else { { let t4942 = v_r.clone(); Py::Bool({ let t4943 = Py::None; if py::is(&t4942, &t4943) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4982(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5047: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pdecode(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5047 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5048 = v_r.clone(); Py::Bool({ let t5049 = Py::Bool(true); if py::is(&t5048, &t5049) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5054 = { let t5050 = v_r.clone(); Py::Bool({ let t5051 = Py::Bool(false); if py::is(&t5050, &t5051) { true } else { false } }) }; if t5054.truthy() { t5054 } else { { let t5052 = v_r.clone(); Py::Bool({ let t5053 = Py::None; if py::is(&t5052, &t5053) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5106(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5132: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_pencode(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5132 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5133 = v_r.clone(); Py::Bool({ let t5134 = Py::Bool(true); if py::is(&t5133, &t5134) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5139 = { let t5135 = v_r.clone(); Py::Bool({ let t5136 = Py::Bool(false); if py::is(&t5135, &t5136) { true } else { false } }) }; if t5139.truthy() { t5139 } else { { let t5137 = v_r.clone(); Py::Bool({ let t5138 = Py::None; if py::is(&t5137, &t5138) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5157(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5163: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5158(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5163 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5164 = v_r.clone(); Py::Bool({ let t5165 = Py::Bool(true); if py::is(&t5164, &t5165) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5170 = { let t5166 = v_r.clone(); Py::Bool({ let t5167 = Py::Bool(false); if py::is(&t5166, &t5167) { true } else { false } }) }; if t5170.truthy() { t5170 } else { { let t5168 = v_r.clone(); Py::Bool({ let t5169 = Py::None; if py::is(&t5168, &t5169) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5158(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5162 = f__iv(py::getitem(&v_o.clone(), &Py::str("m"))?)?; if !t5162.truthy() { t5162 } else { { let t5160 = py::getitem(&v_o.clone(), &Py::str("m"))?; Py::Bool({ let t5161 = py::pow_mod(&py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t5160, &t5161) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5227(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5243: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5228(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5243 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5244 = v_r.clone(); Py::Bool({ let t5245 = Py::Bool(true); if py::is(&t5244, &t5245) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5250 = { let t5246 = v_r.clone(); Py::Bool({ let t5247 = Py::Bool(false); if py::is(&t5246, &t5247) { true } else { false } }) }; if t5250.truthy() { t5250 } else { { let t5248 = v_r.clone(); Py::Bool({ let t5249 = Py::None; if py::is(&t5248, &t5249) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5228(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_q: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_e: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(3).cloned().unwrap_or(Py::None);
            return Ok({ let t5242 = { let t5229 = py::getitem(&v_o.clone(), &Py::str("n"))?; Py::Bool({ let t5230 = py::mul(&v_p.clone(), &v_q.clone())?; if py::eq(&t5229, &t5230) { true } else { false } }) }; if !t5242.truthy() { t5242 } else { { let t5241 = { let t5231 = py::modulo(&py::mul(&v_e.clone(), &py::getitem(&v_o.clone(), &Py::str("d"))?)?, &py::mul(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::sub(&v_q.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t5232 = Py::Int(1i64); if py::eq(&t5231, &t5232) { true } else { false } }) }; if !t5241.truthy() { t5241 } else { { let t5240 = f__iv(py::getitem(&v_o.clone(), &Py::str("m"))?)?; if !t5240.truthy() { t5240 } else { { let t5239 = { let t5234 = Py::Int(0i64); Py::Bool({ let t5235 = py::getitem(&v_o.clone(), &Py::str("m"))?; if py::le(&t5234, &t5235)? { { let t5236 = py::mul(&v_p.clone(), &v_q.clone())?; if py::lt(&t5235, &t5236)? { true } else { false } } } else { false } }) }; if !t5239.truthy() { t5239 } else { { let t5237 = py::pow_mod(&py::getitem(&v_o.clone(), &Py::str("m"))?, &v_e.clone(), &py::mul(&v_p.clone(), &v_q.clone())?)?; Py::Bool({ let t5238 = v_c.clone(); if py::eq(&t5237, &t5238) { true } else { false } }) } } } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("q"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("e"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_524(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t554: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_br(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t554 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t555 = v_r.clone(); Py::Bool({ let t556 = Py::Bool(true); if py::is(&t555, &t556) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t561 = { let t557 = v_r.clone(); Py::Bool({ let t558 = Py::Bool(false); if py::is(&t557, &t558) { true } else { false } }) }; if t561.truthy() { t561 } else { { let t559 = v_r.clone(); Py::Bool({ let t560 = Py::None; if py::is(&t559, &t560) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5268(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5274: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5269(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5274 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5275 = v_r.clone(); Py::Bool({ let t5276 = Py::Bool(true); if py::is(&t5275, &t5276) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5281 = { let t5277 = v_r.clone(); Py::Bool({ let t5278 = Py::Bool(false); if py::is(&t5277, &t5278) { true } else { false } }) }; if t5281.truthy() { t5281 } else { { let t5279 = v_r.clone(); Py::Bool({ let t5280 = Py::None; if py::is(&t5279, &t5280) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5269(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5273 = f__iv(py::getitem(&v_o.clone(), &Py::str("c"))?)?; if !t5273.truthy() { t5273 } else { { let t5271 = py::getitem(&v_o.clone(), &Py::str("c"))?; Py::Bool({ let t5272 = py::pow_mod(&py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("e"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t5271, &t5272) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5334(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5350: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5335(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5350 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5351 = v_r.clone(); Py::Bool({ let t5352 = Py::Bool(true); if py::is(&t5351, &t5352) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5357 = { let t5353 = v_r.clone(); Py::Bool({ let t5354 = Py::Bool(false); if py::is(&t5353, &t5354) { true } else { false } }) }; if t5357.truthy() { t5357 } else { { let t5355 = v_r.clone(); Py::Bool({ let t5356 = Py::None; if py::is(&t5355, &t5356) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5335(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_q: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_e: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok({ let t5349 = f__iv(py::getitem(&v_o.clone(), &Py::str("d"))?)?; if !t5349.truthy() { t5349 } else { { let t5348 = { let t5337 = py::getitem(&v_o.clone(), &Py::str("n"))?; Py::Bool({ let t5338 = py::mul(&v_p.clone(), &v_q.clone())?; if py::eq(&t5337, &t5338) { true } else { false } }) }; if !t5348.truthy() { t5348 } else { { let t5347 = { let t5339 = py::getitem(&v_o.clone(), &Py::str("phi"))?; Py::Bool({ let t5340 = py::mul(&py::sub(&v_p.clone(), &Py::Int(1i64))?, &py::sub(&v_q.clone(), &Py::Int(1i64))?)?; if py::eq(&t5339, &t5340) { true } else { false } }) }; if !t5347.truthy() { t5347 } else { { let t5346 = { let t5341 = Py::Int(0i64); Py::Bool({ let t5342 = py::getitem(&v_o.clone(), &Py::str("d"))?; if py::lt(&t5341, &t5342)? { { let t5343 = py::getitem(&v_o.clone(), &Py::str("phi"))?; if py::lt(&t5342, &t5343)? { true } else { false } } } else { false } }) }; if !t5346.truthy() { t5346 } else { { let t5344 = py::modulo(&py::mul(&v_e.clone(), &py::getitem(&v_o.clone(), &Py::str("d"))?)?, &py::getitem(&v_o.clone(), &Py::str("phi"))?)?; Py::Bool({ let t5345 = Py::Int(1i64); if py::eq(&t5344, &t5345) { true } else { false } }) } } } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::int(&py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("q"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("e"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5375(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5381: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5376(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5381 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5382 = v_r.clone(); Py::Bool({ let t5383 = Py::Bool(true); if py::is(&t5382, &t5383) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5388 = { let t5384 = v_r.clone(); Py::Bool({ let t5385 = Py::Bool(false); if py::is(&t5384, &t5385) { true } else { false } }) }; if t5388.truthy() { t5388 } else { { let t5386 = v_r.clone(); Py::Bool({ let t5387 = Py::None; if py::is(&t5386, &t5387) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5376(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5380 = f__iv(py::getitem(&v_o.clone(), &Py::str("sequences"))?)?; if !t5380.truthy() { t5380 } else { { let t5378 = py::getitem(&v_o.clone(), &Py::str("sequences"))?; Py::Bool({ let t5379 = py::prod(&py::mul(&Py::list(vec![py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?]), &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?; if py::eq(&t5378, &t5379) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5431(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5453: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_5432(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5453 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5454 = v_r.clone(); Py::Bool({ let t5455 = Py::Bool(true); if py::is(&t5454, &t5455) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5460 = { let t5456 = v_r.clone(); Py::Bool({ let t5457 = Py::Bool(false); if py::is(&t5456, &t5457) { true } else { false } }) }; if t5460.truthy() { t5460 } else { { let t5458 = v_r.clone(); Py::Bool({ let t5459 = Py::None; if py::is(&t5458, &t5459) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5432(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t5452 = f__iv(py::getitem(&v_o.clone(), &Py::str("spanning_trees"))?)?; if !t5452.truthy() { t5452 } else { { let t5434 = py::getitem(&v_o.clone(), &Py::str("spanning_trees"))?; Py::Bool({ let t5435 = f__bareiss(f__laplacian_minor(f__graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false), Py::Bool(false))?)?)?; if py::eq(&t5434, &t5435) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5518(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5548: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_sqrtmod(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5548 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5549 = v_r.clone(); Py::Bool({ let t5550 = Py::Bool(true); if py::is(&t5549, &t5550) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5555 = { let t5551 = v_r.clone(); Py::Bool({ let t5552 = Py::Bool(false); if py::is(&t5551, &t5552) { true } else { false } }) }; if t5555.truthy() { t5555 } else { { let t5553 = v_r.clone(); Py::Bool({ let t5554 = Py::None; if py::is(&t5553, &t5554) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5597: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_stars(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5597 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5598 = v_r.clone(); Py::Bool({ let t5599 = Py::Bool(true); if py::is(&t5598, &t5599) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5604 = { let t5600 = v_r.clone(); Py::Bool({ let t5601 = Py::Bool(false); if py::is(&t5600, &t5601) { true } else { false } }) }; if t5604.truthy() { t5604 } else { { let t5602 = v_r.clone(); Py::Bool({ let t5603 = Py::None; if py::is(&t5602, &t5603) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5624(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5642: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_s1(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5642 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5643 = v_r.clone(); Py::Bool({ let t5644 = Py::Bool(true); if py::is(&t5643, &t5644) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5649 = { let t5645 = v_r.clone(); Py::Bool({ let t5646 = Py::Bool(false); if py::is(&t5645, &t5646) { true } else { false } }) }; if t5649.truthy() { t5649 } else { { let t5647 = v_r.clone(); Py::Bool({ let t5648 = Py::None; if py::is(&t5647, &t5648) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5669(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5679: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_s2(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5679 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5680 = v_r.clone(); Py::Bool({ let t5681 = Py::Bool(true); if py::is(&t5680, &t5681) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5686 = { let t5682 = v_r.clone(); Py::Bool({ let t5683 = Py::Bool(false); if py::is(&t5682, &t5683) { true } else { false } }) }; if t5686.truthy() { t5686 } else { { let t5684 = v_r.clone(); Py::Bool({ let t5685 = Py::None; if py::is(&t5684, &t5685) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5723(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5745: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_scc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5745 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5746 = v_r.clone(); Py::Bool({ let t5747 = Py::Bool(true); if py::is(&t5746, &t5747) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5752 = { let t5748 = v_r.clone(); Py::Bool({ let t5749 = Py::Bool(false); if py::is(&t5748, &t5749) { true } else { false } }) }; if t5752.truthy() { t5752 } else { { let t5750 = v_r.clone(); Py::Bool({ let t5751 = Py::None; if py::is(&t5750, &t5751) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5787(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5811: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_topo(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5811 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5812 = v_r.clone(); Py::Bool({ let t5813 = Py::Bool(true); if py::is(&t5812, &t5813) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5818 = { let t5814 = v_r.clone(); Py::Bool({ let t5815 = Py::Bool(false); if py::is(&t5814, &t5815) { true } else { false } }) }; if t5818.truthy() { t5818 } else { { let t5816 = v_r.clone(); Py::Bool({ let t5817 = Py::None; if py::is(&t5816, &t5817) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t596: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_584(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t596 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t597 = v_r.clone(); Py::Bool({ let t598 = Py::Bool(true); if py::is(&t597, &t598) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t603 = { let t599 = v_r.clone(); Py::Bool({ let t600 = Py::Bool(false); if py::is(&t599, &t600) { true } else { false } }) }; if t603.truthy() { t603 } else { { let t601 = v_r.clone(); Py::Bool({ let t602 = Py::None; if py::is(&t601, &t602) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_584(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t595 = { let t585 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t586 = py::getitem(&f_caesar(py::getitem(&v_i.clone(), &Py::str("message"))?, py::getitem(&v_i.clone(), &Py::str("shift"))?, py::getitem(&v_i.clone(), &Py::str("mode"))?)?, &Py::str("result"))?; if py::eq(&t585, &t586) { true } else { false } }) }; if !t595.truthy() { t595 } else { { let t587 = py::getitem(&f_caesar(py::getitem(&v_o.clone(), &Py::str("result"))?, py::getitem(&v_i.clone(), &Py::str("shift"))?, (if ({ let t589 = py::getitem(&v_i.clone(), &Py::str("mode"))?; Py::Bool({ let t590 = Py::str("encrypt"); if py::eq(&t589, &t590) { true } else { false } }) }).truthy() { Py::str("decrypt") } else { Py::str("encrypt") }))?, &Py::str("result"))?; Py::Bool({ let t588 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t591 in py::iter(&(py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("message"))?)), "upper", &[])?))? { let mut v_c: Py; 
v_c = t591;
if ({ let t592 = Py::str("A"); Py::Bool({ let t593 = v_c.clone(); if py::le(&t592, &t593)? { { let t594 = Py::str("Z"); if py::le(&t593, &t594)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) }])?; if py::eq(&t587, &t588) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5889(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_tt(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5907 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t5908 = v_r.clone(); Py::Bool({ let t5909 = Py::Bool(true); if py::is(&t5908, &t5909) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5914 = { let t5910 = v_r.clone(); Py::Bool({ let t5911 = Py::Bool(false); if py::is(&t5910, &t5911) { true } else { false } }) }; if t5914.truthy() { t5914 } else { { let t5912 = v_r.clone(); Py::Bool({ let t5913 = Py::None; if py::is(&t5912, &t5913) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6020: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_twosq(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6020 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6021 = v_r.clone(); Py::Bool({ let t6022 = Py::Bool(true); if py::is(&t6021, &t6022) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6027 = { let t6023 = v_r.clone(); Py::Bool({ let t6024 = Py::Bool(false); if py::is(&t6023, &t6024) { true } else { false } }) }; if t6027.truthy() { t6027 } else { { let t6025 = v_r.clone(); Py::Bool({ let t6026 = Py::None; if py::is(&t6025, &t6026) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6051(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6067: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_twos(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6067 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6068 = v_r.clone(); Py::Bool({ let t6069 = Py::Bool(true); if py::is(&t6068, &t6069) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6074 = { let t6070 = v_r.clone(); Py::Bool({ let t6071 = Py::Bool(false); if py::is(&t6070, &t6071) { true } else { false } }) }; if t6074.truthy() { t6074 } else { { let t6072 = v_r.clone(); Py::Bool({ let t6073 = Py::None; if py::is(&t6072, &t6073) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6086(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6099: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_6087(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6099 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6100 = v_r.clone(); Py::Bool({ let t6101 = Py::Bool(true); if py::is(&t6100, &t6101) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6106 = { let t6102 = v_r.clone(); Py::Bool({ let t6103 = Py::Bool(false); if py::is(&t6102, &t6103) { true } else { false } }) }; if t6106.truthy() { t6106 } else { { let t6104 = v_r.clone(); Py::Bool({ let t6105 = Py::None; if py::is(&t6104, &t6105) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6087(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t6098 = { let t6088 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t6089 = py::getitem(&f_vigenere(py::getitem(&v_i.clone(), &Py::str("message"))?, py::getitem(&v_i.clone(), &Py::str("key"))?, py::getitem(&v_i.clone(), &Py::str("mode"))?)?, &Py::str("result"))?; if py::eq(&t6088, &t6089) { true } else { false } }) }; if !t6098.truthy() { t6098 } else { { let t6090 = py::getitem(&f_vigenere(py::getitem(&v_o.clone(), &Py::str("result"))?, py::getitem(&v_i.clone(), &Py::str("key"))?, (if ({ let t6092 = py::getitem(&v_i.clone(), &Py::str("mode"))?; Py::Bool({ let t6093 = Py::str("encrypt"); if py::eq(&t6092, &t6093) { true } else { false } }) }).truthy() { Py::str("decrypt") } else { Py::str("encrypt") }))?, &Py::str("result"))?; Py::Bool({ let t6091 = py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t6094 in py::iter(&(py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("message"))?)), "upper", &[])?))? { let mut v_c: Py; 
v_c = t6094;
if ({ let t6095 = Py::str("A"); Py::Bool({ let t6096 = v_c.clone(); if py::le(&t6095, &t6096)? { { let t6097 = Py::str("Z"); if py::le(&t6096, &t6097)? { true } else { false } } } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) }])?; if py::eq(&t6090, &t6091) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6147(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6156: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_walks(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6156 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6157 = v_r.clone(); Py::Bool({ let t6158 = Py::Bool(true); if py::is(&t6157, &t6158) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6163 = { let t6159 = v_r.clone(); Py::Bool({ let t6160 = Py::Bool(false); if py::is(&t6159, &t6160) { true } else { false } }) }; if t6163.truthy() { t6163 } else { { let t6161 = v_r.clone(); Py::Bool({ let t6162 = Py::None; if py::is(&t6161, &t6162) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6171(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_word(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6184 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t6185 = v_r.clone(); Py::Bool({ let t6186 = Py::Bool(true); if py::is(&t6185, &t6186) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6191 = { let t6187 = v_r.clone(); Py::Bool({ let t6188 = Py::Bool(false); if py::is(&t6187, &t6188) { true } else { false } }) }; if t6191.truthy() { t6191 } else { { let t6189 = v_r.clone(); Py::Bool({ let t6190 = Py::None; if py::is(&t6189, &t6190) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_666(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t698: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_carm(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t698 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t699 = v_r.clone(); Py::Bool({ let t700 = Py::Bool(true); if py::is(&t699, &t700) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t705 = { let t701 = v_r.clone(); Py::Bool({ let t702 = Py::Bool(false); if py::is(&t701, &t702) { true } else { false } }) }; if t705.truthy() { t705 } else { { let t703 = v_r.clone(); Py::Bool({ let t704 = Py::None; if py::is(&t703, &t704) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_720(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t726: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_catalan(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t726 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t727 = v_r.clone(); Py::Bool({ let t728 = Py::Bool(true); if py::is(&t727, &t728) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t733 = { let t729 = v_r.clone(); Py::Bool({ let t730 = Py::Bool(false); if py::is(&t729, &t730) { true } else { false } }) }; if t733.truthy() { t733 } else { { let t731 = v_r.clone(); Py::Bool({ let t732 = Py::None; if py::is(&t731, &t732) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_750(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t770: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_cfr(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t770 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t771 = v_r.clone(); Py::Bool({ let t772 = Py::Bool(true); if py::is(&t771, &t772) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t777 = { let t773 = v_r.clone(); Py::Bool({ let t774 = Py::Bool(false); if py::is(&t773, &t774) { true } else { false } }) }; if t777.truthy() { t777 } else { { let t775 = v_r.clone(); Py::Bool({ let t776 = Py::None; if py::is(&t775, &t776) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_797(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t819: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_cfs(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t819 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t820 = v_r.clone(); Py::Bool({ let t821 = Py::Bool(true); if py::is(&t820, &t821) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t826 = { let t822 = v_r.clone(); Py::Bool({ let t823 = Py::Bool(false); if py::is(&t822, &t823) { true } else { false } }) }; if t826.truthy() { t826 } else { { let t824 = v_r.clone(); Py::Bool({ let t825 = Py::None; if py::is(&t824, &t825) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t879: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_chrom(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t879 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t880 = v_r.clone(); Py::Bool({ let t881 = Py::Bool(true); if py::is(&t880, &t881) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t886 = { let t882 = v_r.clone(); Py::Bool({ let t883 = Py::Bool(false); if py::is(&t882, &t883) { true } else { false } }) }; if t886.truthy() { t886 } else { { let t884 = v_r.clone(); Py::Bool({ let t885 = Py::None; if py::is(&t884, &t885) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_924(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t946: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_clique(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t946 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t947 = v_r.clone(); Py::Bool({ let t948 = Py::Bool(true); if py::is(&t947, &t948) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t953 = { let t949 = v_r.clone(); Py::Bool({ let t950 = Py::Bool(false); if py::is(&t949, &t950) { true } else { false } }) }; if t953.truthy() { t953 } else { { let t951 = v_r.clone(); Py::Bool({ let t952 = Py::None; if py::is(&t951, &t952) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t980: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_975(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t980 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t981 = v_r.clone(); Py::Bool({ let t982 = Py::Bool(true); if py::is(&t981, &t982) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t987 = { let t983 = v_r.clone(); Py::Bool({ let t984 = Py::Bool(false); if py::is(&t983, &t984) { true } else { false } }) }; if t987.truthy() { t987 } else { { let t985 = v_r.clone(); Py::Bool({ let t986 = Py::None; if py::is(&t985, &t986) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_975(mut v_i: Py, mut v_o: Py) -> R {
    return Ok({ let t979 = f__iv(py::getitem(&v_o.clone(), &Py::str("combinations"))?)?; if !t979.truthy() { t979 } else { { let t977 = py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("combinations"))?, &py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?, &py::factorial(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?)?)?; Py::Bool({ let t978 = py::factorial(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?; if py::eq(&t977, &t978) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_98(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t128: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__v_ap(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t128 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t129 = v_r.clone(); Py::Bool({ let t130 = Py::Bool(true); if py::is(&t129, &t130) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t135 = { let t131 = v_r.clone(); Py::Bool({ let t132 = Py::Bool(false); if py::is(&t131, &t132) { true } else { false } }) }; if t135.truthy() { t135 } else { { let t133 = v_r.clone(); Py::Bool({ let t134 = Py::None; if py::is(&t133, &t134) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::float(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
