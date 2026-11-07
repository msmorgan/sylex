---
needs: [lean-v3-spell-abilities-share-targets, plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# A later spell ability's "instead" replaces an earlier spell ability's effect

## Why

A self-replacement can be printed as its own paragraph, usually after an ability word [CR#614.15],
and each paragraph is its own ability [CR#113.2c]. `.syl` cannot write that form: `replace(replaced,
replacement)` (`plugins/builtin/primitives/action/replace.syl`) is one `Action` holding both
halves, and a `Replacement` (`instead`, `doIf`) is not an `Action`, so a later spell ability cannot
name the earlier paragraph's effect as the event it replaces. *Fatal Push*: "Destroy target creature
if it has mana value 2 or less. / Revolt — Destroy that creature if it has mana value 4 or less
instead if a permanent left the battlefield under your control this turn."

## Design

- `spellReplacement(replacement: Replacement): Ability`, a primitive beside `spell`, replaces the
  effect of the immediately preceding spell ability of the same spell [CR#113.2c,614.15]. It is
  not folded into that ability as one `replace(replaced, replacement)`.
- The replacement reads the referents the replaced spell ability introduced, its targets included
  ("that creature", "that spell").
- A `spellReplacement` with no immediately preceding spell ability (the card's first ability, or
  one preceded only by a non-spell ability) is refused.

## Done when

- The `spellReplacement` primitive exists. Its doc comment cites [CR#113.2c,614.15], states the
  three Design rules, and says in one line why self-replacement has two forms: the paragraph is a
  separate ability, and folding it into one `replace` would erase that ability boundary and its
  ability word.
- The five witnesses are in `plugins/canon/cards/`, each later paragraph written as a
  `spellReplacement` under its ability word. *Bring the Ending* needs a `corrupted` ability word,
  which `plugins/builtin/templates/ability_word/` lacks.

## Not in scope

The checker that steps `spellReplacement` and refuses one with no preceding spell ability: it is
built with the greenfield discourse checker, which takes the Design's rules as requirements.

## Pointers

- `plugins/builtin/primitives/ability/spell.syl`, `plugins/builtin/primitives/action/replace.syl`,
  `plugins/builtin/primitives/static/instead.syl`
- `plugins/builtin/templates/ability_word/` (`revolt`, `ferocious`, `morbid`, `threshold`)
- `docs/decisions/semantics.md` §2 (reference across a spell's abilities)
