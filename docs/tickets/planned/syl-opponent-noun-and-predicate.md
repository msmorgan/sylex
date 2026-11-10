---
needs: [cleanup-step6-surface]
---

# Tie each head noun to the Predicate of the same word

## Why

"Opponent" has two declarations that nothing ties together. The head noun `opponent` at
`NounWord` (`plugins/builtin/primitives/noun/opponent.syl`) is wording only and restricts
nothing. "An opponent", "each opponent", "your opponents" and "target opponent" go through the
Predicate `isOpponent` (`isOpponentOf(you)`) and the Templates `anOpponent`, `eachOpponent`,
`yourOpponents` and `target(isOpponent)`, never through the noun. So:

- `enchanted(opponent)` names an opponent only because a separate `enchant(isOpponent)` restricts
  the Aura (*Tenuous Truce*, *Psychic Possession*), and `that(opponent)` or `those(opponent)`
  would not require its referent to be an opponent. Both are now writable; 69 Vintage-legal cards
  print "that opponent" and 3 "those opponents".
- "That player" and "they" read an opponent: *Tenuous Truce*'s `that(player)` and `they` read
  `enchanted(opponent)`, *Oath of Kaya*'s `that(player)` reads `anOpponent`, whose term has no
  head noun at all, and *Black Vise*'s reads the chosen opponent. `semantics.md` §2.4 item 3
  admits a referent whose "Head Noun fit[s] the anaphor's carrier word", and nothing states that
  an opponent fits "player".
- Encore [CR#702.141a] prints "attacks that opponent", and `encore.syl` writes `that(player)`.

The split is general. `player` at `NounWord` and `isPlayer` at `Predicate` are two unrelated
primitives spelled "player"; so are `card` and `isCard`, `token` and `isToken`; `permanent` is a
bare primitive while `isPermanent` means `in(battlefield)`; `spell` is a bare primitive while
`isSpell` means `and([in(stack), not(abilityHead(anyOnStack))])`. A card type or subtype reaches
`NounWord` through a coercion (`type`, `subtype`) and `Predicate` through `hasType` or
`hasSubtype`, still two terms with nothing between them.

## Open questions

- **What ties a head noun to the objects it names?**
  - (a) Each head noun word entails its Predicate: `opponent` carries `isOpponent`, `player`
    `isPlayer`, `permanent` `isPermanent`, and so on, and every NounWord consumer (`that`,
    `those`, `enchanted`, `headed`, `theGrantor`) adds it to what the phrase denotes. "That
    opponent" then requires an opponent, and `enchant(isOpponent)` still states the Aura's
    restriction [CR#702.5a], which is the keyword's rather than the noun's.
  - (b) A stated fits relation between head nouns: `opponent` fits `player`, a type noun fits
    `permanent`, `card` or `spell` by zone ([CR#109.2]), and §2.4 item 3 reads it. The noun still
    restricts nothing, so `that(opponent)` keeps reading any player.
  - (c) One declaration carries both: a word is declared once and stands at both `NounWord` and
    `Predicate` with one meaning, so `opponent` and `isOpponent` are one word.
    `syl-type-predicate-coercions` meets the same split for card types and subtypes, and its
    option (c) weighs separate head-noun and modifier words.

  Recommendation: (a), with (b)'s fits relation stated in §2.4 item 3 for anaphors: (a) makes
  "that opponent" mean what it prints, and the fits relation is still needed where the carrier
  word ("that player", "they") is more general than the referent's head noun.

## Done when

- The open question is answered, and its answer is stated in `docs/decisions/semantics.md` §2.4
  and in the doc comments of `opponent.syl` and the other head nouns it covers.
- Encore writes "that opponent" as printed, and every canon and builtin "that opponent" or
  "those opponents" writes `opponent`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/noun/{opponent,player,permanent,spell,card,token}.syl`,
`plugins/builtin/primitives/predicate/{isPlayer,isCard,isToken,isOpponentOf}.syl`,
`plugins/builtin/templates/predicate/{isOpponent,isPermanent,isSpell}.syl`,
`plugins/builtin/templates/noun/`, `plugins/builtin/templates/pronoun/{that,those}.syl`,
`plugins/builtin/definitions/keyword_ability/encore.syl`; `docs/decisions/semantics.md` §2.4;
`syl-type-predicate-coercions`, `syl-card-head-noun-sweep`.
