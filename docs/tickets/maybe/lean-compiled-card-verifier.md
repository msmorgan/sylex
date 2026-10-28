---
needs: [lean-generated-canon-modules]
---
# [design] A compiled Lean verifier for card definitions at corpus scale

Lean will check card definitions produced by the CxG layer, heading toward the full ~32k-card
corpus (owner, 2026-10-28). Kernel `decide` on `Card.check card = []` costs ~0.24 s of kernel
time per card. That is ~2 CPU-hours at 32k cards, though sharded and incremental.

A `lake exe` built from the same `Card.check` would check the corpus in seconds. It trusts the
Lean compiler instead of the kernel: `implemented_by`/`extern` overrides and code generation.

`lean-generated-canon-modules` separates `Cards.Data` from `Cards.Check`, so a compiled verifier
can replace or augment the Check side without touching data, `Definitions` or the proofs.

## Questions for the dialogue

- Which cards keep kernel proofs: canon, generated, or a sample?
- Is a compiled run acceptable as a gate, or only as a fast pre-check before kernel proofs?
- How does the verifier report refusals and positions?
