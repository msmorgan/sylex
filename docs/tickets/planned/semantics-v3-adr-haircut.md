---
needs: []
---
# A progressive-disclosure pass and a haircut for the semantics-v3 ADR

Standard constraints apply. `docs/decisions/semantics-v3.md` is the design authority for the
semantics-v3 rewrite, and it is too long to serve as one: dense paragraphs mix principles, rulings,
per-card examples and Lean implementation detail (Lean constructor and field names, `.lean` file
paths, ticket provenance).

## Change

- Lead with what a reader needs first: the principles and the shape of the pipeline (`.syl`,
  name resolution and expansion, discourse, lowering), each in a few sentences, with the details
  behind them further down or in linked sections.
- Cut what is not a decision: Lean implementation detail, histories of how a ruling was reached,
  ticket-by-ticket provenance, and duplicated examples. Keep each ruling and the one example that
  shows it; keep the CR citations a ruling rests on.
- Describe the model in the project's terms (the glossary, `.syl`), not Lean's, so the ADR stays
  true once Lean is gone.
- No ruling changes meaning. Where the pass finds two rulings that contradict each other or one
  that no longer holds, report it to the owner rather than resolving it.

## Tests

- `cargo xtask cite check` 0 stale and `cargo xtask cite coverage --check` holds (a citation cut
  from the ADR is kept where the rule is still cited, or its loss is reported).
- The owner reviews the result.
