---
needs: [cleanup-step6-surface]
---

# A copy's ", except …" is written where and as it is printed

## Why

A copy effect's exceptions ([CR#707.9]) print as one comma-set clause after the clause that makes
the copy: "…, except it's 7/7" (*Quicksilver Gargantuan*). The English grammar anchors such a
post-adjunct to a finite Clause, never to a verb phrase, a noun phrase or a relative's Clause
(`crates/english/grammar/english.fug`, the comment on the comma-set post-adjunct). 205
Vintage-legal cards print "copy of …, except". The Readings (`cargo xtask english parse`) bear it
out: in "If you do, create a token that's a copy of target creature, except it isn't legendary."
", except it isn't legendary" adjoins the imperative clause "create a token that's a copy of
target creature", inside the fronted "If you do,", and not the relative clause "that's a copy of
target creature" that holds the copied object. Its complement is a full finite clause, unmarked
("except he has hexproof", *Oko, the Ringleader*, after "until end of turn") or marked by "that"
(*Fork*: "Copy target instant or sorcery spell, except that the copy is red.").

The declarations take the exceptions beside the copied object instead: `copyOf(source,
exceptions)` (TokenSpec), `asCopyOf(source, exceptions)` (MoveRider), `copy(sort, subject, times,
exceptions)` (Action), `copyChange` and `copyOnStack`, each a `List(CopyExcept)`. Two things
follow:

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

## Design

- The exceptions print at the end of the clause that makes the copy, after whatever else that
  clause prints, so the copy's declaration prints them as a Tail on that clause
  (`<Clause: , except …>`, `spelling.md` on Tails), and they stay an argument of the copy.
- "Except" and "except that" are two printed forms of one meaning, so two Tails, each its own
  Template wording.

## Open questions

- **What the exception is.** Options: (a) one optional clause of the copy, the coordination of
  exception clauses in their printed words, each with the subject the card prints ("it", "the
  token", "the copy", "their power"), and "except" printed once by the copy's declaration; (b) the
  list of `CopyExcept` items kept, with "except" moved to the copy's declaration and each item's
  wording taking its printed subject. Either lowers to the copy's exceptions, part of its copiable
  values [CR#707.9b], never to a continuous effect. Recommendation: (b), which keeps each
  exception a value of the closed `CopyExcept` sort that lowering already reads, and prints the
  subjects and the "and" the cards print.
  Under (b) every item's wording is a whole finite clause with its subject ("he has hexproof",
  "the copy is red", "their power is half that creature's power"), since that is what "except"
  takes in the Readings.
- **Which Clause the Tail's host is.** A Tail's host is the smallest phrase of its Category that
  contains the call's Hole (`spelling.md` on Tails). For a token copy that is the relative clause
  "that's a copy of target creature", where the grammar refuses ", except", and not the
  imperative clause the Reading gives, which also stands inside a fronted "If you do,". Options:
  a Tail Category that excludes a relative's Clause, so the host is the nearest clause that can
  carry the adjunct; or the exceptions taken by the declaration of the verb that makes the token
  ("create … , except …") rather than by `copyOf`. Recommendation: the first, the host being
  the nearest Clause the grammar lets carry the adjunct, as its own anchoring rule says; the Tail
  notation in `spelling.md` states it.

## Done when

- The open questions are answered and written in the declarations' doc comments.
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
`plugins/builtin/primitives/action/copy.syl`,
`plugins/builtin/primitives/static/{copyChange,alsoHas,characteristicEdits,exceptThisAbility,
withEntryCounters}.syl`, `plugins/builtin/templates/action/copyOnStack.syl`,
`plugins/builtin/templates/ability/{isntLegendary,hasPT,isColor,isNamed,isAlso}.syl`;
`plugins/canon/cards/{Irenicus's Vile Duplication,Saw in Half,Donal, Herald of Wings,Quicksilver
Gargantuan,Fork}.syl`.
