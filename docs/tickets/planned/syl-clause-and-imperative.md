---
needs: [cleanup-step6-spine, syl-verb-phrase-sort-name, syl-definition-actor-parameter, syl-damage-and-life-noun-phrases, syl-counters-noun-phrase]
specs: [voice-and-object-phrases, kind-sorts-and-position-unions]
---

# A sentence is a `Clause`: a subject with its verb phrase, or an imperative

## Why

No sort says that a position holds a sentence, and a sentence's subject is written four unrelated
ways:

- as a Handoff, `instruct(subject, action)` (209 calls in `plugins/canon/`, 29 in
  `plugins/builtin/`, by `grep`), named for handing a deed to a performer, which a creature that
  gets +3/+3 is not;
- not at all, where the card prints "you": *Agate-Blade Assassin*'s "defending player loses 1 life
  and you gain 1 life" is `sequentially([instruct(defendingPlayer, loseLife(1)), gainLife(1)])`,
  the same bare term an imperative "Gain 1 life" would be;
- inside the wording `may` gives itself, "You may <body>", which is wrong under a Handoff:
  *Apathy* prints "that player may discard a card at random" and writes
  `instruct(that(player), may(…))`;
- as the first parameter of each subject-first declaration (`syl-continuous-effect-subject`).

So one subject cannot be written once over two verb phrases, an effect and an imperative are the
same term, and the eight `action` Definitions that hold a Handoff (`fight`, `fightEachOther`,
`connive`, `explore`, `endure`, `airbend`, `regeneration`, `theRingTemptsYou`) have no other way
to say who does what (spec §1, §4).

## Design

- `may` stands at `VerbPhrase`, worded "may $body", and keeps `ifDid` and `ifNot`, each an
  `Option(Clause)`. `withContinuation` and the Templates over it are retyped the same way and not
  restructured. The spec's word at `Clause` that holds an optional clause and the sentences after
  it (§1) is not built here, since §13 item 6 reopens it; until it is settled, a follow-up
  sentence stands inside `may`'s verb phrase, as it does today.
- `sequentially` is declared at `Clause` and at `VerbPhrase`. This ticket adds no coordinator and
  no sequencing Template: how sentences join is not designed (spec §13), and
  `syl-sequentially-wordings` owns its wordings.

## Open questions

- **The name of the self-replacement `replace`** ("X. Y instead", `primitives/action/replace.syl`,
  [CR#614.15]). It holds sentences, so it stands at `Clause`, and `Clause` is in
  Reaches(`StaticSpec`) once a clause stands at `StaticSpec` (`syl-continuous-effect-subject`),
  where the static `replace` stands, so `syl.md` §3 refuses the shared name (spec §1). The spec
  gives no name. Recommendation: `selfReplace`, for what [CR#614.15] calls the effect.
- **A cost that prints its own subject.** The spec makes a cost a `VerbPhrase` (§1), and two
  Vintage-legal cards print a cost as a clause: *Wall of Shards*'s "Cumulative upkeep—An opponent
  gains 1 life." and *Roar of Jukai*'s "Splice onto Arcane—An opponent gains 5 life." (a regular
  expression over Oracle text for a dash before a subject other than "you"). *Wall of Shards*
  writes `cumulativeUpkeep(perform(instruct(anOpponent, gainLife(1))))` today, and a `clause` is
  no `VerbPhrase`. Options: a second word at `Cost` that takes a `Clause`, beside `perform`;
  `perform` taking a `Clause`, with `imperative(…)` written at each of its calls (79 in canon, 41
  in builtin); or `have(anOpponent, …)`, a word the card does not print. Recommendation: the
  second word at `Cost`, with *Wall of Shards* as its caller, which leaves every other cost the
  verb phrase the spec makes it.

## Done when

- The open questions are answered, and each answer is in the doc comment of the declaration it
  shapes.
- The sort `Clause` exists with `clause(subject: NounPhrase, phrase: VerbPhrase): Clause`, worded
  "$subject $phrase@Present", and `imperative(phrase: VerbPhrase): Clause`. No coercion joins
  `VerbPhrase` and `Clause`.
- `instruct` is gone, and every call of it is a `clause`. The expansion tests
  (`crates/xtask/src/dsl_expand/tests.rs`) name `clause` where they name `instruct`.
- Every parameter and record field that holds an effect takes a `Clause` (`sequentially`,
  `doIf`, `doUnless`, `instead`, the effect of `spell`, `activated` and the trigger words, a
  Mode's); one that holds a cost, or a phrase under an event, lookback, replacement or deontic
  word, stays a `VerbPhrase`. A declaration whose wording is one or more sentences stands at
  `Clause`, and one whose wording is a verb phrase stays at `VerbPhrase`; `may` with an `ifDid`
  or `ifNot` is the one exception (Design). The landing lists each
  declaration and parameter that was at `VerbPhrase` with the sort it now has and, where the
  choice is not the spec's (§1), the card line or rule text that shows it.
- An `action` Definition stands at `VerbPhrase` and its `body` is a `Clause` or a word that holds
  clauses, reading its call's performer through `$actor` (`mill`'s body is a clause whose subject
  is `$actor`). The eight Definitions named above are clauses, each checked against its rule's
  text.
- At every site, an effect with a printed subject is a `clause` with that subject, and an effect
  with no printed subject is `imperative(…)`. Each bare effect in `plugins/canon/` is checked
  against its card's Oracle text for a printed "you": *Agate-Blade Assassin*'s effect is the
  clauses "defending player loses 1 life" and "you gain 1 life". The landing lists the sites
  where a printed "you" became the subject.
- Every `may` site has its printed subject: "you may …" is `clause(you, may(…))`, and *Apathy*'s
  "that player may discard a card at random. If the player does, untap that creature." is
  `clause(that(player), may(discard(…), ifDid = imperative(untap(that(creature)))))`.
- The self-replacement `replace` stands at `Clause` under its new name.
- `have(performer: NounPhrase, phrase: VerbPhrase): VerbPhrase` exists, and *Invigorate*'s "you
  may have an opponent gain 3 life" writes it where its cost writes
  `perform(instruct(anOpponent, gainLife(3)))` today.
- A one-off script, kept with an old-to-new table in `docs/evidence/`, makes the rewrite and
  checks that each rewritten term is the term before it with `instruct` read as `clause`, an
  `imperative(…)` or `clause(you, …)` added where an effect stood bare, and nothing else changed;
  the landing reports the counts it compared.
- `docs/design/surface.md` §7.1's table has the imperative, clause and cost rows of the spec's
  §1 table; no sentence of `docs/design/semantics.md` §2.1, `docs/design/surface.md` §6.1 or
  `docs/design/spelling.md` §3.3 names a Handoff; `docs/design/syl.md` §4 says an `action`
  Definition's body is clauses. In the glossary (`docs/contexts/game-model/CONTEXT.md`),
  **Handoff** is retired, **Clause** is new, and
  **Actor** no longer says "unless a Handoff names another".
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- A relative clause composed from a clause with a gap stands at `RelativeClause`
  (`docs/specs/kind-sorts-and-position-unions.md` §4), and the landing lists the fixed relative
  Templates it replaces and those it leaves.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A clause standing at `StaticSpec`, and the declarations that take their subject as a
  parameter: `syl-continuous-effect-subject`.
- The sort `Subject` and the event, lookback, replacement and deontic words that take it:
  `syl-passive-participle-and-trigger-events` and the tickets after it. `does`, `would`, `did`
  and the deontic words keep their `Subject` here.
- The other cards that print "have" with a noun phrase and a bare verb phrase, and
  `mayHaveEnter`: `syl-have-performer-phrase`.
- A coercion from `VerbPhrase` to `Clause`, "If you do" as a sentence of its own, and how
  sentences join: not designed (spec §13).

## Pointers

- `docs/specs/voice-and-object-phrases.md` §1, §4, §10, §13
- `plugins/builtin/primitives/action/{instruct,sequentially,doIf,withContinuation,replace}.syl`,
  `plugins/builtin/templates/action/{may,doIfDone,doUnless,mayOrElse,offerWhen}.syl`,
  `plugins/builtin/primitives/ability/{spell,activated,triggered}.syl`,
  `plugins/builtin/definitions/keyword_action/`
- `crates/xtask/src/dsl_check/schema.rs` (the `action` kind, the record sorts),
  `crates/xtask/src/dsl_expand/tests.rs`
- `docs/design/semantics.md` §2.1; `docs/design/surface.md` §6.1, §7.1; `docs/design/syl.md` §3, §4;
  `docs/design/spelling.md` §3.3
