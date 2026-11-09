//! Dependency cycles between declarations.

/// Every dependency cycle in `edges` (node to the nodes it reaches): a set of
/// nodes each reaching the next, or one reaching itself. Each cycle is sorted.
pub fn cycles(edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    strongly_connected(edges)
        .into_iter()
        .filter(|component| component.len() > 1 || edges[component[0]].contains(&component[0]))
        .collect()
}

/// Tarjan's strongly connected components, each sorted.
fn strongly_connected(edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    struct State<'e> {
        edges: &'e [Vec<usize>],
        index: Vec<Option<usize>>,
        low: Vec<usize>,
        stack: Vec<usize>,
        on_stack: Vec<bool>,
        next: usize,
        out: Vec<Vec<usize>>,
    }
    fn visit(s: &mut State<'_>, v: usize) {
        s.index[v] = Some(s.next);
        s.low[v] = s.next;
        s.next += 1;
        s.stack.push(v);
        s.on_stack[v] = true;
        for &w in &s.edges[v] {
            match s.index[w] {
                None => {
                    visit(s, w);
                    s.low[v] = s.low[v].min(s.low[w]);
                }
                Some(index) if s.on_stack[w] => s.low[v] = s.low[v].min(index),
                Some(_) => {}
            }
        }
        if Some(s.low[v]) == s.index[v] {
            let mut component = Vec::new();
            loop {
                let w = s.stack.pop().expect("v is on the stack");
                s.on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            component.sort_unstable();
            s.out.push(component);
        }
    }
    let n = edges.len();
    let mut state = State {
        edges,
        index: vec![None; n],
        low: vec![0; n],
        stack: Vec::new(),
        on_stack: vec![false; n],
        next: 0,
        out: Vec::new(),
    };
    for v in 0..n {
        if state.index[v].is_none() {
            visit(&mut state, v);
        }
    }
    state.out
}
