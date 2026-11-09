---
needs: [syl-token-definitions, cleanup-step6-surface]
---
# Empower takes its token as a parameter

## Why

`plugins/builtin/definitions/keyword_action/empowerJace.syl` defines "empower Jace N"
[CR#701.71a] with the Jace subtype and the token's characteristics written into its body, and a
fixed-phrase lemma "empower Jace". Every printed use is "empower Jace" (*Countersculpt*, in canon,
and 34 more Vintage-legal cards, among them *Theorist's Proxy*, *Jace, Reality Sculptor* and
*Fatehold Charm*).

Written against the surface (`cleanup-step6-surface`).

## Design

- `empower(token, amount: Amount)` is one keyword action parameterized by the token it makes, as
  amass is by subtype [CR#701.47a]. The CR defines only Empower Jace [CR#701.71], and more
  walkers are expected to get their own empower; a later one needs a new token Definition, not a
  new action.
- The check ("If you don't control a Jace planeswalker token") and the choice ("Choose a Jace
  planeswalker token you control") [CR#701.71a] match the subtype the token Definition declares,
  not a hard-coded `jace` and not the whole token.

## Done when

- `jaceToken` is a `token` Definition under `plugins/builtin/definitions/token/`: blue,
  planeswalker, subtype Jace, loyalty 0, "[−1]: Surveil 1" and "[−3]: Draw a card"
  [CR#701.71a].
- `empowerJace.syl` is replaced by `empower(token, amount: Amount)`, whose body creates the token,
  and tests and chooses by the token Definition's declared subtype.
- `plugins/canon/cards/Countersculpt.syl` calls `empower(jaceToken, 1)`.
- The lemma is "empower" with the token complement before the amount, both in the declaration's
  `lexical` block and in `crates/vocabulary/catalogs/keyword_actions.ron`'s [CR#701.71] entry.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

How the printed "Jace" in this slot parses to `jaceToken`; it must parse, but its design is the
spelling layer's.

## Pointers

- `plugins/builtin/definitions/keyword_action/empowerJace.syl`,
  `plugins/builtin/definitions/keyword_action/amass.syl`
- `plugins/canon/cards/Countersculpt.syl`
- `crates/vocabulary/catalogs/keyword_actions.ron`
