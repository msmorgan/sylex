---
needs: [cleanup-step6-spine, syl-spelling-optional-segments, syl-spelling-conventions, syl-clause-and-imperative, syl-union-membership]
specs: [voice-and-object-phrases]
---

# Continuous-effect and "enters" phrases take no subject, and a clause stands at `StaticSpec`

## Why

A sentence's subject stands outside its verb phrase, in `clause(subject, phrase)`
(`syl-clause-and-imperative`). The statements of a static ability and the continuous effects of
a resolving one do not use it: no clause can stand at `StaticSpec`, and these declarations take
the grammatical subject as a parameter, with their canon and builtin call counts (re-count when
claimed):

| Declaration | Sort | canon | builtin |
|---|---|---|---|
| `getsPt` "$subject gets P/T" | StaticSpec | 80 | 8 |
| `abilityGrant` "$subject has $abilities" | StaticSpec | 75 | 13 |
| `qualityChange` | StaticSpec | 61 | 2 |
| `get` | VerbPhrase | 47 | 12 |
| `costShift` | StaticSpec | 42 | 6 |
| `gain` | VerbPhrase | 16 | 4 |
| `entersTapped` "$subject enters tapped" | StaticSpec | 13 | 0 |
| `entersWithCounters` | StaticSpec | 12 | 8 |
| `establishFor` | VerbPhrase | 12 | 0 |
| `getsBase` | StaticSpec | 11 | 1 |
| `allAbilityLoss` | StaticSpec | 10 | 0 |
| `entersWithAdditionalCounters` | StaticSpec | 8 | 0 |
| `keywordGrant` | StaticSpec | 7 | 0 |
| `getsBoth` (two overloads) | VerbPhrase, StaticSpec | 6 | 4 |
| `abilityLoss` | StaticSpec | 5 | 0 |
| `becomeColor` | VerbPhrase | 4 | 0 |
| `become` | VerbPhrase | 3 | 1 |
| `copyChange` | StaticSpec | 3 | 0 |
| `enters` | StaticSpec | 3 | 4 |
| `addSubtype` | StaticSpec | 2 | 0 |
| `abilityGrantFrom` | StaticSpec | 2 | 0 |
| `abilityGrantSelected` | StaticSpec | 0 | 1 |
| `characteristicChange` (primitive) | StaticSpec | 0 | 20 |

`gain` and `get` also carry their duration inside each call (`establish(…, $duration)`).

With the subject inside each verb's call, a subject printed once over two verbs cannot be
written once. *Predator's Strike* prints "Target creature gets +3/+3 and gains trample until end
of turn": one subject and one duration over two verbs. Written with `get` and `gain`, it copies
`target(…)` into each call, which introduces two targets, or re-mentions it with an `it` the card
does not print. A floated "each" is the same: *Abandon Reason* prints "Up to two target creatures
each get +1/+0 and gain first strike until end of turn", and the English grammar's `FloatedEach`
is a Clause production whose subject, floating quantifier and verb phrase are siblings
(`crates/english/grammar/english.fug`), so a Template for it needs the verb phrase to stand
alone. A central adverb needs it too: `CentralAdverbial` is `VerbPhrase -> adjunct:AdverbPhrase
VerbPhrase`, inside the verb phrase that `SubjectPredicate` joins to its subject, so "also" on a
continuous effect is a Template over the subjectless phrase (`syl-focus-also`). And the cards
coordinate an "enters" phrase under one subject with another verb phrase: *Grimgrin,
Corpse-Born* prints "Grimgrin enters tapped and doesn't untap during your untap step".

## Open questions

- **Which declarations print their `subject` parameter as a subject.** 60 of the 82 declarations
  at `StaticSpec` under `plugins/builtin/` take a parameter named `subject` or `subj`, and the
  spec has each give it up and stand under `clause` (§1). The wording of some prints that
  parameter elsewhere:
  - `altCost`, "You may $cost rather than pay $subject's mana cost", and `ratherThanPay`;
  - `addedCost`, "As an additional cost to cast $subject, $cost";
  - `reducedBySymbols`, "$subject's total cost is reduced by $cost";
  - `entryChoice`, "As $subject enters, choose $chosen", with `entersChoosing`,
    `entersChoosingFrom`, `entersChoosingPlayer` and `entersChoosingPlayerSecretly`;
  - `attachChoosing`, "As $subject becomes attached, choose a $sort.";
  - `controlGrant`, "$player controls $subject", whose printed subject is `player`.

  Recommendation: a declaration gives its parameter up only where its wording prints it as the
  subject of the declaration's own finite verb; the ones above are sentences of another shape and
  keep their parameter until a static ability's statement is a clause throughout, which is not
  designed (spec §13). The landing lists all 60 with the wording that decided each.
- **Where the duration stands.** In the Readings (`cargo xtask english parse`) a trailing
  duration adjoins the verb phrase, below the subject, not the clause: "Target creature gets
  +3/+3 until end of turn." is `SubjectPredicate(NP, VP(VP "gets +3/+3", PP "until end of
  turn"))`. *Predator's Strike* attaches it to the verb-phrase coordination "gets +3/+3 and gains
  trample"; *Abandon Reason* the same, under `FloatedEach`; *Efflorescence*'s "also" wraps "gains
  trample and indestructible until end of turn", duration included. Two shapes are clause-level:
  *Gift of Tusks*'s fronted "Until end of turn, target creature loses all abilities and becomes
  …" adjoins the clause, and *Moku, Meandering Drummer*'s "Moku gets +2/+1 and creatures you
  control gain haste until end of turn" attaches one duration to a coordination of two clauses
  (`CoordinatedClauseAdjunct`). The spec keeps a duration a parameter of the word it limits and
  leaves the duration words to this ticket (§8, §11). Options:
  - (a) The duration is a parameter of one word over the subjectless phrase: `establish`,
    retyped to take a `VerbPhrase` and give one, worded "$phrase $duration". A phrase with no
    duration needs no word, since a resolving effect's continuous-effect phrase lasts until the
    game ends when none is printed [CR#611.2a]. Clause-level wordings remain only for the fronted
    duration and for coordinated clauses sharing a trailing one.
  - (b) The duration stays a parameter of each verb's declaration, as in `get` and `gain`. A
    duration printed once over two verbs is then written on one of them or on both.
  - (c) A clause-level "$clause $duration", which has no constituent to spell for a single
    subject, since the duration sits inside the verb phrase.

  Recommendation: (a), which every parse above fits. It makes `establish` a word that wraps a
  verb phrase with its duration, where the spec adds no adverbial word that wraps a verb phrase
  (§8) and plans one list of a verb phrase's dependents in printed order (§13 item 1); (b) is the
  option that adds none.
- **`get` beside `getsPt`.** `getsBoth` becomes one declaration (spec §1). `get` and `getsPt`
  print the same words, "gets +P/+T", and differ only in `get`'s duration; with both at
  `VerbPhrase` one text has two terms. `gain` ("gains") and `abilityGrant` ("has") print
  different verbs. Recommendation: under (a), `get` is deleted and `getsPt` is the phrase;
  `gain` and `abilityGrant` both stay, each without its subject and `gain` without its duration.

## Done when

- The open questions are answered and written in `docs/design/surface.md` §7.6 and the
  declarations' doc comments.
- `Clause` is a member of `StaticSpec` (`docs/specs/kind-sorts-and-position-unions.md` §2), so a static ability's statement is `static(clause(…))`. The declarations that wrap a `StaticSpec` (`asLongAs`, `if`,
  `conditionally`, `conjunction`, `onlyWhile` and the others) keep their sort and wrap a clause
  through it. `docs/design/syl.md` §2 and the glossary list it.
- No declaration whose wording prints its subject takes it as a parameter; each is a
  `VerbPhrase` under `clause`: the declarations in the table, the "enters" family (`enters`,
  `entersTapped`, `entersTappedUnless`, `entersUnless`, `entersWithCounters`,
  `entersWithAdditionalCounters`, `entersWithFewerCounters`), and every other one a survey finds,
  apart from those Not in scope gives to later tickets.
  The survey is a one-off script, kept in `docs/evidence/`, that lists each declaration whose
  `spelling`, or the wording its doc comment gives, starts with a noun-phrase Hole or a fixed
  subject before a finite verb (`getEmblem`'s "you get an emblem with $abilities",
  `fightEachOther`'s "$pair fight each other").
- Every call in `plugins/` is rewritten by a script, with the script and an old-to-new table in
  `docs/evidence/`; the script checks that each rewritten call carries the same subject,
  arguments and duration before and after, and the landing reports the counts it compared.
- `docs/design/spelling.md` §3.3 says *Predator's Strike*'s two phrases stand under one `clause`.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The deontic words and the untap statics (`cant`, `can`, `must`, `gatedBy`, `doesnt`,
  `asThough`, `doesntUntap`, `mayDeclineUntap`, `untapDuring`): `syl-deontic-verb-phrase-operators`.
  `mayHaveEnter`: `syl-have-performer-phrase`. `becomeBlocked` and `resolve`:
  `syl-passive-participle-and-trigger-events`.
- Writing *Predator's Strike* and the other coordinations that share a phrase:
  `syl-shared-phrase-written-once`.
- "Also": `syl-focus-also`, which writes it over this ticket's subjectless phrase.
- The referent "each" introduces: `syl-each-plural-referent`.
- `static` taking a `Clause`, and the 12 `StaticSpec` declarations that neither take a subject
  nor wrap a spec (`replace`, `partSkip`, `manaRetention` and the rest): not designed (spec §13).

## Pointers

- `docs/specs/voice-and-object-phrases.md` §1, §8, §11, §13
- `docs/design/spelling.md` §2, §3.3; `docs/design/surface.md` §7.1, §7.6; glossary **Static
  Spec**, **Coercion** (`docs/contexts/game-model/CONTEXT.md`)
- `plugins/builtin/templates/action/{gain,get,getsBoth,establishFor,become}.syl`,
  `plugins/builtin/templates/ability/{getsPt,abilityGrant,getsBase,enters,entersTapped}.syl`,
  `plugins/builtin/primitives/action/establish.syl`, `plugins/builtin/coercions/`
- `crates/xtask/src/dsl_check/coercion.rs`
- `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` ("Design assessment 1")
