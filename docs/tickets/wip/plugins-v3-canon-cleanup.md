---
needs: [plugins-v3-canon-authoring-surface, plugins-v3-conferral-inlining]
---
# Rewrite the 908 `plugins_v3/canon/cards` files to house style and printed-name filenames

Standard constraints apply. This ticket uses the surface built by
`plugins-v3-canon-authoring-surface`. Do it with a script (scratchpad), not by hand and not
with parallel agents. Land it in three changes, comparing the expansion baseline after each one.

## Steps

1. **Rewrite the syntax.** The baseline must show zero changes, keyed by declaration.
   - Replace `card <ident> : Spelled := …` with `card := …`.
   - Drop `body :=`, `singleFacedCard` and `{ characteristics := … }`.
   - Use the record constructors for multi-part cards.
   - Use bare numerals for stats and costs (`cost := [2, black]`, `power := 3`). Drop `stat `,
     `generic ` and `pip .`.
   - Use bare words in place of dot constructors.
   - Write lists for `sequentially` and `and`, and bare `StaticSpec` in `text`.
   - Use `this_`, not `this`.
   - Format records and ability lists consistently.

   Because keys change from camelCase identifiers to printed names, this step also:
   - Migrates every proof reference: about 155 `Cards.<id>` references and about 100 `<id>.card`
     uses across about 17 `lean/Semantics/Proofs/*.lean` files, not only `Cards*.lean`. They
     become `cardNamed "<name>"`.
   - Updates `deckmaste_semantics_v3/tests/lower.rs` `live_canon_cards_elaborate_with_builtin`.
   - Has the baseline compared with an explicit old-key to new-key map.
2. **Rename the files.** Rename each file to `<card_filename(name)>.lean`, using
   `deckmaste_core::plugin::card_filename`.
   - It escapes `< > : " / \ | ? *` as `{less}`…`{star}` and leaves everything else literal.
     For example: `Saw in Half.lean`, `Circle of Protection{colon} Red.lean`,
     `Branchloft Pathway {slash}{slash} Boulderloft Pathway.lean`.
   - The canon bundle is read by directory walk (`load_bundle`, `Bundle::load`), not by
     `import`, so filenames with spaces and braces are safe.
   - Store names in NFC.
3. **Strip the name-only comments.** About 406 files have a one-line `/-- <Card Name> -/`.
   - Keep or carry over (from `plugins/canon/cards/`) only comments that explain why the card
     is canonical, meaning which rule or interaction it pins. Examples are
     `adaptiveAutomaton` and `hitRun`.

## Acceptance

- Every file is named `card_filename` of its key. Its first non-comment token is `card :=`.
- Zero matches in `plugins_v3/canon/cards` for any of the following:
  - `: Spelled`
  - `body :=`
  - `singleFacedCard`
  - `characteristics :=`
  - `\bstat `
  - `\bgeneric `
  - `pip \.`
  - `^/-- [^\n]* -/$` whose text equals the card's name
  - any dot constructor that is not on `plugins-v3-canon-authoring-surface`'s list of kept
    exceptions
- The old `card <ident>` syntax and the `body` field are removed from `Authoring.lean` and the
  Rust reader.
- The baseline shows zero changed expansions under the key map.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3` and
  `xtask`.
- Tickets that cite canon cards by camelCase name or old path are re-pointed. This includes
  `lean-v3-card-head-noun-sweep`, which should not run concurrently with this ticket.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-canon-cleanup/before
```

Before rewriting the declaration headers, the migration script writes all 908
old identifier keys and new printed-name keys to `docs/evidence/plugins-v3-expansions/plugins-v3-canon-cleanup/renames.json`
as JSON `[old_key, new_key]` pairs (including ` : Spelled` on both endpoints).
Keep names verbatim, including spaces, punctuation and ` // `. After each of the
three landed steps compare to the original baseline with the same map:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-canon-cleanup/before \
  --rename-file docs/evidence/plugins-v3-expansions/plugins-v3-canon-cleanup/renames.json
```

Require `0 differences, 908 renames` and exit 0; file moves and comment removal
add no differences. Do not use `--expect` to conceal changed expansions.

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.

Declaration rename maps also normalize `<decl>.<param>` and `<decl>.actor` mention
IDs in callers, simultaneously, so swaps and chains work. Renaming a repeated
`NounPhrase` parameter changes `<decl>.<param>` IDs in callers; those changes
remain visible and require review and explicit expectations where intended.
