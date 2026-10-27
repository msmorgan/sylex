---
needs: [lean-v3-bundle-loader-and-dsl]
---
# Migrate `Definitions/Keywords.lean` into `plugins_v3/builtin/`

Extract every declaration from `lean/Semantics/Definitions/Keywords.lean` into its own loose
Lean file and load the bundle at the bottom of `lean/Semantics/Definitions.lean`. Second of four
atomic migration tickets ([semantics-v3](../../decisions/semantics-v3.md) §§4.4–4.5).
Standard constraints apply.

## Inventory and placement

The owner ruling of 2026-10-27 makes the current declarations authoritative. `castWithSplice`,
`hasProtectionFromEach`, and `typedCycling` were older spike names; do not recreate them.
The source contains 279 declarations:

- 207 keyword ability Definitions, including `typecycling` and `Ability.splice`, become `keyword`
  declarations in `plugins_v3/builtin/definitions/keyword_ability/<name>.lean`.
- 61 ability-word Templates (`adamant` through `willOfTheCouncil`) become `ability_word`
  declarations in `plugins_v3/builtin/templates/ability_word/<name>.lean`.
- Eight Ability Templates (`devourEntry`, `asThoughFlashSacrificedAtCleanup`, `giftACard`,
  `giftAFood`, `giftATappedFish`, `giftATreasure`, `giftAnExtraTurn`, `giftAnOctopus`) become
  `template` declarations in `plugins_v3/builtin/templates/ability/`.
- `gainHaste` becomes a `template` in `plugins_v3/builtin/templates/action/`.
- `asThoughHadFlash`, returning `AsThough`, becomes a `template` in
  `plugins_v3/builtin/templates/condition/`, as authorized by the owner.
- `Action.devour` becomes an `action` in `plugins_v3/builtin/definitions/keyword_action/devour.lean`.
  Its own Reference Scope identifies a deed of the devour ability, not a keyword action label.
  Support `ofAbility := devour` and order the action before the dependent ability body while
  making its label available independently of that body.

## Authoring

Each file contains exactly one declaration, with its original documentation and citations,
without import, namespace, or open headers. Directory placement is reviewed independently of
compilation: the loader does not infer categories from paths.

Copy nested `lexical := { ... }` metadata from `keyword_abilities.ron` or `ability_words.ron` in
`deckmaste_lexical_source/catalogs/`; retain those catalogs. The legacy spelling reference in
this checkout is `plugins_v2/builtin/macros/keyword_abilities/`. Preserve canonical keyword labels
for variants and the existing human-readable ability-word labels.

Top-level spelling uses `<parameter>` placeholders. Where literal substitution cannot express
Oracle wording, omit spelling and include `-- TODO(spelling):` explaining the wording and
limitation. Oracle style guide §3 (periods and em dashes) and §14 (keyword and ability-word
forms) govern punctuation, inflection, and labels. Gift's six licensed Templates retain their
fixed spellings; its generic Action parameter is not a printable gift noun phrase.

Keyword bodies omit generated caller scope, mention identities, and keyword wrappers. References
to keyword labels use identifiers with no raw `.named` string literals. Preserve parameter
meaning when deriving keyword arguments, including the Subject parameters of enchant and champion.

## Proof

Delete `Keywords.lean`, replace its import with `load_bundle "../plugins_v3/builtin"`, and update
any direct importers. Verify exact file inventory, metadata, category placement, and absence of
manual wrappers or raw `.named` strings. Pin ability-word labels and the devour label/body loading
relationship with authoring tests.

Run `cargo xtask lean check` with zero errors, warnings, and sorry; citation noncompliance must be
empty and stale citations zero. Audit the complete diff with `cargo xtask cite audit --diff` and
bless any new citations. Report all commits, category counts, spelling TODOs, and gate output.
