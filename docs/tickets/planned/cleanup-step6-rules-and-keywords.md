---
needs: [syl-banding-one-label, syl-basic-land-mana-abilities, syl-clash, syl-companion-pregame-rule, syl-counter-conferrals, syl-craft-materials, syl-daybound-reconfigure-bodies, syl-deathtouch-creature-conferral, syl-declaration-rules-field, syl-designation-conferrals, syl-empower-token-parameter, syl-enlist-optional-attack-cost, syl-indirect-phase-out, syl-legend-rule-multiname-groups, syl-one-declaration-per-keyword, syl-permanent-type-read, syl-role-rule, syl-room-doors-as-designations, syl-speed-rules, syl-state-based-conferral-zone, syl-venture-dungeon]
---

# Step 6 middle: rules and keywords

## Why

This stage writes rules rows and keyword bodies in full: conferrals, game rules and keyword
declarations that today state their rule only in a comment or leave it out. Later stages write
against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
- A checkpoint pass over `plugins/builtin/rules/` and the keyword ability, counter,
  designation, subtype and card type declarations under `plugins/builtin/definitions/` (keyword
  actions and the Deeds keyword abilities perform are the deeds checkpoint's) finds rules still left where Lean put them: an empty or partial keyword body, a rule stated only
  in a doc comment or left to an engine name table, a keyword variant joined to its keyword only
  by a `label` string, or a parameter its declaration never reads. Each one found is fixed in the
  checkpoint's own landing or ticketed, and the landing lists them.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.
