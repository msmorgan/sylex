---
needs: [plugins-v3-expansion-baseline, plugins-v3-card-class]
---
# Mechanical authoring-surface pass over `plugins_v3/builtin`

Standard constraints apply. This ticket preserves meaning: the expansion baseline
(`plugins-v3-expansion-baseline`) must report zero changed expansions. The only exceptions are
the explicit key renames below.

## Why this is a script

During `plugins-v3-builtin-primitives-and-templates` (`uvnrnvqq`), a delaborator rewrote the
builtin. It wrote names in fully qualified form: about 5,400 `Semantics.` qualifiers in
`plugins_v3/builtin/{definitions,templates,rules}/`, plus `Option.some`, `Option.none` and
`Bool.*`. Every change here is a rewrite that can be expressed in regex or AST terms. Do it
with a script kept in the session scratchpad, not by hand-editing files and not with parallel
agents. Land each step as its own change, with the baseline compared after each one.

## Steps

1. **Unqualify names.** Rewrite `Semantics.Templates.<Sort>.<name>` and
   `Semantics.Definitions.<name>` to the bare `<name>`. Where a bare name is shared across
   return types, `(name, return_type)` expected-type dispatch resolves it
   (`docs/decisions/semantics-v3.md` §4.5). Rewrite `Option.some x` / `Option.none` /
   `Bool.false` to `some x` / `none` / `false`.
2. **Rewrite data-type leaves.** These are about 700 occurrences, such as `Semantics.NounWord.*`,
   `Semantics.Comparator.*` and `Semantics.TurnPart.*`. Rewrite each to the dot form `.ctor`,
   which Lean and the Rust reader both resolve against the expected type. Bundle files may not
   `open` a namespace. Bare-word spellings for these sorts are
   `plugins-v3-canon-authoring-surface`'s job.
   - Where a dot form is ambiguous to the elaborator, keep the qualified form.
   - List every kept site in the change description. A kept site is a defect to ticket, not an
     accepted exception.
3. **Rename the `Type.ident.lean` files.** There are 47 `*.*.lean` files. Rename each to
   `<ident>.lean` and remove the sort prefix from its declaration header. Bundle keys already
   use `(bare_name, return_type)`, so the declaration key does not change.
   - Rename `templates/action/Status.become.lean` to `becomeStatus.lean`, with declaration
     `becomeStatus (status : Status) : Action`. Record
     `Status.become : Action` → `becomeStatus : Action` in the baseline rename map.
   - Move `primitives/event/WouldClause.nthOccurrence.lean` to
     `primitives/would/nthOccurrence.lean`, preserving `(ordinal : Ordinal)`
     `(clause : WouldClause)` `(per : Option Period := none) : WouldClause`.
   - Move `primitives/mana/ManaTypeTerm.ofColor.lean` to
     `primitives/mana_type/ofColor.lean` and `primitives/noun/Subject.any.lean` to
     `primitives/subject/any.lean`. Distinct return types permit shared bare names,
     but distinct files are still required.
   - Preserve every declaration's parameters, types, defaults and body; change only the name.
     Preserve call-site arguments, including the three proofs' actual untapped/tapped values.
   - `Authoring.lean` retains return-sort aliases. The five parameter-shadowed calls in
     `create`, `reveal`, `lookAt`, `eventCount` and `eventSum`, and `Action.devour` in
     `devourEntry`, still resolve through those aliases.
   - Application dispatch unwraps `Option` before choosing a return sort. Token coercion
     evidence uses `Templates.TokenSpec.written`; exports and authoring-tree expectations
     follow the renamed declarations.
4. **Retire `primitives/noun/any.lean` and `primitives/subject/any.lean`.** Their call sites
   use the `AnyValue` `any` instance instead (`Words.lean:20`, `Primitives.lean:80-81`).
5. **Inline `templates/predicate/army.lean` into `definitions/keyword_action/amass.lean`.**
   - `army` was a local helper for amass [CR#701.47a], not a general predicate.
   - Inside the amass body, a bare `army` resolves to `Subtype.army`. Bind the predicate once
     in the body (it is used twice) instead of pasting it twice.
   - The expected baseline changes are the removed `army : Predicate` key and the unchanged
     amass expansion.

## Step 2 kept sites and follow-up ownership

Change `ouqkwnvy` lists all 102 retained qualified sites. They comprise:

- `Semantics.CardType.*` and `Semantics.Definitions.*`: untyped bundle-DAG cycle false
  positives, authored-call resolution and deed/parameter identity handling.
- Helper and projection calls in templates: `Characteristics.*`, `Cost.asPaid`,
  `Location.observedOnDeparture` and `QualityPayload.edits` use qualified Rust dispatch.
- `QualityOp` in `copyCharacteristics`: the conditional needs type information, and
  `Characteristics.valueEdits` requires a single-token final argument; `.sets` exposes
  the multi-token argument check in `crates/deckmaste_semantics_v3/src/lower/compute.rs:495`.
- Qualified pattern heads retain their structural matching meaning.

These are defects tracked by `plugins-v3-kept-qualified-sites`, after
`plugins-v3-canon-authoring-surface`. Shared-name typeclass dispatch belongs to
`lean-v3-overloaded-primitive-typeclasses`; the 11 prefixed structural primitive names
belong to `syl-sort-prefixed-primitive-names`, after `plugins-v3-primitive-template-boundary`.

## Acceptance

- Zero `Semantics.` in `plugins_v3/builtin/{definitions,templates,rules}/`, except the sites
  kept and listed under step 2.
- Zero `*.*.lean` files in `plugins_v3/`.
- The baseline comparison reports only the renames listed in steps 3 and 5.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/before
```

After steps 1 and 2, require zero differences:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/before
```

The rewrite script for step 3 writes `docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/renames.json`, a JSON array
of `[old_key, new_key]` pairs containing the settled replacement for
`Status.become : Action`. File moves whose declaration key stays the same need no entry.
After step 3:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/before \
  --rename-file docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/renames.json
```

After step 4, compare to the same saved baseline, retaining the rename map and
accepting precisely the two retired declarations:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/before \
  --rename-file docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/renames.json \
  --expect 'any : NounPhrase' --expect 'any : Subject'
```

After step 5, add the removed helper; amass must remain unchanged:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/before \
  --rename-file docs/evidence/plugins-v3-expansions/plugins-v3-builtin-cleanup/renames.json \
  --expect 'any : NounPhrase' --expect 'any : Subject' --expect 'army : Predicate'
```

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.

Declaration rename maps also normalize `<decl>.<param>` and `<decl>.actor` mention
IDs in callers, simultaneously, so swaps and chains work. Renaming a repeated
`NounPhrase` parameter changes `<decl>.<param>` IDs in callers; those changes
remain visible and require review and explicit expectations where intended.
