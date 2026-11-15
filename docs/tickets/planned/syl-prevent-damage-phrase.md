---
needs: [cleanup-step6-spine, syl-damage-and-life-noun-phrases, syl-would-operator-and-replacement-clauses]
specs: [voice-and-object-phrases]
---

# `prevent` takes the damage it prevents as a noun phrase

## Why

`preventDamage(cut, damage, also)` (30 calls in `plugins/canon/`, 5 in `plugins/builtin/`, by
`grep`) spreads the damage a card names over a `PreventCut` (`allDamage`, `theNext(n)`,
`ofThatDamage(n)`, `allBut(n)`, `half(rounding)`) and an optional would-clause, and holds the
prevention's additional effect as a third argument. The cards print one noun phrase after
"prevent":

| Card | Oracle |
|---|---|
| *Glacial Chasm* | "Prevent all damage that would be dealt to you." |
| *Defang* | "Prevent all damage that would be dealt by enchanted creature." |
| *Temper* | "Prevent the next X damage that would be dealt to target creature this turn." |
| *Stormwild Capridor* | "If noncombat damage would be dealt to this creature, prevent that damage." |
| *Excruciator* | "Damage that would be dealt by this creature can't be prevented." |

The phrase takes a determiner, a premodifier, a `would` relative and a demonstrative, and it is
the subject of *Excruciator*'s passive. None of that can be written while it is a cut and a
clause (spec §6).

## Open questions

- **The cuts the spec does not place.** It reads "all" as every such damage and "the next N" as
  a shield [CR#615.7]. Nine calls, eight in canon cards and one in `absorb`, take the other
  three: `ofThatDamage(n)` (*Thunderstaff*'s "prevent 1 of that damage", *Sphere of Law*'s
  "prevent 2 of that damage"), `allBut(n)` (*Forcefield*'s "prevent all but 1 of that damage")
  and `half(rounding)` (*Dark Sphere*'s "prevent half that damage, rounded down"). Each prints a
  part of "that damage". Recommendation: each is the noun phrase its line prints, a partitive or
  "half" over `that(damage)`, and `PreventCut` goes; the rounding stays `syl-round-each-time`'s.
- **The additional effect, `also`** [CR#615.5], which the spec does not place. Nine canon cards
  write it, and each prints it as ordinary text after the prevention: a coordinated verb phrase
  (*Jared Carthalion, True Heir*'s "prevent that damage and put that many +1/+1 counters on
  it"), or a following sentence (*Reverse Damage*'s "You gain life equal to the damage prevented
  this way.", *Temper*'s "For each 1 damage prevented this way, put a +1/+1 counter on that
  creature."). Recommendation: no parameter; the effect is written where the card prints it and
  reads the amount through `preventedThisWay`, and lowering takes it as the prevention's
  additional effect.
- **A prevention under "If … would" that prints no "instead"** (*Stormwild Capridor*). The
  sentence is `replace(event, instead(…))` today, and `instead` is the word for a printed
  "instead". Options: a `Replacement` Template over `instead` that prints no word, for a
  prevention; or `instead` printing nothing where its clause's verb is `prevent`. Recommendation:
  the Template, since "prevent" is itself what marks a prevention effect [CR#615.1a].

## Done when

- The open questions are answered, and each answer is in the doc comment of the declaration it
  shapes.
- `prevent(damage: NounPhrase)` exists and `preventDamage` is gone. A damage phrase with a
  `would` relative is what `prevent` reads as the prevention's event pattern, and the five cards
  in the table write their phrases as printed, each checked against its Oracle text;
  *Excruciator*'s line is a clause whose subject is the damage phrase, over
  `cant(be(done(prevent(subject))))` [CR#615.12].
- `preventAll`, `preventAllBy` and `preventNext`, or the Template `syl-spelling-optional-segments`
  merged them into, are gone, their callers writing `prevent` over the phrase.
- Every call in `plugins/` is rewritten by a one-off script, kept with an old-to-new table in
  `docs/evidence/`, each site checked against its Oracle line; the script checks that each
  rewritten call names the same cut, event pattern and additional effect before and after, and
  the landing reports the counts it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- `redirectDamage` and a clause whose subject carries the `would` relative:
  `syl-replacement-stated-as-clause`.
- One "be" over two participles sharing a phrase (*Cephalid Illusionist*'s "dealt to and dealt
  by target creature you control"): `syl-shared-phrase-written-once`.
- A line whose agent does not print last (*Healing Grace*'s "dealt to any target this turn by a
  source of your choice") and an object relative under `would` (*Harm's Way*): not designed
  (spec §11, §13).
- The relative-clause words: `syl-nominal-descriptions`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §3, §6, §8
- `plugins/builtin/primitives/action/preventDamage.syl`,
  `plugins/builtin/primitives/static/{theNext,allBut,half,ofThatDamage,allDamage}.syl`,
  `plugins/builtin/templates/action/{preventAll,preventAllBy,preventNext}.syl`,
  `plugins/builtin/templates/quantity/preventedThisWay.syl`,
  `plugins/builtin/definitions/keyword_ability/{absorb,protection}.syl`
- `plugins/canon/cards/{Glacial Chasm,Defang,Temper,Stormwild Capridor,Excruciator}.syl`
