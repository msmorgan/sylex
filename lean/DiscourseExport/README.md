# Discourse export

An oracle for the Rust `discourse` crate (`semantics-rust-rewrite`), from Lean's checker.
Disposable: it is deleted with Lean (`lean-retirement`), together with the checker's trace
hooks (`Semantics/Check/Trace.lean`).

From `lean/`:

```sh
lake build -q discourseExport
lake exe discourseExport <label>             # both parts
lake exe discourseExport bindings <label>    # per-card bindings only
lake exe discourseExport fixtures <label>    # proof fixtures only
```

Output goes to the gitignored `../docs/evidence/lean-discourse/<label>/`. Any card error,
alignment failure or unmatched catalog name is listed in `summary.txt` and exits 2.
`DISCOURSE_DEBUG=<printed name>` also prints that card's positions and raw trace to stderr.
The executable is opt-in: neither the default build nor `scripts/build` (the Lean check
gate) includes it.

## `fixtures.jsonl` and `checklist.jsonl`

Every theorem of `Semantics.Proofs.*` whose statement is a closed `entry args = rhs` over
`List Refusal`, where `entry` is a checker `*.check`, `*.checkAt`, `*.checkAll` or `*.checkText`
defined under `Semantics.Check`, is one fixture line:

```json
{"theorem":"File.name","declaration":"…","entry":"Semantics.Action.check",
 "args":[["drs","<value>"],["i","<value>"]],"accepted":false,"refusals":["choiceClause"],
 "flagDefect":false}
```

`args` are the entry point's explicit arguments in the `DumpValues` constructor encoding
(`DumpValues/README.md`); a computed string (`operandKey 0`) is written as its literal. A fixture compares accepted against refused and the sorted set of
refusal kinds (constructor names), never Lean's refusal list itself. Every other theorem
(quantified, `∧`, `≠`, `let`, statements over checker internals) is one checklist line with
the reason it is not a fixture; `docs/semantics-spec-catalog.md` covers those. `theorem` is
the catalog's `File.theorem` spelling. `flagDefect` marks the names the catalog marks FLAG
DEFECT (`discourse-parameter-argument-resolution`): those in a marked entry's `from:` clause,
and all of section 5 (Binding identity), whose heading marks its entries.

## `bindings.jsonl`

One line per canon card, sorted by printed name:

```json
{"card":"…","refusals":[],"bindings":[{"mention":{"path":[…],"node":"Semantics.NounPhrase.pro"},
 "rule":"D′","antecedents":[{"path":[…],"role":"Intro(0)"}],"flagDefect":false}],
 "alignmentFailures":[]}
```

A path is the list of steps `[constructor, field, list indices…]` from the card root through its
expanded core AST (templates and Definitions applied); an `Option` is transparent. A mention is a
referring noun phrase; `rule` is how it resolved: `D′` (bare pronoun), `AC>R1` (demonstrative),
`shuffledLastKnown`, `PrincipleA` (reflexive), `relative`, `firstMention`, `laterMention`,
`delayedTrigger`, `definite` or `participial` (a description reading back an earlier mention).
An antecedent is keyed by the path of the node that introduced it and a role:

- `Intro(k)` / `Created(k)`: that node's k-th introduced referent (a created token or emblem);
  the card root's are the face's cost letters' context,
- `Outcome(sort)`, `Letter(l)`,
- `MovedSuccessor(k)`: the node's k-th zone move, with `from` the moved referent's key,
- `LoopAggregate(target|created|outcome)`: what a kind loop published.

`via` names the kind of a frame the checker built itself, when the introducing node is not in
the card. `flagDefect` marks a mention through a Definition parameter used several times (a path
through `firstMention`, or a first or later mention itself).

## How bindings are found

The checker logs, when the exporter switches its trace on, the frames of its stepping closures
(`stepOps`), the referents `DRS.prepend` mints, zone-move successors (`applyZoneMove`), loop
aggregates (`publishLoop*`) and each noun phrase's returned ids with its rule. The log is an
`IO.Ref` outside the `DRS`; tracing off, the hooks are the identity (`Trace.frame_eq`,
`Trace.emit_eq`). The exporter rebuilds each card's reduced constructor tree with every stepped
node its own allocation, evaluates it, runs `Card.check` on it, and aligns each frame to its
position by address, or, where the checker steps a rewritten copy (`NounPhrase.narrowBareTo`,
logged by `Trace.rewrite`), by the original's. A node with no fields is a boxed scalar shared by
every such node: one candidate in its nearest aligned frame places it at once; otherwise, when
that frame ends, its visits of the constructor are placed in reading order
(`SylGen.ReadOrder.fieldOrders`) on the candidates outside the nodes its other frames covered,
and the two counts must agree. Any disagreement, unknown antecedent id, mention of a node not in
the card, or shared allocation is an alignment failure.
