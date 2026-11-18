//! Discrete Math: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p discrete-math-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&discrete_math_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "bell", "inputs": {"n": "5"}}));
    println!();
    show(json!({"key": "affine_cipher", "inputs": {"a": "5", "b": "8", "message": "AFFINE CIPHER", "mode": "encrypt"}}));
    println!();
    show(json!({"key": "articulation_points", "inputs": {"edges": "A-B, B-C, C-A, C-D, D-E"}}));
    println!();
    show(json!({"key": "logic_contradiction", "inputs": {"formula": "p & ~p"}}));
    println!();
    show(json!({"key": "crt", "inputs": {"residues": [2, 3, 2], "moduli": [3, 5, 7]}}));
    println!();
    show(json!({"key": "base_convert", "inputs": {"digits": "1011", "from_base": "2", "to_base": "10"}}));
    println!();
    show(json!({"key": "carmichael_lambda", "inputs": {"n": "15"}}));
    println!();
    show(json!({"key": "venn_three_sets", "inputs": {"a": "30", "b": "25", "c": "20", "ab": "10", "ac": "8", "bc": "5", "abc": "3", "universe": "60"}}));
    println!();
    show(json!({"key": "apportionment", "inputs": {"method": "jefferson", "seats": "10", "populations": [2400, 3300, 4300]}}));
    println!();
    show(json!({"key": "bernoulli", "inputs": {"n": "10"}}));
    println!();
    show(json!({"key": "caesar_cipher", "inputs": {"message": "HELLO WORLD", "mode": "encrypt", "shift": "3"}}));
    println!();
    show(json!({"key": "bipartite", "inputs": {"edges": "A-B, B-C, C-D, D-A"}}));
    println!();
    show(json!({"key": "logic_evaluate", "inputs": {"formula": "(p -> q) & ~r", "assignment": "p = true, q = false, r = false"}}));
    println!();
    show(json!({"key": "linear_congruence", "inputs": {"a": "3", "b": "4", "m": "7"}}));
    println!();
    show(json!({"key": "twos_complement", "inputs": {"n": "-5", "bits": "8"}}));
}
