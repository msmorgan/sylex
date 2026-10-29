---
needs: []
---
# Make every `plugins_v3` declaration a first-order constructor tree, and delete `compute.rs`

Standard constraints apply. This prepares `plugins-v3-data-language`.

The data language has no evaluation and no local bindings. It has no `fun`, `match`, `if`,
`List.*`/`Option.*` functions, `::`/`++`, helper projections, `have`, tuples or `⟨…⟩`. Each
declaration must be a tree of constructors, primitives, Templates, literals, records, lists and
parameters.

Today the builtin still computes, which is why `crates/deckmaste_semantics_v3/src/lower/compute.rs`
(763 lines) exists. Remove every site in today's syntax, while the Rust expansion baseline still
works as a meaning-preservation oracle, then delete `compute.rs`. Any evaluation left behind then
fails Rust lowering loudly, and the data-language conversion only has to translate.

## Sites (2026-10-28; re-grep before starting)

- **Higher-order parameters:**
  - `templates/action/chooseModesOver.lean` (`splice (α → Action)`; used by Call Damage Control
    and Wild Shape).
  - `templates/ability/alsoForKeywords.lean` (`KeywordTerm → Ability`, with `List.map`,
    `List.filterMap` and `match`). Its canon users are Odric, Lunarch Marshal; Bleeding Effect;
    Cairn Wanderer; Concerted Effort; Death-Mask Duplicant; Escaped Shapeshifter; and Urborg
    Scavengers.
- **`List.map` building modes:** `chooseModes` and `chooseSpree`.
- **`Option.getD` / `Option.map` on arguments:**
  - `putInto`, `putOntoBattlefield`, `putOntoBattlefieldTapped`,
    `putOntoBattlefieldUnderYourControl`, `putOntoBattlefieldAttachedTo` and `returnToBattlefield`
    (`from_ |>.getD wherever`);
  - `loseCounters` (`Option.map exactlyOf`);
  - `enter`.
- **`match` / `if` on arguments:** `statEdit`, `ptDefinition`, `keywordGrant`, `keywordOf`,
  `triggers`, `zoneChange` (`if Location.observedOnDeparture …`) and `copyCharacteristics`.
- **Helper projections on AST values:**
  - `Cost.asPaid` in `pay`, `doUnless` and `doUnlessKeywordCost`;
  - `Characteristics.typeChanges` in `isAlso`;
  - `QualityPayload.edits` in `qualityChange` and `qualityDefinition`;
  - `.plur` in `destroyCantRegenerate`;
  - the `Characteristics` field reads in `copyCharacteristics`.
- **List operators:** `::` in `theRingTemptsYou` and `++` in `copyCharacteristics`.
- **`have`**, in 9 definitions:
  - `ninjutsu` and `sneak` (`returnedCreaturesDefender`);
  - `empowerJace` (`jaceToken`);
  - `amass` (`isArmy`);
  - `discover` (`fits`);
  - `cascade` (`cheaper`);
  - `saga` (`hasChapterAbilities`);
  - `hideaway` (`exiledCard`);
  - `theRingTemptsYou` (`ringBearers`, `tempting`, `timesTempted`).
- **Tuples (`×`, `(a, b)`):**
  - the mode lists in `chooseSpree` and `actionChooseModes`;
  - `did`'s `ordinal : Option (Ordinal × Option Period)`.
- **`⟨…⟩` anonymous constructors**, on three types:
  - `KeywordCategory`, in 9 canon cards (`anyIn ⟨hexproof, none⟩`, …);
  - `FaceDownValues`, in `faceDownCreature` and Yedora, Grave Gardener;
  - `CharacteristicBundle`, in Mutavault, Volrath's Laboratory and Soulstone Sanctuary.

## How

Pick per site, preferring the first option that applies:

1. **A binder default** replaces `getD` (`(from_ : Location := wherever)`).
2. **A coercion or 1-to-1 constructor** replaces a `List.map` that only wraps, a tuple or a
   `⟨…⟩`. Examples: a `Mode` type with `Action → Mode` and `(Cost, Action) → Mode` constructors; a
   first-order `ordinal` constructor for `did`; a record or explicit arguments for `anyIn`, face-down
   values and bundles.
3. **Separate Templates, one per arm,** replace a `match` or `if` on an argument. Use this when the
   arms are different printed phrasings, or when each caller already knows the arm:
   - `observedOnDeparture`: each caller knows its own origin;
   - `typeChanges` in `isAlso`: `[]` to `none` is the `Option` list handling;
   - `.plur` in `destroyCantRegenerate`: two Templates, or a number parameter.
4. **A constructor, with the computation moved downstream** into the checker in Lean and lowering
   in Rust. Use this only when the helper is a semantic normalization that no caller can decide,
   such as `Cost.asPaid`, `QualityPayload.edits` and `copyCharacteristics`'s field logic. The
   plugin file only states it.
5. **Write it out**, in two cases:
   - **A construction used by only one or two cards whose printed text repeats it.**
     `chooseModesOver` is deleted this way, with the modes written out as printed in Call Damage
     Control and Wild Shape, as ruled in `plugins-v3-data-language`. `splice` and `CallerSplice`
     go with it.
   - **Every `have`.** Inline the binding at each use. `have` is substitution today, so the
     expansion is identical, and the text often repeats the phrase too: amass's CR text says "Army
     creature you control" twice. Dropping it also removes local bindings, and with them the
     bottom-up type inference in `compute.rs`.
   - `theRingTemptsYou` is written out fully inline for now. The owner may later move the Ring
     emblem into a Template.

## `alsoForKeywords`

Ruled (owner, 2026-10-29): (b). A constructor holds the frame and the keyword list
(`Ability.sameIsTrueFor`); the frame names its keyword by the `thatKeyword` atom, and the checker and
lowering distribute over the keywords. Expansion stays plain substitution.

## Acceptance

- `rg -n '\bfun\b|=>|\bmatch\b|\bif\b|\bList\.[a-z]|\bOption\.(getD|map|bind)|::|\+\+|\bhave\b|×|⟨|\.asPaid\b|\.edits\b|typeChanges|observedOnDeparture|\.plur\b' plugins_v3/`
  finds nothing outside comments and `spelling` strings. Each site was rewritten by one of the
  rules above, and the change description lists which.
- **`crates/deckmaste_semantics_v3/src/lower/compute.rs` is deleted,** along with the
  bottom-up `infer`/`type_of` path and the evaluation branches in `lower/mod.rs` that
  served it. Lowering refuses any remaining evaluation with an error naming the site.
- **Canon meaning check.** Save a `cargo xtask expansions` baseline (builtin and canon) before the
  change.
  - **Every canon card that uses no new constructor shows 0 differences.**
  - Canon cards whose expansion changes only by a rule-4 or `alsoForKeywords` constructor are
    listed. Their diffs are read and show only that constructor substitution.
- **Builtin differences are only the reshaped declarations,** each `--expect`ed or renamed and its
  diff read. A Template split per arm must reproduce the old arm's expansion.
- **Behaviour moved downstream is pinned.** Checker pins show the same verdicts as before, and
  `deckmaste_lowering_v3` tests show the same lowered output.
- Gates: `cargo xtask lean check`. Run `cargo test` and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`. The old crates must keep passing until
  `plugins-v3-data-language` deletes them.
