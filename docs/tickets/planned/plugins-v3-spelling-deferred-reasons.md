---
needs: []
---
# Replace the generated bucket-3 `TODO(spelling)` reasons with real ones

Standard constraints apply.

`plugins-v3-builtin-spelling-todos` left 151 markers as genuinely deferred. 123 of them carry
generated text instead of a reason:

- "Include or omit the clauses supplied by <param>" (most often for `duration`, `defender` and
  `kind`).
- "Format and coordinate the <list> list entries, including the empty case".

That ticket required each kept marker to say what is deferred. Optional parameters were also
handled inconsistently: `leave (from_ := battlefield)` got `"leaves <from_>"`, while
`returnToHand (from_ := wherever)`, `loseGame` and `gainHaste` stayed deferred.

## Change

Revisit each of the 123 markers:

- If the fixed wording exists and only an optional clause is open, write the `spelling`. Write
  the optional clause the way existing spellings with optional parameters do. Examples are
  `doublePower` and `triplePower`, whose wording ("double <subject>'s power") is in their own
  doc comments.
- If something genuinely is deferred, replace the marker text with the specific wording or
  realization that is missing.
- Settle one convention for optional and list parameters in `spelling`, and record it where
  `spelling` is defined (`docs/decisions/semantics-v3.md` §4.4). Apply it to the inconsistent
  cases above.

## Acceptance

- Zero remaining markers carry either generated text.
- Every remaining marker names something specific.
- The expansion baseline shows only `spelling` differences, each one `--expect`ed and its diff
  read.
