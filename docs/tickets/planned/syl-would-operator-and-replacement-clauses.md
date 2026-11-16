---
needs: [cleanup-step6-spine, syl-passive-participle-and-trigger-events]
specs: [voice-and-object-phrases]
---

# `would` is a verb-phrase operator, and a replacement's event is a clause

## Why

A replacement's event is `would(subj: Subject, action): WouldClause` (82 calls in
`plugins/canon/`, 40 in `plugins/builtin/`; 34 of them `would(any, …)` and 7 `would(by(…), …)`,
by `grep`), a form that only `replace` and its wrappers can hold. The cards print "would" in more
places than that: inside a relative clause on a damage phrase (*Glacial Chasm*'s "Prevent all
damage that would be dealt to you."), and they print the event's subject as an ordinary noun
phrase that the replacement then reads back (*Hardened Scales*'s "If one or more +1/+1 counters
would be put on a creature you control, that many plus one +1/+1 counters are put on it
instead."). With the voice in a `Subject` word, the printed subject of a passive event is not
the term's subject, and one site writes a passive the card does not print: *Moonlit
Meditation*'s "The first time you would create one or more tokens each turn" is
`would(by(you), …)`.

## Done when

- `would(phrase: VerbPhrase): VerbPhrase` is a verb-phrase operator, and `replace`'s `event` is
  a `Clause` whose verb phrase is `would(…)`. *Phytohydra*'s replacement is the spec's §8 term,
  with `bare(damage)` its event's subject.
- The sort `WouldClause` is gone: `nthOccurrence`, `causedBy`, `replaceEvent`,
  `replaceNextEvent` and the `damage` parameters of `preventDamage` and `redirectDamage` take a
  `Clause`.
- `cargo xtask dsl check` refuses a `replace` whose event's verb phrase is not a `would`, looking
  through `nthOccurrence` and `causedBy`, with a negative test.
- No `would` call holds `actor(…)`, `any` or `by(…)`. Each former `would(any, …)` and
  `would(by(…), …)` site is read from its card's Oracle line, a printed passive written
  `would(be(done(…)))` said of its printed subject and a printed active written active (*Moonlit
  Meditation*); the landing lists each with its line.
- The `would(actor(x), …)` sites (55 today), whose subject only loses its wrapper and moves into
  a `clause`, are rewritten by a one-off script, kept in `docs/evidence/`, which checks that each
  carries the same subject and verb phrase before and after; the landing reports the counts it
  compared.
- A rule body's pattern that no card prints writes `any`, the wildcard word at `NounPhrase`, as
  its clause's subject (spec §8): `infect`'s `would(any, loseLife(anyAmount))` is a clause over
  `would(loseLife(…))` whose subject is `any`. The landing lists them.
- `instead` holds a clause for a passive effect: *Hardened Scales*'s replacement is
  `instead(clause(…, be(done(putCounters(subject, it)))))`, with the counters phrase its subject.
- `untapSelf` and `getCounters` exist, Templates whose bodies call `untap` and `putCounters` with
  `subject`, each with its caller checked against Oracle text: *Bewitching Leechcraft*'s granted
  "If this creature would untap during your untap step" and *Winding Constrictor*'s "If you
  would get one or more counters, you get that many plus one of each of those kinds of counters
  instead."
- `docs/design/surface.md` §7.1's table has the replacement row of the spec's §1 table, and
  `docs/design/semantics.md` §3.1 and `docs/design/surface.md` §7.6 name `would` as an operator
  on a clause's verb phrase.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- `prevent` over a damage phrase with a `would` relative, and what becomes of `preventDamage`:
  `syl-prevent-damage-phrase`.
- A clause whose subject carries a `would` relative ("The next 1 damage that would be dealt to
  target creature this turn is dealt to another target creature instead", *Carom*) and
  `redirectDamage`: `syl-replacement-stated-as-clause`.
- Where "instead" prints inside its clause, a Template per placement (spec §11); an object
  relative under `would` (*Harm's Way*); *Illusionary Mask*'s third sentence: not designed
  (spec §11).
- Which heads a `would` admits: `syl-event-gate`.
- A replacement that excepts the nth event: `syl-replacement-except-nth-event`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §4, §8, §9
- `plugins/builtin/primitives/event/{would,causedBy}.syl`,
  `plugins/builtin/primitives/would/nthOccurrence.syl`,
  `plugins/builtin/primitives/static/{replace,instead}.syl`,
  `plugins/builtin/templates/action/{replaceEvent,replaceNextEvent}.syl`,
  `plugins/builtin/templates/ability/enters.syl`,
  `plugins/builtin/definitions/keyword_ability/infect.syl`
- `plugins/canon/cards/{Phytohydra,Hardened Scales,Moonlit Meditation,Winding Constrictor,Bewitching Leechcraft}.syl`
- `docs/design/surface.md` §7.1, §7.6; `docs/design/semantics.md` §3.1
