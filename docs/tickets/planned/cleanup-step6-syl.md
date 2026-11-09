---
needs: [syl-doc-comment-rationales, syl-implicit-coercions]
---

# Step 6: the `.syl` iteration is complete

## Why

Step 6 of the cleanup reshapes the `.syl` corpus under `cargo xtask dsl check` before step 7
builds the Rust AST from it. This ticket closes the staged step-6 graph: `syl-single-face-card-call`
starts it, then the spine (`cleanup-step6-spine`), then the surface (`cleanup-step6-surface`), then
three parallel middle stages (`cleanup-step6-constructs-and-cards`,
`cleanup-step6-rules-and-keywords`, `cleanup-step6-deeds`), then the close:
`syl-dead-weight-sweep`, followed by `syl-doc-comment-rationales` and `syl-implicit-coercions`, the
two tickets this one needs. A new step-6 ticket joins one stage: it needs the previous stage's
gate, and its stage's sub-epic needs it unless another ticket in that stage does.

## Done when

- Every step-6 ticket is in `done/`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.
