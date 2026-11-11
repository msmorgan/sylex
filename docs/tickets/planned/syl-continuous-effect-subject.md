---
needs: [cleanup-step6-spine, syl-spelling-optional-segments]
---

# Continuous-effect declarations take no subject

## Why

A deed's subject stands outside its Action: a Handoff is `instruct(subject, action)` (208 canon
calls), and the deontic rules take their Subject and Action apart. A continuous effect's does
not. These declarations take the grammatical subject as their first parameter, with their canon
and builtin call counts:

| Declaration | Sort | canon | builtin |
|---|---|---|---|
| `getsPt` "$subject gets P/T" | StaticSpec | 80 | 8 |
| `abilityGrant` "$subject has $abilities" | StaticSpec | 75 | 13 |
| `qualityChange` | StaticSpec | 61 | 2 |
| `get` | Action | 47 | 12 |
| `costShift` | StaticSpec | 42 | 6 |
| `gain` | Action | 16 | 4 |
| `establishFor` | Action | 12 | 0 |
| `getsBase` | StaticSpec | 11 | 1 |
| `allAbilityLoss` | StaticSpec | 10 | 0 |
| `keywordGrant` | StaticSpec | 7 | 0 |
| `getsBoth` (two overloads) | Action, StaticSpec | 6 | 4 |
| `abilityLoss` | StaticSpec | 5 | 0 |
| `becomeColor` | Action | 4 | 0 |
| `untapDuring` | StaticSpec | 4 | 0 |
| `become` | Action | 3 | 1 |
| `copyChange` | StaticSpec | 3 | 0 |
| `doesntUntap` | StaticSpec | 3 | 0 |
| `addSubtype` | StaticSpec | 2 | 0 |
| `abilityGrantFrom` | StaticSpec | 2 | 0 |
| `mayDeclineUntap` | StaticSpec | 1 | 0 |
| `abilityGrantSelected` | StaticSpec | 0 | 1 |
| `characteristicChange` (primitive) | StaticSpec | 0 | 20 |

About 390 canon call sites in all. `gain` and `get` also carry their duration inside each call
(`establish(…, $duration)`).

With the subject inside each verb's call, a subject printed once over two verbs cannot be
written once. *Predator's Strike* prints "Target creature gets +3/+3 and gains trample until end
of turn", and `spelling.md` §3.3 already prescribes its term: "one Handoff over the Actions",
with one subject and one duration. Written with `get` and `gain`, it copies `target(…)` into
each call, which introduces two targets, or re-mentions it with an `it` the card does not print.
A floated "each" is the same: *Abandon Reason* prints "Up to two target creatures each get +1/+0
and gain first strike until end of turn", and the English grammar's `FloatedEach` is a Clause
production whose subject, floating quantifier and verb phrase are siblings
(`crates/english/grammar/english.fug`), so a Template for it needs the verb phrase to
stand alone.

A central adverb attaches below the subject too: `CentralAdverbial` is
`VerbPhrase -> adjunct:AdverbPhrase VerbPhrase` (`crates/english/grammar/english.fug`), inside the verb phrase
that `SubjectPredicate` joins to its subject. *Efflorescence* prints "that creature
also gains trample and indestructible", and *Beast Mode* a sentence-initial "Also put a +1/+1
counter on that creature". No Vintage-legal card prints "also can't", "still can't" or "also
must", so the deontics, which already take their Subject apart, are not involved.

What it does not buy. Where the shared subject has different roles in the arms, as in *Aether
Tunnel*'s "gets +1/+0 and can't be blocked", the coordination still binds it once by Factoring
(`spelling.md` §3.3).

## Open questions

- **Whether "also", "still" and "no longer" need the subjectless phrase.** This turns on whether
  spelling keeps general one-sided adjunction (`spelling.md` §2), which is not settled.
  - If it keeps it, "also" attaches inside the clause its argument spells, as `cant` places
    "can't", and the subject may stay bundled; this ticket only simplifies `syl-focus-also`.
  - If spelling drops it, keeping substitution, optional and list Holes, Tails, Factoring and its
    closed bridges, then a central adverb is a Construction over the verb phrase alone, which
    exists only once this ticket takes the subject out: "also", "still" and "no longer" on a
    continuous effect require it.

  Recommendation: none until the formalism is ruled; the ticket is needed by its other uses in
  either branch.
- **Where the duration stands.** One duration may cover two verbs under one subject (*Predator's
  Strike*, *Abandon Reason*), two clauses with two subjects (*Moku, Meandering Drummer*: "Moku
  gets +2/+1 and creatures you control gain haste until end of turn"), or be printed before the
  subject (*Gift of Tusks*: "Until end of turn, target creature loses all abilities and becomes a
  green Elephant with base power and toughness 3/3."). So the duration belongs to no one subject.
  Options:
  - (a) The declaration that takes the subject also takes the duration. It cannot write *Moku*'s
    one duration over two subjects, or *Gift of Tusks*'s fronted one, without a second duration
    or another declaration per shape.
  - (b) Taking a subject builds a StaticSpec from the subject and the subjectless phrase, and the
    duration stays where it is today, on `establish(spec, duration)`, whose spec may be a
    `conjunction` of such StaticSpecs. Templates over `establish` give its wordings: the duration
    after the spec ("… until end of turn") and before it ("Until end of turn, …").

  Recommendation: (b). It writes all four cards with one duration each, `get` and `gain` already
  expand to `establish` over a StaticSpec, and a static ability (*Glorious Anthem*) is the same
  StaticSpec with no `establish`.
- **The Action and StaticSpec split.** `gain` and `get` are Actions (`get` is
  `establish(getsPt(…), $duration)`), while `getsPt` and `abilityGrant` are StaticSpecs with the
  same wording. Under (b) above there is one subjectless phrase per verb, the StaticSpec that
  gives it a subject, and `establish` for the one-shot: no Action-level subject form remains.
  Recommendation: follow (b); `gain`, `get` and `getsBoth` at `Action` are deleted.
- **Names.** The sort for the subjectless phrase, the StaticSpec that gives it a subject, and the
  `establish` Templates for each placement of the duration. `instruct` names a Handoff, which
  makes its subject the Actor (glossary **Handoff**), and a creature that gets +3/+3 performs
  nothing, so it is not reused. Recommendation: the owner names the sort and the StaticSpec, with
  glossary entries; the Templates are named for their wording.
- **The subject-first replacement statics** `entersTapped` (13 canon calls),
  `entersWithCounters` (12 canon, 8 builtin), `entersWithAdditionalCounters` (8) and `enters`
  (3, 4) have the same shape. Recommendation: leave them, since what they replace is an event
  whose subject `would(subject, action)` already holds apart (`semantics.md` §4.1).

## Done when

- The open questions are answered and written in `docs/decisions/semantics.md` §4.7 and the
  glossary.
- `spelling.md` §3.3 names the declaration that gives *Predator's Strike*'s phrases their one
  subject, where it now says "one Handoff": a Handoff makes its subject the Actor, and a creature
  that gets +3/+3 performs nothing.
- No declaration in the table takes the subject of its continuous effect, and every call in
  `plugins/` is rewritten by a script, with the script and an old-to-new table in
  `docs/evidence/`; the script checks that each rewritten call carries the same subject,
  arguments and duration before and after, and the landing reports the counts it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Writing *Predator's Strike*, *Aether Tunnel* and the other coordinations that share a phrase:
  `syl-shared-phrase-written-once`, which writes them against this ticket's declarations.
- "Also": `syl-focus-also`, which depends on this ticket in one branch of the first open
  question.
- The referent "each" introduces: `syl-each-plural-referent`.

## Pointers

- `docs/decisions/spelling.md` §2, §3.3; `docs/decisions/semantics.md` §4.1, §4.7;
  glossary **Handoff**, **Subject**, **Static Spec** (`docs/contexts/game-model/CONTEXT.md`)
- `plugins/builtin/templates/action/{gain,get,getsBoth,establishFor,become}.syl`,
  `plugins/builtin/templates/ability/{getsPt,abilityGrant,getsBase}.syl`,
  `plugins/builtin/primitives/action/instruct.syl`
- `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` ("Design assessment 1")
