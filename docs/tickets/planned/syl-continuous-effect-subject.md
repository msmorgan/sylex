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
of turn": one subject and one duration over two verbs, which the term writes once, with the
coordinated phrases under the one subject (`spelling.md`, shared material at a coordination).
Written with `get` and `gain`, it copies `target(…)` into each call, which introduces two
targets, or re-mentions it with an `it` the card does not print. A floated "each" is the same:
*Abandon Reason* prints "Up to two target creatures each get +1/+0 and gain first strike until
end of turn", and the English grammar's `FloatedEach` is a Clause production whose subject,
floating quantifier and verb phrase are siblings (`crates/english/grammar/english.fug`), so a
Template for it needs the verb phrase to stand alone.

A central adverb needs it too. `CentralAdverbial` is `VerbPhrase -> adjunct:AdverbPhrase
VerbPhrase`, inside the verb phrase that `SubjectPredicate` joins to its subject
(`crates/english/grammar/english.fug`). Spelling is substitution, with no adjunction that could
place "also" inside a clause a subject-bundled declaration spells, so "also", "still" and "no
longer" on a continuous effect are Templates over the subjectless phrase. *Efflorescence* prints
"that creature also gains trample and indestructible" (`syl-focus-also`). No Vintage-legal card
prints "also can't", "still can't" or "also must", so the deontics, which already take their
Subject apart, are not involved.

What it does not buy. Where the shared subject has different roles in the arms, as in *Aether
Tunnel*'s "gets +1/+0 and can't be blocked", the coordination still binds it once by Factoring.

## Open questions

- **Where the duration stands.** In the Readings (`cargo xtask english parse`) a trailing
  duration adjoins the verb phrase, below the subject, not the clause: "Target creature gets
  +3/+3 until end of turn." is `SubjectPredicate(NP, VP(VP "gets +3/+3", PP "until end of
  turn"))`, so "Target creature gets +3/+3" is no constituent. *Predator's Strike* attaches it to
  the verb-phrase coordination "gets +3/+3 and gains trample"; *Abandon Reason* the same, under
  `FloatedEach`; *Efflorescence*'s "also" wraps "gains trample and indestructible until end of
  turn", duration included. Two shapes are clause-level: *Gift of Tusks*'s fronted "Until end of
  turn, target creature loses all abilities and becomes …" adjoins the clause, and *Moku,
  Meandering Drummer*'s "Moku gets +2/+1 and creatures you control gain haste until end of turn"
  attaches one duration to a coordination of two clauses (`CoordinatedClauseAdjunct`, which
  requires its head to be a coordination). Options:
  - (a) The duration attaches to the subjectless phrase, "$effect $duration", and that phrase
    substitutes into whatever gives it its subject: the plain clause, floated "each", "also".
    Clause-level wordings remain only for the fronted duration ("$duration, $spec") and for two
    or more coordinated clauses sharing a trailing duration. `establish(spec, duration)` stays
    the expansion; since a first-order Template cannot place one argument inside another's call,
    the subject reaches the phrase's StaticSpec as a binding the subject-giving declaration makes
    (as `instruct` binds the Actor of its Action), read by the phrase's subject.
  - (b) A clause-level "$spec $duration" over a subject-taking StaticSpec. It has no constituent
    to spell for a single subject, since the duration sits inside the verb phrase.

  Recommendation: (a), which every parse above fits.
- **The Action and StaticSpec split.** `gain` and `get` are Actions (`get` is
  `establish(getsPt(…), $duration)`), while `getsPt` and `abilityGrant` are StaticSpecs with the
  same wording. Under (a) above there is one subjectless phrase per verb, a duration form of it,
  the declaration that gives it a subject, and `establish` as the one-shot's expansion.
  Recommendation: follow (a); `gain`, `get` and `getsBoth` at `Action` are deleted.
- **Names.** The sort for the subjectless phrase, the declaration that gives it a subject and
  the word that reads that subject inside it, and the duration Templates (the phrase-level one,
  the fronted one, the coordinated-clauses one). `instruct` names a Handoff, which
  makes its subject the Actor (glossary **Handoff**), and a creature that gets +3/+3 performs
  nothing, so it is not reused. Recommendation: the owner names the sort, the declaration and the word,
  with glossary entries; the Templates are named for their wording.
- **The subject-first replacement statics** `entersTapped` (13 canon calls),
  `entersWithCounters` (12 canon, 8 builtin), `entersWithAdditionalCounters` (8) and `enters`
  (3, 4) have the same shape. Recommendation: leave them, since what they replace is an event
  whose subject `would(subject, action)` already holds apart (`semantics.md` §4.1).

## Done when

- The open questions are answered and written in `docs/decisions/semantics.md` §4.7 and the
  glossary.
- `spelling.md`'s account of shared material at a coordination names the declaration that gives *Predator's Strike*'s phrases their one
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
- "Also": `syl-focus-also`, which writes it over this ticket's subjectless phrase.
- The referent "each" introduces: `syl-each-plural-referent`.

## Pointers

- `docs/decisions/spelling.md` (substitution, shared material at a coordination); `docs/decisions/semantics.md` §4.1, §4.7;
  glossary **Handoff**, **Subject**, **Static Spec** (`docs/contexts/game-model/CONTEXT.md`)
- `plugins/builtin/templates/action/{gain,get,getsBoth,establishFor,become}.syl`,
  `plugins/builtin/templates/ability/{getsPt,abilityGrant,getsBase}.syl`,
  `plugins/builtin/primitives/action/instruct.syl`
- `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` ("Design assessment 1")
