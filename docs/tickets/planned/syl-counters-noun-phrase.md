---
needs: [cleanup-step6-spine, syl-nominal-descriptions, english-counter-kind-premodifies-counter]
specs: [voice-and-object-phrases]
---

# The counters a verb puts or removes are a noun phrase

## Why

`putCounters(amount, kind, on)` and `removeCounters(quantity, kind, from)` spread their object
over a scalar, a kind and the word "counters" fixed in their wording, so the phrase a card prints
there is not in the term:

| Card | Oracle | Today |
|---|---|---|
| *Hardened Scales* | "If one or more +1/+1 counters would be put on a creature you control, that many plus one +1/+1 counters are put on it instead." | `putCounters(anyAmount, printedKind(p1p1Counter), …)`, then `putCounters(plus(thatMany, 1), printedKind(p1p1Counter), it)` |
| *Solemnity* | "Counters can't be put on artifacts, creatures, enchantments, or lands." | `putCounters(anyAmount, anyKind, bare(or([…])))` |
| *Winding Constrictor* | "that many plus one of each of those kinds of counters are put on that permanent instead" | `putCounters(plus(thatMany, 1), thoseCounters, that(permanent))` |

Each phrase is a clause's subject on its card, and none can be one while it is three arguments;
the passive of "put" has nothing to promote until the counters are a phrase (spec §3, §6).
`putCounters` is called 93 times in `plugins/canon/` and 45 in `plugins/builtin/`,
`removeCounters` 16 and 8 (`grep`; re-count when claimed).

## Design

- A counters phrase is a nominal whose head is `counter` and whose kind is a premodifier:
  "+1/+1 counters" is `nominal(pre = [<the +1/+1 measure>], counter)`, "a flying counter"
  `nominal(pre = [flying], counter)` and "a charge counter" `nominal(pre = [charge], counter)`.
  A named kind is a `CounterKind` word spelled by its name alone (`charge` is "charge"); a
  keyword counter's kind is the keyword [CR#122.1b] and a +X/+Y counter's its measure
  [CR#122.1a], so neither has a kind word of its own. Where no kind is printed the head stands
  alone.
- A coordination of kinds is a coordination in `pre`: *Denry Klin, Editor in Chief*'s "a +1/+1,
  first strike, or vigilance counter".
- `printedKind` and `anyKind` are deleted, and `thoseCounters`, `chosenKind`,
  `distinctChosenKinds` and `sameKindAs` are each replaced by the noun phrase their line prints
  (*Winding Constrictor*'s "of each of those kinds of counters", *Helica Glider*'s "your choice
  of a flying counter or a first strike counter").
- Every declaration that fixes "counters" in its wording beside a scalar and a kind takes the
  phrase: `putCounters`, `removeCounters`, `moveCounters`, `distributeCounters`,
  `removeAllCounters`, `loseCounters`, and the move rider `withCounters` with
  `entersWithCounters`, `entersWithAdditionalCounters`, `entersWithFewerCounters`,
  `exileWithCounters` and `returnToBattlefieldWithCounters` over it. `countersOn` stays a measure
  over a kind.
- The term does not refuse a kind word before another head ("charge creature"); semantics does.

## Open questions

- **The word for a measure before "counter".** "+1/+1" there is the measure a power/toughness
  change prints, which `syl-pt-change-sort` makes one value, `pt(power, toughness)`.
  Recommendation: that value is the premodifier, and this ticket needs `syl-pt-change-sort`.
- **What a counter confers.** A keyword counter confers its keyword and a +X/+Y counter its
  measure by rule [CR#122.1a,122.1b], so their `counter` declarations (`flyingCounter`,
  `p1p1Counter`) and the uncalled `grants` Template say nothing the rule does not.
  Recommendation: the two rules are each one conferral over the kind, and those declarations and
  `grants` are deleted. The `counter` declarations whose `lexical` lemma only repeats a measure
  or keyword are `p1p1Counter` and `m1m1Counter` (the measures +1/+1 and -1/-1) and
  `deathtouchCounter`, `decayedCounter`, `doubleStrikeCounter`, `exaltedCounter`,
  `firstStrikeCounter`, `flyingCounter`, `hasteCounter`, `hexproofCounter`,
  `indestructibleCounter`, `lifelinkCounter`, `menaceCounter`, `reachCounter`, `shadowCounter`,
  `trampleCounter` and `vigilanceCounter` (the keyword counters [CR#122.1b]).

## Done when

- The open questions are answered, and each answer is in the doc comment of the declaration it
  shapes.
- `putCounters(counters: NounPhrase, on)` and `removeCounters(counters: NounPhrase, from)` are
  the declarations, the others the Design lists take the phrase the same way, and no call writes
  an amount or a kind beside the phrase.
- `counter` is the head at `Nominal`, each kind stands in `pre` as the Design says, and no
  `counter` declaration's lemma or spelling ends in "counter". `docs/design/syl.md` §2 and the
  glossary entry **Coercion** (`docs/contexts/game-model/CONTEXT.md`) list the coercions that
  exist.
- The eight cards `english-counter-kind-premodifies-counter` names are written or authored with
  their coordination in `pre`.
- *Hardened Scales*, *Solemnity* and *Winding Constrictor* write the phrases in the table above
  as noun phrases, each checked against its Oracle text; where a line prints the phrase as a
  passive clause's subject, this ticket writes the phrase and
  `syl-would-operator-and-replacement-clauses` and `syl-deontic-verb-phrase-operators` the
  clause.
- Every call in `plugins/` is rewritten by a one-off script, kept with an old-to-new table in
  `docs/evidence/`. The script checks that each rewritten call carries the same amount, kind and
  holder before and after, and the landing reports the counts it compared. A call whose amount
  was `anyAmount` or whose kind was `anyKind` is read from its card's Oracle line ("one or more
  +1/+1 counters", "counters"); the landing lists those sites with the phrase each now writes.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The passive of `putCounters` and the Template `getCounters`:
  `syl-passive-participle-and-trigger-events` and the tickets after it.
- Whether a counters phrase introduces a Referent that "those counters" and "that many" read:
  not decided (spec §6), and `syl-numeric-anaphors` owns the readbacks.
- Damage, life and tokens: `syl-damage-and-life-noun-phrases`, `syl-token-records-to-templates`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §6
- `plugins/builtin/primitives/action/{putCounters,removeCounters,moveCounters}.syl`,
  `plugins/builtin/templates/action/{removeAllCounters,loseCounters,distributeCounters}.syl`,
  `plugins/builtin/primitives/zone/withCounters.syl`,
  `plugins/builtin/primitives/noun/{printedKind,anyKind,thoseCounters,chosenKind,distinctChosenKinds,sameKindAs}.syl`,
  `plugins/builtin/definitions/counter/`, `plugins/builtin/coercions/`
- `crates/xtask/src/dsl_check/coercion.rs`
