---
needs: [dsl-parser, dsl-parse-differential, plugins-v3-data-language, lean-proofs-spec-catalog, dsl-named-arguments-anywhere, dsl-plain-word-names]
---
# Stage 1a of `semantics-rust-rewrite`: `deckmaste_dsl::bundle` and `semantics` resolution

Standard constraints apply. First half of Stage 1 of `semantics-rust-rewrite`: load and cache
preparsed `.syl` bundles in `deckmaste_dsl::bundle`, register declaration signatures, resolve names
and arguments by expected sort (`docs/decisions/semantics-v3.md` §4.5) into a resolved `Term` tree
in `crates/semantics`, check dependency cycles, and expose `cargo xtask semantics check`.

- Calls with interleaved named and positional arguments resolve as `deckmaste_dsl` parses them
  (`dsl-named-arguments-anywhere`).
- Lean stays in the tree and in CI throughout.

---

## Design Specification

### 1. Preparsed Bundle Loader (`deckmaste_dsl::bundle`)

The v3 pipeline has no separate top-level `deckmaste_plugin` crate (v1 `deckmaste_plugin` is
deletion-bound at `semantics-v1-cutover` per `CLAUDE.md`):

- `deckmaste_dsl::bundle::Bundle::load` walks a plugin directory (`plugins_v3/builtin`,
  `plugins_v3/canon`), reads each `.syl` file into memory (closing the OS file handle immediately;
  no `std::fs::File` handles are retained), parses it once into `Arc<SourceFile>`
  (`{ id: FileId, path: PathBuf, source: String, ast: dsl::File }`), and returns an `Arc<Bundle>`
  exposing both a deterministic path map (`BTreeMap<PathBuf, Arc<SourceFile>>`) and a
  `FileId → Arc<SourceFile>` table for resolving `Span`s to `path:line:col`.
- Positioning `Bundle` in `deckmaste_dsl` lets `semantics` (and later `deckmaste_lexical_source`
  and `deckmaste_spelling`) share the exact same `Arc<SourceFile>` allocations without re-reading
  disk or introducing cross-layer dependencies.

### 2. `crates/semantics` Dependencies & CLI Entry Point

- Depends on `deckmaste_dsl`, `dpsi` (already a workspace dependency of `deckmaste_dsl` for
  `Ident`), and `thiserror` (zero dependency on `deckmaste_core` or `deckmaste_english`).
- Exposes `cargo xtask semantics check` to validate `plugins_v3/builtin` and `plugins_v3/canon`
  from the CLI with `path:line:col` diagnostics.

### 3. Pass 1: Header & Envelope Validation + Signature Registration

Enforces all declaration-level checks deferred from `deckmaste_dsl` (`dsl-parser.md`) and populates
the signature registry (`SylGen/Emit.lean`):
- **Declaration kinds**: Validates the 14 kinds (`primitive`, `template`, `action`, `ability`,
  `ability_word`, `counter`, `designation`, `subtype`, `card_type`, `relation`, `token`, `sba`,
  `conferral`, `card`).
- **Header shape & derived sort**:
  - Only `card` may use the nameless header shape; derives `printedName` for `card` (joining
    multi-face names with `" // "`).
  - Only `primitive` and `template` write `: Sort`; all other kinds derive their result sort from
    `kind` (`action → Action`, `ability → Ability`, `ability_word → Ability`,
    `counter → CounterKind`, `designation → DesignationLabel`, `subtype → Subtype`,
    `card_type → CardType`, `relation → RelationLabel`, `token → TokenSpec`, `sba → SbaRule`,
    `conferral → ConferralRule`, `card → Card`).
  - Only `primitive` may write `= .constructor`, must have no `body`, must not write a redundant
    `= .<same_name>`, and its parameters must match the target constructor's fields 1-to-1 in
    count, name, and sort.
- **Name & field hygiene**:
  - Rejects reserved words (`none`, `true`, `false`) as declaration or parameter names.
  - Rejects duplicate parameter names, duplicate record fields, and record fields not in
    `allowedFields(kind)` (`Emit.lean:18–35`).
  - Validates `identity` (allowed only on a parameterless `NounPhrase` template with a non-empty
    string, i.e. `theActor`).
- **Registry insertion**:
  - Registers `(name, result_sort) → DeclId`, rejecting duplicates within a bundle.
  - Secondary sort registrations:
    - `ability` also registers `(name, KeywordLabel)` and `(name, KeywordTerm)`.
    - `action` (unless `ofAbility` is present or `deed = none`) also registers `(name, Deed)`.

### 4. Pass 2: Expected-Sort Resolution (`dsl::Value` → `Term` + Dependency Graph)

Resolves every declaration's parameter defaults (each default seeing only earlier parameters) and
meaning fields against expected `Sort` (`SylGen/Resolve.lean`), producing a resolved `Term` tree
and recording directed body dependencies `decl_id → callee_decl_id`:
1. **`$name`**: Looks up `name` in the enclosing declaration's parameters; sort must equal
   `expected` (or `U` when `expected = Option(U)`, wrapping in `Some`).
2. **`none` / `some`**: `none` is valid only at `Option(U)` with 0 arguments; explicit `some` is
   always rejected (`"some is implicit; write the value"`).
3. **Head lookup at `(head, expected)`**:
   - Checks bundle declarations `(head, expected)`, then fallback/prelude declarations, then
     `(head, U)` when `expected = Option(U)`.
   - Special sort projections:
     - At `KeywordLabel`, an `ability` declaration resolves to its `KeywordLabel::Named`.
     - At `KeywordTerm` with 0 arguments, a `KeywordLabel` match wraps in `KeywordTerm::The`.
     - At `Deed`, an `action` declaration with a deed resolves to its `Deed::Action`.
     - References to a `KeywordLabel`, `KeywordTerm`, or `Deed` atom do **not** add a body
       dependency edge, so self-referential keyword/deed labels (`hasKeyword(flying)` in `flying`,
       `theRingTemptsYou` in `eventCount`) do not form cycles.
   - Falls back to AST constructors / enum variants `(head, expected)` (and `Delta::{Up,Down,Set}`
     instantiated at `Amount` and `ManaCostTerm`).
   - Enforces `sourceConstructorRefusal` (`Environment.lean:103–122`), preserving the temporary
     canonical-path exception for `builtin/definitions/keyword_action/theRingTemptsYou.syl` while
     the AST is frozen (`semantics-v3.md` §4.5 item 3, `macro-ring-emblem`).
4. **Card & CardFace record shorthands (`Resolve.lean:57–61, 77–81`)**:
   - A record at expected sort `Card` wraps in `Card::SingleFaced`.
   - A non-`singleFaced` `Card` call with a single positional record argument unpacks the record's
     fields as named arguments.
   - A `CardFace` record without a `characteristics` field wraps its fields as `Characteristics`.
5. **`Option(U)` auto-wrapping** and **direct sort atoms**:
   - Strings at `String` / `Ident`.
   - Integers at `Int`, `Nat` (refusing negative), `Amount` (`Amount::Lit`), and `ManaSymbol`
     (refusing negative, wrapping in `ManaSymbol::Simple(SimpleManaSymbol::Generic(n))`).
   - Color words (`white`, `blue`, `black`, `red`, `green`) with 0 args at `Color`,
     `ColorOrColorless`, and `ManaSymbol`.
   - Lists `[...]` at `List(U)`.
   - Records `{ field = val, ... }` validated against the target structure's fields and defaults.
6. **Call argument binding (`Resolve.lean:130–158`)**:
   - Binds arguments in written order: a named argument binds its named parameter; a positional
     argument binds the first parameter not yet bound (`dsl-named-arguments-anywhere`).
   - Refuses unknown, duplicate, excess, or missing required arguments with source spans, then
     orders bound arguments into declaration parameter order (`0..params.len()`).

### 5. Pass 3a: SCC Topological Sort & Action Deed Hygiene

- Computes SCCs / topological order over body dependency edges, refusing any dependency cycle.
- Validates `action` `ofAbility` vs `deed`/`permanent` exclusivity and `DeedPermanent` agreement
  across declarations sharing a deed label (`Registry.lean:143–149`).

### 6. Verification

- `cargo xtask semantics check` resolves all declarations across `plugins_v3/builtin` and
  `plugins_v3/canon` with zero errors.
- Focused negative unit tests in `crates/semantics` cover every header, resolution, argument-binding,
  constructor-refusal, and cycle error in Passes 1–3a (mirroring `lean/SylGen/tests/negative.sh`).
