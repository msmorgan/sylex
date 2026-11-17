---
needs: [cleanup-step6-spine, syl-nominal-descriptions, syl-kind-sorts-and-position-unions]
specs: [voice-and-object-phrases]
---

# Damage and life are noun phrases

## Why

`dealDamage(amount, recipient, kind)`, `gainLife(amount)` and `loseLife(amount)` take a scalar
beside a noun fixed in their wording, so the phrase a card prints there is not in the term. The
cards print it as a noun phrase like any other:

| Card | Oracle | Today |
|---|---|---|
| *Phytohydra* | "If damage would be dealt to this creature, …" | `would(any, dealDamage(recipient = this(creature)))` |
| *Stormwild Capridor* | "If noncombat damage would be dealt to this creature, prevent that damage." | `dealDamage(recipient = this(creature), kind = noncombat)` |
| *Grollub* | "each opponent gains that much life" | `gainLife(thatMuch)` |
| *Aggressive Instinct* | "Target creature you control deals damage equal to its power to target creature you don't control." | `dealDamageOwnPower(target(…), target(…))` |

A scalar cannot stand as a clause's subject, take a premodifier, a relative clause or a
demonstrative, or be read back by "that damage", and the passive of "deal" has nothing to promote
until the damage is a phrase (spec §3, §6). `dealDamage` is written 186 times in
`plugins/canon/` and 25 in `plugins/builtin/`, `gainLife` 54 and 3, `loseLife` 25 and 8 (`grep`;
re-count when claimed).

## Open questions

- **The other declarations that spread a damage or life phrase.** The spec's table (§6) retypes
  `dealDamage`, `gainLife` and `loseLife`. Three more declarations fix the same nouns in their
  wording beside a scalar:
  - `payLife(amount)`, "pay $amount life" (12 canon calls). Its body passes the amount
    to `loseLife`, and cards print the same phrase after "pay" as after "lose": about 300
    Vintage-legal cards print "pay N life" and 15 "pay life equal to …" (a regular expression over
    Oracle text, reminder text removed).
  - `dealDivided(source, amount, among)`, "$source deals $amount damage divided as you choose
    among $among" (3 canon calls; 72 Vintage-legal cards print "damage divided"). The
    spec lists *Impact Resonance*'s "X damage divided as you choose among any number of target
    creatures" among the phrases of §6.
  - `markDamage(subject, amount)`, "$amount damage is marked on $subject" (3 builtin calls, in
    rules bodies no card prints).

  Recommendation: each takes the phrase. `payLife` takes the life phrase `loseLife` takes.
  `dealDivided` goes: its callers write `deal`, said of the source, over a damage phrase that
  carries the postmodifier "divided as you choose among $among", the phrase the spec lists for
  *Impact Resonance*. `markDamage` takes the damage phrase in place of its amount, and its
  wording stays the passive it fixes today until `syl-passive-participle-and-trigger-events`.
- **"That much damage plus 1"** (*Sulfuric Vapors*: "it deals that much damage plus 1 to that
  permanent or player instead"; 23 Vintage-legal cards print "that much damage plus"). The amount
  stands on both sides of the head, which no rule of §6 places; the card writes
  `dealDamage(amount = plus(thatMuch, 1), …)` today. Options: a Template at `NounPhrase` for the
  wording, with Holes on both sides of the head, as `syl-nominal-descriptions` recommends for an
  operator printed on both sides of a head; or a postmodifier "plus $n" on a nominal whose
  determiner is "that much". Recommendation: the Template, with *Sulfuric Vapors* as its caller.

## Done when

- The open questions are answered, and each answer is in the doc comment of the declaration it
  shapes.
- `deal(damage: NounPhrase, to: NounPhrase = any)` exists and `dealDamage` is gone; `gainLife` and
  `loseLife` each take one `life: NounPhrase`. No declaration takes a `DamageKind`, and the sort
  is gone: "combat" and "noncombat" are premodifiers of the head `damage`.
- `damage` and `life` are heads at `Nominal`, and `bare(damage)` is "damage" with no determiner.
- `measured`, the measuring determiner of a mass head, exists beside `counted` and prints its
  number `$n@Arabic`; *Agate-Blade Assassin*'s "defending player loses 1 life and you gain 1
  life" calls it.
- An amount printed after its head is a postmodifier of the nominal ("damage equal to its
  power", "life equal to …"), and no call writes a scalar amount beside the phrase.
  `dealDamageOwnPower` is deleted, and its four canon callers write the phrase with that
  postmodifier.
- Every call in `plugins/` is rewritten by a one-off script, kept with an old-to-new table in
  `docs/evidence/`. The script checks that each rewritten call carries the same amount, kind and
  recipient before and after, and the landing reports the counts it compared. A call that left
  its amount to the default `anyAmount` is read from its card's Oracle line, since the card
  prints either no determiner or one the term dropped; the landing lists those sites with the
  phrase each now writes.
- *Phytohydra*, *Stormwild Capridor*, *Grollub* and *Aggressive Instinct* write the phrases in
  the table above, each checked against its Oracle text.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The passives of `deal` and the Template `dealIndirect`:
  `syl-passive-participle-and-trigger-events`.
- `preventDamage` and `redirectDamage`: `syl-prevent-damage-phrase` and
  `syl-replacement-stated-as-clause`. This ticket rewrites the `dealDamage` calls inside the
  `prevent…` Templates and leaves their prevention as it is.
- Counters and tokens: `syl-counters-noun-phrase`, `syl-token-records-to-templates`.
- Whether such a phrase introduces a Referent that "that damage" and "that much" read: not
  decided (spec §6), and `syl-numeric-anaphors` owns the readbacks.
- Where "equal to …" prints away from its noun ("deals damage to any target equal to …"):
  `syl-tail-placement`.
- A glossary name for these phrases: not chosen (spec §13).

## Pointers

- `docs/specs/voice-and-object-phrases.md` §6
- `plugins/builtin/primitives/action/{dealDamage,changeLife,markDamage}.syl`,
  `plugins/builtin/templates/action/{gainLife,loseLife,payLife,dealDivided,dealDamageOwnPower}.syl`,
  `plugins/builtin/templates/determiner/{counted,bare}.syl`,
  `plugins/builtin/primitives/damageKind/`, `plugins/builtin/primitives/event/combat.syl`
- `docs/design/spelling.md` §7 (form labels)
