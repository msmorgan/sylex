---
needs: [plugins-v3-canon]
---
# Clean up `plugins_v3/builtin` authoring surface and unify `CardType.cardClass`

**Repair five authoring and model defects introduced during the `plugins_v3/builtin` extraction
([`plugins-v3-builtin-keywords`](../done/plugins-v3-builtin-keywords.md),
[`plugins-v3-builtin-actions-and-definitions`](../done/plugins-v3-builtin-actions-and-definitions.md),
[`plugins-v3-builtin-primitives-and-templates`](../done/plugins-v3-builtin-primitives-and-templates.md))
and `CardType` Card Class modeling.** Standard constraints apply.

## Scope

1. **Replace `CardType.permanent` and `CardType.instantOrSorcery` with `cardClass : Option CardClass := none`:**
   - A card's Card Class ([CR#110.4,110.4a], `docs/contexts/game-model/CONTEXT.md` "Card Class") is
     a single mutually exclusive classification: `some .permanentCard` (artifact, battle, creature,
     enchantment, land, planeswalker), `some .instantOrSorceryCard` (instant, sorcery [CR#304.1,307.1]),
     or `none` (kindred [CR#308.1] and dungeon [CR#309.2c]). Do not model mutually exclusive classes
     as two boolean flags by accretion.
   - Move `inductive CardClass` above `structure CardType` in `lean/Semantics/Words.lean`, replace
     `permanent : Bool` and `instantOrSorcery : Bool := false` on `CardType` with
     `cardClass : Option CardClass := none`, and update `cardClassOf`, `lean/Semantics/Check/`,
     `lean/Semantics/Authoring.lean` (`card_type` field list and elaborator),
     `plugins_v3/builtin/definitions/card_type/*.lean`, `crates/deckmaste_semantics_v3` (`ast::CardType`,
     `reader.rs`, `lower/`), `crates/deckmaste_lowering_v3` (`reads/anchors.rs`), `lean/CONTRACTS.md`,
     and `docs/contexts/game-model/CONTEXT.md` ("Registry Definition").

2. **Strip `Semantics.*` qualification spam from `plugins_v3/builtin/{definitions,templates,rules}/`:**
   - During `plugins-v3-builtin-primitives-and-templates` (`uvnrnvqq`), 224 files in
     `plugins_v3/builtin/{definitions,rules}/` and the extracted files in
     `plugins_v3/builtin/templates/` were rewritten through a delaborator that emitted fully
     qualified names (`Semantics.Templates.Ability.whenever`, `Semantics.Templates.Subject.any`,
     `Semantics.Definitions.destroy`, `Option.some`, `Option.none`, `Bool.false`, etc.).
   - Restore clean unqualified authoring syntax across `plugins_v3/builtin/{definitions,templates,rules}/`
     (using the pre-delaboration bodies at `wupllkou` in `jj` history as reference), using bare
     primitive/template/definition names, `any` (`AnyValue.any`), domain list coercions, and bare
     noun/color/mana coercions. Only `plugins_v3/builtin/primitives/` may reference
     `Semantics.Primitives.*`.

3. **Rename `Type.ident.lean` files and strip sort-prefixed declaration headers:**
   - Rename the 47 `Type.ident.lean` files in `plugins_v3/builtin/{primitives,templates}/`
     (`Predicate.and.lean`, `Action.target.lean`, `Condition.did.lean`, `AbilityLost.written.lean`,
     etc.) to bare `<ident>.lean` (`and.lean`, `target.lean`, `did.lean`, `written.lean`, etc.),
     rehoming any same-folder collisions to the directory matching their return sort (e.g.
     `WouldClause.nthOccurrence.lean` to `primitives/static/nthOccurrence.lean`), and use bare
     declaration names (`primitive and`, `primitive target`, `template did`, `action devour`,
     `template instant`, `template sorcery`, `template kindred`, `template spell`) dispatched by
     `(bare_name, return_type)`.
   - Retire `primitives/noun/NounPhrase.any.lean` and `primitives/noun/Subject.any.lean` in favor of
     the `AnyValue.any` (`any`) typeclass instance across `plugins_v3/builtin/`.

4. **Clean up `-- TODO(spelling):` comments and restore uninflected `spelling := "..."` strings:**
   - Remove `-- TODO(spelling):` from structural `primitive` and helper declarations that are never
     spelled directly as standalone surface constructions (e.g. `No fixed surface wording; this
     helper composes semantic structure`, `Structural constructor; surface wording belongs to its
     phrasing templates`, `byCardClass`, `flavorWord`, and rules-table conferral helpers).
   - Restore uninflected `spelling := "..."` strings on declarations where `spelling` was omitted
     only because of inflection, article selection, or cost-separator punctuation (`amass`,
     `affinity`, `bandsWithOther`, `champion`, cost keywords such as `ward`/`cycling`/`equip`,
     `activatedBy`/`activatedOnlyDuring`/`activatedOnlyIf`/`activatedOnlyOnce`, `attachChoosing`,
     `crew`, `saddle`, etc.), since the CxG layer (`deckmaste_spelling`) owns inflection and
     realization. Retain `-- TODO(spelling):` only on declarations that genuinely have a surface
     spelling whose template representation is still deferred.

5. **Delete `plugins_v3/builtin/templates/predicate/army.lean` and inline in `amass.lean`:**
   - `army` ("Army creature you control") was a local helper `def` in `Actions.lean` for `amass`
     ([CR#701.47a]), not a general predicate template (and its name collides conceptually with
     `Subtype.army`). Delete `plugins_v3/builtin/templates/predicate/army.lean` and inline
     `and [hasSubtype army, isCreature, actorControls]` in
     `plugins_v3/builtin/definitions/keyword_action/amass.lean`.

6. **Restore dropped doc comments and CR citations across `plugins_v3/builtin/`:**
   - During extraction, files were generated only from the `lean/Semantics/` Lean docstrings rather
     than merging the comments and CR citations that already lived on the `plugins_v2/builtin`
     macro files (at `tkryskrm`, before `mxtlquxz` deleted `plugins_v2/builtin`):
     - `definitions/keyword_ability/`: only 68 of 207 files have doc comments and `[CR#702.1]`
       citations, whereas 211 of 211 `plugins_v2/builtin/macros/keyword_abilities/*.ron` files had
       comments (202 with CR citations). Also strip stray `/- ## Keyword abilities -/`
       section-header fragments left at the top of extracted files (`absorb.lean`, etc.).
     - `definitions/keyword_action/`: 21 of 89 files lack doc comments and `[CR#701.1]` citations.
     - `definitions/designation/`: only 6 of 21 files have doc comments and CR citations,
       whereas 20 of 20 `plugins_v2/builtin/macros/designations/*.ron` files had them.
     - `definitions/relation/`: 0 of 3 files (`encodedOn`, `haunting`, `paired`) have doc comments or
       CR citations, whereas 3 of 3 `plugins_v2/builtin/macros/relations/*.ron` files had them.
     - `definitions/counter/`: only 4 of 218 files have doc comments/citations, whereas 29
       `plugins_v2/builtin/macros/counter_kinds/*.ron` files had them.
     - `definitions/subtype/`: only 7 of 463 files have doc comments/citations, whereas 122
       `plugins_v2/builtin/macros/subtypes/*.ron` files had them.
     - `primitives/` and `templates/`: 319 of 624 primitives and 145 of 473 templates lack doc
       comments, whereas `plugins_v2/builtin/macros/{abilities,instructions,predicates,events,nouns,amounts,conditions,determiners,durations,pronouns,face_down,text_changes,turn_parts}/*.ron`
       carried doc comments and CR citations across 878 files (503 with CR citations).
   - Port all missing doc comments and CR citations from `plugins_v2/builtin` (`tkryskrm`)
     and `plugins/builtin`, and add CR-cited doc comments for any remaining undocumented
     `keyword_ability`, `keyword_action`, `designation`, `relation`, `card_type`, `primitives`, and
     `templates` files.

## Acceptance

- Zero `Semantics.` qualifiers in `plugins_v3/builtin/{definitions,templates,rules}/`.
- Zero `*.*.lean` (`Type.ident.lean`) filenames in `plugins_v3/`.
- `plugins_v3/builtin/templates/predicate/army.lean` is deleted.
- Every `keyword_ability`, `keyword_action`, `designation`, `relation`, and `card_type` in
  `plugins_v3/builtin/definitions/` carries a `/-- ... -/` doc comment with its CR citation,
  and all comments/citations from `plugins_v2/builtin` (`tkryskrm`) are restored across
  `plugins_v3/builtin/{definitions,primitives,templates}/`.
- `CardType` carries `cardClass : Option CardClass := none` with no `permanent` or
  `instantOrSorcery` boolean flags in Lean or Rust.
- `cargo test -p deckmaste_semantics_v3 -p deckmaste_lowering_v3 -p xtask`,
  `cargo clippy -p deckmaste_semantics_v3 -p deckmaste_lowering_v3 -p xtask --tests -- -D warnings`,
  `cargo xtask lean check`, and the CR citation gates pass.
