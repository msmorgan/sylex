---
needs: []
---
# Make every `plugins_v3` declaration a first-order constructor tree

Standard constraints apply. This prepares `plugins-v3-data-language`.

The data language has no evaluation: no `fun`, `match`, `if`, `List.*`/`Option.*` functions, and
no helper projections. Each declaration must be a tree of constructors, primitives, Templates,
literals and parameters. Today 24 builtin files still compute, which is why
`crates/deckmaste_semantics_v3/src/lower/compute.rs` exists. The work here is done in today's
syntax, while the Rust expansion baseline still works as a meaning-preservation oracle, so that
the data-language conversion only has to translate.

## Sites (2026-10-28; re-grep before starting)

- **Higher-order parameters:**
  - `templates/action/chooseModesOver.lean` (`splice (α → Action)`; used by Call Damage Control
    and Wild Shape).
  - `templates/ability/alsoForKeywords.lean` (`KeywordTerm → Ability`, with `List.map`,
    `List.filterMap` and `match`). Its canon users are Odric, Lunarch Marshal; Bleeding Effect;
    Cairn Wanderer; Concerted Effort; Death-Mask Duplicant; Escaped Shapeshifter; and Urborg
    Scavengers.
  - Check the arrow types in `primitives/static/appendTextOf.lean` and `revealedAsCast.lean`.
- **`List.map` building modes:** `chooseModes`, `chooseSpree`.
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
  - the `Characteristics` field reads in `copyCharacteristics`;
  - `definitions/keyword_action/theRingTemptsYou.lean`.

## How

Pick per site, preferring the first option that applies:

1. **A binder default** replaces `getD` (`(from_ : Location := wherever)`).
2. **A coercion or 1-to-1 constructor** replaces a `List.map` that only wraps, e.g. `Action → Mode`
   and `(Cost, Action) → Mode`.
3. **Separate Templates, one per arm,** replace a `match` or `if` on an argument, when the arms are
   different printed phrasings.
4. **A constructor, with the computation moved downstream** into the checker in Lean and lowering
   in Rust, when the helper is a semantic normalization rather than a phrasing (`Cost.asPaid`,
   `QualityPayload.edits`, `Characteristics.typeChanges`, `observedOnDeparture`). The checker or
   lowering applies it; the plugin file only states it.
5. **Write the card out** when only one or two cards use the construction and the printed text
   repeats it.
   - `chooseModesOver` is deleted this way, with the modes written out as printed in Call Damage
     Control and Wild Shape. This was already ruled in `plugins-v3-data-language`.
   - Delete `splice` and `CallerSplice` with it.

`have name := expr; body` stays. It is pure substitution and carries no evaluation, and 47 files
use it. `plugins-v3-data-language` keeps it as the grammar's only local binding.

## Owner decision needed: `alsoForKeywords`

The printed text is a list: "…flying. The same is true for first strike, double strike, …". Writing
it out per keyword is unfaithful to that, and long. The options:

- **(a) A placeholder atom.** A first-order Template takes the frame once, with a `thatKeyword`
  placeholder (analogous to the `which` relative-clause atom, `docs/decisions/semantics-v3.md`
  §3.4), and a `List KeywordTerm`. Expansion substitutes each keyword.
- **(b) A constructor.** `alsoForKeywords` becomes an AST constructor holding the frame and the
  list, and the checker and lowering distribute it.

(a) keeps the core AST unchanged; (b) keeps expansion trivial. Rule before implementing.

## Acceptance

- `rg -n '\bfun\b|=>|\bmatch\b|\bif\b|\bList\.[a-z]|\bOption\.(getD|map|bind)|\.asPaid\b|\.edits\b|typeChanges|observedOnDeparture|\.plur\b' plugins_v3/`
  finds nothing outside comments and `spelling` strings. Each site was rewritten by one of the
  rules above, and the change description lists which.
- **The canon is the meaning check.** `cargo xtask expansions --compare` against a baseline saved
  before the change shows **0 differences for every canon card**.
- **Builtin differences are only the reshaped declarations,** each `--expect`ed or renamed and
  its diff read. A split Template's arms must reproduce the old arm's expansion.
- Logic moved downstream comes with checker pins showing the same verdicts as before.
- Gates: `cargo xtask lean check`. Run `cargo test` and clippy for `deckmaste_semantics_v3` and
  `xtask`. The old crates must keep passing until `plugins-v3-data-language` deletes them.
