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
   - One rename collides: `templates/action/Status.become.lean` would become a second
     `become : Action`. Give it a distinct name and record it as an explicit baseline rename.
   - The other same-folder pairs (`nthOccurrence`, `ofColor`) differ in return type and are
     fine.
4. **Retire `primitives/noun/NounPhrase.any.lean` and `Subject.any.lean`.** Their call sites
   use the `AnyValue` `any` instance instead (`Words.lean:20`, `Primitives.lean:80-81`).
5. **Inline `templates/predicate/army.lean` into `definitions/keyword_action/amass.lean`.**
   - `army` was a local helper for amass [CR#701.47a], not a general predicate.
   - Inside the amass body, a bare `army` resolves to `Subtype.army`. Bind the predicate once
     in the body (it is used twice) instead of pasting it twice.
   - The expected baseline changes are the removed `army : Predicate` key and the unchanged
     amass expansion.

## Acceptance

- Zero `Semantics.` in `plugins_v3/builtin/{definitions,templates,rules}/`, except the sites
  kept and listed under step 2.
- Zero `*.*.lean` files in `plugins_v3/`.
- The baseline comparison reports only the renames listed in steps 3 and 5.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.
