---
needs: [semantics-rust-rewrite, lean-discourse-export]
---
# `discourse` agrees with Lean across the corpus

Standard constraints apply. The gate `lean-retirement` waits on.

- Per card, `discourse`'s bindings equal the `lean-discourse-export` bindings across the whole
  `plugins_v3` corpus, except entries marked FLAG DEFECT, where `discourse` fixes the defect and the
  difference is checked to be that fix.
- The exported fixture snapshot passes as one data-driven Rust test.
- Every entry of `docs/semantics-spec-catalog.md` exists as a native Rust test (the `discourse`
  tickets port the entries they implement; this ticket closes the remainder).
- A difference is resolved by fixing `discourse`, or, where Lean is wrong, by recording the
  difference in the catalog with the rule that decides it.
