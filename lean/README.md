# Semantics workbench in Lean 4

The semantics workbench models the card language's syntax and checks its
structural obligations. `Semantics/*.lean` contains plain inductives;
`Semantics/Check/*` computes bindings, kinds, zones, and refusals over that
syntax. `Card.check` returns every refusal, and each generated card has a separate
kernel-checked proof that its refusal list is empty. Runtime game execution
remains in Rust.

Run `lean/scripts/build` from the repository root, or `./scripts/build` from
this directory. It builds every library (syntax and checker, builtin definitions and their
`ok_` theorems, the generated card bench, and the pin suites) with warnings treated as failures.
The inner loops, from this directory, are `lake build -q Semantics` for checker work, a bare
`lake build -q` (the default targets `Semantics`, `SemanticsDefinitions` and
`SemanticsCheckBuiltin`) for builtin work, `lake build -q SemanticsCards` for card work, and the
editor or `lake build -q SemanticsProofs` for proofs. `lakefile.lean` lists them.

Lean is the active workbench.

## Workbench gate

`cargo xtask lean check` runs `lake build -q --wfail` over every library in `lean/`, the same
full workbench gate as `lean/scripts/build`, and propagates build failures. It then reruns the
Lake generation regression (`crates/xtask/src/lean.rs`): the real `lakefile.lean` over the stub
card tree in `test-fixtures/lake-generation/`, checking that a clean tree builds, that a card
edit rebuilds exactly its data and check shards, that a fixture-card edit rebuilds `Fixtures`
and the proof importing it, and that `lake setup-file` regenerates.

The obsolete `plugins_v2` card/definition emitters and macro generator have
been retired. `lean definitions` and `lean macros generate|check|align` are no
longer commands. The v3 declarations live in `plugins/builtin/`, one `.syl` declaration
per file. `Semantics/Definitions.lean` imports the generated bundle, placing primitives and templates in
`Semantics.Templates` and the rules-defined vocabulary in `Semantics.Definitions`.
v2 RON output must not regenerate them. Existing v2 readers elsewhere remain until their
consumers are cut over.

### Retired tooling tests

The following tests retired with their implementations. The native Lean
proofs and card bench continue to run; the new command tests cover successful
builds, failed builds and failure to launch Lake.

- `lean_macros` (unit): `every_param_type_has_a_lean_reader`, `every_syntax_kind_has_a_lean_reader`, `the_committed_module_matches_the_declarations`, `a_hole_is_the_whole_argument`.
- `lean_check` (unit): `a_plugin_directory_names_a_lean_module_component`, `a_lean_diagnostic_keeps_its_position_severity_and_whole_message`, `a_warning_is_a_diagnostic_too_because_the_build_treats_it_as_one`, `a_diagnostic_lands_on_the_card_whose_block_it_sits_in`, `a_card_that_fails_to_elaborate_records_the_failing_head`, `a_diagnostic_above_the_first_card_is_a_gate_defect`, `a_diagnostic_in_a_file_no_card_owns_is_a_gate_defect`, `a_lake_failure_that_names_no_card_is_a_gate_defect`, `a_module_that_was_neither_built_nor_diagnosed_is_a_gate_defect`, `a_report_fails_iff_any_card_did_not_prove`.
- `definition_check` (unit): `emit_keeps_the_build_cache`.
- `lean_check` (integration): `the_testing_plugin_proves_every_card`, `a_card_that_breaks_a_card_law_is_singled_out_and_the_gate_fails`, `a_lake_that_fails_without_naming_a_card_stops_the_gate`, `a_real_run_recovers_after_a_stubbed_lake_failure`, `every_builtin_registry_definition_proves`, `a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`, `a_lake_that_fails_without_naming_a_definition_stops_the_definition_gate`.

The macro tests checked v2 reader parity, parameter-hole extraction and exact
v2 regeneration. The card/definition tests checked the retired v2 emitter,
its diagnostic attribution and generated-file handling. These contracts no
longer apply to the native v3 workbench.

## Syntax, templates and definitions

The syntax modules are `Words`, `Events`, `Abilities`, `Card`, and `Rules`;
`Primitives` supplies small Lean proof helpers; `.syl` uses explicit wrappers. The builtin bundle's
`primitives/` files expose individual constructors; `templates/` files compose those bindings
into phrasings. `definitions/` contains rules-defined vocabulary (card types, keyword actions
and abilities, counters, tokens and designations), and `rules/` contains rule rows.
A `primitive` binds a constructor; ordinary calls resolve by expected sort. A constructor earns
its place through an obligation or structural distinction its expansion would not carry.
Common phrasings such as "permanent", "colorless", "spell" and "attacking" expand into shared
predicates.

Actions and their templates and definitions use dictionary forms (`exile`, `mill`, `gainLife`);
static-spec templates use nouns (`modification`, `abilityGrant`), except that the deontic
templates use their modal (`cant`, `can`, `must`, `gatedBy`) and a replacement effect is
`replace`, the word it shares with a self-replacement action, its replacement marked `instead`
where the text says so (`replace (would you (draw 1)) (instead (draw 2))`).
Event constructors retain their predication forms. Action composition uses `sequentially` and `simultaneously`; fixed repetition
lives in `Repetition.fixed`. No action writes its performer: it is
`actor`, the controller unless an enclosing `instruct performer body` hands it to
another player or a permanent; "target player draws a card" is
`instruct (target isPlayer) (draw 1)`. A choice's chooser and an added
turn's taker are the actor too ("target player chooses a creature" is
`instruct (target isPlayer) (choose (a isCreature))`). `choose` accepts
`(disclosure := .secretly)`.

Combat predicates and events carry a `CombatRelation`; `statOf` takes a
`ProjAxis`; arithmetic takes an `ArithOp`; `Predicate.or` joins alternatives
with their own kinds. A designation is a catalog value whose declared scope the checker reads
off the value (see "Registry facts"). `Characteristics` is flat and follows
[CR#109.3]. Printed stat slots use `Option Amount`: `none` represents a printed
`*`, with a Characteristic-Defining Ability (`StaticSpec.characteristicDefining`)
supplying its value. `CardFace`
adds what a printed face carries; `CharacteristicBundle` describes what an
effect writes, including token qualities [CR#111.3].

## Rules tables

`Rules.lean` is the rules-as-data a plugin authors beside its cards: an
`SbaRule` [CR#704.1], a `ConferralRule` [CR#306.5b], and a `PredefinedToken`
catalog entry [CR#111.10]. The results of damage [CR#120.3] are conferrals
of a property, not an ability: to a player, a table row; to a creature,
planeswalker or battle, on that card type's `RegistryDefinition.cardType` row. Each row is scoped by a `Predicate`
instead of printed on a face, and `Check/Rules` checks it with the same
functions a card's text obeys — the row's own obligation is only what the scope adds:
the kind it may bind. A conferral is an ordinary `Ability` or `StaticSpec` and a
catalog entry an ordinary `CharacteristicBundle`, checked through
`TokenSpec.written`; neither is a twin type. The pins are `Proofs/Rules`, whose
bench items are the rows `v1/plugins/builtin/rules` and `v1/plugins/builtin/tokens`
write.

## Registry facts

There are no registry tables. Keyword actions, keyword abilities, counters, designations,
relations and subtypes are each a self-contained declaration in `plugins/builtin/definitions/` whose
value carries its own columns (a counter's holder, a designation's scope, whether a permanent
performs a keyword action). A keyword
ability's Definition generates its label, and its typed signature fixes its parameters;
an Action Definition generates its deed. `.syl` uses their bare names at the expected
sort (`kicker` at `KeywordLabel`, `exile` at `Deed`). Generated Lean exposes
`kicker.label` and `exile.deed` for proofs and inspection. The catalog pins
(`Semantics/Proofs/Tables.lean`) remain part of `lean/scripts/build`.

## Numbers

The only numbers the game uses are integers [CR#107.1], so `Amount.lit`
carries an `Int` and a printed negative face is spellable — Spinal Parasite's
−1/−1, Char-Rumbler's −1 power — while a written `*` stays `none`. What
[CR#107.1b] adds is that most positions clamp: a calculation that determines
the result of an effect yields zero rather than a negative, except where the
effect sets, doubles, or triples a life total or a creature's power and
toughness. So every position that consumes an `Amount` declares its regime as
data the engine reads, in `Action.numberSlots`, `StaticSpec.numberSlots`,
`Replacement.numberSlots`, `Cost.numberSlots`, and `GameEvent.numberSlots`: `clamped` for damage, life
gain and loss, draws, counts, costs, and choices; `signed` for
an effect that sets a value, for exchanges of values, for the raw reads
and comparisons inside an `Amount`, which are calculations and not effects,
and for the value the letter X is defined as, which the slot consuming X
clamps or not.
Nothing here is a refusal — "gains −1 life" means gain 0, and that meaning is
the slot's declared regime. The pins are `Proofs/Numbers`.

Present power, toughness, loyalty, and defense in a printed box must be
`Amount.lit`; a nonliteral receives `cardBox`, including on shared-line halves,
level bands, and prototype frames. Effect-written characteristics keep their
context-aware expression checks. `Parity` is the even/odd number vocabulary,
also used when a deck condition tests mana values.

## The checker

1. **Syntax** (`Semantics/*.lean`) supplies the card-language terms.
2. **Attributes** (among them `Check/Words`, `Check/PredicateShape`, `Check/Selections` and
   `Check/Queries`) hold the discourse state (`DRS`), the anaphor resolver, and projections such
   as kind, number, zone and type. The stepper (`Check/Phrase`, whose action, cost, static and
   ability entry points `Check/Abilities` re-exports) threads the `DRS` through each term in
   reading order, so field contexts determine which earlier introductions each child can read.
3. **Rules** (`Check/Phrase`, `Check/Events`, `Check/Keywords`, `Check/AbilityRules`,
   `Check/Card`) check those projections and return named `Refusal` values (`Check/Refusal`);
   `Check/Rules` checks the rules-as-data rows. Laws read structural features and the columns
   a catalog value carries.
4. **Entry point**: `Card.check : Card → List Refusal`. Generated check modules
   prove `Card.check card = []` separately from the card data modules.

`Predicate.kind?` and `NounPhrase.kind?` infer kinds. A context expecting a
kind refuses a `kindMismatch` when they disagree. Each disjunct of a joined
phrase is checked at its own kind. The [checker contracts](CONTRACTS.md)
record field contexts, enclosed scopes, and the checks or planned redesigns
that enforce them.

## Pins

A positive pin proves that the exact refusal list is empty; its negative twin
proves the exact expected list. For example, `Proofs/Anaphora.lean` contains:

```lean
theorem badChooseYou :
    Action.check (DRS.ofList []) (.choose none .you .openly none) = [.choiceClause] := by
  decide
```

`okX` / `badX` names and card sentences are retained when a term is re-spelled.
Changing the syntax does not authorize weakening the asserted result or
removing its twin. `Proofs/Tables.lean` reads catalog declarations back through the
accessors the checker uses (a counter's holder, a designation's scope, the keyword actions a
permanent performs, a card type's permanence and stats). `decide` evaluates these laws in the
kernel.

## Cards

`plugins/canon/cards/<name>.syl` contains one declaration per printed card.
`sylGen` parses and resolves the corpus, then emits plain Lean definitions under
`.lake/build/gen/Generated/`. `Generated.Canon` imports the card data;
`Generated.CanonChecks` imports their separate checker proofs. Lake tracks the
resolved dependencies and rebuilds affected modules.

`Semantics.Registry` records generated declarations and card source names for the
dump tool and `check_canon_card`. It also verifies consistent action deeds.
`SylGen/Tests/Pins.lean` and `SylGen/Tests/negative.sh` test dispatch, defaults,
scopes, mentions, and rejection of invalid input. See
[Card authoring and trusted expansion](CONTRACTS.md#card-authoring-and-trusted-expansion).

## Fuel is load-bearing

`decide` asks the kernel to evaluate the checker, and the kernel cannot unfold
well-founded recursion. The phrase, trigger and ability grammar is one mutual block in
`Abilities.lean`, elaborated by the scoped `mutual` elaborator `open scoped Semantics.FastInductives`
installs, which omits the eager `brecOn`, `SizeOf` and injectivity theorems the block's nested
occurrences would make expensive. Without `brecOn` nothing recurses structurally on a spine
type, so every recursive function over the syntax, in `Check/*` and beside the syntax
(`Location.positiveZones?`), takes a `fuel : Nat` and carries `termination_by structural fuel`;
a function that would fall back to well-founded recursion is a build error, not a slow proof.
The block also derives no instances: `Amount`, `Predicate`, `Action` and the rest have no
`DecidableEq`, `BEq` or `Repr`, so a pin equating two terms closes by `rfl`, not `decide`
(`Proofs/Numbers`).

Fuel bounds nesting depth, not the length of a card's text. The stepper (`stepOps`,
`Check/Phrase.lean`) spends one unit per level of nested phrase, action, cost or ability, and
when it runs out refuses `recursionLimit` rather than admitting what it did not read
(`exhaustedTraversalRefuses`, `conditionOverTooDeepReplacementIsRefused`). The entry points
pass 64; a pin over a deep fixture may pass more (`stepAction 512 …`). An attribute reader
(`Predicate.seedTy` and its kin) returns its no-information value when its fuel runs out: no
card types, no zone, `false`. A term the bench needs read deeper raises the entry point's
fuel; it never moves a function to well-founded recursion.

## Conventions

- Syntax types live in `namespace Semantics`; definition collections use
  their module path, such as `Semantics.Templates` and `Semantics.Proofs.Anaphora`.
- Types use UpperCamelCase; constructors and functions use lowerCamelCase in
  the type's namespace. Fields and arguments use descriptive words.
- Lean keywords used as names take a trailing underscore, such as `if_`,
  `from_`, and `repeat_`.
- Write templates, definitions, and closed word-tag constructors bare (`creature`, `legendary`,
  `sets`, `upkeep`); write semantic-expression and internal constructors with a leading dot in
  checker and proof code (`.hasType`), and keep `.some` on `ColorSpec` (`.some [red]`, `.some []`)
  and `zone .exile none` in `exileZone`. The bench opens `Semantics.Templates` and
  `Semantics.Definitions`; it does not open syntax namespaces whose constructors would shadow core
  names.
- Syntax is unindexed. Contexts and inferred attributes are explicit inputs
  and results of the checker. Sort-specific constructors retain the
  distinctions needed to check their fields.
- Types outside the spine's mutual block derive `Repr` and `BEq`, and `DecidableEq` where
  derivation supports their shape; the block derives none (see "Fuel is load-bearing").
  Rules-bearing docstrings use bracketed Comprehensive Rules citations.
