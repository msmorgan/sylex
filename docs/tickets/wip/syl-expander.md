---
needs: []
---

# Expand `.syl` terms, and pin properties of expansions in tests

## Why

`docs/decisions/syl.md` §4 defines expansion: a Template or Definition call is replaced by its
body, its arguments bound to its parameters. `cargo xtask dsl check` resolves every name at its
expected sort, but nothing expands, so nothing can be said about what a term amounts to once its
vocabulary is unfolded. Some such claims carry the model. "Die" is an observation about a thing
that occurs, as against an instruction to someone to do a deed; destroying is moving a permanent
from the battlefield to its owner's graveyard [CR#701.8a], which is what dying is [CR#700.4].
Tests over expansions pin properties like these before any lowering exists, so a declaration
that breaks one is caught where it is written.

## Design

- Expansion takes a resolved term to a tree of primitive calls as plain data. It needs no Rust
  AST for the card language: each node is named by its primitive's constructor clause
  (`syl.md` §1).
- The tree keeps a mark where a Definition was entered, carrying the Definition's Deed: the place
  a Deed stamp goes (`docs/decisions/semantics.md` §3, §3.1). Defaults are filled in and
  coercions applied.
- Parameters are bound as `syl.md` §4 states: each argument is held once, at a binding the
  expansion generates, and each use of the parameter refers to that binding; no use is marked
  first or later, and call and parameter boundaries are kept.
- A command prints the expansion of one card or of one term.
- The tests state structural properties of expansions. One operation is allowed in a test before
  comparing: binding the subject word (`actor` at `NounPhrase` today) to a value. First
  properties:
  - the expansion of `destroy(X)` contains, inside its Definition mark, the expansion of `die`
    with its subject bound to X. Today `destroy`'s body is
    `move($subject, zone(battlefield, none), graveyard, [])`, where `graveyard` is a Template at
    `Location` whose body is `zone(graveyard, none)`, and `die`'s is
    `move(actor, zone(battlefield, none), zone(graveyard, none), [])`. The test states whether
    the two are one value after expansion; a difference is a finding to report, not to paper
    over;
  - every keyword action's expansion carries exactly one Definition mark of its own Deed;
  - a verb phrase that observes something occurring (`die`, `enter`, `leave`) expands with no
    `instruct` and no performer in it.

## Open questions

- Where the expander lives and what it starts from. `dsl check`'s resolver
  (`crates/xtask/src/dsl_check.rs`) checks and keeps no resolution, and its module doc calls it
  temporary until the Rust resolution layer replaces it. Options: the resolver records what each
  name resolved to and the expander, in `xtask`, reads that; or resolution and expansion move to
  `sylex_syl`. The untracked cleanup plan (`docs/cleanup/07-rust-ast.md`) puts the resolved
  tree and the expander on step 7's Rust AST, with a derive crate and RON as the expanded form's
  serialization; whether that step later replaces this expander or builds on it.
- What "one value" means in the first property. §4 keeps every call and parameter boundary, so
  `destroy(X)` keeps the boundary of the `graveyard` Template and of the `$subject` binding, and
  `die` the boundary of its own Template. Options: compare with Template and parameter
  boundaries erased, Definition marks kept; compare exactly; or let each property say which
  boundaries it ignores.
- What a performer is in the third property. `actor` at `NounPhrase` is declared as "whoever
  performs this action: the actor of the current instruction" (`primitives/pronoun/actor.syl`),
  and `semantics.md` §2.3 says it reads the innermost Actor; `die`, `enter` and `leave` use it
  for the thing that dies, enters or leaves. Options: `actor` counts as a performer, and the
  three bodies fail the property today; or `actor` there is the clause's subject, and its
  declaration is reworded or split into a subject word and a performer word.
- Which declarations "every keyword action" ranges over. All 90 `action` declarations are filed
  in `definitions/keyword_action/`, but a folder carries no meaning (`syl.md` §2); `resolve` is
  not a keyword action (`syl-core-deeds-out-of-keyword-actions`); `regenerate` has `deed = none`
  and its body reaches `destroy`'s Deed, while `regeneration` carries the `"regenerate"` Deed
  (`syl-explicit-deeds`); `devour` and `beamUp` name their keyword ability by `ofAbility` and
  declare no Deed of their own; and variants share a Deed label (`deed = "exile"`). Options: every
  `action` declaration, those with a Deed of their own, or the keyword actions the CR defines
  [CR#701.1].
- What §4 leaves undefined:
  - a defaulted parameter: whether its default, written in the declaration, is read in the
    caller's scope like an argument (a Definition wraps every parameter for caller-scope
    reading) or in the declaration's;
  - a coercion: whether its expansion keeps a boundary as a Template call does;
  - a parameter the body never uses: whether its argument is held at a binding that nothing
    refers to (`syl-unused-parameter-refusal` refuses most such parameters, and
    `syl-explicit-deeds` exempts a Deed's);
  - a value at an `Option` position written bare (`syl.md` §3): whether the expansion records
    `some`;
  - how a test writes an opaque value X to pass as an argument, and how a term with free
    parameters (a declaration's body alone) is printed.
- How a property that today's corpus violates lands: the declaration is fixed in this ticket, or
  the violation is ticketed and the test is `#[ignore]`d naming that ticket.

## Done when

- Every builtin declaration and every canon card expands, and the print command shows the
  expansion of a canon card that uses `destroy` and of the term `destroy(X)`.
- The three properties above are tests over every declaration they range over. The landing
  reports, for each, the declarations that violate it, and whether `destroy(X)` and `die` with
  its subject bound to X are one value, with the difference if not; each violation is handled as
  the owner rules under the last open question.
- A test shows that an argument used twice in a body is held once, with both uses referring to
  that binding (`syl.md` §4).
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate;
  `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Not in scope

- Lowering to `core`, referents and Reference Scopes, and read order.
- Any behavior: that indestructible stops a destroy, that a trigger fires.
- Reshaping Deeds (`syl-explicit-deeds`, `syl-one-deed-per-keyword-action`).

## Pointers

- `docs/decisions/syl.md` §1, §3, §4; `docs/decisions/semantics.md` §2.3, §3, §3.1.
- `crates/xtask/src/dsl_check.rs` (`Resolver`), `dsl_check/schema.rs` (`KINDS`, the `action`
  kind's `deed` field, `EXPANSION_CONSTRUCTORS`), `dsl_check/coercion.rs`, `dsl_check/cycles.rs`;
  `crates/syl/src/`.
- `plugins/builtin/definitions/keyword_action/{destroy,regenerate,regeneration,devour}.syl`,
  `plugins/builtin/templates/action/{die,enter,leave}.syl`,
  `plugins/builtin/templates/zone/graveyard.syl`, `plugins/builtin/primitives/pronoun/actor.syl`,
  `plugins/builtin/primitives/action/{move,instruct}.syl`.
