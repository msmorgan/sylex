---
needs: [cleanup-step6-spine, syl-spelling-optional-segments]
---

# ", where X is …" is written as a supplement of what it follows

## Why

A printed ", where X is …" defines a letter for the clause before it. The English grammar
anchors it as a comma-set Supplement to a finite Clause, to a non-final verb-phrase conjunct under
a shared subject, or to a keyword line's item (the comma-set Supplement adjuncts of `crates/english/grammar/english.fug`). The corpus writes it as a sibling of what it supplements instead:

- In an effect, as the Action `define` ("$letter is $amount", over
  `establish(letterDefinition(…))`), a step of a `sequentially`: 19 canon files.
  *Celebrate the Harvest* prints "Search your library for up to X basic land cards, where X is
  the number of different powers among creatures you control. Put those cards onto the
  battlefield tapped, then shuffle." and writes `define(x, …)` as the second of four steps, so
  the term reads as a step that a sequence performs and the supplement's attachment to the search
  is lost.
- In a static ability, as the StaticSpec `letterDefinition` beside the spec it supplements in a
  `conjunction`: 9 canon files. *Death's Shadow* ("This creature gets -X/-X, where X is your life
  total.") writes `conjunction([getsPt(…), letterDefinition(x, lifeTotalOf(you))])`, and *Phyrexian
  Ingester* writes two `letterDefinition`s for "where X is … and Y is …". *Aspect of Wolf*
  ("Enchanted creature gets +X/+Y, where X is half the number of Forests you control, rounded
  down, and Y is half the number of Forests you control, rounded up.") has the same shape.

Neither form has a wording that prints ", where", and `syl-sequentially-wordings` would otherwise
spell a `define` step as a sequenced instruction.

## Open questions

- **The declaration's shape.** Options: one Template per host sort (Action, StaticSpec, and the
  keyword-line item where a keyword prints "X, where X is …") that takes the host and its letter
  definitions, worded "$host, where $definitions<and>"; one declaration generic over its host's
  sort, which `syl.md` has no form for; or the definitions as an optional parameter of each host
  declaration. Recommendation: one Template per host sort, added as a card needs it. Whether
  spelling keeps general one-sided adjunction does not change this: the supplement stands at its
  host's right edge, so the host is a Hole and the wording is substitution, and a host printed
  away from the supplement's anchor would be a Tail.
- **A supplement on a non-final conjunct** ("searches their library for up to X basic land cards,
  where X is …, puts them onto the battlefield tapped, then shuffles"): the supplement's host is
  that conjunct's verb phrase. Recommendation: the Template at Action takes the conjunct as its
  host, inside the sequence.
- **Names.** Recommendation: named for the wording; the owner names it.

## Done when

- The open questions are answered and written in `docs/decisions/spelling.md` or the declarations'
  doc comments.
- No `define` or `letterDefinition` in `plugins/` stands as a sibling step or conjunct of what it
  supplements; every site is rewritten by a script, with the script and an old-to-new table in
  `docs/evidence/`, each checked against Oracle text, and the script checks that each letter has
  the same definition before and after.
- `define` is deleted if nothing calls it.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- ", rounded up" and "Round up each time.": `half` and `syl-round-each-time`.

## Pointers

`plugins/builtin/templates/action/define.syl`,
`plugins/builtin/primitives/static/letterDefinition.syl`,
`plugins/canon/cards/{Celebrate the Harvest,Death's Shadow,Phyrexian Ingester}.syl`;
`crates/english/grammar/english.fug` (the comma-set Supplement adjuncts); `docs/decisions/spelling.md`
§2, §3.2.
