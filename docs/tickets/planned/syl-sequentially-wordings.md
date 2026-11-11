---
needs: [cleanup-step6-spine, syl-spelling-optional-segments, syl-where-letter-supplement]
---

# Each printed sequence of instructions is a Template over `sequentially`

## Why

`sequentially(steps)` (`plugins/builtin/primitives/action/sequentially.syl`) has no spelling and
is called 296 times (218 in 200 canon files, 78 in `plugins/builtin/`). It stands for several
printed constructions, which the English grammar reads as different trees:

- Separate sentences: *Duneblast*'s "Choose up to one creature. Destroy the rest."
  (`SentenceSequence`, `crates/english/grammar/english.fug`).
- ", then", a Linked Coordination (`LinkedClause`, `LinkedVerbPhrase`):
  *Cloudshift*'s "Exile target creature you control, then return that card to the battlefield
  under your control." A "then" with no comma (*Land Equilibrium*, "puts that land onto the
  battlefield then sacrifices a land") and a sentence opening "Then" (*Juxtapose*'s ". Then
  exchange control of artifacts the same way.") are further wordings.
- "And": *Agate-Blade Assassin*'s "defending player loses 1 life and you gain 1 life". An "and"
  with a comma before it is a different tree from one without (the coordination compiler's
  `binary` and `binary-delimited` drafts, `crates/syntax/src/compile.rs`).
- A verb-phrase coordination under one subject: *Bonus Round*'s "that player copies it and may
  choose new targets for the copy", which the card writes as two `instruct(that(player), …)`, so
  the printed subject is written twice.

No program spells terms yet, so `cargo xtask dsl check` cannot see that none of these round-trips.

A `define(x, …)` step (19 canon files, as *Celebrate the Harvest*'s second step) is no step: it is
the ", where X is …" supplement of the step before it. `syl-where-letter-supplement` takes it out
of the sequences and lands first, so the steps this ticket spells are the printed instructions.

## Design

- The steps' meaning is one: the instructions are followed in the order written [CR#608.2c].
  `sequentially` stays the one expansion, and each printed wording is its own Template over it,
  never one Template with a join or coordinator parameter that carries only wording
  (`spelling.md`, on the resolved term as the derivation). "And" and "then" between steps are two Templates, `$steps<and>` and
  `$steps<then>`.
- A verb-phrase coordination under one subject is one `instruct` over one Template of the steps,
  as `spelling.md` writes a shared subject (shared material at a coordination).

## Open questions

- **The comma and the sentence break.** ", and", ", then" and ". " are not joins: `spelling.md`'s
  notation has a closed set (`""`, `","`, `"and"`, `"or"`, `"and/or"`, `"then"`, `"$name"`), and a
  Reading stores no punctuation, which realization derives. Yet the grammar builds the
  delimited and undelimited coordinations as different trees, and a wording's fixed text may
  already hold a comma (`when`'s "When $event, if $condition, $action", `syl.md` §5). Options:
  the English layer derives the comma or period from structure, and one Template serves both;
  or the Readings differ, and the delimited forms are fixed Templates whose wording holds the
  comma ("$first, then $rest"), with §6 saying so; or sentences are juxtaposed `Sentence`
  elements with the `<>` join, each period coming from its sentence. Recommendation: parse the
  lines of *Cloudshift*, *Agate-Blade Assassin*, *Duneblast* and *Juxtapose* first and record
  their Readings; where the comma or period separates two Readings, fixed Templates, and
  `spelling.md` states the case where it says what punctuation a wording holds.
- **Names** for the Templates. Recommendation: named for their wording, chosen with the answer
  above; the glossary's **Linked Coordination** names the ", then" form.

## Done when

- The open questions are answered and written in `docs/decisions/spelling.md`.
- No call in `plugins/` writes `sequentially` where a Template spells its printed wording, and
  every call is checked against its card's Oracle text or the rule text its declaration cites;
  a sequence no Template spells yet keeps a `TODO(spelling)` naming the wording. A script makes
  the rewrite, with the script and an old-to-new table in `docs/evidence/`, and checks that every
  rewritten call expands to the same `sequentially` steps.
- *Bonus Round* writes "that player" once.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A printed "you" with no Handoff (*Agate-Blade Assassin*'s "you gain 1 life" is `gainLife(1)`;
  *Harsh Sustenance*'s "and you gain X life"): an argument of the step, not of the sequence.
- Comma-joined and "and"-joined costs (`compound`, *Goblin Bomb*'s "Remove five fuse counters …
  and sacrifice it"): `Cost`, not `sequentially`.

## Pointers

- `docs/decisions/spelling.md` (shared material at a coordination, the resolved term as the derivation, the round
  trip, the notation); `docs/decisions/syl.md` §5
- `plugins/builtin/primitives/action/sequentially.syl`;
  `plugins/canon/cards/{Duneblast,Cloudshift,Agate-Blade Assassin,Bonus Round,Juxtapose}.syl`
- `crates/english/grammar/english.fug` (`SentenceSequence`, `LinkedClause`, `LinkedVerbPhrase`),
  `crates/syntax/src/compile.rs` (coordination drafts); glossary **Linked Coordination**
  (`docs/contexts/oracle-english/CONTEXT.md`)
