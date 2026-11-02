---
needs: [lean-v3-token-definitions]
---
# Empower takes its token as a parameter

`keyword_action/empowerJace.syl` defines "empower Jace N" [CR#701.71a] as `empowerJace N`, with
the Jace subtype and the token's characteristics written into its body. It becomes
`empower jaceToken N`: one keyword action parameterised by the token it makes, as amass is
parameterised by its subtype (`amass(subtype, count)`, [CR#701.47a]). A later "empower <name>"
then needs a new token, not a new action.

## Change

- Declare the Jace planeswalker token as a `token` Definition, `jaceToken`, under
  `definitions/token/`: blue, planeswalker, subtype Jace, loyalty 0, "[−1]: Surveil 1" and
  "[−3]: Draw a card" [CR#701.71a]. The rules define it, so it is a Definition, not a Template.
- Replace `empowerJace.syl` with `empower(token, amount: Amount)`, taking a token Definition. The
  body creates that token and puts `amount` loyalty counters on the chosen one, as now.
- The rule's "Jace" is a subtype test in the check and the choice: "If you don't control a Jace
  planeswalker token", "Choose a Jace planeswalker token you control" [CR#701.71a]. Any such token
  qualifies, whatever its abilities, so the body matches the subtype the token Definition declares
  (named through `lean-v3-token-definitions`). It does not hard-code `jace`, match the whole token,
  or take the subtype as a second parameter that could disagree with the token.
- Settled: the varying part is the whole token, its subtype and its own loyalty abilities, so one
  token parameter covers both. Amass takes a subtype because its Army token differs only by that
  subtype [CR#701.47a]; empower's token also carries its own abilities. Mapping a subtype to "its" token is rejected: it is not
  general across Predefined Tokens (the seven Roles share one subtype [CR#111.10j..111.10r]).
- [CR#701.71] defines only "Empower Jace". The parameter anticipates further "empower <walker>"
  cards; it is a decision, not something the rule says.
- `canon/cards/Countersculpt.syl`: `empowerJace(1)` becomes `empower(jaceToken, 1)`.
- `lean/Semantics/Proofs/KeywordActions.lean`: `okEmpowerJace` checks `empower jaceToken 1`.

## English

- `catalogs/keyword_actions.ron`, `[CR#701.71]`: the lemma becomes "empower", inflected as an
  ordinary verb, with the token as its complement before the amount (amass's `TransitiveAmount`
  is the precedent). The entry keeps the CR's name, "Empower Jace".
- The printed word is "Jace", not "Jace token", so "Jace" in this slot must reach `jaceToken`.
  Designing that is out of scope here beyond saying it must parse.

## Cards

Every printed use is "empower Jace": Countersculpt (in canon), and 34 more Vintage-legal cards,
among them Theorist's Proxy, Jace, Reality Sculptor, Repurposed Enforcer, Overwrite the Multiverse,
Violent Echoes and Fatehold Charm.
