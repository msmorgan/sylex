---
needs: [cleanup-step6-surface]
---

# An anaphor's carrier word fits the head noun of what it reads

## Why

`syl-nominal-descriptions` merges the head nouns at `NounWord` into `Nominal`, where each head word
means what today's description word of that name means: `opponent` what `isOpponent`
(`isOpponentOf(you)`) means, `player` what `isPlayer` means, `permanent` what `isPermanent` means.
So "that opponent" (`that(opponent)`) requires an opponent, which `that(opponent)` over today's
wording-only `NounWord` does not. Two things are left:

- "That player" and "they" read an opponent: *Tenuous Truce*'s `that(player)` and `they` read
  `enchanted(opponent)`, *Oath of Kaya*'s `that(player)` reads `anOpponent`, and *Black Vise*'s
  reads the chosen opponent. `semantics.md` §2.4 item 3 admits a referent whose "Head Noun fit[s]
  the anaphor's carrier word", and nothing states that an opponent fits "player", or which other
  head nouns fit which (a type noun fits "permanent", "card" or "spell" by zone [CR#109.2]).
- Encore [CR#702.141a] prints "attacks that opponent", and `encore.syl` writes `that(player)`.
  69 Vintage-legal cards print "that opponent" and 3 "those opponents".

## Open questions

- **Where the fits relation is stated.** Options: a table of head nouns in `semantics.md` §2.4
  item 3, read by reference resolution; or each head word's declaration names the more general
  heads it fits. Recommendation: the declarations, so a head word added later states its fits
  where it is declared, with §2.4 item 3 saying the relation is read from them.

## Done when

- The open question is answered, and its answer is stated in `docs/decisions/semantics.md` §2.4
  and in the doc comments of `opponent` and the other head nouns it covers.
- Encore writes "that opponent" as printed, and every canon and builtin "that opponent" or
  "those opponents" writes `opponent`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/noun/{opponent,player,permanent,spell,card,token}.syl`,
`plugins/builtin/templates/pronoun/{that,those}.syl`,
`plugins/builtin/definitions/keyword_ability/encore.syl`; `docs/decisions/semantics.md` §2.4;
`syl-nominal-descriptions`, `syl-card-head-noun-sweep`.
