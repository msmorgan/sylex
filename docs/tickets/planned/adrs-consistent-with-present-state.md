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

- `english.md`: §1 and the layer table name `deckmaste_lexical_source_v4`
  and `deckmaste_english_v4` as live crates; §7 Cutover 2 still lists the suffix
  drop as future work, though it is done, and keeps the catalog move and the
  `sylex_features_v1` deletion; Cutover 1 says `plugins_v2/builtin/` stays
  until `semantics-v3-dsl-and-plugins`, which must match the actual schedule;
  §5 contrasts the crate with "`deckmaste_lexical_source`'s 4-layer salvage
  pipeline", which now reads as the crate itself because the old crate's name
  was reused.
- `english-lexical-analysis.md`: the header says it pairs with
  `deckmaste_english_v4`; names the v4 crate throughout.
- `semantics.md`: cites `deckmaste_spelling` (since deleted as a stub) as the consumer of linked
  abilities (§ on linked abilities), `plugins_v2/` and `deckmaste_semantics_v2`
  deletion timing, and the v1 `deckmaste_english` translation path; the
  deletion-bound and cutover bullets (lines near 54-57 and 608-610) predate
  `retire-v1-english-and-spelling`.
- `README.md` (the index): its English v4 entry names the renamed crates and
  describes a "two-stage cutover" whose first stage and suffix drop are done.
- `workbench-ron-shaped-and-label-rulings.md`: cites `plugins_v2/builtin/macros/`
  and `idris/src/Experimental/` paths as live evidence, but the workbench is Lean
  now.
- `card-authoring-binds-no-implicits.md`: cites `Experimental/Macros.idr` and
  `idris/src/Experimental/Cards.idr` as live; superseded for the Lean workbench
  already, and the remaining Idris greps need a historical marker.
- `lean-is-the-workbench.md`: `idris-retirement` amended its status lines for the
  `idris/` deletion; its migration appendix still names `idris/src/Experimental.idr`
  as the origin, which is history.

Standard constraints apply.
