---
needs: [cleanup-step6-surface, syl-nominal-descriptions]
---

# Created tokens are written as printed, not as raw records

## Why

Fourteen canon cards write a created token's characteristics as a record literal in a token slot
(`written({characteristics = {…}})`, or the `token` argument of `createTappedAttacking`). A
record written in place has no wording (`docs/decisions/spelling.md` §5), so none of these tokens
round-trips. Each prints its characteristics:

| Card | Oracle |
|---|---|
| *Archfiend's Vessel* | "a 5/5 black Demon creature token with flying" |
| *At Knifepoint* | "a 1/1 red Mercenary creature token with "{T}: Target creature you control gets +1/+0 until end of turn. Activate only as a sorcery."" |
| *Aya of Alexandria* | "a 1/1 black Assassin creature token with menace" |
| *Chrome Replicator* | "a 4/4 colorless Construct artifact creature token" |
| *Divine Visitation* | "that many 4/4 white Angel creature tokens with flying and vigilance are created instead" |
| *Garruk, the Veil-Cursed* | "Create a 1/1 black Wolf creature token with deathtouch." |
| *Harried Spearguard* | "a 1/1 black Rat creature token with "This token can't block."" |
| *History of Benalia* | "a 2/2 white Knight creature token with vigilance" |
| *Saheeli, Filigree Master* | "two 1/1 colorless Thopter artifact creature tokens with flying" |
| *Seraphic Greatsword* | "a 4/4 white Angel creature token with flying that's tapped and attacking that player" |
| *Dark Depths* | "create Marit Lage, a legendary 20/20 black Avatar creature token with flying and indestructible" |
| *Tuktuk the Explorer* | "create Tuktuk the Returned, a legendary 5/5 colorless Goblin Golem artifact creature token" |
| *Phyrexian Rebirth* | "an X/X colorless Phyrexian Horror artifact creature token" |
| *Volrath's Laboratory* | "a 2/2 creature token of the chosen color and type" |

The fixed-slot token Templates (`creatureToken`, `creatureTokenOf`, `artifactCreatureToken`)
spell the first ten in part and none of the last four: no slot takes a name, a supertype, an
`Amount` power and toughness beside the artifact type, or chosen qualities. They also cannot say
what a nominal can: a premodifier printed inside the token's description ("create a tapped 2/2
…"; `createTappedToken` is spelled "create <count> tapped <token>", which prints "tapped" before
the article), and "colorless" printed against no color word printed, which both write as an
omitted `colors` (*Chrome Replicator* against *Volrath's Laboratory*; `spelling.md` §7 lists a
word for an absent value as a known limit). The scan of every `{characteristics = …}` record is
in `docs/evidence/syl-token-records-to-templates/`.

## Open questions

- **How a created token is written.** Options:
  - (a) As a nominal: the token's description is a `Nominal` whose head is `token`, with its
    size, colors, types and subtypes as premodifiers and its abilities as a postmodifier, built
    from `syl-nominal-descriptions`'s sorts, and lowered to the token's characteristics. "A tapped
    2/2 black Zombie creature token" and "colorless" are premodifiers like any other.
  - (b) Fixed-slot Templates, one per token shape, with a new Template for each shape no slot
    covers (named legendary, `Amount`-sized artifact, chosen qualities), as `spelling.md` §5 names
    `creatureToken` the Template for *Archfiend's Vessel*'s token.

  Recommendation: (a). It writes every printed token in the table with no new shape, and (b)
  adds a Template for every shape the corpus has not met yet. Under (a), `spelling.md` §5's
  `creatureToken` example is rewritten.
- **A named token** ("create Marit Lage, a legendary 20/20 … token"): the name is the created
  object and the description stands beside it, set off by a comma. Options: a Template taking the
  name and the description; or a `token` Definition in the card's plugin. Recommendation: a
  Template, since `token` Definitions are the tokens the rules define
  (`plugins/builtin/definitions/token/`) and a Template adds no meaning.
- **Chosen qualities** (*Volrath's Laboratory*, *Riptide Replicator*, *Sarpadian Empires, Vol.
  VII*, which print "creature token of the chosen color and type"). Recommendation: under (a), a
  postmodifier "of the chosen color and type"; under (b), a Template with fixed `ofChosen(color)`
  and `ofChosen(subtype(creature))`.

## Done when

- The open questions are answered, and `spelling.md` §5 gives an example that exists.
- No raw `{characteristics = …}` record remains in a token slot under `plugins/`; the fourteen
  cards above write their tokens as printed, each checked against its Oracle text, with
  parameters in printed order (`syl.md` §5).
- The token Templates the answer retires are deleted, with every caller (*Kalitas, Bloodchief of
  Ghet*, *Nissa, Ascended Animist*, `endure` and the cards above) rewritten.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The 34 canon records outside a token slot (30 `bundle` in 27 files, 3 `become`, 1
  `faceDownAs`) and `amass.syl`'s `become`: `syl-become-replaces-types`.
- Predefined tokens (`defined(…)`), which are `token` Definitions.

## Pointers

`docs/decisions/spelling.md` §4, §5, §7; `plugins/builtin/templates/noun/{creatureToken,
creatureTokenOf,artifactCreatureToken}.syl`; `plugins/builtin/primitives/noun/written.syl`;
`plugins/builtin/templates/action/{createTappedToken,createTappedAttacking}.syl`;
`docs/evidence/syl-token-records-to-templates/`.
