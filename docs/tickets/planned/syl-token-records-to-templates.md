---
needs: [cleanup-step6-surface]
---

# Created tokens are written with token Templates, not raw records

## Why

Fourteen canon cards write a created token's characteristics as a record literal in a token slot
(`written({characteristics = {…}})`, or the `token` argument of `createTappedAttacking`). A
record written in place has no wording (`docs/decisions/spelling.md` §5), and where a Template's
expansion could also be written directly, the written form is the Template (§4). So each such
record is a second written form of a token a Template already spells, or a printed token no
Template spells yet.

A scan of every `{characteristics = …}` record under `plugins/` (script and output in
`docs/evidence/syl-token-records-to-templates/`) finds 49: 14 in a token slot, all on
Vintage-legal canon cards, and 35 outside one (30 `bundle`, 4 `become`, 1 `faceDownAs`). None of
the 14 is legitimately a record: each prints its characteristics.

Expressible today (10), with the call each becomes:

| Card | Call | Oracle |
|---|---|---|
| *Archfiend's Vessel* | `creatureToken(5, 5, [black], [demon], [flying])` | "a 5/5 black Demon creature token with flying" |
| *At Knifepoint* | `creatureToken(1, 1, [red], [mercenary], [activatedOnlyDuring(…)])` | "a 1/1 red Mercenary creature token with "{T}: Target creature you control gets +1/+0 until end of turn. Activate only as a sorcery."" |
| *Aya of Alexandria* | `creatureToken(1, 1, [black], [assassin], [menace])` | "a 1/1 black Assassin creature token with menace" |
| *Chrome Replicator* | `artifactCreatureToken(4, 4, [], [construct])` | "a 4/4 colorless Construct artifact creature token" |
| *Divine Visitation* | `creatureToken(4, 4, [white], [angel], [flying, vigilance])` | "that many 4/4 white Angel creature tokens with flying and vigilance are created instead" |
| *Garruk, the Veil-Cursed* | `creatureToken(1, 1, [black], [wolf], [deathtouch])` | "Create a 1/1 black Wolf creature token with deathtouch." |
| *Harried Spearguard* | `creatureToken(1, 1, [black], [rat], [static(cant(actor(thisToken), block))])` | "a 1/1 black Rat creature token with "This token can't block."" |
| *History of Benalia* | `creatureToken(2, 2, [white], [knight], [vigilance])` | "a 2/2 white Knight creature token with vigilance" |
| *Saheeli, Filigree Master* | `artifactCreatureToken(1, 1, [], [thopter], [flying])` | "two 1/1 colorless Thopter artifact creature tokens with flying" |
| *Seraphic Greatsword* | `createTappedAttacking(1, creatureToken(4, 4, [white], [angel], [flying]), that(player))` | "a 4/4 white Angel creature token with flying that's tapped and attacking that player" |

Needing a Template that does not exist (4):

- A named legendary token, printed "create <name>, a legendary …" (2): *Dark Depths* ("create
  Marit Lage, a legendary 20/20 black Avatar creature token with flying and indestructible") and
  *Tuktuk the Explorer* ("create Tuktuk the Returned, a legendary 5/5 colorless Goblin Golem
  artifact creature token"). No token Template takes a name or a supertype.
- An artifact creature token whose power and toughness are an Amount (1): *Phyrexian Rebirth*
  ("an X/X colorless Phyrexian Horror artifact creature token"). `artifactCreatureToken` takes
  `Int`; `creatureTokenOf` takes `Amount` but writes only the creature type.
- A creature token of chosen qualities (1): *Volrath's Laboratory* ("a 2/2 creature token of the
  chosen color and type"), written with a `qualities` field beside `characteristics`.

## Open questions

- Is a card's named token a Template taking the name, or a `token` Definition in the card's
  plugin? Recommendation: a Template, as `namedLegendaryCreatureToken(name, power, toughness,
  colors, subtypes, abilities)` with an artifact sibling. `token` Definitions are the tokens the
  rules define (`plugins/builtin/definitions/token/`); *Marit Lage* and *Tuktuk the Returned* are
  defined by one card's text each, and a Template adds no meaning.
- How does *Phyrexian Rebirth*'s X/X artifact token get its Template? `creatureToken` (`Int`) is
  `creatureTokenOf` (`Amount`) over `intToAmount`, so the two are one expansion with one wording,
  the double written form §4 rules out; the split predates the Lean retirement. Recommendation:
  one Template per token shape, taking `Amount` (`creatureToken`, `artifactCreatureToken`), with
  `creatureTokenOf` folded into `creatureToken` and its callers (*Kalitas, Bloodchief of Ghet*,
  *Nissa, Ascended Animist*, `endure`) updated, rather than adding an `artifactCreatureTokenOf`
  twin.
- What does *Volrath's Laboratory*'s Template take: the chosen qualities as a parameter
  (`List(WrittenQuality)`), or fixed `ofChosen(color)` and `ofChosen(subtype(creature))` in its
  body, as the printed "of the chosen color and type" names both? Three Vintage-legal cards print
  "creature token of the chosen color and type": *Volrath's Laboratory* (2/2), *Riptide
  Replicator* (X/X) and *Sarpadian Empires, Vol. VII* (1/1, whose choice is one of five
  color-and-type pairs). Recommendation: fixed in the body, with `Amount` power and toughness, if
  *Sarpadian Empires*' paired choice can be read as a chosen color and a chosen creature type;
  otherwise the qualities are a parameter.

## Done when

- No raw token record remains in a token slot under `plugins/` where an existing Template spells
  it: the scan in `docs/evidence/syl-token-records-to-templates/` reports none expressible, and the
  ten cards above call the Template shown.
- Each missing Template is declared under `plugins/builtin/templates/noun/` with its card calling
  it (*Dark Depths* and *Tuktuk the Explorer*, *Phyrexian Rebirth*, *Volrath's Laboratory*), or
  stays listed here as an Open question.
- Each rewritten card is checked against its Oracle text; parameters stay in printed order
  (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

The 35 `{characteristics = …}` records outside a token slot (`bundle`, `become`, `faceDownAs`):
the scan lists them, and `syl-become-replaces-types` owns the four `become` callers. Predefined
tokens (`defined(…)`), which are `token` Definitions.

## Pointers

`docs/decisions/spelling.md` §4–5; `plugins/builtin/templates/noun/{creatureToken,
creatureTokenOf,artifactCreatureToken}.syl`; `plugins/builtin/primitives/noun/written.syl`;
`plugins/builtin/templates/action/{createTappedToken,createTappedAttacking}.syl`;
`plugins/builtin/coercions/intToAmount.syl`; `docs/evidence/syl-token-records-to-templates/`.
