---
needs: [cleanup-step6-spine, syl-continuous-effect-subject, syl-would-operator-and-replacement-clauses, syl-lookback-operator-and-holds]
specs: [voice-and-object-phrases]
---

# The deontic words are verb-phrase operators, and the sort `Subject` goes

## Why

`cant`, `can`, `must`, `gatedBy`, `doesnt` and `asThough` each take a `Subject` and a verb phrase
and give a whole `StaticSpec` (`cant` is called 90 times in `plugins/canon/` and 40 in
`plugins/builtin/`, 47 of them `cant(any, …)`; `can` 48 and 41; by `grep`). So a prohibition
cannot share a subject with another verb phrase, and the subject of a passive prohibition is
buried in it:

- *Aether Tunnel*'s "Enchanted creature gets +1/+0 and can't be blocked." is a `conjunction` of
  `getsPt(enchanted(creature), …)` and `cant(any, block(attacker = it))`, with an `it` the card
  does not print.
- *Arrest*'s "Enchanted creature can't attack or block" is two `cant` calls, each with its own
  copy of `actor(enchanted(creature))`.
- *Abrupt Decay*'s "This spell can't be countered." and *Solemnity*'s "Counters can't be put on
  artifacts, creatures, enchantments, or lands." name their printed subjects only inside the
  verb phrase, under `any`.

These are the last words that take a `Subject`, so the sort and its three words go with them
(spec §1, §5).

## Open questions

- **"Doesn't untap during … untap step."** `doesntUntap(subject, whose, unless)`,
  `mayDeclineUntap` and `untapDuring` are subject-first Templates whose bodies are
  `cant(any, untap($subject))` and `must(any, untap($subject))` inside `duringUntapStep`: the
  printed "doesn't untap" is written as a prohibition of the `untap` Deed, which
  `doesntUntap`'s doc comment rests a stun counter's behavior on [CR#122.1d,614.17c]. The spec
  makes the printed verb the inchoative `untapSelf` (§4) and `doesnt` a verb-phrase operator
  (§5), but `doesnt` takes no Unless Condition, and `doesntUntap` has one. 311 Vintage-legal
  cards print "doesn't untap" or "don't untap", and 4 canon cards (*Apathy*, *Bombur, Gentle
  Dreamer*, *Dance of the Dead*, *Marooned*). Options: `doesntUntap` stays a Template, at
  `VerbPhrase` and with no subject, whose body is the prohibition; or the cards write
  `doesnt(untapSelf)`, and `doesnt` gains the modifiers the prohibition has. Recommendation: the
  Template, which keeps the prohibition the rules reading rests on and still coordinates under
  one subject (*Grimgrin, Corpse-Born*'s "enters tapped and doesn't untap during your untap
  step").

## Done when

- The open question is answered.
- `cant`, `can`, `must`, `gatedBy`, `doesnt` and `asThough` each take a verb phrase and give
  one, their modifiers staying their own parameters, and a static deontic rule is a `clause`
  standing at `StaticSpec`.
- No call of them holds `actor(…)`, `any` or `by(…)`. Each former `any` and `by(…)` site is read
  from its card's Oracle line or its rule's text, and the landing lists each with its line:
  *Abrupt Decay* is `static(clause(this(spell), cant(be(done(counter(subject))))))`, and
  *Solemnity*'s two lines are a clause over `cant(getCounters(…))` and a clause whose subject is
  the counters phrase over `cant(be(done(putCounters(subject, …))))`. A rule body's pattern that
  no card prints writes `any`, the wildcard word at `NounPhrase`, as its subject (spec §8).
- `and` and `or` stand at `VerbPhrase`. *Aether Tunnel*'s line is one clause over an `and` of
  `getsPt(…)` and `cant(be(done(block(attacker = subject))))`, and *Arrest*'s "can't attack or
  block" one `cant` over an `or`, each checked against its Oracle text.
- The sort `Subject` is gone, with `actor(np)`, `any` at `Subject` and `by(np)`; no `actor(` call
  is left in `plugins/`. A one-off script, kept in `docs/evidence/`, removes the remaining
  wrappers and checks that every file parses to the same tree apart from them; the landing
  reports the counts it compared.
- `docs/design/surface.md` §7.6 states a deontic rule and a negative declarative as the spec's §5
  does, `docs/design/semantics.md` §3.3 states the Unless Condition and §2.2 item 3 the force of an
  indefinite subject and where each operator's box opens as §5 does, and `docs/design/surface.md`
  §7.1's table has the deontic and negative-declarative rows of the spec's §1 table.
  `docs/design/syl.md` §2 no longer names `actor(…)` as a wrap written at the call site, and
  `docs/design/spelling.md` §3.3 gives *Aether Tunnel* as an ordinary coordination.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- What a deontic operator admits beyond events ("can't have or gain flying"), what `and` means
  in an event pattern, whether `doesnt` is gated, negation over `be` ("attacks and isn't
  blocked"), and "can't be the target of": not designed (spec §11).
- "Have" with a participle (*Melira's Keepers*'s "can't have counters put on it"): it waits on
  `english-have-past-participial-complement`.
- One event over an `or` of verb phrases and the phrases a coordination shares:
  `syl-shared-phrase-written-once`.
- The engine's untap step: `engine-untap-active-player-deed`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §1, §4, §5, §10
- `plugins/builtin/templates/static/{cant,can,must,gatedBy}.syl`,
  `plugins/builtin/primitives/static/{staticSpecCant,staticSpecCan,staticSpecMust,staticSpecGatedBy,doesnt,asThough}.syl`,
  `plugins/builtin/templates/ability/{doesntUntap,mayDeclineUntap,untapDuring}.syl`,
  `plugins/builtin/primitives/noun/{actor,by}.syl`, `plugins/builtin/primitives/subject/any.syl`
- `crates/xtask/src/dsl_check/schema.rs` (`DeonticMods`)
- `plugins/canon/cards/{Aether Tunnel,Arrest,Abrupt Decay,Solemnity,Apathy}.syl`
- `docs/design/semantics.md` §2.2; `docs/design/surface.md` §7.1, §7.6
