---
needs: [cleanup-step6-surface, syl-nominal-descriptions, english-names-read-from-text]
specs: [voice-and-object-phrases, kind-sorts-and-position-unions]
---

# Created tokens are written as printed, not as raw records

## Why

Fourteen canon cards write a created token's characteristics as a record literal in a token slot
(`written({characteristics = {…}})`, or the `token` argument of `createTappedAttacking`). A
record written in place has no wording (`docs/design/spelling.md`, on records), so none of
these tokens round-trips. Each prints its characteristics:

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
omitted `colors` (*Chrome Replicator* against *Volrath's Laboratory*; a word printed for an
absent value has no notation in `spelling.md`). The scan of every `{characteristics = …}` record is
in `docs/evidence/syl-token-records-to-templates/`.

The verb that creates them spreads the same phrase over arguments: `create(count, token,
riders)` and `createObject(count, spec, under)` take the number apart from the description, so
the tokens are no noun phrase and cannot be a passive clause's subject. *Divine Visitation*'s "If
one or more creature tokens would be created under your control, that many 4/4 white Angel
creature tokens with flying and vigilance are created instead." and *Primal Vigor*'s "twice that
many of those tokens are created instead" each print them as one (spec §6).

## Design

- A created token's description is a `Nominal` whose head is `token`, with its size, colors,
  types and subtypes as premodifiers and its abilities as a postmodifier, built from
  `syl-nominal-descriptions`'s sorts and lowered to the token's characteristics. "A tapped 2/2
  black Zombie creature token" and "colorless" are premodifiers like any other.
- `create` and `createObject` take the tokens as one noun phrase, whose determiner is the count
  (spec §6).
- A token created by name with no other characteristics (*Tarmogoyf Nest*'s `"Create a Tarmogoyf
  token."`, *Mutable Explorer*'s `"create a tapped Mutavault token."`) takes the characteristics of
  the Oracle card of that name [CR#111.11]: `<OtherCardName>` premodifying *token(s)* in the Object
  of *create* opts in to `OtherCardName()` (`docs/design/lexicon.md` §6), refused where a predefined
  token or subtype spells the name, and the `.syl` value is that card's name (while a predefined
  token such as `"a Treasure token"` stays its `token` Definition).

## Open questions

- **An emblem under `createObject`.** `getEmblem` and `getNamedEmblem` expand to
  `createObject(1, emblem(…))`: "[Player] gets an emblem with [ability]" means that player puts
  such an emblem into the command zone [CR#114.2]. The spec gives `createObject` a tokens phrase
  and does not say what an emblem is under it. Options: `emblem` is a head at `Nominal` with its
  abilities a postmodifier, as cards print "an emblem with …", so `createObject` takes one noun
  phrase for either; or an emblem gets a primitive of its own. Recommendation: the head `emblem`.
- **A named token** ("create Marit Lage, a legendary 20/20 … token"; *Tuktuk the Explorer*; *There
  and Back Again*'s "Create Smaug, a legendary 6/6 red Dragon creature token …"; *Bonny Pall,
  Clearcutter*'s "create Beau, a legendary blue Ox creature token …"). The English grammar reads the
  name as the clause's object and the description as a supplement of the whole clause:
  `NounPhraseSupplement: Clause -> Clause ',' supplement:NounPhrase`
  (`crates/english/grammar/english.fug`), whose comment says Oracle uses it for a named token's
  description. *Dark Depths*'s Reading (`cargo xtask english parse`) is that: the supplement over
  the imperative clause "create Marit Lage", inside the fronted "If you do,". So the description is
  not inside the object noun phrase, and a wording "$name, $spec" at the object's Hole would print
  it before anything else the clause prints after its object. Options: a Template taking the name
  and the description, with the description a Tail on the clause (`%Clause: , $spec`, `spelling.md`
  on Tails); or a `token` Definition in the card's plugin. Recommendation: the Template with the
  Tail, since `token` Definitions are the tokens the rules define
  (`plugins/builtin/definitions/token/`) and a Template adds no meaning. The name sets the token's
  name in place of its subtypes plus "Token" [CR#111.4].
- **A token's ability that names the token.** Smaug's "When Smaug dies, create fourteen Treasure
  tokens." and Beau's "Beau's power and toughness are each equal to the number of lands you
  control." name the token their ability is on, which means that object [CR#201.5].
  `syl-self-name-references`'s name forms name the enclosing face, which here is the card that
  creates the token, not the token. Recommendation: settle it in `syl-self-name-references`,
  which lists the case among its open questions; this ticket writes these cards after it.
- **Chosen qualities** (*Volrath's Laboratory*, *Riptide Replicator*, *Sarpadian Empires, Vol.
  VII*, which print "creature token of the chosen color and type"). Recommendation: a
  postmodifier "of the chosen color and type".

## Done when

- The open questions are answered, and `spelling.md`'s token example is one that exists.
- `create` and `createObject` take their tokens as one noun phrase, and no call writes a count
  beside a description. *Divine Visitation*'s and *Primal Vigor*'s replacements are passive
  clauses whose subjects are the tokens phrases they print, over `done(create(subject))`, each
  checked against its Oracle text.
- *There and Back Again* and *Bonny Pall, Clearcutter* are in `plugins/canon/cards/`, their named
  tokens written as printed, each checked against its Oracle text.
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

`docs/design/spelling.md` (the surface term as the derivation, records, the notation); `plugins/builtin/templates/noun/{creatureToken,
creatureTokenOf,artifactCreatureToken}.syl`; `plugins/builtin/primitives/noun/written.syl`;
`plugins/builtin/templates/action/{createTappedToken,createTappedAttacking}.syl`;
`docs/evidence/syl-token-records-to-templates/`.
