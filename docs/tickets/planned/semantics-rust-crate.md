---
needs: [semantics-rust-resolve, lean-elaborated-value-dump]
---
# [design] Stage 1b of `semantics-rust-rewrite`: typed AST, expansion, and Lean value-dump parity

Standard constraints apply. Second half of Stage 1 of `semantics-rust-rewrite`, building on
`semantics-rust-resolve` (bundle loader, signature registry, expected-sort resolution to `Term`,
and SCC cycle checking): define the typed Semantics AST in `crates/semantics`, expand resolved
`Template` and `Definition` declarations into concrete `Arc` AST values, and verify per declaration
and card that `semantics`'s expansion equals the value Lean elaborates (`lean-elaborated-value-dump`).

- Lean stays in the tree and in CI throughout.
- Spelling-order argument stepping is deferred to `discourse-spelling-order` (after `lean/` is
  retired) so `discourse` steps in the same order as the live Lean oracle during Stage 2.

---

## Design Specification

### 1. Plain Rust AST Representation & Traversal Traits (`crates/semantics/src/ast`)

`semantics` defines the 183 AST types mirroring `lean/Semantics/{Words,Events,Abilities,Card,Rules}.lean`
(10 main recursive enums with 275 variants, plus 173 supporting enums/structs totaling 857 variants):

- **Checked-in plain Rust `pub enum` / `pub struct` declarations**:
  - Bootstrapped via a disposable generator script from the Lean inductives (to prevent field or
    variant transcription drift) and checked in as ordinary, readable Rust types rather than
    wrapping the type definitions inside a giant `macro_rules!`.
  - Uses plain Rust field and variant names (`from`, `to`, `then`, `class`, `end`; `r#where`,
    `r#match`), dropping Lean's keyword-collision trailing underscores (`dsl-plain-word-names`).
    Only the disposable `DumpRender` test formatter maps those 7 names back to Lean's underscored
    names when rendering strings for `values.txt`.
  - **Frozen-AST parity note (`Action::DoIf.postposed`)**: `Action::DoIf` retains `postposed: bool`
    while the AST is frozen for `values.txt` parity and for `discourse`'s initial parity against
    the Lean oracle; it is removed in `discourse-spelling-order` once `lean/` is retired.
- **`Arc` everywhere for size and $O(1)$ expansion cloning**:
  - **List fields**: Explicit `Arc<[T]>` across all AST types (`sequentially`, `and`, `or`,
    `types`, `subtypes`, `text`, etc.). Never alias `type Vec<T> = Arc<[T]>`.
  - **Recursive subtrees & heavy enum payloads**: Wrapped in `Arc<T>` so every AST enum satisfies
    `clippy::large_enum_variant` without suppressions (matching `deckmaste_semantics` v1) and
    multi-mention parameter substitution clones subtrees in $O(1)$ via `Arc::clone`.
  - **Identifiers (`dpsi::Ident`)**: All names and labels (`CardFace.name`, `CardType.name`,
    `Subtype`, `KeywordLabel`, `AbilityWordLabel`, `CounterKind`, `DesignationLabel`,
    `RelationLabel`, `Deed::Action`, `VoteLabel`, `NameSource::PrintedName`, predefined token
    names) use `dpsi::Ident` (`Copy`, interned, $O(1)$ equality/hashing).
- **Generic `ParamBinding<T>` on the 16 `CallerScope` enums (`CallerScopeNode` trait)**:
  - Rather than porting Lean's 16 `.inCaller` constructors, `NounPhrase.firstMention`,
    `NounPhrase.laterMention`, or `SylGen/ReadOrder.lean`, the 16 `CallerScope` AST enums
    (`Action`, `Cost`, `StaticSpec`, `Replacement`, `Amount`, `Predicate`, `Quantity`, `Location`,
    `Condition`, `NounPhrase`, `GameEvent`, `Ability`, `TokenSpec`, `MoveRider`, `PlayPayment`,
    `Ballot`) implement a shared `CallerScopeNode` trait and carry a single uniform parameter
    wrapper variant `Param(ParamBinding<Self>)`:
    ```rust
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BindingId {
        pub call: CallId,    // Monotonic call-expansion instance ID (prevents cross-call collisions)
        pub param: ParamIdx, // Parameter declaration index (0..params.len(), or synthetic actor slot)
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ParamBinding<T> {
        pub id: BindingId,
        pub value: Arc<T>,
    }
    ```
  - **Policy-free facts**: Every `CallerScope` parameter substitution in both `Template` and
    `Definition` (`action`, `ability`) bodies wraps uniformly in `Param(ParamBinding<Self>)`
    (with no `in_caller: bool` flag and no `≥ 2` mention threshold), and the expanded environment
    maps `CallId → CallSiteInfo` (`{ decl: DeclId, kind: DeclarationKind }`). This leaves the open
    opacity and call-site binding questions (`semantics-rust-rewrite`,
    `discourse-parameter-argument-resolution`, `discourse-catalog-open-items`) entirely to
    `discourse`, while `lowering` peels `Param` via `CallerScopeNode::peel`.
- **Generated `AstWalk` / `AstVisit` and `DumpRender` traits**:
  - `AstWalk` / `AstVisit` provides default structural child recursion across all AST types so
    downstream `discourse` passes (`NumberSlots`, `PredicateShape`, `LocalShape`, `CardFacts`,
    `Events`) override only the variants they inspect instead of repeating 2,000+ match arms.
  - `DumpRender` formats any expanded AST value into `DumpValues`'s deterministic constructor-tree
    string `ConstructorName{TypeArg,...}(field1,field2,...)`.

---

### 2. Concrete AST Expansion (Pass 3b)

Building on the topological order computed in `semantics-rust-resolve`, `semantics` expands every
resolved declaration into concrete `Arc` AST values, allocating a fresh `CallId` for each expanded
call and recording `CallId → CallSiteInfo`:
- **`primitive`**: Constructs the bound AST variant/struct directly.
- **`template`**: Wraps every `CallerScope` argument in
  `Param(ParamBinding { id: BindingId { call, param }, value })` and substitutes arguments into the
  body.
- **`action`**: Wraps every `CallerScope` argument in `Param(ParamBinding { id, value })` (plus
  every occurrence of `theActor`'s `identity = "actor"` in its synthetic actor parameter slot) and
  wraps the body in `Action::OwnScope { deed, body }`.
- **`ability`**: Wraps every `CallerScope` argument in `Param(ParamBinding { id, value })`,
  collects `KeywordParam`s (`Cost`, `Amount`, `Predicate` subject/quality, `DeckCondition`,
  `Ability`), and constructs `Ability::Keyword { label, params, body }`.
- **`ability_word`**: Expands through the `abilityWord` template (`Emit.lean:147–154`).
- **`counter`, `designation`, `subtype`, `card_type`, `relation`**: Constructs both the primary
  value (`CounterKind`, `DesignationLabel`, `Subtype`, `CardType`, `RelationLabel`) and its
  companion `RegistryDefinition` (`rules` / `confers`).
- **`sba`, `conferral`, `token`, `card`**: Materializes `SbaRule`, `ConferralRule`, `TokenSpec`,
  and `Card` (stripping `lexical` and `spelling` fields from `card` records).

---

### 3. Verification Gate (`lean-elaborated-value-dump`)

A test in `crates/semantics` reads the gitignored Lean dump at
`docs/evidence/lean-dumps/<label>/values.txt` (`lean-elaborated-value-dump`) and verifies
wrapper-erased tree equality:

- Peels `Param(ParamBinding)` on the Rust side via `CallerScopeNode::peel`, and strips
  `Semantics.<Sort>.inCaller(...)`, `Semantics.NounPhrase.firstMention("<id>", ...)`, and
  `Semantics.NounPhrase.laterMention("<id>", ...)` on the Lean dump string.
- Formats the peeled `semantics` AST via `DumpRender` into `DumpValues`'s deterministic
  constructor-tree string `ConstructorName{TypeArg,...}(field1,field2,...)` and asserts exact
  string equality for:
  - Every `["builtin", bare_name, return_type, sample]` row in `values.txt` (parsing the sample
    argument strings from `sample`, using the parameter's default expression for `"unsampled"` or
    `"default"`, and appending ` rules=<rendered RegistryDefinition>` when the declaration produces
    a companion registry definition).
  - Every `["card", printed_name]` row in `values.txt`, plus asserting that the card count matches
    the `.syl` file count in `plugins_v3/canon/cards/`.
