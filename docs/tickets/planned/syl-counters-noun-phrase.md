---
needs: [cleanup-step6-spine, syl-nominal-descriptions, english-counter-kind-premodifies-counter, syl-union-membership]
specs: [voice-and-object-phrases, kind-sorts-and-position-unions, confers-and-rules]
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

- A counters phrase is a nominal whose head is `counter(kind)`: the kind is the head's argument,
  printed before it, and where no kind is printed the head stands alone ("counters", *Solemnity*).
  No other head takes a kind, and a kind stands nowhere else.
- A kind is a value of `CounterKind`:
  - a named kind is a word spelled by its name alone (`charge` is "charge", `blood` "blood").
    Twelve kind names are also subtype words (blood, book, egg, fungus, gold, incarnation, mine,
    omen, plan, tower, trap, treasure); they share no position with a subtype, and a subtype
    prints its capital;
  - a +X/+Y kind is `boost(power, toughness)`, a parameterized kind: "+1/+1 counters" is
    `counter(boost(…))` [CR#122.1a];
  - a keyword counter's kind is a parameterized kind over its keyword, as `boost` is over its
    numbers [CR#122.1b]. The keyword is not itself a kind word: echo, fuse, plot and training are
    kind names and also keywords that no keyword counter can be.
- A coordination of kinds is written inside the argument: *Denry Klin, Editor in Chief*'s "a
  +1/+1, first strike, or vigilance counter".
- What a counter confers is stated once per rule, in the `confers` of the parameterized kind,
  enabled once per counter (`docs/specs/confers-and-rules.md` §1): `boost` gives its numbers
  [CR#122.1a] and the keyword kind its keyword [CR#122.1b]. The per-kind `counter` declarations
  that only repeat their measure or keyword are deleted, with the uncalled `grants` Template:
  `p1p1Counter` and `m1m1Counter`, and `deathtouchCounter`, `decayedCounter`,
  `doubleStrikeCounter`, `exaltedCounter`, `firstStrikeCounter`, `flyingCounter`, `hasteCounter`,
  `hexproofCounter`, `indestructibleCounter`, `lifelinkCounter`, `menaceCounter`, `reachCounter`,
  `shadowCounter`, `trampleCounter` and `vigilanceCounter`. The counters whose rules are their own (shield, stun, finality, rad, hone, poison)
  are `syl-counter-conferrals`'s.
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

## Done when

- `putCounters(counters: NounPhrase, on)` and `removeCounters(counters: NounPhrase, from)` are
  the declarations, the others the Design lists take the phrase the same way, and no call writes
  an amount or a kind beside the phrase.
- `counter` is the head at `Nominal` with its kind as the Design says, and no `counter`
  declaration's lemma or spelling ends in "counter".
- Counter kinds have one source. Today `crates/vocabulary/catalogs/counters.ron` lists 241 kinds
  for the lexicon and `plugins/builtin/definitions/counter/` declares 220 for `.syl`, and they
  disagree: eleven named kinds printed on in-scope cards have no declaration (blight, coin, death,
  devotion, echo, everything, healing, necrodermis, plot, training, unlock), and eleven +X/+Y
  kinds none (+0/+1, +0/+2, +1/+0, +1/+2, +2/+0, +2/+2, -0/-1, -0/-2, -1/-0, -2/-1, -2/-2), which
  `boost` covers. Every named kind is a `CounterKind` word, and either the catalog is derived
  from the declarations or a load check refuses a difference.
- The eight cards `english-counter-kind-premodifies-counter` names are written or authored with
  their coordination of kinds.
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
  `plugins/builtin/definitions/counter/`
