---
needs: [syl-union-membership]
specs: [kind-sorts-and-position-unions]
---

# A keyword's quality parameter is a `Quality`

## Why

Thirteen keywords declare their parameter `quality: Predicate` or `subject: Predicate`
(`protection`, `landwalk`, `affinity`, `enchant`, `hexproofFrom`, `champion`, `devourQuality`,
`emergeFrom`, `equipQuality`, `offering`, `partnerWith`, `splice`, `typecycling`), and their calls
write the quality with `Predicate` copies of words declared elsewhere (`hasType`, `hasColor`,
`in`, `youControl`, `named`, …). The cards print a nominal ("Enchant creature card in a
graveyard", *Animate Dead*), an adjective phrase ("protection from red") or a noun phrase
("protection from each color among permanents you control", *Pledge of Loyalty*), which is the
union `Quality`.

## Done when

- Each of the thirteen parameters is a `Quality`, and every call writes the nominal, adjective
  phrase or noun phrase its card or rule prints, checked against the unit's parse.
- No declaration the spec's §5 lists under "Keyword qualities" remains at `Predicate`, and the
  `Predicate` coordinators go if nothing else calls them; the landing lists what is left at
  `Predicate` and what consumes it.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §2, §5;
`plugins/builtin/definitions/keyword_ability/`;
`plugins/builtin/{primitives,templates}/predicate/`.
