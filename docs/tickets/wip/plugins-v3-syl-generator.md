---
needs: [lean-generated-canon-modules, plugins-v3-first-order-templates]
---
# A Lake generator that turns `.syl` declarations into plain Lean modules

Standard constraints apply. First of three tickets that replace the Lean-syntax `plugins_v3` DSL
with standalone `.syl` files (`plugins-v3-syl-conversion` converts the files,
`plugins-v3-data-language` deletes the authoring machinery). The specification is
`docs/decisions/semantics-v3.md` §4.4 (grammar and concrete surface syntax) and §4.5 (dispatch,
`Option` wrapping, direct sort atoms, expansion, mentions, Lake modules). This ticket builds the
generator and proves it on a fixture corpus; it converts no real plugin file.

## Change

- **Parser.** A `.syl` parser in the Lake generator of `lean/lakefile.lean` (the one
  `lean-generated-canon-modules` added, behind `Module.inputFacet`, ending with `setTrace .nil`),
  covering every production and token in §4.4.
- **Resolution, in the generator.** Each node resolves top-down by `(name, expected sort)` in the
  order of §4.5 item 2, using the parameter sorts, the declaration headers of every `.syl` file and
  the primitives' signatures:
  - a parameter at its declared sort; a bare word naming a parameter at any other sort is a
    generator error;
  - `none`, then a declaration `(name, T)` and then `(name, U)` for `T = Option U`;
  - otherwise a constructor or enum word, emitted as Lean's leading-dot form at the known expected
    sort, so Lean's own elaboration reports an unknown word;
  - `Option` presence wraps in `some`; omitted arguments and fields take their declared defaults;
  - direct sort atoms (integers, colour words, `ability` and `action` names) as §4.5 lists them.
- **Load errors:** a non-primitive declaration sharing its name with a constructor or enum word of
  its sort; two declarations with one `(name, sort)`; a parameter used at the wrong sort.
- **Emission.** Plain Lean `def`s, under the same constant names today's `primitive`, `template`,
  `action`, `keyword` and `card` command elaborators produce (`lean/Semantics/Authoring.lean`), so
  proofs that name them keep compiling. A body-less `primitive` emits the binding of its
  same-named constructor, matching parameters to fields by name. Definitions emit the `inCaller`
  wrapping, `ownScope` / `Ability.keyword` wrapper and keyword label that the commands emit today.
- **Mentions.** `firstMention` / `laterMention` are inserted per declaration body in today's order:
  the expression's order, with each `Predicate.or` alternative in its own scope
  (`rewriteMentions` in `Authoring.lean`).
- **Modules.** Each strongly connected component of the dependency graph becomes one gitignored
  module under `lean/.lake/build/gen/` with real `import`s. Override `LeanLib.modulesFacet` so the
  library awaits the generator before listing its modules, and add that case to the generator's
  regression test.

## Fixture corpus and acceptance

- Hand-written `.syl` fixtures, outside `plugins_v3/` and alongside today's `.lean` bundle, cover:
  every production and token; every resolution step and each load error (as negative cases); a
  body-less primitive; a Template and a Definition with a `NounPhrase` parameter mentioned twice;
  `doForEachKind(axis, domain, sort, body)`; a Template whose parameter is mentioned in two `or`
  alternatives; an `ability` with a self-reference (`hasKeyword(flying)` inside `flying`).
- For each fixture that mirrors an existing declaration, a pin proves the generated value equal to
  the macro-elaborated one.
- `cargo xtask lean check` passes; the existing bundle and proofs are untouched.
