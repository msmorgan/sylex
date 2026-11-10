# `confers` and `rules`: what one holding gives and what is enabled once

A declaration that carries rules states each one in one of two fields, and the field says how
many times the rule is enabled. Top-level rules rows can be named, so that card text can name a
rule the way it names a keyword.

The `.syl` below is the shape of the design. It has not been run through `cargo xtask dsl check`;
every word it uses exists at the sort it is used at unless it is marked NEW.

## 1. The two fields

- **`confers`** is what one holding gives: one counter, one ability instance, or, for a card type,
  subtype, supertype or designation, the one time its holder has it. A `confers` entry is enabled
  once per holding, and `this` in it is the holder. Two +1/+1 counters give +1/+1 twice
  [CR#122.1a]; a creature with two exalted counters has two instances of exalted [CR#122.1b] (the
  *Emissary of Soulfire* ruling: "A creature with multiple exalted counters will have that many
  instances of exalted"). A type is held once, so for a type "per holding" is "per holder".
- **`rules`** (NEW field) is a list of `{scope, confer}` records, the fields of a `conferral`
  row (`ConferralRule`). Each record is enabled once per entity that matches its `scope`, however
  many holdings that entity has, and `this` in it is that entity. It goes on `card_type`,
  `subtype`, `supertype`, `counter`, `designation` and `ability`.
- A top-level rules row (`sba`, `conferral`) is already enabled once per `scope` match; a `rules`
  record is the same thing written on the declaration whose rule it is.

## 2. What exists today

Every `confers` entry and every row is correct under these readings; what is missing is a way to
write a once-per-entity rule on a counter or a keyword.

| Where | Enabled |
|---|---|
| `card_type` `confers`: `artifact`, `battle`, `creature`, `enchantment`, `instant`, `land`, `planeswalker`, `sorcery` (`kindred` has none) | per holder |
| `subtype` `confers`: `aura`, `equipment`, `fortification`, `saga`, `siege` | per holder |
| `supertype` `confers`: `legendary`, `world` (`basic` and `snow` have none) | per holder |
| `designation` `confers`: `monarch`, `initiative` | per holder |
| `counter` `confers`: `p1p1Counter`, `m1m1Counter`, `flyingCounter` | per counter |
| `sba deathtouchDestruction` (`scope = isCreature`) | per creature |
| `sba legendRule` (`scope = isPlayer`) | per player |
| `conferral phasingTurnBased` (`scope = isPermanent`) | per permanent |
| `conferral playerDamageLifeLoss` (`scope = isPlayer`) | per player |
| `conferral toxicPoison` (`scope = and([isCreature, hasKeyword(theKeyword(toxic))])`) | per creature with toxic |
| `conferral foretoldCardLook`, `conferral plottedCardCast` (foretold or plotted cards in exile) | per such card |

A counter's `confers` can only be per counter, so a rule the CR gives "one or more" counters
cannot be written on its counter: it would be enabled once per counter. Shield, stun and finality
counters create a single effect for one or more counters [CR#122.1c,122.1d,122.1h], one or more rad
counters cause one trigger [CR#122.1i], the +1/+1 and -1/-1 annihilation removes N of each once
[CR#704.5q], and ten or more poison counters lose the game once [CR#122.1f,704.5c].

## 3. Why a field, not a flag on the declaration

One declaration can carry both multiplicities. `p1p1Counter` gives +1/+1 per counter (`confers`)
and is annihilated against -1/-1 counters once per permanent [CR#704.5q] (`rules`, with
`scope = and([isPermanent, hasCounter(p1p1Counter), hasCounter(m1m1Counter)])`). A flag on the
declaration would make both per counter or both once.

Counter rules are therefore split this way:

- In `confers`, per counter: the +1/+1 and -1/-1 boosts [CR#122.1a], hone [CR#122.1j], and every
  keyword counter's keyword [CR#122.1b].
- In `rules`, with a `hasCounter(<kind>)` scope: shield, stun, finality and rad
  [CR#122.1c,122.1d,122.1h,122.1i], the annihilation [CR#704.5q] and the ten-poison loss
  [CR#122.1f,704.5c].

## 4. Named rows

A top-level rules row takes `lexical` and `spelling` and is a catalog declaration, so a card can
name it; `legendRule` is one. "Legend rule" is printed vocabulary: 10 Vintage-legal cards print
"the "legend rule" doesn't apply" (*Brothers Yamazaki*, *Cadric, Soul Kindler*, *Council of Reeds*,
*Hall of Echoes*, *Mirror Box*, *Mirror Gallery*, *Sakashima of a Thousand Faces*, *Sliver
Gravemother*, *Spider-Verse*, *The Master, Multiplied*).

## 5. Exempting objects from a named rule

"The "legend rule" doesn't apply to tokens you control" (*Cadric, Soul Kindler*) is written with
a NEW `doesntApply(rule, to)`: `rule` names a row, and `to` is a `Predicate` over the objects it
exempts. Exempt objects are outside the rule: neither counted nor choosable. The *Cadric* rulings
say so: with a token and one nontoken legendary permanent of a name, "the legend rule will not
cause you to put either one into your graveyard"; with more nontoken ones, "the legend rule
applies to only the nontoken permanents … You may not choose one of the token permanents and you
may not put any of the tokens into your graveyard this way." No card in the corpus prints it yet;
the ticket that writes the first such card adds `doesntApply`.

## 6. The Role rule

The Role rule [CR#704.5z,303.7a] needs no `rules`: like the world rule on `world`, it is an
ordinary per-holder `stateBased` entry, on `role`'s `confers`. A Role is put into its owner's
graveyard if another Role controlled by the same player is attached to the same permanent and has
a more recent timestamp. It reads a NEW measure, `timestampOf(<object>)`.

## 7. A keyword's number

An `ability` block takes a NEW field `number`: the value its keyword's number projects, which
`keywordNumber(<keyword>)` reads. Toxic writes `number = $amount`, so `keywordNumber(toxic)`
reads each instance's N and `totalToxicValue` sums them [CR#702.164b]; *Goliath Hatchery* reads
"its total toxic value" from outside the keyword. Toxic's rule moves from `conferral toxicPoison`
into `toxic.syl` under `rules`, enabled once per creature with toxic, so a creature with toxic 2
and toxic 1 gives three poison counters once [CR#702.164c]. Its scope names `toxic` inside
`toxic`'s own block:

    ability toxic(amount: Amount) {
      lexical = {lemma = "toxic", parameters = [[Amount]]},
      spelling = "toxic <amount>",
      number = $amount,                      # NEW field
      body = [],
      rules = [{                             # NEW field
        scope = and([isCreature, hasKeyword(theKeyword(toxic))]),
        confer = property(
          asEvent(
            does(actor(this), dealDamage(recipient = a(isPlayer), kind = combat)),
            putCounters(totalToxicValue(this), printedKind(poison), that(player))))}]
    }

## 8. No parameter reads inside `rules`

`cargo xtask dsl check` refuses a `$name` read inside a `rules` record. A record is enabled once
for every holding at once, so there is no single instance whose argument it could read; a value
it needs from the instances is read through what each instance publishes, as toxic's `number`.

## 9. What stays out

- The Two-Headed Giant team poison threshold [CR#704.6b]: whether it is written with the
  ten-counter loss is open on `syl-counter-conferrals`.
- Goad's per-goader storage [CR#701.15c]: one goaded instance per goader is the engine's
  (`designation-storage-axes`), not a multiplicity of `confers` or `rules`.
- `prepared`'s exile copy [CR#722.3c]: whether it is a conferral or a row is open on
  `syl-designation-conferrals`.
- Companion's pregame timing word: the word for "before the game begins" is
  `syl-companion-pregame-rule`'s; that its row is scoped to players is not.

## 10. New words

- Fields: `rules` on `card_type`, `subtype`, `supertype`, `counter`, `designation` and `ability`;
  `number` on `ability`.
- `timestampOf`, a measure.
- `doesntApply`.
