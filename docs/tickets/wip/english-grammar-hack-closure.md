---
needs: [english-require-on-unvalued-features, english-object-coordination-exclusions, english-outside-the-game-frame, english-fused-each-partitive, english-specifying-clause, english-tied-for-superlatives, english-synesis-each-partitive, english-renamed-children-head-agreement, english-interleaved-manner-gate, english-single-card-productions, english-cgel-citation-corrections, english-gate-ablation-records, english-targeting-marker-recursion, english-among-plural-complement, english-participle-coordination-inflection, english-quantity-bracketing, english-keyword-premodifier-bare, english-multiplied-degree-quantity]
---

# Epic: every construct in the English grammar is the right mechanism, honestly described

## Why

Some declarations in `crates/english/grammar/english.fug` exist to make particular cards parse
or to remove an unwanted Reading, and describe themselves in general-grammar terms they may not
earn: a citation that covers a different claim, a gate whose stated reason is not its real one,
a duplicate suppressed downstream of its cause, or an equation that does not enforce what its
comment says. Each row below is one such construct or class of construct, with the ticket that
confirms the defect against its source and then fixes it or refutes it. This ticket closes when
the rows in its `needs:` are closed.

| Construct | Owning ticket | Kind |
|---|---|---|
| `=` on a feature the child may leave unvalued | `english-require-on-unvalued-features` | Gate that may admit everything |
| `ObjectCoordination`'s pairwise exclusions | `english-object-coordination-exclusions` | Equations enforce less than the comment |
| The `FixedFrame` declarations for *outside the game* | `english-outside-the-game-frame` | One lexeme; cause in the lexicon |
| Fused *each* banned from Complements, then worked around | `english-fused-each-partitive` | Suppression downstream of the cause |
| `SpecifyingClause` and `Variable` | `english-specifying-clause` | One card's sentence |
| The *tied for* Productions | `english-tied-for-superlatives` | Justification contradicts its citation |
| `SynesisDistributivePartitive` | `english-synesis-each-partitive` | One card; citation covers another claim |
| Children renamed away from `head` | `english-renamed-children-head-agreement` | Workaround for head agreement |
| `InterleavedManner` | `english-interleaved-manner-gate` | Style advice as a hard gate |
| Productions serving one or two cards | `english-single-card-productions` | General rule stated for a single card |
| Citations that do not support their claim | `english-cgel-citation-corrections` | Comment accuracy |
| Gates with no ablation record | `english-gate-ablation-records` | Working rule not followed |
| A determined Targeting Marker taken as a Marker again | `english-targeting-marker-recursion` | Equation admits more than the comment |
| *Among* with a singular Complement | `english-among-plural-complement` | Frame admits more than English |
| `ParticipialAttribute`'s test on a coordination with no Inflection | `english-participle-coordination-inflection` | Gate that admits everything |
| *Any one color* and *half X damage* built twice; `MultipliedAmount` | `english-quantity-bracketing` | Second tree of one analysis |
| A keyword with a parameter as a noun's premodifier | `english-keyword-premodifier-bare` | Feature that does not mean what its comment says |
| *Twice that many cards*; *target* over a quantity | `english-multiplied-degree-quantity` | Bracketing; determiner admitted over a quantity |

These are not required for this ticket to close. Each rests on a single reading, or on two that
disagree, and joins `needs:` if it is moved to `planned/`:

| Construct | Ticket |
|---|---|
| `unlinked` on `FiniteComplementClause` | `english-unlinked-complement-clause` |
| The comparative Productions as a hand-expanded cross product | `english-comparative-productions` |
| The Targeting Marker's Number in a quantity's oblique | `english-targeting-marker-number` |
| Coordination gates on Noun Phrases, noun premodifiers and relatives | `english-coordination-gate-review` |
| Comments that claim more than their equations enforce | `english-comment-equation-drift` |
| Categories that restate Productions as twins | `english-category-restriction` |
| A determined slash measure marked as a compound's first base | `english-measure-nominal-compound-part` |
| Postposed Object modifiers built on an open Gap | `english-postposed-modifier-gap` |
| `SingularOrQuantified` and the quantified plural it claims | `english-singular-or-quantified-value` |
| Number transparency for *any number of* only | `english-transparent-quantity-determiners` |
| Stranded auxiliaries licensed by spelling | `english-stranded-auxiliary-lookahead` |
| Features that carry several unrelated meanings | `english-features-as-tags` |
| Modifiers made determinatives before a bare singular noun | `english-bare-singular-determinatives` |
| Complement selection written as grammar classes | `english-frames-held-in-grammar` |

## Done when

- Every ticket in `needs:` is in `done/`, each either fixed or closed as refuted against its
  source.
- No fix replaced a construct with another of the same kind: a gate removed at its cause is not
  re-added downstream, and a comment that named a general rule for one card now names the card
  or the general rule holds.
- A defect of these kinds found while working a row is routed to the row's ticket or to a new
  one, and this table gains its row, before this ticket closes.

## Not in scope

- Whether a gate has an effect on the parse forest: ablation measures that, and the rows here
  are about whether the construct is the right mechanism and truthfully described.
- Shortening or restoring comments that are accurate.

## Pointers

- `docs/decisions/english.md` §3 to §6 and §8.
- `crates/english/README.md`, "Working rules".
- The owning tickets in the tables.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md`,
  `docs/evidence/english-fug-hax-check/gemini.md`.
