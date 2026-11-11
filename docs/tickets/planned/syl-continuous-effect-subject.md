---
needs: [cleanup-step6-spine, syl-argument-naming, syl-spelling-optional-segments]
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
(`crates/english/grammar/english.fug`, line 2488), so a Template for it needs the verb phrase to
stand alone.

What it does not buy. Where the shared subject has different roles in the arms, as in *Aether
Tunnel*'s "gets +1/+0 and can't be blocked", the coordination still binds it once by Factoring
(`spelling.md` §3.3). A central adverb ("also") needs nothing from it: one-sided adjunction
already places "also" inside the clause its argument spells, as `cant` places "can't"
(`spelling.md` §2), which is `syl-focus-also`'s design.

## Open questions

- **The duration comes out with the subject.** *Predator's Strike* and *Abandon Reason* print one
  duration over two verbs, and `gain` and `get` each carry one. Options: the declaration that
  takes the subject also takes the duration, and the verbs carry neither; or each verb keeps its
  duration and the coordination binds a shared one. Recommendation: the first, so a duration
  printed once is written once (`syl.md` §2) with no second Factoring case.
- **The Action and StaticSpec split.** `gain` and `get` are Actions (`get` is
  `establish(getsPt(…), $duration)`), while `getsPt` and `abilityGrant` are StaticSpecs with the
  same wording. Options: one sort for the subjectless continuous-effect phrase, taken by an
  Action that establishes it for a duration and by a StaticSpec that holds it; or a sort for each.
  Recommendation: one sort, so each verb has one wording (`spelling.md` §1).
- **Names.** The sort for the subjectless phrase, and the two declarations that give it a subject.
  `instruct` names a Handoff, which makes its subject the Actor (glossary **Handoff**), and a
  creature that gets +3/+3 performs nothing, so it is not reused. Recommendation: `establish`,
  which already wraps a StaticSpec with a duration, for the Action, a new word for the StaticSpec,
  and the sort named by the owner with a glossary entry.
- **The subject-first replacement statics** `entersTapped` (13 canon calls),
  `entersWithCounters` (12 canon, 8 builtin), `entersWithAdditionalCounters` (8) and `enters`
  (3, 4) have the same shape. Recommendation: leave them, since what they replace is an event
  whose subject `would(subject, action)` already holds apart (`semantics.md` §4.1).

## Done when

- The open questions are answered and written in `docs/decisions/semantics.md` §4.7 and the
  glossary.
- No declaration in the table takes the subject of its continuous effect, and every call in
  `plugins/` is rewritten by a script, with the script and an old-to-new table in
  `docs/evidence/`; the script checks that each rewritten call carries the same subject,
  arguments and duration before and after, and the landing reports the counts it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Writing *Predator's Strike*, *Aether Tunnel* and the other coordinations that share a phrase:
  `syl-shared-phrase-written-once`, which writes them against this ticket's declarations.
- "Also": `syl-focus-also`.
- The referent "each" introduces: `syl-each-plural-referent`.

## Pointers

- `docs/decisions/spelling.md` §2, §3.3; `docs/decisions/semantics.md` §4.1, §4.7;
  glossary **Handoff**, **Subject**, **Static Spec** (`docs/contexts/game-model/CONTEXT.md`)
- `plugins/builtin/templates/action/{gain,get,getsBoth,establishFor,become}.syl`,
  `plugins/builtin/templates/ability/{getsPt,abilityGrant,getsBase}.syl`,
  `plugins/builtin/primitives/action/instruct.syl`
- `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` ("Design assessment 1")
