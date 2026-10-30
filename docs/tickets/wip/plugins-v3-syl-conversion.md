---
needs: [plugins-v3-syl-generator, lean-elaborated-value-dump]
---
# Convert every `plugins_v3` file to `.syl` and build the builtin from generated modules

Standard constraints apply. Second of three tickets (after `plugins-v3-syl-generator`, before
`plugins-v3-data-language`). The conversion changes syntax only: the elaborated values do not
change. Specification: `docs/decisions/semantics-v3.md` §4.4–§4.5.

## Change

- **Citations first.** `cargo xtask cite` scans `.syl` `#` comments like `.lean` and `.rs` comments,
  so the converted files' CR citations stay checked.
- **Convert by script, never by hand.** The script is a purpose-built converter or a final emitter
  run from the current `deckmaste_semantics_v3`. It stays untracked, under `docs/evidence/`. It
  writes every `plugins_v3/**/*.lean` declaration and card as `.syl`:
  - every coercion and domain-list coercion written as its explicit wrapper call;
  - no leading dots, except a primitive naming a differently named constructor
    (`primitive actionSearch(...): Action = .search`); reserved words written plain (`in`, `from`,
    `this`, ...);
  - the `keyword` kind written `ability`; result sorts as `): Sort` on `primitive` and `template`;
  - primitives body-less;
  - the six record-building primitives (`can`, `cant`, `must`, `gatedBy`, both `did`s) turned into
    Templates over renamed primitives, so no two declarations share one `(name, sort)`;
  - keyword `lexical.parameters` kept as written; comments and doc comments as `#` lines.
- **Delete** `plugins_v3/builtin/templates/noun/bracketed.lean` and the `bracketed` pins in
  `Proofs/TextChanges.lean`, naming each retired pin (they assert a template that no longer
  exists). Cleave stays backburnered until `plugins-v3-cleave-bracketed-text`.
- **Cards are plain `Card` values.** The generator emits a canon card as its `Card`, not as a
  `Spelled` value carrying `Authoring.Form` evidence (that machinery goes in
  `plugins-v3-data-language`). Adapt `lean/Semantics/CanonCheck.lean` (it reads `Spelled.card`),
  and rewrite `untapStepCardsUseUntapDuring` (`Proofs/UntapStep.lean`) to assert on the cards' values
  directly instead of their authoring calls.
- **Switch the build.** The builtin and canon are built from the generator's modules. Delete
  `load_bundle`, `topoSortBundle`, `referencedIdents`, the directory walk and the `builtinBundle`
  input. The `exileZone` (`zone .exile none`) and `devourEntry` (`Action.devour`) workarounds
  disappear.
- **Delete the old Rust crates in this same change**, because the old parser cannot read `.syl`:
  `deckmaste_semantics_v3` and `deckmaste_lowering_v3` with their tests, `xtask expansions`, and
  `crates/xtask/tests/suite/plugins_v3_declarations.rs`. If the script uses the old crate as its
  emitter, run it first. Delete any ticket text that still names these crates as live.
- Additive `lean-v3-*` tickets that land before the conversion are carried over by the script.

## Acceptance

- The Lean dump (`lean-elaborated-value-dump`), saved before converting, shows 0 differences
  afterwards under the conversion's rename map. The map is generated and gitignored, never
  tracked; the report states its entry count. (The Rust `xtask expansions` cannot read `.syl`, so
  it is no oracle here.)
- The generator parses every converted file without error. `cargo xtask lean check` passes.
