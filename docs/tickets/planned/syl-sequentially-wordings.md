---
needs: [cleanup-step6-spine, syl-spelling-optional-segments, syl-where-letter-supplement, syl-spelling-conventions, syl-clause-and-imperative]
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
  choose new targets for the copy", which the card writes as two sentences, each with its own
  `that(player)`, so the printed subject is written twice.

No program spells terms yet, so `cargo xtask dsl check` cannot see that none of these round-trips.

A `define(x, …)` step (19 canon files, as *Celebrate the Harvest*'s second step) is no step: it is
the ", where X is …" supplement of the step before it. `syl-where-letter-supplement` takes it out
of the sequences and lands first, so the steps this ticket spells are the printed instructions.

## Design

- The steps' meaning is one: the instructions are followed in the order written [CR#608.2c].
  `sequentially` is declared at `Clause`, for a sequence of sentences, and at `VerbPhrase`, for
  verb phrases under one subject (`syl-clause-and-imperative`). It stays the one expansion at
  each sort, and each printed wording is its own Template over it, never one Template with a
  parameter that carries only wording (`spelling.md`, on the resolved term as the derivation); a
  join is fixed in the wording, never chosen by a parameter. "And" and "then" between steps are
  two Templates, `$steps<and>` and `$steps<then>`.
- A verb-phrase coordination under one subject is one `clause` over a Template of the steps at
  `VerbPhrase`, as `spelling.md` writes a shared subject (shared material at a coordination).
- Separate sentences are a list Hole whose elements are runs of sentences (`spelling.md` §2, a
  Hole may hold a run), juxtaposed, each period coming from its own sentence: spelling reads a
  sentence sequence as flat, so the grammar's right-recursive `SentenceSequence` imposes no
  grouping on the steps. A step whose own wording is several sentences is one element: *Advice
  from the Fae*'s "If you control more creatures than each other player, put two of those cards
  into your hand. Otherwise, put one of them into your hand." is one `doIf` step, and "Then put
  the rest on the bottom of your library in any order." the next. A sequence that a dependent
  printed in its first sentence governs (a trigger clause, `replaceEvent`'s would-clause) is
  governed whole by the run rule, so no Template here prints that dependent.

## Open questions

- **The comma before a coordinator.** ", and" and ", then" are not joins: `spelling.md`'s
  notation has a closed set (nothing, `","`, `"and"`, `"or"`, `"and/or"`, `"then"`), and a
  Reading stores no punctuation, which realization derives. Yet the grammar builds the
  delimited and undelimited coordinations as different trees, and a wording's fixed text may
  already hold a comma (`when`'s "When $event, if $condition, $action", `syl.md` §5). Options:
  the English layer derives the comma from structure, and one Template serves both; or the
  Readings differ, and the delimited forms are fixed Templates whose wording holds the comma
  ("$first, then $rest"), with §6 saying so. Recommendation: parse the lines of *Cloudshift*,
  *Agate-Blade Assassin* and *Land Equilibrium* first and record their Readings; where the comma
  separates two Readings, fixed Templates, and `spelling.md` states the case where it says what
  punctuation a wording holds.
- **Names** for the Templates. Recommendation: named for their wording, chosen with the answer
  above; the glossary's **Linked Coordination** names the ", then" form.
- **How much of the way sentences join this ticket settles.**
  `docs/specs/voice-and-object-phrases.md` §13 lists four things as not designed: a sequence of
  sentences against a coordination of
  clauses ("Draw two cards, then discard two cards"; "… . Then …"), a dependent in a run's first
  sentence that governs the run, "Otherwise", and a trailing sentence over what precedes it
  ("Activate only as a sorcery."). The Design above answers the first with Templates over
  `sequentially` at each sort and the second with the run rule, and says nothing of the other
  two. Recommendation: the owner confirms the first two before the corpus is rewritten;
  "Otherwise" stays `doIf`'s, and the trailing sentence its own declarations'
  (`activatedOnly…`), until a design says otherwise.

## Done when

- The open questions are answered and written in `docs/design/spelling.md`.
- No call in `plugins/` writes `sequentially` where a Template spells its printed wording, and
  every call is checked against its card's Oracle text or the rule text its declaration cites;
  a sequence no Template spells yet keeps a `TODO(spelling)` naming the wording. A script makes
  the rewrite, with the script and an old-to-new table in `docs/evidence/`, and checks that every
  rewritten call expands to the same `sequentially` steps.
- *Bonus Round* writes "that player" once.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A printed "you" (*Agate-Blade Assassin*'s "you gain 1 life", *Harsh Sustenance*'s "and you
  gain X life"): the subject of its own clause, which `syl-clause-and-imperative` writes.
- Comma-joined and "and"-joined costs (`compound`, *Goblin Bomb*'s "Remove five fuse counters …
  and sacrifice it"): `Cost`, not `sequentially`.

## Pointers

- `docs/design/spelling.md` (a Hole that holds a run and the run rule in §2, shared material
  at a coordination, the resolved term as the derivation, the round trip, the notation);
  `docs/design/syl.md` §5
- `plugins/builtin/primitives/action/sequentially.syl`;
  `plugins/canon/cards/{Duneblast,Cloudshift,Agate-Blade Assassin,Bonus Round,Juxtapose}.syl`
- `crates/english/grammar/english.fug` (`SentenceSequence`, `LinkedClause`, `LinkedVerbPhrase`),
  `crates/syntax/src/compile.rs` (coordination drafts); glossary **Linked Coordination**
  (`docs/contexts/oracle-english/CONTEXT.md`)
