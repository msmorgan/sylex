---
needs: [cleanup-step6-surface]
---

# A copy's ", except …" is written where and as it is printed

## Why

A copy effect's exceptions ([CR#707.9]) print as one comma-set clause after the clause that makes
the copy: "…, except it's 7/7" (*Quicksilver Gargantuan*). The English grammar anchors such a
post-adjunct to the finite Clause, never to a verb phrase or a noun phrase
(`crates/english/grammar/english.fug`, the comment on the comma-set post-adjunct: "*, except it has flying*"). 205
Vintage-legal cards print "copy of …, except".

The declarations take the exceptions beside the copied object instead: `copyOf(source,
exceptions)` (TokenSpec), `asCopyOf(source, exceptions)` (MoveRider), `copyChange` and
`copyOnStack`, each a `List(CopyExcept)`. Two things follow:

- Words printed between the copied object and ", except" have no place before the exceptions.
  *Shredder, Shadow Master* prints "create a token that's a copy of Shredder tapped and attacking
  that player, except it isn't legendary".
- Each `CopyExcept` prints its own "except": `alsoHas` "except it has $abilities", `isntLegendary`
  "except it isn't legendary", `hasPT`, `isColor`, `isNamed` and `exceptThisAbility` alike. So two
  exceptions print "except" twice and always say "it". *Irenicus's Vile Duplication* prints
  "except the token has flying and it isn't legendary" and writes `exceptions = [alsoHas([flying]),
  isntLegendary]`. *Saw in Half* ("except their power is half that creature's power and their
  toughness is half that creature's toughness") and *Donal, Herald of Wings* ("except the copy is
  a 1/1 Spirit in addition to its other types") use the unspelled `characteristicEdits`.

## Open questions

- **What the exception is.** Options: (a) one optional clause of the copy, the coordination of
  exception clauses in their printed words, each with the subject the card prints ("it", "the
  token", "the copy", "their power"), and "except" printed once by the copy's declaration; (b) the
  list of `CopyExcept` items kept, with "except" moved to the copy's declaration and each item's
  wording taking its printed subject. Either lowers to the copy's exceptions, part of its copiable
  values [CR#707.9b], never to a continuous effect. Recommendation: (b), which keeps each
  exception a value of the closed `CopyExcept` sort that lowering already reads, and prints the
  subjects and the "and" the cards print.
- **Where it attaches.** It prints at the end of the clause that makes the copy, after whatever
  else that clause prints. Whether spelling keeps general one-sided adjunction is not settled:
  - With it, a call that takes the copying clause as its host and adjoins ", except …" at the
    clause's right edge, or a Tail.
  - Without it, a Tail on the copy's declaration (`<Clause: , except …>`, `spelling.md` §3.2),
    the clause being away from the copied object's Hole.

  Recommendation: the Tail, which serves in both branches and keeps the exceptions an argument of
  the copy.

## Done when

- The open questions are answered and written in `docs/decisions/spelling.md` §3.2 or the
  declarations' doc comments.
- Every copy in `plugins/` writes its exceptions as printed: one "except", the printed subjects,
  and the printed coordinator, checked against Oracle text; `characteristicEdits` is called only
  where no spelled exception says it, and each such site keeps a `TODO(spelling)` naming the
  wording.
- *Shredder, Shadow Master* is in `plugins/canon/cards/`, its exception after "tapped and
  attacking that player".
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale.

## Not in scope

- Characteristic-setting outside a copy ("becomes a 3/3 Angel creature"): `syl-become-replaces-types`.

## Pointers

`plugins/builtin/primitives/noun/copyOf.syl`, `plugins/builtin/primitives/zone/asCopyOf.syl`,
`plugins/builtin/primitives/static/{copyChange,alsoHas,characteristicEdits,exceptThisAbility,
withEntryCounters}.syl`, `plugins/builtin/templates/action/copyOnStack.syl`,
`plugins/builtin/templates/ability/{isntLegendary,hasPT,isColor,isNamed,isAlso}.syl`;
`plugins/canon/cards/{Irenicus's Vile Duplication,Saw in Half,Donal, Herald of Wings,Quicksilver
Gargantuan}.syl`.
