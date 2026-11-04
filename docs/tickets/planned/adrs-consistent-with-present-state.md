---
needs: []
---
# Make the ADRs consistent with the present state

The `docs/decisions/` ADRs drifted from the code as crates were renamed, deleted
and rescheduled. `drop-v4-suffixes` (2026-10-26) added a one-line dated note near
the top of the four ADRs that name the renamed crates, and did nothing else.
This ticket brings the ADR bodies into line. Do not rewrite a decision: correct
or annotate only the statements of present fact, and keep the ruling itself.
Where an ADR is superseded in whole, say so at the top and link the successor.

Scope:

- The crate names and command: `deckmaste_english_v4` and
  `deckmaste_lexical_source_v4` are now `sylex_english` and
  `sylex_vocabulary`; the census command is `cargo xtask english`.
- Deleted crates still described as live: v1 English, `deckmaste_spelling` (deleted as an
  empty stub; recreated as `sylex_spelling` when spelling starts), the construction crates (`deckmaste_construction_v3*`,
  `deckmaste_construction_compiler`, `deckmaste_constructions_macro`), and the
  timing of `plugins_v2/builtin/`.
- Superseded cutover plans.
- The semantics ADRs that cite deleted paths.

Verify each item against the tree and `CLAUDE.md` ("Crate fates") before editing;
the lines below are where to look, not a finished audit.

## ADRs the ticket covers

- `semantics.md`: cites `deckmaste_spelling` (since deleted as a stub) as the consumer of linked
  abilities (§ on linked abilities), `plugins_v2/` and `deckmaste_semantics_v2`
  deletion timing, and the v1 `deckmaste_english` translation path; the
  deletion-bound and cutover bullets (lines near 54-57 and 608-610) predate
  `retire-v1-english-and-spelling`.
