---
needs: [cleanup-step6-surface]
---

# Sorts with one value

## Why

Seven sorts have one declared value, so every parameter at them is written with that value and
says nothing a reader could not supply. Each is either a slot that carries nothing, or a real
distinction whose other value no card in canon has needed yet.

| Sort: value | Parameter | Callers |
|---|---|---|
| `CombatRole`: `defending` | `combatPlayer(role)` | `defendingPlayer` |
| `EventSide`: `before` | `relativeToEvent(side, event)` | `beforePart` |
| `ColorFreedom`: `sameColor` | `ofAnyColor(freedom)` | `goldToken`, `treasureToken` and 9 canon cards |
| `SpentMode`: `affectsIt` | `whenSpentOn(mode, spell, says)` | `spendOnlyToCastAnd`, *Animal Attendant*, *Generator Servant* |
| `StateBasedCause`: `nonpositiveLife` | `conclude(verb, cause)`, `loseGame(cause)`, both `Option` | *Phyrexian Unlife* |
| `HiddenSort`: `numbers` | `revealChoices(sort)` | *Menacing Ogre* |
| `LockState`: `locked` | `doorOf(state, room)`, an `Option` | *Ghostly Keybearer* |

`ColorFreedom` also stores what the card does not print: every "one mana of any color" is
`counted(1, ofAnyColor(sameColor))`, though with one mana nothing is shared, and "N mana in any
combination of colors" (*Terrarion*'s "Add two mana in any combination of colors") cannot be
written.

## Open questions

- `CombatRole`: a two-valued distinction [CR#506.2]. Vintage-legal cards print "attacking
  player" (*Goblin Goon*'s "This creature can't block unless you control more creatures than
  attacking player"). Recommendation: keep the sort and declare `attacking` with the first card
  that needs it.
- `EventSide`: two-valued. *Glyph of Reincarnation* prints "Cast this spell only after combat".
  Recommendation: keep it and declare the after side with that card.
- `ColorFreedom`: `counted` already makes each choice once per mana unless the inner term says the
  mana share a color. Recommendation: `ofAnyColor`'s freedom becomes an `Option`, omitted for "one
  mana of any color" and `sameColor` for "of any one color"; whether "in any combination of
  colors" is the omitted case under a Template of its own wording, or a second value, is the open
  part.
- `SpentMode`: [CR#106.6] names both an additional effect on the spell the mana is spent on
  (*Boseiju, Who Shelters All*'s "If that mana is spent on an instant or sorcery spell, that spell
  can't be countered") and a delayed triggered ability that triggers when it is spent
  (*Pyromancer's Goggles*' "When that mana is spent to cast a red instant or sorcery spell, copy
  that spell"). Recommendation: keep the distinction, but as the trigger-versus-rider structure
  the CR draws, decided when *Pyromancer's Goggles* is written, not as a mode word.
- `StateBasedCause`: every Vintage-legal "lose the game for …" prints "for having 0 or less life"
  (*Lich*, *Pact Weapon*, *Phyrexian Unlife* and four more) [CR#704.5a]. The `Option` tells "lose
  the game" apart from "lose the game for having 0 or less life", so it is a legitimate presence
  flag. Recommendation: keep it as it is.
- `HiddenSort`: what is revealed is whatever the secret choice chose, and the text names it by
  anaphor: *Menacing Ogre*'s "those numbers", *Call to the Void*'s "those choices", Secret council's
  "those votes". Recommendation: `revealChoices` reads the secret choices it reveals as a noun
  phrase and takes no sort.
- `LockState`: the `Option` tells "a locked door" from "a door", and "unlocked doors" is printed
  (*Rampaging Soulrager*'s "two or more unlocked doors among Rooms you control"), so it is a
  real distinction with `unlocked` not yet declared. Recommendation: keep `Option(LockState)` and
  declare `unlocked` with the first card that needs it.

## Done when

- Each sort above is deleted with its parameter, kept with its doc comment naming the card or
  rule that needs its other value, or reshaped as settled; the landing lists each.
- Every "one mana of any color" stores no freedom the card does not print.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale.

## Pointers

`plugins/builtin/primitives/{combatRole,eventSide,colorFreedom,spentMode,stateBasedCause,hiddenSort,lockState}/`
and their callers above; `plugins/builtin/primitives/producedMana/{ofAnyColor,counted}.syl`.
