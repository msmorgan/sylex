# Unusual cards

Real cards whose printed Oracle text means something, or gets a rules treatment, that defeats a
natural simplifying assumption. An entry names an in-scope card (Vintage `Legal` or `Restricted`
in `data/mtgjson/AllPrintings.sqlite`, Un-set cards deferred), quotes its current Oracle text and
says what is unusual about it, citing the CR.

## Timing and clause order

### Fatal Push

> Destroy target creature if it has mana value 2 or less.
> Revolt — Destroy that creature if it has mana value 4 or less instead if a permanent left the
> battlefield under your control this turn.

Each paragraph is a separate ability [CR#113.2c], yet the second replaces the first ("instead")
rather than following it. Later text can modify the meaning of earlier text [CR#608.2c].

### Savage Punch

> Target creature you control fights target creature you don't control.
> Ferocious — The creature you control gets +2/+2 until end of turn before it fights if you control
> a creature with power 4 or greater.

The later paragraph's effect happens before the earlier paragraph's fight, so instructions do not
simply run in printed order [CR#113.2c,608.2c].

### Pox

> Each player loses a third of their life, then discards a third of the cards in their hand, then
> sacrifices a third of the creatures they control of their choice, then sacrifices a third of the
> lands they control of their choice. Round up each time.

One trailing sentence gives the rounding direction [CR#107.1a] for all four fractions in the
preceding "then" chain, each computed at its own step.

### Lightmine Field

> Whenever one or more creatures attack, this enchantment deals damage to each of those creatures
> equal to the number of attacking creatures.

The damage amount is not the size of the triggering group: it is counted when the effect applies
[CR#608.2h], so a creature removed from combat [CR#506.4] in the meantime is not counted.

### Quakebringer

> At the beginning of your upkeep, Quakebringer deals 2 damage to each opponent. This ability
> triggers only if Quakebringer is on the battlefield or if Quakebringer is in your graveyard and
> you control a Giant.

The condition is not an intervening "if", which must immediately follow the trigger condition and is
rechecked on resolution [CR#603.4]; "triggers only if" is checked only when the event occurs. The
same sentence states the zones the ability functions from [CR#113.6b].

### Xenosquirrels

> After you roll a die, you may remove a +1/+1 counter from this creature. If you do, increase or
> decrease the result by 1.

"After you roll a die" does not make a triggered ability, which uses "when", "whenever" or "at"
[CR#113.3c]. It is a modifier applied before the roll's result is final [CR#706.2,706.2a].

## Costs and actions

### Hotshot Mechanic

> This creature crews Vehicles as though its power were 2 greater.

The power change exists only for the crew cost's total-power sum [CR#702.122a,702.122b,609.4]:
crewing is something the creature does, though paying the cost is tapping it.

### Phyrexian Infiltrator

> {2}{U}{U}: Exchange control of this creature and target creature. (This effect lasts
> indefinitely.)

An exchange is not two gains of control: if the whole exchange can't be completed, no part of it
happens [CR#701.12a], and it does nothing if one player controls both permanents [CR#701.12b].

### Braided Net

> Craft with artifact {1}{U}

"Artifact" without "card" would ordinarily mean a permanent on the battlefield [CR#109.2]; in craft
materials it also matches an artifact card in a graveyard [CR#702.167a,702.167b].

## Effects that talk about themselves

### Guardian Beast

> As long as this creature is untapped, noncreature artifacts you control can't be enchanted, they
> have indestructible, and other players can't gain control of them. This effect doesn't remove
> Auras already attached to those artifacts.

The card's own text limits what its effect does: a sentence whose subject is "this effect" switches
off one consequence of it, the state-based action that puts an illegally attached Aura into the
graveyard [CR#704.5m]. The CR has no general rule for "this effect doesn't remove"; the nearest
analogues are the protection rules for Auras that say it [CR#702.16n,702.16p], and Guardian Beast
grants no protection.

### Titania's Song

> Each noncreature artifact loses all abilities and becomes an artifact creature with power and
> toughness each equal to its mana value. If this enchantment leaves the battlefield, this effect
> continues until end of turn.

A static ability's effect applies only while its source is on the battlefield [CR#611.3b]; this
card's text gives its effect a lifetime beyond its source.

### Leonin Arbiter

> Players can't search libraries. Any player may pay {2} for that player to ignore this effect until
> end of turn.

The effect carries its own per-player escape: paying is a special action [CR#116.2d], and it frees
only the player who paid. Damping Engine, Lost in Thought and Volrath's Curse use the same template
with other payments.

### Calming Licid

> {W}, {T}: This creature loses this ability and becomes an Aura enchantment with enchant creature.
> Attach it to target creature. You may pay {W} to end this effect.

The effect of a resolved ability names a payment that ends it; paying is a special action available
whenever the player has priority [CR#116.2c].

### Siren's Call

> At the beginning of the next end step, destroy all non-Wall creatures that player controls that
> didn't attack this turn. Ignore this effect for each creature the player didn't control
> continuously since the beginning of the turn.

Here "ignore this effect" exempts objects from the effect's scope; it is not the special action of
a static ability's ignore clause [CR#116.2d].

### Undergrowth

> Prevent all combat damage that would be dealt this turn. If this spell's additional cost was paid,
> this effect doesn't affect combat damage that would be dealt by red creatures.

The second sentence narrows the first sentence's prevention effect [CR#615.1a,608.2c] according to
how the spell was cast.

### Power Artifact

> Enchanted artifact's activated abilities cost {2} less to activate. This effect can't reduce the
> mana in that cost to less than one mana.

The rules let a reduced mana cost reach {0} [CR#118.7]; this effect sets its own floor. Valiant
Changeling caps its own reduction instead ("This effect can't reduce the amount of mana this spell
costs by more than {5}."). Biomancer's Familiar, Heartstone, Training Grounds and Zirda, the
Dawnwaker print the same floor sentence as Power Artifact.

### Edgewalker

> Cleric spells you cast cost {W}{B} less to cast. This effect reduces only the amount of colored
> mana you pay. (For example, if you cast a Cleric spell with mana cost {1}{W}, it costs {1} to
> cast.)

A colored reduction beyond the cost's colored component would otherwise reduce generic mana
[CR#118.7b,118.7c]; this effect switches that off for itself.

### Spikeshell Harrier

> When this creature enters, return target creature or Vehicle an opponent controls to its owner's
> hand. If that opponent's speed is greater than each other player's speed, reduce that opponent's
> speed by 1. This effect can't reduce their speed below 1.

The speed rules describe only how speed starts and increases [CR#702.179a..702.179d]; the card
supplies both the reduction and its own floor.

## Gaps in the CR

### Sorrow's Path

> {T}: Choose two target blocking creatures controlled by the same opponent. If each of those
> creatures could block all creatures that the other is blocking, remove both of them from combat.
> Each one then blocks all creatures the other was blocking.

The CR never defines "could block" (nor "could have blocked"); it defines only the declaration of
blockers [CR#509.1g].

### Biomancer's Familiar

> {T}: The next time target creature adapts this turn, it adapts as though it had no +1/+1 counters
> on it.

The CR defines adapt only as an instruction [CR#701.46a], and every other card writes it as one
("Adapt 2"). This card alone treats adapting as something a creature does, and the CR has no
definition of a creature adapting.
