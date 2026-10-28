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
- The ten `card_type` declarations (artifact, battle, creature, dungeon, enchantment,
  instant, kindred, land, planeswalker and sorcery) use `cardClass : Option CardClass`.
- Every other expansion changes only by mechanically replacing embedded `CardType`
  literals and sampled `CardType.mk` invocations with their corresponding `card_class`
  representation. Comparison against a mechanically converted copy of the baseline must
  report **0 differences**; retain every sample and all other content and metadata.
- Gates: `cargo xtask lean check`. Run tests and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-card-class/before
```

After the change, retain the original baseline and make a mechanically converted copy
in a temporary directory outside the workspace (for example,
`/tmp/card-class-normalized-before/snapshot.json`). Convert only:

- Expanded `CardType { name, permanent, stats, instant_or_sorcery }` literals to
  `CardType { name, card_class, stats }`, preserving `name` and `stats` exactly.
  `(true, false)` maps to `Some(PermanentCard)`, `(false, true)` maps to
  `Some(InstantOrSorceryCard)`, and `(false, false)` maps to `None`. Reject contradictory
  `(true, true)` flags rather than selecting a class.
- Sampled `Semantics.CardType.mk name permanent stats instantOrSorcery` invocations to
  `Semantics.CardType.mk name cardClass stats`, with the same mapping to `some`/`none`.
  Preserve all samples, including both present classes, and their bounded-product order.

Do not change any other AST fields, conferrals, spelling, lexical metadata or sample
content, and do not drop records or collapse distinct sample identities. The ten
`card_type` registry expansions and every embedded CardType use this same substitution.

If correcting the generic sampler changes its sample basis, first regenerate a control
baseline from the pre-CardType tree with that same corrected sampler, then apply the
mechanical conversion to the control baseline. Retain the original saved baseline and
report added samples and any displaced bounded-product samples separately. Sampling
optional finite enums and booleans must be generic, without a CardType-specific branch.

Run the strict comparison against that copy without an allowance list:

```bash
cargo xtask expansions --compare /tmp/card-class-normalized-before
```

The command must exit 0 with **0 differences**. There is no declaration rename map.
Alternatively, `--expect` may list every changed declaration only when a separate
comparison verifies zero residual differences beyond the exact mechanical substitutions
above; allowances alone do not satisfy acceptance.

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.
