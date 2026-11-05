//! Discrete Mathematics' own live figures: what each capability's page shows for its worked example (a cipher's
//! letter map, a search tree, a colouring, a decoded tree, a value over two whole-number inputs, a verdict along n,
//! truth tables, tables of symbols), drawn for ANY inputs from the solved answer and from further solves of the model
//! itself (ax_engine::figkit::Solved). Nothing is recomputed from a formula written here; a drawing that cannot be made
//! for the inputs is left out.

use ax_engine::figkit::{self as k, num, Canvas, Plot, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::pack::Val;
use ax_engine::{Answer, Input};
use serde_json::Value;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "affine_cipher" | "caesar_cipher" | "vigenere_cipher" => vec![letters(s)],
        "bfs_order" => vec![bfs_tree(s)],
        "chromatic_number" => vec![colouring(s)],
        "bracelets" => vec![field2(s, "colours", Ax::from1(10), "beads", Ax::from1(14), "bracelets",
                                   "Bracelets over the number of colours and beads")],
        "necklaces" => vec![field2(s, "colours", Ax::from1(10), "beads", Ax::from1(14), "necklaces",
                                   "Necklaces over the number of colours and beads")],
        "combinations" => vec![field2(s, "n", Ax::from1(20), "k", Ax::from1(11), "combinations",
                                      "Combinations C(n, k) over n and k")],
        "extended_gcd" => vec![field2(s, "b", Ax::Log, "a", Ax::Log, "gcd", "gcd(a, b) over a and b")],
        "jacobi_symbol" => vec![field2(s, "a", Ax::Log, "n", Ax::LogOdd, "symbol", "Jacobi symbol (a/n) over a and n")],
        "lattice_paths" => vec![field2(s, "up", Ax::from1(11), "right", Ax::from1(13), "paths",
                                       "Lattice paths over the steps up and right")],
        "mod_reduce" => vec![field2(s, "m", Ax::from1(13), "a", Ax::from1(34), "remainder",
                                    "Remainder of a modulo m over m and a")],
        "multisets" => vec![field2(s, "k", Ax::from1(11), "n", Ax::from1(13), "multisets",
                                   "Combinations with repetition over k and n")],
        "partitions_k" => vec![field2(s, "n", Ax::from1(20), "k", Ax::from1(11), "partitions",
                                      "Partitions of n into exactly k parts over n and k")],
        "permutations" => vec![field2(s, "k", Ax::from1(11), "n", Ax::from1(20), "permutations",
                                      "Permutations P(n, k) over k and n")],
        "pigeonhole_max" => vec![field2(s, "boxes", Ax::from1(15), "items", Ax::from1(50), "guaranteed",
                                        "Guaranteed load over the numbers of boxes and objects")],
        "pigeonhole_needed" => vec![field2(s, "at_least", Ax::from1(11), "boxes", Ax::from1(12), "needed",
                                           "Objects needed for a guarantee over the count and the categories")],
        "sequences_with_repetition" => vec![field2(s, "k", Ax::from1(12), "n", Ax::from1(20), "sequences",
                                                   "Sequences with repetition over length and symbols")],
        "twos_complement" => vec![field2(s, "bits", Ax::Range(2, 16), "n", Ax::Around(10), "unsigned",
                                         "Two's complement pattern as an unsigned number over word length and n")],
        "crt" | "gcd_lcm" | "graphical_sequence" | "minimal_sop_minterms" | "multinomial" | "truth_table" => vec![io(s)],
        "divisor_functions" => vec![verdict(s, "n", "classification", "count", 1, 720)],
        "integer_sqrt" => vec![verdict(s, "n", "is_square", "isqrt", 1, 2000)],
        "is_prime" => vec![verdict(s, "n", "is_prime", "smallest_prime_factor", 1, 194)],
        "mobius" => vec![verdict(s, "n", "squarefree", "mu", 1, 60)],
        "integer_partitions" => vec![partition_stems(s)],
        "legendre_symbol" => vec![legendre_table(s)],
        "minimal_cnf" | "minimal_dnf" => vec![minimal_truth(s)],
        "logic_evaluate" => vec![evaluate_truth(s)],
        "graph_degrees" => vec![degree_bars(s)],
        "prufer_decode" => vec![prufer_matrix(s), prufer_tree(s)],
        "sqrt_mod" => vec![roots_plane(s)],
        "stirling2" => vec![stirling_heat(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
/// A JSON value as a node name or short text.
fn name_of(v: &Value) -> String {
    match v {
        Value::String(t) => t.clone(),
        Value::Number(n) => n.as_i64().map(|i| i.to_string()).unwrap_or_else(|| num(n.as_f64().unwrap_or(f64::NAN))),
        Value::Bool(b) => if *b { "true".into() } else { "false".into() },
        other => other.to_string(),
    }
}

/// A value as the figure writes it: numbers to 6 digits, lists joined by commas, text as is.
fn show(v: &Value) -> String {
    match v {
        Value::Number(n) => n.as_i64().map(|i| i.to_string()).unwrap_or_else(|| ax_engine::num::py_g(n.as_f64().unwrap_or(f64::NAN), 6)),
        Value::Array(a) => a.iter().map(show).collect::<Vec<_>>().join(", "),
        Value::Object(o) => o.iter().map(|(k, v)| format!("{k}: {}", show(v))).collect::<Vec<_>>().join(", "),
        other => name_of(other),
    }
}

fn input_text(i: &Input) -> String {
    match i {
        Input::Text(t) => t.clone(),
        Input::Int(n) => n.to_string(),
        Input::Float(x) => num(*x),
        Input::Json(v) => show(v),
    }
}

/// The meaning of an output, as the model names it.
fn meaning(s: &Solved, out: &str) -> String {
    s.p.procedure(s.key).and_then(|p| p.outputs.iter().find(|o| o.symbol == out)).map(|o| o.meaning.clone())
        .unwrap_or_else(|| out.to_string())
}

fn cap_name(s: &Solved) -> String {
    s.p.procedure(s.key).map(|p| p.name.clone()).unwrap_or_else(|| s.key.replace('_', " "))
}

/// This capability solved again with whole-number inputs set.
fn with_ints(s: &Solved, set: &[(&str, i64)]) -> Option<Answer> {
    let v: Vec<(&str, Input)> = set.iter().map(|(k, x)| (*k, Input::Int(*x))).collect();
    s.with_input(&v)
}

fn truthy(a: &Answer, sym: &str) -> Option<String> {
    match &a.values.iter().find(|x| x.0 == sym)?.1 {
        Value::Bool(b) => Some(if *b { "yes".into() } else { "no".into() }),
        Value::String(t) => Some(t.clone()),
        Value::Null => None,
        v => Some(show(v)),
    }
}

/// Edges of a graph input.
fn edges_of(s: &Solved, sym: &str) -> Option<Vec<(String, String, Option<f64>)>> {
    match s.val(sym)? {
        Val::Edges(e) if !e.is_empty() && e.len() <= 60 => Some(e),
        _ => None,
    }
}

fn nodes_of(e: &[(String, String, Option<f64>)]) -> Vec<String> {
    let mut n: Vec<String> = vec![];
    for (u, v, _) in e {
        for x in [u, v] {
            if !n.contains(x) {
                n.push(x.clone());
            }
        }
    }
    n
}

/// Nodes on an ellipse inside the canvas (pixel positions).
fn circle_layout(n: usize, cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<P> {
    (0..n).map(|i| {
        let a = -std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * i as f64 / n.max(1) as f64;
        (cx + rx * a.cos(), cy + ry * a.sin())
    }).collect()
}

/// A graph drawn at pixel positions: edges (index pairs; `hot` ones thick in the accent colour, arrowheads when
/// directed), each node in its class, a note under each node.
#[allow(clippy::too_many_arguments)]
fn draw_graph(c: &mut Canvas, names: &[String], xy: &[P], edges: &[(usize, usize)], hot: &[bool], directed: bool,
              class: &[&str], notes: &[String]) {
    let r = if names.len() > 16 { 10.0 } else { 14.0 };
    for pass in [false, true] {
        for (i, (a, b)) in edges.iter().enumerate() {
            let h = hot.get(i).copied().unwrap_or(false);
            if h != pass {
                continue;
            }
            let (p, q) = (xy[*a], xy[*b]);
            let (dx, dy) = (q.0 - p.0, q.1 - p.1);
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            let (ux, uy) = (dx / d, dy / d);
            let p2 = (p.0 + ux * r, p.1 + uy * r);
            let q2 = (q.0 - ux * (r + 1.0), q.1 - uy * (r + 1.0));
            if directed && h {
                c.raw(&format!("<path class=\"axp-c\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p2.0, p2.1, q2.0 - ux * 8.0,
                               q2.1 - uy * 8.0));
                let (bx, by) = (q2.0 - ux * 9.0, q2.1 - uy * 9.0);
                c.raw(&format!("<path class=\"axp-d\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", q2.0, q2.1,
                               bx - uy * 5.0, by + ux * 5.0, bx + uy * 5.0, by - ux * 5.0));
            } else if directed {
                c.arrow(p2, q2);
            } else if h {
                c.raw(&format!("<path class=\"axp-c\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p2.0, p2.1, q2.0, q2.1));
            } else {
                c.line(p2, q2);
            }
        }
    }
    // the centre of the drawing: notes go on the far side of each node from it, off the edges
    let (mx, my) = (xy.iter().map(|p| p.0).sum::<f64>() / xy.len().max(1) as f64,
                    xy.iter().map(|p| p.1).sum::<f64>() / xy.len().max(1) as f64);
    for (i, n) in names.iter().enumerate() {
        let (x, y) = xy[i];
        // a light disc ringed in the node's colour, its name readable on it
        let ring = match class.get(i).copied().unwrap_or("axp-d4") {
            "axp-d" => "axp-c",
            "axp-d2" => "axp-c2",
            "axp-d3" => "axp-c3",
            _ => "axp-k",
        };
        c.raw(&format!("<circle class=\"axp-ka\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r}\"/>\
                        <circle class=\"{ring}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r}\"/>"));
        let t: String = n.chars().take(4).collect();
        c.raw(&format!("<text class=\"axp-bt\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>", y + 4.5,
                       esc(&t)));
        if let Some(nt) = notes.get(i) {
            if !nt.is_empty() {
                let (dx, dy) = (x - mx, y - my);
                let d = (dx * dx + dy * dy).sqrt();
                if d < 1.0 {
                    c.value(x, y + r + 15.0, "middle", nt);
                } else if dx.abs() >= dy.abs() * 0.6 {
                    let side = if dx > 0.0 { "start" } else { "end" };
                    c.value(x + dx.signum() * (r + 6.0), y + 4.5, side, nt);
                } else if dy > 0.0 {
                    c.value(x, y + r + 15.0, "middle", nt);
                } else {
                    c.value(x, y - r - 7.0, "middle", nt);
                }
            }
        }
    }
}

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

// ------------------------------------------------------------------------------------------------ ciphers
fn letters(s: &Solved) -> Option<Figure> {
    let msg: Vec<char> = s.txt("message")?.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_ascii_uppercase())
        .collect();
    let res: Vec<char> = s.out_text("result")?.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if msg.is_empty() || msg.len() != res.len() {
        return None;
    }
    const PER: usize = 12;
    const ROWS: usize = 4;
    let shown = msg.len().min(PER * ROWS);
    let rows = shown.div_ceil(PER);
    let h = 50.0 + rows as f64 * 104.0;
    let mut c = Canvas::new(h);
    let mode = s.txt("mode").unwrap_or_default();
    let x0 = 92.0;
    let cw = 35.0;
    for r in 0..rows {
        let y = 34.0 + r as f64 * 104.0;
        c.name(x0 - 12.0, y + 24.0, "end", "message");
        c.name(x0 - 12.0, y + 86.0, "end", "result");
        for j in 0..PER {
            let i = r * PER + j;
            if i >= shown {
                break;
            }
            let x = x0 + j as f64 * cw;
            c.raw(&format!("<rect class=\"axp-ka\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"29\" height=\"32\"/>\
                            <rect class=\"axp-b\" x=\"{x:.1}\" y=\"{:.1}\" width=\"29\" height=\"32\"/>", y + 62.0));
            c.bold(x + 14.5, y + 21.0, "middle", &msg[i].to_string());
            c.bold(x + 14.5, y + 83.0, "middle", &res[i].to_string());
            c.arrow((x + 14.5, y + 35.0), (x + 14.5, y + 59.0));
        }
    }
    let more = if shown < msg.len() { format!(" (the first {shown} of {} letters shown)", msg.len()) } else {
        String::new()
    };
    let res_s: String = res.iter().collect();
    let msg_s: String = msg.iter().collect();
    c.bold(16.0, 20.0, "start", &format!("{} → {}", clip(&msg_s, 26), clip(&res_s, 26)));
    Some(s.figure("letters", "Letter-by-letter map of the cipher",
                  &format!("The stated message's letters (top row, {mode}) and the text returned: '{res_s}', letter by \
                            letter{more}; each arrow joins a letter to its image."), c.body(), h))
}

fn clip(t: &str, n: usize) -> String {
    let c: Vec<char> = t.chars().collect();
    if c.len() <= n { t.to_string() } else { format!("{}...", c[..n - 3].iter().collect::<String>()) }
}

// ------------------------------------------------------------------------------------------------ graphs
fn bfs_tree(s: &Solved) -> Option<Figure> {
    let e = edges_of(s, "edges")?;
    let names = nodes_of(&e);
    if names.len() > 24 {
        return None;
    }
    let directed = matches!(s.val("directed"), Some(Val::Bool(true))) || s.val("directed").map(|v| v.num() == 1.0)
        .unwrap_or(false);
    let order: Vec<String> = s.outj("order")?.as_array()?.iter().map(name_of).collect();
    let dist = s.outj("distances")?.as_object()?.clone();
    let d_of = |n: &str| dist.get(n).and_then(|v| v.as_f64());
    let idx = |n: &str| names.iter().position(|x| x == n);
    // each reached vertex's parent: the first vertex visited before it, adjacent to it, one edge nearer the source
    let adj = |a: &str, b: &str| e.iter().any(|(u, v, _)| (u == a && v == b) || (!directed && u == b && v == a));
    let mut tree: Vec<(String, String)> = vec![];
    for (k, v) in order.iter().enumerate().skip(1) {
        let dv = d_of(v)?;
        let parent = order[..k].iter().find(|u| adj(u, v) && d_of(u) == Some(dv - 1.0))?;
        tree.push((parent.clone(), v.clone()));
    }
    let edges: Vec<(usize, usize)> = e.iter().filter_map(|(u, v, _)| Some((idx(u)?, idx(v)?))).collect();
    let hot: Vec<bool> = e.iter().map(|(u, v, _)| tree.iter().any(|(a, b)| (a == u && b == v) || (!directed && a == v && b == u)))
        .collect();
    // tree edges drawn from parent to child
    let edges: Vec<(usize, usize)> = edges.iter().zip(e.iter()).map(|(&(i, j), (u, v, _))| {
        if tree.iter().any(|(a, b)| a == v && b == u) { (j, i) } else { (i, j) }
    }).collect();
    let xy = circle_layout(names.len(), 260.0, 178.0, 190.0, 118.0);
    let src = order.first()?.clone();
    let class: Vec<&str> = names.iter().map(|n| if *n == src { "axp-d" } else if d_of(n).is_some() { "axp-d2" } else {
        "axp-d4"
    }).collect();
    let notes: Vec<String> = names.iter().map(|n| d_of(n).map(|d| format!("d = {}", num(d))).unwrap_or_default()).collect();
    let mut c = Canvas::new(330.0);
    c.bold(16.0, 20.0, "start", &format!("visiting order: {}", clip(&order.join(", "), 50)));
    draw_graph(&mut c, &names, &xy, &edges, &hot, directed, &class, &notes);
    c.name(504.0, 322.0, "end", "thick: the search tree");
    Some(s.figure("graph-2", "Breadth-first search order: the network and its search tree",
                  &format!("The {} edges as given with the breadth-first search tree in the accent colour: each reached \
                            vertex joined to the vertex it was found from, one edge nearer the source. The source {src} is \
                            filled; under each vertex its distance from the source as returned. Visiting order returned: \
                            {}; reached = {}.", e.len(), order.join(", "), order.len()), c.body(), 330.0))
}

fn colouring(s: &Solved) -> Option<Figure> {
    let e = edges_of(s, "edges")?;
    let names = nodes_of(&e);
    let col = s.outj("colouring")?.as_object()?.clone();
    let chi = s.out("chromatic_number")?;
    let colour: Vec<i64> = names.iter().map(|n| col.get(n).and_then(|v| v.as_i64())).collect::<Option<Vec<_>>>()?;
    const CL: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
    let class: Vec<&str> = colour.iter().map(|k| CL[((k - 1).max(0) as usize) % 4]).collect();
    let notes: Vec<String> = colour.iter().map(|k| format!("colour {k}")).collect();
    let edges: Vec<(usize, usize)> = e.iter().filter_map(|(u, v, _)| {
        Some((names.iter().position(|x| x == u)?, names.iter().position(|x| x == v)?))
    }).collect();
    let xy = circle_layout(names.len(), 260.0, 178.0, 190.0, 118.0);
    let mut c = Canvas::new(330.0);
    c.bold(16.0, 20.0, "start", &format!("chromatic number = {}", num(chi)));
    draw_graph(&mut c, &names, &xy, &edges, &[], false, &class, &notes);
    Some(s.figure("graph-2", "Chromatic number: the network coloured",
                  &format!("The {} edges as given, each vertex filled with the colour of the optimal colouring returned \
                            (colours 1 to {}; no edge joins two vertices of one colour): {}.", e.len(), num(chi),
                           names.iter().zip(&colour).map(|(n, k)| format!("{n} = {k}")).collect::<Vec<_>>().join(", ")),
                  c.body(), 330.0))
}

fn tree_edges(s: &Solved) -> Option<Vec<(i64, i64)>> {
    let a = s.outj("edges")?.as_array()?;
    let mut out = vec![];
    for x in a {
        let p = x.as_array()?;
        if p.len() != 2 {
            return None;
        }
        out.push((p[0].as_i64()?, p[1].as_i64()?));
    }
    (!out.is_empty() && out.len() <= 40).then_some(out)
}

fn prufer_matrix(s: &Solved) -> Option<Figure> {
    let e = tree_edges(s)?;
    let m: Vec<Vec<f64>> = e.iter().map(|(u, v)| vec![*u as f64, *v as f64]).collect();
    let rows: Vec<String> = (1..=e.len()).map(|i| format!("edge {i}")).collect();
    let n = s.out("vertices")?;
    let body = k::heat(&m, &rows, &["u".into(), "v".into()], &format!("{} edges, {} vertices", e.len(), num(n)));
    Some(s.chart("matrix", "Tree from a Prüfer code: its edges as a matrix",
                 &format!("The edges returned for the sequence, one row per edge (u < v), each cell shaded by the vertex \
                           number: {}; vertices = {}.", e.iter().map(|(u, v)| format!("{u}-{v}")).collect::<Vec<_>>()
                          .join(", "), num(n)), &body))
}

fn prufer_tree(s: &Solved) -> Option<Figure> {
    let e = tree_edges(s)?;
    let seq: Vec<i64> = s.vec("sequence")?.iter().map(|x| *x as i64).collect();
    let n = s.out("vertices")? as i64;
    if !(2..=30).contains(&n) {
        return None;
    }
    let names: Vec<String> = (1..=n).map(|i| i.to_string()).collect();
    let nb = |v: i64| -> Vec<i64> {
        let mut o: Vec<i64> = e.iter().filter_map(|(a, b)| if *a == v { Some(*b) } else if *b == v { Some(*a) } else { None })
            .collect();
        o.sort();
        o
    };
    // layers from the vertex of largest degree (the first such), children in ascending order
    let root = (1..=n).max_by_key(|v| (nb(*v).len(), -v))?;
    let mut level = vec![-1i64; n as usize + 1];
    level[root as usize] = 0;
    let mut layers: Vec<Vec<i64>> = vec![vec![root]];
    loop {
        let mut next = vec![];
        for v in layers.last()? {
            for w in nb(*v) {
                if level[w as usize] < 0 {
                    level[w as usize] = level[*v as usize] + 1;
                    next.push(w);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        layers.push(next);
    }
    if layers.iter().map(|l| l.len()).sum::<usize>() != n as usize {
        return None;
    }
    // a tidy layout: leaves in depth-first order take the slots across, each parent centred over its children
    let children = |v: i64| -> Vec<i64> { nb(v).into_iter().filter(|w| level[*w as usize] == level[v as usize] + 1).collect() };
    let mut slot = vec![0.0f64; n as usize + 1];
    let mut next_leaf = 0.0;
    fn place(v: i64, children: &dyn Fn(i64) -> Vec<i64>, slot: &mut Vec<f64>, next_leaf: &mut f64) {
        let ch = children(v);
        if ch.is_empty() {
            slot[v as usize] = *next_leaf;
            *next_leaf += 1.0;
        } else {
            for w in &ch {
                place(*w, children, slot, next_leaf);
            }
            slot[v as usize] = ch.iter().map(|w| slot[*w as usize]).sum::<f64>() / ch.len() as f64;
        }
    }
    place(root, &children, &mut slot, &mut next_leaf);
    let leaves = next_leaf.max(1.0);
    let mut xy = vec![(0.0, 0.0); n as usize];
    let dy = (250.0 / layers.len().max(2) as f64).min(80.0);
    let w = 470.0 / leaves;
    for v in 1..=n {
        xy[v as usize - 1] = (25.0 + w * (slot[v as usize] + 0.5), 62.0 + dy * level[v as usize] as f64);
    }
    let edges: Vec<(usize, usize)> = e.iter().map(|(a, b)| ((a - 1) as usize, (b - 1) as usize)).collect();
    let class: Vec<&str> = (1..=n).map(|v| if seq.contains(&v) { "axp-d" } else { "axp-d4" }).collect();
    let h = 62.0 + dy * (layers.len() - 1) as f64 + 50.0;
    let mut c = Canvas::new(h.max(160.0));
    c.bold(16.0, 20.0, "start", &format!("the tree decoded from ({})", clip(&seq.iter().map(|x| x.to_string())
        .collect::<Vec<_>>().join(", "), 40)));
    c.raw("<circle class=\"axp-c\" cx=\"22\" cy=\"35\" r=\"6\"/>");
    c.name(34.0, 40.0, "start", "named in the sequence");
    c.raw("<circle class=\"axp-k\" cx=\"210\" cy=\"35\" r=\"6\"/>");
    c.name(222.0, 40.0, "start", "leaves");
    draw_graph(&mut c, &names, &xy, &edges, &[], false, &class, &[]);
    Some(s.figure("tree", "A labelled tree decoded from its Prüfer sequence",
                  &format!("The tree returned for the sequence ({}): {} vertices, edges {}. Filled: the vertices the \
                            sequence names (each appears one time fewer than its degree); grey: the leaves.",
                           seq.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "), n,
                           e.iter().map(|(u, v)| format!("{u}-{v}")).collect::<Vec<_>>().join(", ")), c.body(), h.max(160.0)))
}

// ------------------------------------------------------------------------------------------------ fields
#[derive(Clone, Copy)]
enum Ax {
    /// whole numbers from 1 to at least this, or twice the value
    From1(i64),
    /// whole numbers from lo to at least hi, or twice the value
    Range(i64, i64),
    /// whole numbers around the value, at least this far either side
    Around(i64),
    /// whole numbers 1 to twice the value, logarithmic steps
    Log,
    /// odd numbers 1 to twice the value, logarithmic steps
    LogOdd,
}

impl Ax {
    fn from1(n: i64) -> Ax {
        Ax::From1(n)
    }
    /// The grid points (at most 20) and whether the axis is logarithmic.
    fn points(&self, x0: i64) -> (Vec<i64>, bool) {
        let even = |lo: i64, hi: i64| -> Vec<i64> {
            let n = (hi - lo + 1).clamp(1, 20);
            let mut v: Vec<i64> = (0..n).map(|i| lo + ((hi - lo) as f64 * i as f64 / (n - 1).max(1) as f64).round() as i64)
                .collect();
            v.dedup();
            v
        };
        let logs = |hi: i64, odd: bool| -> Vec<i64> {
            let mut v: Vec<i64> = k::span(1.0, hi.max(2) as f64, 18, true).iter().map(|x| {
                let r = x.round() as i64;
                if odd && r % 2 == 0 { r + 1 } else { r }
            }).collect();
            v.sort();
            v.dedup();
            v
        };
        match *self {
            Ax::From1(m) => (even(1, m.max(2 * x0)), false),
            Ax::Range(lo, m) => (even(lo, m.max(2 * x0)), false),
            Ax::Around(d) => {
                let d = d.max(x0.abs());
                (even(x0 - d, x0 + d), false)
            }
            Ax::Log => (logs(2 * x0.max(1), false), true),
            Ax::LogOdd => (logs(2 * x0.max(1), true), true),
        }
    }
}

fn in_meaning(s: &Solved, sym: &str) -> String {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.meaning.clone())
        .unwrap_or_default()
}

#[allow(clippy::too_many_arguments)]
fn field2(s: &Solved, xs_: &str, xr: Ax, ys_: &str, yr: Ax, out: &str, title: &str) -> Option<Figure> {
    let x0 = s.val(xs_)?.int();
    let y0 = s.val(ys_)?.int();
    let z0 = s.out(out)?;
    let (xi, xlog) = xr.points(x0);
    let (yi, ylog) = yr.points(y0);
    if xi.len() < 2 || yi.len() < 2 {
        return None;
    }
    let mut vals: Vec<Vec<Option<f64>>> = vec![];
    let mut n = 0;
    for x in &xi {
        let mut col = vec![];
        for y in &yi {
            let v = with_ints(s, &[(xs_, *x), (ys_, *y)]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    // values over many orders of magnitude are shaded by their logarithm, else every cell but one is light
    let logshade = lo >= 0.0 && hi > 1000.0 * lo.max(1.0);
    let shade: Vec<Vec<Option<f64>>> = if logshade {
        vals.iter().map(|c| c.iter().map(|v| v.map(|x| (1.0 + x).log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let xs: Vec<f64> = xi.iter().map(|x| *x as f64).collect();
    let ys: Vec<f64> = yi.iter().map(|x| *x as f64).collect();
    let header = clip(&if logshade { format!("{out} = {} (shade: log10(1+v))", num(z0)) } else {
        format!("{out} = {}", num(z0))
    }, 46);
    let lab = |sym: &str| {
        let m = in_meaning(s, sym);
        if m.is_empty() { sym.to_string() } else { format!("{sym}: {m}") }
    };
    let body = k::field(&xs, &ys, &shade, (x0 as f64, y0 as f64), &lab(xs_), &lab(ys_), &header, (xlog, ylog));
    Some(s.chart("field", title,
                 &format!("{} ({out}) over {xs_} from {} to {} and {ys_} from {} to {}{}: each of the {} cells is one \
                           computation of the model, shaded by its value{} (darker is larger; a blank cell: inputs the \
                           model refuses); the values run from {} to {}. Marked: the inputs given, {out} = {}.",
                          meaning(s, out), xi[0], xi[xi.len() - 1], yi[0], yi[yi.len() - 1],
                          if xlog || ylog { " (logarithmic axes)" } else { "" }, n,
                          if logshade { " on a logarithmic scale" } else { "" }, num(lo), num(hi), num(z0)), &body))
}

// ------------------------------------------------------------------------------------------------ verdicts along n
fn verdict(s: &Solved, xsym: &str, vout: &str, lout: &str, lo: i64, min_hi: i64) -> Option<Figure> {
    let x0 = s.val(xsym)?.int();
    let v0 = truthy(s.a, vout)?;
    let hi = min_hi.max(2 * x0);
    let log = hi - lo > 120;
    let xs: Vec<i64> = if log {
        let mut v: Vec<i64> = k::span(lo.max(1) as f64, hi as f64, 90, true).iter().map(|x| x.round() as i64).collect();
        v.push(x0);
        v.sort();
        v.dedup();
        v
    } else {
        (lo..=hi).collect()
    };
    let mut pts: Vec<(f64, String, Option<f64>)> = vec![];
    for x in &xs {
        if let Some(a) = with_ints(s, &[(xsym, *x)]) {
            if let Some(v) = truthy(&a, vout) {
                pts.push((*x as f64, v, k::num_of(&a, lout)));
            }
        }
    }
    if pts.len() < 3 {
        return None;
    }
    let mut kinds: Vec<String> = vec![];
    for p in &pts {
        if !kinds.contains(&p.1) {
            kinds.push(p.1.clone());
        }
    }
    let ys: Vec<f64> = pts.iter().filter_map(|p| p.2).collect();
    let (ylo, yhi) = if ys.is_empty() { (0.0, 1.0) } else {
        ys.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
    };
    let (ylo, yhi) = (ylo.min(0.0), if yhi > ylo.min(0.0) { yhi } else { ylo.min(0.0) + 1.0 });
    let pad = (yhi - ylo) * 0.08;
    let mut p = Plot::new((pts[0].0, pts[pts.len() - 1].0), (ylo - pad, yhi + pad), &format!("{xsym}: {}", in_meaning(s, xsym)),
                          &format!("{lout}"), &format!("{xsym} = {x0} → {v0}"), (log, false), false);
    const BANDS: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
    let mid = |a: f64, b: f64| if log { (a * b).sqrt() } else { (a + b) / 2.0 };
    for (i, q) in pts.iter().enumerate() {
        let l = if i == 0 { q.0 } else { mid(pts[i - 1].0, q.0) };
        let r = if i + 1 == pts.len() { q.0 } else { mid(q.0, pts[i + 1].0) };
        let cl = BANDS[kinds.iter().position(|x| *x == q.1).unwrap_or(0) % 4];
        p.rect((l, ylo - pad), (r, yhi + pad), cl, "");
    }
    let line: Vec<P> = pts.iter().map(|q| (q.0, q.2.unwrap_or(f64::NAN))).collect();
    // whole-number values at whole numbers: a dot each, joined by a thin line
    p.path(&line, "axp-c4");
    for q in &line {
        p.dot(*q, if line.len() > 70 { 2.2 } else { 3.0 }, "axp-d");
    }
    if let Some(y) = s.out(lout) {
        p.mark((x0 as f64, y), &format!("{lout} = {}", num(y)));
    }
    let leg: Vec<(String, &str)> = kinds.iter().enumerate().take(4).map(|(i, kd)| (kd.clone(), BANDS[i % 4]))
        .collect();
    p.legend(&leg);
    // where the verdict changes
    let mut changes = vec![];
    for w in pts.windows(2) {
        if w[0].1 != w[1].1 && changes.len() < 6 {
            changes.push(format!("{} to {}", w[0].0, w[1].0));
        }
    }
    Some(s.chart("decision", &format!("{}: where the verdict changes ({vout})", cap_name(s)),
                 &format!("{} ({vout}) along {xsym} from {} to {}{}: {} computations of the model, each band coloured by \
                           the verdict returned ({}); the line is {lout} ({}). The verdict changes between {}{}. Marked: \
                           the inputs given, {xsym} = {x0} → {v0}.", meaning(s, vout), pts[0].0, pts[pts.len() - 1].0,
                          if log { " (logarithmic axis)" } else { "" }, pts.len(), kinds.join(", "), meaning(s, lout),
                          if changes.is_empty() { "no neighbouring points".to_string() } else { changes.join("; ") },
                          if changes.len() >= 6 { " and further on" } else { "" }), &p.body()))
}

// ------------------------------------------------------------------------------------------------ inputs and results
fn io(s: &Solved) -> Option<Figure> {
    let ins: Vec<String> = s.given.iter().map(|(n, v)| format!("{n} = {}", input_text(v))).collect();
    let outs: Vec<String> = s.a.values.iter().filter(|v| !v.1.is_null()).map(|x| format!("{} = {}", x.0, show(&x.1)))
        .collect();
    if ins.is_empty() || outs.is_empty() {
        return None;
    }
    let (ni, no) = (ins.len().min(6), outs.len().min(8));
    let h = 40.0 + 22.0 * ni as f64 + 112.0 + 22.0 * no as f64 + 10.0;
    let mut c = Canvas::new(h);
    c.name(260.0, 22.0, "middle", "stated");
    let mut y = 46.0;
    for t in ins.iter().take(ni) {
        c.value(260.0, y, "middle", &clip(t, 60));
        y += 22.0;
    }
    c.arrow((260.0, y - 8.0), (260.0, y + 20.0));
    c.block(110.0, y + 22.0, 300.0, 40.0, &clip(&cap_name(s), 36));
    c.arrow((260.0, y + 64.0), (260.0, y + 92.0));
    c.name(370.0, y + 82.0, "start", "returned");
    y += 116.0;
    for t in outs.iter().take(no) {
        c.raw(&format!("<text class=\"axp-mt\" x=\"260\" y=\"{y:.1}\" text-anchor=\"middle\">{}</text>", esc(&clip(t, 50))));
        y += 22.0;
    }
    Some(s.figure("io", &format!("{}: inputs and results", cap_name(s)),
                  &format!("The inputs as stated ({}) and the results the model returned for them: {}.", ins.join("; "),
                           outs.join("; ")), c.body(), h))
}

// ------------------------------------------------------------------------------------------------ counts and tables
fn partition_stems(s: &Solved) -> Option<Figure> {
    let n0 = s.val("n")?.int();
    let p0 = s.out("partitions")?;
    let top = n0.clamp(20, 60);
    let mut pa = vec![];
    let mut qa = vec![];
    for n in 1..=top {
        if let Some(y) = with_ints(s, &[("n", n)]).and_then(|a| k::num_of(&a, "partitions")) {
            pa.push((n as f64, y));
        }
        if let Some(y) = s.solve_other("partitions_distinct", &[("n", Input::Int(n))]).and_then(|a| k::num_of(&a, "partitions")) {
            qa.push((n as f64, y));
        }
    }
    if pa.len() < 3 {
        return None;
    }
    let ymax = pa.iter().chain(qa.iter()).fold(1.0f64, |a, p| a.max(p.1));
    let mut p = Plot::new((0.5, top as f64 + 0.5), (1.0, ymax), "n", "number of partitions (logarithmic)",
                          &format!("p({n0}) = {}", num(p0)), (false, true), true);
    for (pts, dx, cl) in [(&pa, -0.17, "axp-d"), (&qa, 0.17, "axp-d2")] {
        for q in pts.iter() {
            p.seg((q.0 + dx, 1.0), (q.0 + dx, q.1), "axp-c4");
            p.dot((q.0 + dx, q.1), 3.4, cl);
        }
    }
    if (1..=top).contains(&n0) {
        p.mark((n0 as f64 - 0.17, p0), &format!("p({n0}) = {}", num(p0)));
    }
    p.legend(&[("p(n): all partitions".into(), "axp-b"), ("q(n): distinct parts".into(), "axp-b2")]);
    let q_of = |n: f64| qa.iter().find(|q| q.0 == n).map(|q| num(q.1)).unwrap_or_else(|| "?".into());
    Some(s.chart("stems", "Integer partitions, all and into distinct parts",
                 &format!("The number of ways to write n as a sum of positive integers, p(n), and as a sum of distinct \
                           ones, q(n), for n = 1 to {top}, each computed by the model (logarithmic scale): p({top}) = {}, \
                           q({top}) = {}. Marked: the n given, p({n0}) = {}.", num(pa[pa.len() - 1].1), q_of(top as f64),
                          num(p0)), &p.body()))
}

fn legendre_table(s: &Solved) -> Option<Figure> {
    let a0 = s.val("a")?.int();
    let p0 = s.val("p")?.int();
    let sym0 = s.out("symbol")?;
    let mut primes: Vec<i64> = vec![3, 5, 7, 11, 13];
    if !primes.contains(&p0) {
        primes.push(p0);
        primes.sort();
        if primes.len() > 6 {
            let i = primes.iter().position(|x| *x == p0).unwrap_or(0);
            primes.remove(if i == 0 { 1 } else { 0 });
        }
    }
    let start = if a0 > 12 { a0 - 11 } else if a0 < 1 { a0 } else { 1 };
    let cols: Vec<i64> = (start..start + 12).collect();
    let mut rows = vec![];
    let mut mark = None;
    for (i, p) in primes.iter().enumerate() {
        let mut r = vec![p.to_string()];
        for a in &cols {
            let v = with_ints(s, &[("a", *a), ("p", *p)]).and_then(|x| k::num_of(&x, "symbol"));
            r.push(v.map(|v| format!("{}{}", if v > 0.0 { "+" } else { "" }, num(v))).unwrap_or_else(|| "-".into()));
        }
        if *p == p0 {
            mark = Some(i);
        }
        rows.push(r);
    }
    let mut head = vec!["p \\ a".to_string()];
    head.extend(cols.iter().map(|a| a.to_string()));
    let (body, h) = k::table(&head, &rows, mark, &format!("({a0} / {p0}) = {}", num(sym0)));
    Some(s.figure("table", "Legendre symbols",
                  &format!("Legendre symbol (a / p) computed by the model for a = {} to {} (columns) and the odd primes p \
                            = {} (rows): +1 a quadratic residue, -1 a non-residue, 0 when p divides a; a dash: inputs the \
                            model refuses. Outlined: the row of the p given; ({a0} / {p0}) = {}.", cols[0],
                           cols[cols.len() - 1], primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
                           num(sym0)), &body, h))
}

/// A formula's truth table as the model returns it: (variables, values per row).
fn table_of(s: &Solved, formula: &str) -> Option<(Vec<String>, Vec<f64>)> {
    let a = s.solve_other("truth_table", &[("formula", Input::Text(formula.to_string()))])?;
    let vars: Vec<String> = a.values.iter().find(|x| x.0 == "variables")?.1.as_array()?.iter().map(name_of).collect();
    let vals: Vec<f64> = match &a.values.iter().find(|x| x.0 == "values")?.1 {
        Value::String(t) => t.chars().map(|c| if c == '1' { 1.0 } else { 0.0 }).collect(),
        _ => return None,
    };
    (vals.len() == 1 << vars.len()).then_some((vars, vals))
}

fn minimal_truth(s: &Solved) -> Option<Figure> {
    let f = s.txt("formula")?;
    let e = s.out_text("expression")?;
    let (vars, fv) = table_of(s, &f)?;
    if vars.is_empty() || vars.len() > 5 {
        return None;
    }
    let ev: Vec<f64> = if e == "0" || e == "1" || e.eq_ignore_ascii_case("true") || e.eq_ignore_ascii_case("false") {
        let c = if e == "1" || e.eq_ignore_ascii_case("true") { 1.0 } else { 0.0 };
        vec![c; fv.len()]
    } else {
        let (ev_vars, ev_vals) = table_of(s, &e)?;
        if !ev_vars.iter().all(|v| vars.contains(v)) {
            return None;
        }
        // each row of the formula's variables read on the minimal form's own variables
        let n = vars.len();
        (0..fv.len()).map(|r| {
            let mut idx = 0usize;
            for v in &ev_vars {
                let j = vars.iter().position(|x| x == v).unwrap_or(0);
                idx = idx * 2 + ((r >> (n - 1 - j)) & 1);
            }
            ev_vals[idx]
        }).collect()
    };
    let cols = vec![("formula".to_string(), fv.iter().map(|v| Some(*v)).collect()),
                    ("minimal".to_string(), ev.iter().map(|v| Some(*v)).collect())];
    let body = k::truth(&vars, &cols, None, &format!("minimal: {}", clip(&e, 44)));
    let agree = fv == ev;
    Some(s.chart("truth", "Truth table of the formula and its minimal form",
                 &format!("Truth table of {f} and of the minimal form returned, {e}, over every assignment of {} (1 = \
                           true, 0 = false; rows in binary order); each column is the model's own truth table of that \
                           formula. The two columns {}.", vars.join(", "),
                          if agree { "agree on every row" } else { "differ on some rows" }), &body))
}

fn evaluate_truth(s: &Solved) -> Option<Figure> {
    let f = s.txt("formula")?;
    let asg = s.txt("assignment").unwrap_or_default();
    let v0 = s.out_text("value")?;
    let (vars, fv) = table_of(s, &f)?;
    if vars.is_empty() || vars.len() > 5 {
        return None;
    }
    // the row of the assignment given: each variable's stated truth value, read as written
    let mut row = 0usize;
    let mut found = 0;
    for part in asg.split([',', ';']) {
        let mut kv = part.splitn(2, ['=', ':']);
        let (Some(name), Some(val)) = (kv.next(), kv.next()) else { continue };
        let (name, val) = (name.trim(), val.trim().to_ascii_lowercase());
        let bit = match val.as_str() {
            "true" | "t" | "1" => 1,
            "false" | "f" | "0" => 0,
            _ => continue,
        };
        if let Some(j) = vars.iter().position(|v| v == name) {
            row |= bit << (vars.len() - 1 - j);
            found += 1;
        }
    }
    let mark = (found == vars.len()).then_some(row);
    // the marked row must agree with the value the model returned, or no row is marked
    let mark = mark.filter(|r| (fv[*r] == 1.0) == (v0 == "yes"));
    let cols = vec![(clip(&f, 24), fv.iter().map(|v| Some(*v)).collect())];
    let body = k::truth(&vars, &cols, mark, &format!("value = {}", if v0 == "yes" { "true" } else { "false" }));
    Some(s.chart("truth", "Truth table of the formula with the assignment given",
                 &format!("Truth table of {f} over every assignment of {} (1 = true, 0 = false; rows in binary order), \
                           the model's own truth table of the formula{}; the value returned for the assignment given \
                           ({asg}) is {}.", vars.join(", "),
                          if mark.is_some() { "; outlined: the row of the assignment given" } else { "" },
                          if v0 == "yes" { "true" } else { "false" }), &body))
}

fn degree_bars(s: &Solved) -> Option<Figure> {
    let d = s.outj("degrees")?.as_object()?.clone();
    if d.is_empty() || d.len() > 40 {
        return None;
    }
    let labels: Vec<String> = d.keys().cloned().collect();
    let vals: Vec<f64> = d.values().map(|v| v.as_f64()).collect::<Option<Vec<_>>>()?;
    let mx = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let hl: Vec<usize> = vals.iter().enumerate().filter(|x| *x.1 == mx).map(|x| x.0).collect();
    let body = k::bars(&labels, &vals, &hl, "vertex", "degree", &format!("maximum degree {}", num(mx)));
    Some(s.chart("bars", "Vertex degrees: degree of each vertex",
                 &format!("The degree of each vertex returned by the model, one bar per vertex: {}; highlighted: the \
                           vertices of maximum degree ({}).", labels.iter().zip(&vals).map(|(l, v)| format!("{l} = {}", num(*v)))
                          .collect::<Vec<_>>().join(", "), num(mx)), &body))
}

fn roots_plane(s: &Solved) -> Option<Figure> {
    let r = s.out_list("roots")?;
    let (a, p) = (s.val("a")?.int(), s.val("p")?.int());
    let mine: Vec<(String, (f64, f64))> = r.iter().enumerate().map(|(i, x)| (format!("root {}", i + 1), (*x, 0.0))).collect();
    let body = k::complex(&[], &mine, &format!("square roots of {a} mod {p}: {}",
                                               r.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")));
    Some(s.chart("complex", "Square roots modulo a prime in the complex plane",
                 &format!("The square roots of {a} modulo {p} returned by the model ({}), drawn in the complex plane on \
                           the real axis.", r.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")), &body))
}

fn stirling_heat(s: &Solved) -> Option<Figure> {
    let n0 = s.val("n")?.int();
    let k0 = s.val("k")?.int();
    let v0 = s.out("stirling2")?;
    let r_hi = n0.max(7);
    let r_lo = (r_hi - 7).max(1);
    let c_hi = k0.max(r_hi.min(8));
    let c_lo = (c_hi - 7).max(1);
    let mut m = vec![];
    for n in r_lo..=r_hi {
        let mut row = vec![];
        for kk in c_lo..=c_hi {
            row.push(with_ints(s, &[("n", n), ("k", kk)]).and_then(|a| k::num_of(&a, "stirling2")).unwrap_or(f64::NAN));
        }
        m.push(row);
    }
    if m.iter().flatten().filter(|x| x.is_finite()).count() < 4 {
        return None;
    }
    // the table as shaded cells (shade by the logarithm: the numbers grow very fast), each number sized to its cell
    let (nr, nc) = (m.len(), m[0].len());
    let cell = (400.0 / nc as f64).min(250.0 / nr as f64).min(56.0);
    let (x0, y0) = (80.0 + (420.0 - cell * nc as f64) / 2.0, 52.0);
    let lmax = m.iter().flatten().filter(|x| x.is_finite()).fold(0.0f64, |a, x| a.max((1.0 + x.abs()).log10()));
    let mut c = Canvas::new(y0 + cell * nr as f64 + 30.0);
    c.raw(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>", esc(&format!("S({n0}, {k0}) = {}", num(v0)))));
    for (i, row) in m.iter().enumerate() {
        let n = r_lo + i as i64;
        c.name(x0 - 8.0, y0 + cell * (i as f64 + 0.5) + 4.0, "end", &format!("n = {n}"));
        for (j, v) in row.iter().enumerate() {
            let kk = c_lo + j as i64;
            let (x, y) = (x0 + cell * j as f64, y0 + cell * i as f64);
            if !v.is_finite() {
                continue;
            }
            let op = if lmax > 0.0 { 0.06 + 0.8 * (1.0 + v.abs()).log10() / lmax } else { 0.06 };
            c.raw(&format!("<rect class=\"axp-hp\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                            fill-opacity=\"{op:.2}\"/>", cell - 1.5, cell - 1.5));
            if n == n0 && kk == k0 {
                c.raw(&format!("<rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                               x - 1.0, y - 1.0, cell + 0.5, cell + 0.5));
            }
            let t = if v.abs() < 1e6 { format!("{}", *v as i64) } else { ax_engine::num::py_g(*v, 2) };
            let font = (cell * 1.7 / t.chars().count().max(2) as f64).clamp(7.0, 13.0);
            c.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.1}\">{}</text>",
                           x + cell / 2.0, y + cell / 2.0 + font * 0.35, esc(&t)));
        }
    }
    for j in 0..nc {
        c.name(x0 + cell * (j as f64 + 0.5), y0 + cell * nr as f64 + 16.0, "middle", &format!("k={}", c_lo + j as i64));
    }
    let h = c.h;
    Some(s.figure("heatmap", "Stirling numbers of the second kind",
                 &format!("Stirling numbers of the second kind S(n, k) for n = {r_lo} to {r_hi} (rows) and k = {c_lo} to \
                           {c_hi} (columns), each computed by the model: the ways to split n labelled objects into k \
                           non-empty groups; shaded by the logarithm of the value. Outlined: the inputs given, S({n0}, \
                           {k0}) = {}.", num(v0)), c.body(), h))
}
