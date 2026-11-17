---
needs: [cleanup-step6-surface]
---

# Model personal-pronoun self-references (`he`/`him`/`his`, `she`/`her`)

## Why

Five canon cards print third-person personal pronouns (`"he"`, `"him"`, `"his"`) for the card
itself, and `.syl` represents them inconsistently: *Deadpool, Trading Card* ("exchange his text
box"), *Garruk Relentless // Garruk, the Veil-Cursed* ("on him, transform him", "to him"), and
*Wolverine, Fierce Fighter* ("he fights", "dealt to him") write a self-reference (`this(creature)` or
`this(planeswalker)`), while *Gideon Jura* and *Gideon, Ally of Zendikar* ("dealt to him") write `it`
(`pro(bare, one)`). `plugins/builtin/templates/pronoun/` has no `he`/`him`/`his` or `she`/`her`
template, so neither matches the printed wording (`docs/design/semantics.md` §1 item 1).

## Open questions

- Whether personal-pronoun sites (`"he"`, `"him"`, `"his"`, `"she"`, `"her"`) take dedicated
  templates (or a parameter on `pro`) that spell the printed pronoun, or write `it` (`pro(bare,
  one)`) and leave pronoun selection to `pro`'s spelling.
- Whether *Garruk Relentless*'s collapsed clause ("When Garruk has two or fewer loyalty counters on
  him") keeps its clause-initial name form or splits so `"him"` appears explicitly.

## Done when

- *Deadpool, Trading Card*, *Garruk Relentless // Garruk, the Veil-Cursed*, *Wolverine, Fierce
  Fighter*, *Gideon Jura*, and *Gideon, Ally of Zendikar* (and any `she`/`her` self-references in
  `plugins/`) use the chosen surface form for their printed personal pronouns.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, and `cargo test -p xtask` pass.

## Not in scope

- Self-references where the card prints `"it"`/`"its"` or `"itself"`
  (`syl-printed-it-self-references`), the card's name (`syl-self-name-references`), or
  `"this <head>"` (`syl-this-head-template`).

## Pointers

`plugins/builtin/primitives/pronoun/pro.syl`, `plugins/builtin/templates/pronoun/{it,itself}.syl`,
`plugins/canon/cards/` (*Deadpool, Trading Card*, *Garruk Relentless // Garruk, the Veil-Cursed*,
*Gideon Jura*, *Gideon, Ally of Zendikar*, *Wolverine, Fierce Fighter*),
`docs/oracle-style-guide.md`.
