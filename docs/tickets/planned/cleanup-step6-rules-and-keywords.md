---
needs: [syl-basic-land-mana-abilities, syl-clash, syl-companion-pregame-rule, syl-counter-conferrals, syl-craft-materials, syl-daybound-reconfigure-bodies, syl-deathtouch-creature-conferral, syl-empower-token-parameter, syl-enlist-optional-attack-cost, syl-indirect-phase-out, syl-one-declaration-per-keyword, syl-room-doors-as-designations, syl-speed-rules, syl-venture-dungeon]
---

# Step 6 middle: rules and keywords

## Why

This stage writes rules rows and keyword bodies in full: conferrals, game rules and keyword
declarations that today state their rule only in a comment or leave it out. Later stages write
against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
- A checkpoint pass over `plugins/builtin/rules/` and the keyword ability, keyword action,
  counter, designation, subtype and card type declarations under `plugins/builtin/definitions/`
  finds rules still left where Lean put them: an empty or partial keyword body, a rule stated only
  in a doc comment or left to an engine name table, a keyword variant joined to its keyword only
  by a `label` string, or a parameter its declaration never reads. Each one found is fixed in the
  checkpoint's own landing or ticketed, and the landing lists them.

## Pointers

- Keyword variants joined by a `label` string, beside the families
  `syl-one-declaration-per-keyword` merges: `plugins/builtin/definitions/keyword_ability/`
  `{devourQuality,emergeFrom,equipQuality,megamorph,multikicker,typecycling}.syl`.
- `plugins/builtin/definitions/designation/suspected.syl` confers nothing, though a suspected
  permanent has menace and "This creature can't block" [CR#701.60c].
