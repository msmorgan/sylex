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

- The expander is a throwaway built ahead of the Rust AST for the card language. It lives in
  `xtask`, reads what `dsl check`'s resolver resolves (the resolver records its resolutions for
  it), and produces plain data. When the proper expansion on the Rust AST exists, this code is
  deleted and the property tests are rewritten against that target; the tests' content is what
  lasts. The module's doc comment says so.
- It is expansion-lite. It substitutes each Template and Definition call by its body with the
  arguments in place, fills declared defaults, applies coercions, and keeps exactly one kind of
  boundary: a mark where a Definition was entered, carrying the Deed its declaration states,
  possibly none. Template, parameter-binding and coercion boundaries are not kept. It does not
  model binding and re-mention, scopes, referents, or anything else `syl.md` §4 says about how
  an argument is read.
- Where §4 is silent the lite expander takes the plain structural reading: a default is
  expanded where the callee declares it, reading the callee's earlier parameters; an unused
  parameter's argument appears nowhere; a value written bare at an `Option` position is the value
  itself, with no `some`. The proper expansion must define each of these.
- Two expansions are the same content when they are equal as trees.
- Each node is named by its primitive's constructor clause's word (`syl.md` §1).
- A command prints the expansion of one card or of one term.
- The tests read the clause's subject through one named constant for the word (declared today as
  `actor` at `NounPhrase`). That word means the subject of the clause and is to be named
  `subject`; the rename is `syl-misnamed-declarations`, and in these tests it is a one-line change.
  A test binds the subject word to an opaque value by plain substitution.
- Properties:
  - the expansion of `destroy(X)` contains, inside its Definition mark, the expansion of `die`
    with the subject word bound to X;
  - expanding a call to a Definition yields exactly one mark for that call, carrying the Deed its
    declaration states, and a Definition reached inside its body keeps its own mark nested
    inside. This runs over every `action` declaration;
  - an observation of something occurring (`die`, `enter`, `leave`) expands to a tree with no
    `instruct` in it and no participant word other than the subject word.
- A property that today's declarations violate is a finding for the owner, who rules whether the
  declaration or the property is wrong; the test is neither weakened nor silently ignored.

## Done when

- Every builtin declaration and every canon card expands, and the print command shows the
  expansion of a canon card that uses `destroy` and of the term `destroy(X)`.
- The three properties above are tests over the declarations they range over, and the landing
  reports each one's result.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate;
  `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Not in scope

- Lowering to `core`, referents and Reference Scopes, binding and re-mention, and read order.
- Any behavior: that indestructible stops a destroy, that a trigger fires.
- Reshaping Deeds (`syl-explicit-deeds`, `syl-one-deed-per-keyword-action`).

## Pointers

- `docs/decisions/syl.md` §1, §3, §4; `docs/decisions/semantics.md` §2.3, §3, §3.1.
- `crates/xtask/src/dsl_check.rs` (`Resolver`), `dsl_check/schema.rs` (`KINDS`, the `action`
  kind's `deed` field), `dsl_check/coercion.rs`; `crates/syl/src/`.
- `plugins/builtin/definitions/keyword_action/{destroy,regenerate,regeneration,devour}.syl`,
  `plugins/builtin/templates/action/{die,enter,leave}.syl`,
  `plugins/builtin/templates/zone/graveyard.syl`, `plugins/builtin/primitives/pronoun/actor.syl`,
  `plugins/builtin/primitives/action/{move,instruct}.syl`.
