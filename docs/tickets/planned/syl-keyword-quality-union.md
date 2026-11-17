---
needs: [syl-union-membership]
specs: [kind-sorts-and-position-unions]
---

# What a keyword's quality parameter is typed by

## Why

Thirteen keywords declare their parameter `quality: Predicate` or `subject: Predicate`
(`protection`, `landwalk`, `affinity`, `enchant`, `hexproofFrom`, `champion`, `devourQuality`,
`emergeFrom`, `equipQuality`, `offering`, `partnerWith`, `splice`, `typecycling`), and their calls
write the quality with `Predicate` copies of words declared elsewhere (`hasType`, `hasColor`,
`in`, `youControl`, `named`, …). `Predicate` names no kind of phrase. The parser reads a quality
as one of three kinds: a nominal ("Enchant creature card in a graveyard", *Animate Dead*), a noun
phrase ("protection from black", *White Knight*, where the color is a noun; "protection from each
color among permanents you control", *Pledge of Loyalty*), or, in 17 units, an adjective phrase
("multicolored", "colorless", "snow").

## Open questions

- **The sort.** One sort whose members are `Nominal`, `NounPhrase` and `AdjectivePhrase` matches
  the parse. `and`, `or`, `andOr` and `coord` are declared at both `Nominal` and `NounPhrase`,
  which would collide in its Reaches, so it needs coordinators of its own or one of the two kinds
  to stand through the other.
- **A color as a quality.** A color word is a member of `AdjectivePhrase`, and "protection from
  black" prints it as a noun. Either the color word also stands as a noun there, by a route that
  does not give `red` two readings before a head, or the term writes the adjective and recovery
  maps the noun to it.
- **What is left of `Predicate`.** Its unspelled tests (`compare`, `currentFace`, `hasPossessor`,
  `predicateDid`) duplicate the ones at `Test`, and its spelled copies of phrases (`in`, `named`,
  `youControl`, `hasType`, …) duplicate the kind declarations. Each goes when its last caller
  takes the kind's or `Test`'s.
- **A quality inside a description.** 23 rule bodies write `pre = [hasQuality($quality)]`
  (`protection`, `landwalk`, `affinity`, `champion`, `splice`, `typecycling`, `behold`,
  `devourEntry`, `splitSecond` and others). A noun-phrase quality is no premodifier, so each body
  needs the form its rule prints ("a creature with landwalk can't be blocked as long as the
  defending player controls a land of that type").
- **The other consumers of `Predicate`**: `behold`, `transform`'s and `turnOver`'s `into`,
  `resulting` on `cast`, `activate`, `play` and `procedure`, `except` on `allExcept` and its kin,
  `primitiveKeyword`'s `quality`, `holders` on `keywordGrant`, `oneEachOf`, `matching`,
  `nameOfCard`, `players`, `heldFor`, `aCharacteristic`, `withQuality`, `chosenQuality`,
  `bandsWithOther`, `spendOnlyToCastAnd`. Which of them are qualities, and what the rest take.

## Done when

- The open questions are answered with the parses that decide them, and each answer is in the doc
  comment of the declaration it shapes.
- No keyword parameter is a `Predicate`, every call writes what its card or rule prints, and
  `hasQuality` is gone.
- The landing lists what is left at `Predicate` and what consumes it.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §4; `plugins/builtin/definitions/keyword_ability/`;
`plugins/builtin/{primitives,templates}/predicate/`;
`cargo xtask english forestwalk --card "White Knight"`.
