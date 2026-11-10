---
needs: [syl-doc-comment-rationales]
---

# Step 6: the `.syl` iteration is complete

## Why

Step 6 of the cleanup reshapes the `.syl` corpus under `cargo xtask dsl check` before step 7
builds the Rust AST from it. This ticket closes the staged step-6 graph: `syl-single-face-card-call`
starts it, then the spine (`cleanup-step6-spine`), then the surface (`cleanup-step6-surface`), then
three parallel middle stages (`cleanup-step6-constructs-and-cards`,
`cleanup-step6-rules-and-keywords`, `cleanup-step6-deeds`), then the close:
`syl-dead-weight-sweep`, followed by `syl-doc-comment-rationales`, the ticket this one needs. A new step-6 ticket joins one stage: it needs the previous stage's
gate, and its stage's sub-epic needs it unless another ticket in that stage does.

## Done when

- Every step-6 ticket is in `done/`.
- A corpus-wide checkpoint pass over `plugins/` and `crates/xtask/src/dsl_check*` finds every
  Lean hack the stage checkpoints left: a construct, special case, wrapper, workaround or modeling
  shortcut there because Lean or its generator needed it, and a doc comment written in Lean
  syntax or naming a generated wrapper. Each one found is fixed in the checkpoint's own landing;
  anything that truly needs step 7 or later is ticketed with that reason. The landing lists them.
- No "Step 6:" marker remains in `crates/xtask/src/dsl_check/schema.rs`, and each entry left in
  whatever provisional tables remain there is one the CR or the design calls for; any other is
  fixed in the checkpoint's own landing, or, if it truly needs step 7 or later, ticketed with that
  reason. The landing lists them.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.
