# Project decisions

> 2026-10-26: `deckmaste_english_v4` and `deckmaste_lexical_source_v4` are now `sylex_english` and `sylex_vocabulary`; the census command is `cargo xtask english`.

These documents record durable project contracts for contributors. They describe
the intended design rather than a dated implementation inventory; current code
wins if an incidental implementation description drifts. Changing a decision
requires explicit review rather than an opportunistic refactor.

- [Architecture](architecture.md) — The whole system and why it is shaped so:
  English ⇄ spelling ⇄ semantics → lowering → core → engine; Oracle text is the
  authority, the semantic term the canonical meaning, core a compiled artifact.
- [Authored card surface](authored-card-surface.md) — Keep card RON terse,
  structural, and stable.
- [Minimal core and data-driven rules](minimal-core-data-driven-rules.md) —
  Compose named mechanics from primitives and declared data.
- [Macros are declarative](macros-are-declarative.md) — Plugin macros are typed
  templates, not a control-flow language.
- [Conferrals come from registries](conferrals-come-from-registries.md) —
  Registries are the authority for open named behavior.
- [Lean is the workbench](lean-is-the-workbench.md) — Active semantics modeling
  and theorem-form pins; the Idris workbench is deleted.
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md) —
  One typed template drives both rendering and parsing.
- [English grammar: lexical analysis, Readings, and how the grammar is kept small](english-lexical-analysis.md)
  — Engine contract, round-trip laws, Reading normal forms, and how words reach
  the grammar.
- [English v4](english-v4.md) — The governing record for
  `sylex_syntax`, `deckmaste_english_v4`, and
  `deckmaste_lexical_source_v4`: data-driven schema/production engine,
  frame-driven complementation, CR-catalog separation, stratified buildout, and
  two-stage cutover.
- [Semantics v3](semantics-v3.md) — The design authority for semantics: DRT
  reference resolution (the DRS, boxes, `J1`, `J2`, `D′`, `AC>R1`, Contrast
  Anchors), the Definition/Template split, the action-clause spine (`instruct`,
  `does`, `would`/`instead`, `cant`/`must`/`can`/`gatedBy`, `did`/`tally`, `by`,
  `any`, `which`), and the `.syl` data language under `plugins/`.
- [Card authoring binds no implicits](card-authoring-binds-no-implicits.md) —
  Card authoring binds no implicit parameters; core constructors take every
  field required. Superseded for the Lean workbench (2026-10-21): Templates may
  take optional named parameters after the required ones.
- [Workbench RON-shaped and label rulings](workbench-ron-shaped-and-label-rulings.md)
  — Registry label and RON-shaped syntax rulings.
- [Core](core.md) — The contract of `sylex_core` and `sylex_card`: closed
  regions with declared parameters and register reads, captures, linked
  memory, validate at load, and Card Faces.
- [Engine](engine.md) — The contract of `sylex_engine`: the activation record,
  Decisions, operands, Conferrals, state-based actions and failure.

Superseded historical decision records live in [`docs/archive/decisions/`](../archive/decisions/)
and are not active authority.
