---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# Clash: an opponent and a winner

## Why

`clash` (`plugins/builtin/definitions/keyword_action/clash.syl`) is only [CR#701.30a]'s reveal
and optional move to the bottom. It has no opponent and no winner [CR#701.30b..701.30d], so none
of the 28 Vintage-legal cards that "clash with an opponent" can be spelled, nor "if you win" or a
trigger on clashing (*Entangling Trap*, *Rebellion of the Flamekin*, *Sylvan Echoes*). No clash
card is in `plugins/canon/cards/` yet.

## Design

- The win/lose outcome is a field of the `clash` action, read back for "if you win" with a
  `did(you, clash(… wins …), thisWay)` lookback, as a `flipCoins` outcome is. No "won a clash"
  Condition exists.
- Each clashing player's clash is its own event carrying the `clash` Deed, so "whenever you
  clash" triggers for each player who clashed.
- "Clash with an opponent" is a Template over `clash`, not a Definition: an action the rules
  define as another action is a Template (`docs/decisions/semantics.md` §3.1) [CR#701.30b].
- The simultaneous reveal, the APNAP decisions and the simultaneous move [CR#701.30c] are the
  engine's procedure (`docs/decisions/semantics.md` §1: timing and order belong to the engine).
  The body does not encode them.

## Done when

- `clash` carries the outcome: a player wins if they revealed a card with a higher mana value
  than all other cards revealed in that clash [CR#701.30d].
- "Clash with an opponent" is a Template that chooses an opponent, then you and that opponent
  each clash [CR#701.30b], with the outcome readable afterward.
- At least one "clash with an opponent … if you win" card and one "whenever you clash" card are
  spelled in `plugins/canon/cards/`.

## Pointers

- `plugins/builtin/definitions/keyword_action/clash.syl`
- `plugins/builtin/templates/action/flipCoins.syl`, `plugins/builtin/primitives/action/actionFlipCoins.syl`,
  `plugins/canon/cards/Chance Encounter.syl` (the coin-flip precedent)
- `docs/decisions/semantics.md` §3.1 (keyword actions and Deeds), §4.8 (lookback)
