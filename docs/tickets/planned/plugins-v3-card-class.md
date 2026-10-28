---
needs: [plugins-v3-expansion-baseline-hardening]
---
# Replace `CardType.permanent` and `CardType.instantOrSorcery` with `cardClass : Option CardClass`

Standard constraints apply. A card type contributes at most one Card Class
(`docs/contexts/game-model/CONTEXT.md` "Card Class", [CR#110.4,110.4a]). It is not two
independent booleans. The values are:

- `some .permanentCard` for artifact, battle, creature, enchantment, land and planeswalker.
- `some .instantOrSorceryCard` for instant and sorcery.
- `none` for kindred, because a kindred card always has another card type that supplies its
  class [CR#308.1].
- `none` for dungeon, which is not a permanent [CR#309.2c].

`cardClassOf` keeps its current results. `Proofs/Tables.lean` already pins
`cardClassOf [.kindred] = none` and `[.dungeon] = none`, and those pins stay.

## Change

In `lean/Semantics/Words.lean`, move `inductive CardClass` above `structure CardType` and
replace the two flags with `cardClass : Option CardClass := none`. Then update every
consumer. The verified list follows; re-grep for `permanent` and
`instantOrSorcery`/`instant_or_sorcery` before you start.

- **Lean:**
  - `Words.lean`: the ten `semantic_definition CardType.*` and `cardClassOf`.
  - `Check/Card.lean`.
  - `Check/Words.lean`.
  - `Authoring.lean`: the `card_type` field list and the elaborator.
  - `Proofs/Tables.lean`.
  - `plugins_v3/builtin/definitions/card_type/*.lean`.
- **`deckmaste_semantics_v3`:**
  - `ast.rs`.
  - `reader.rs`.
  - `bundle.rs`.
  - `lower/mod.rs`.
  - `lower/schema.rs`.
  - `tests/bundle.rs`.
  - `tests/lower.rs`.
- **`deckmaste_lowering_v3`:**
  - `reads/anchors.rs`.
  - `reads/words.rs`: the `plain(name, permanent, instant_or_sorcery)` constructor.
  - `reads/descriptions.rs`.
- **`xtask`:** `src/expansions.rs` (inline Lean fixture).
- **Docs:**
  - `lean/CONTRACTS.md`, which names the fields.
  - The "Registry Definition" entry in CONTEXT.md, only if its wording names a field.

## Acceptance

- No `permanent` or `instantOrSorcery` field remains on `CardType` in Lean or Rust.
- The expansion baseline changes only the ten `card_type` declarations.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-card-class/before
```

After the change, inspect the unified diffs for the ten expected registry changes:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-card-class/before \
  --expect 'artifact : CardType' \
  --expect 'battle : CardType' \
  --expect 'creature : CardType' \
  --expect 'dungeon : CardType' \
  --expect 'enchantment : CardType' \
  --expect 'instant : CardType' \
  --expect 'kindred : CardType' \
  --expect 'land : CardType' \
  --expect 'planeswalker : CardType' \
  --expect 'sorcery : CardType'
```

Every other declaration must compare unchanged. No rename map is needed.

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.
