---
needs: [cleanup-step6-spine, syl-nominal-descriptions]
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

## Open questions

- **Which sort the counter-kind coercion starts from.** The spec proposes a coercion to `Nominal`
  "from the counter-kind sort" (§6), and there are two. `CounterKind` is the sort the `counter`
  declarations stand at, each spelled as its compound noun (`p1p1Counter` is "+1/+1 counter").
  `CounterKindSource` is what `putCounters` takes today: `printedKind(kind)` (97 canon uses, 67
  builtin), `anyKind` (7 canon), `thoseCounters` (5 canon), `chosenKind(menu)` (2 canon),
  `distinctChosenKinds(menu)` (1 canon) and `sameKindAs(source)` (1 builtin). Recommendation: the
  coercion starts from `CounterKind`, so a bare `p1p1Counter` is the head; `printedKind` and
  `anyKind` go, the head `counter` standing where no kind is printed; and each of the other four
  is read from the line that prints it (*Winding Constrictor*'s "of each of those kinds of
  counters", *Helica Glider*'s "your choice of a flying counter or a first strike counter") and
  written as the noun phrase that line prints.
- **The other declarations that spread a counters phrase.** The spec's table (§6) retypes
  `putCounters` and `removeCounters`. These fix "counters" in their wording beside a scalar and a
  kind as well: `moveCounters(amount, source, destination, kind)`, `distributeCounters(amount,
  kind, among)`, `removeAllCounters(kind, from)`, `loseCounters(kind, amount, agent)`, the move
  rider `withCounters(amount, kind, …)` with `entersWithCounters`,
  `entersWithAdditionalCounters`, `entersWithFewerCounters`, `exileWithCounters` and
  `returnToBattlefieldWithCounters` over it, and the measure `countersOn(kind, holder)`. Cards
  print the same phrase in each: 565 Vintage-legal cards print "enters with … counter(s) on
  it/them", 47 "distribute … counters among" and 53 "remove all … counters from" (regular
  expressions over Oracle text, reminder text removed). Recommendation: each takes the phrase, in
  this ticket, since "puts" and "removes" in §6's opening sentence cover them; `countersOn` stays
  a measure over a kind.

## Done when

- The open questions are answered, and each answer is in the doc comment of the declaration it
  shapes.
- `putCounters(counters: NounPhrase, on)` and `removeCounters(counters: NounPhrase, from)` are
  the declarations, and no call writes an amount or a kind beside the phrase.
- A counter kind stands at `Nominal` as its compound noun through one coercion, and `counter` is
  the head at `Nominal` where no kind is printed. `docs/design/syl.md` §2 and the glossary entry
  **Coercion** (`docs/contexts/game-model/CONTEXT.md`) list the coercions that exist.
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
- A counter kind printed as a power/toughness pair ("a +2/+0, +1/+1, or +0/+2 counter",
  *Frankenstein's Monster*): a Word-Part Coordination of counter names.
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
