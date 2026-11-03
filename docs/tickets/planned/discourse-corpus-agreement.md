---
needs: [semantics-rust-rewrite, lean-discourse-export]
---
# `deckmaste_discourse` agrees with Lean across the corpus

Standard constraints apply. The gate `lean-retirement` waits on.

- Per card, `deckmaste_discourse`'s bindings and the combined `deckmaste_semantics_v3` +
  `deckmaste_discourse` refusals equal `lean-discourse-export`'s `bindings.jsonl` across the whole
  `plugins_v3` corpus, except entries marked FLAG DEFECT or explicitly fixed in
  `semantics-card-facts` and the `discourse-*` tickets (where Rust fixes the defect and the
  difference is checked to be that fix), with zero `Refusal::Unsupported` remaining
  (`Refusal::Unsupported` is deleted in this ticket).
- The exported `fixtures.jsonl` snapshot passes as one data-driven Rust test over the combined
  `deckmaste_semantics_v3` + `deckmaste_discourse` checks.
- Every entry of `docs/semantics-spec-catalog.md` exists as a native Rust test (the `discourse-*`
  tickets port the entries they implement; this ticket closes the remainder).
- A difference is resolved by fixing `deckmaste_discourse`, or, where Lean is wrong, by recording
  the difference in the catalog with the rule that decides it.
