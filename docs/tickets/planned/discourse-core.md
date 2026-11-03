---
needs: [semantics-card-facts, lean-discourse-export, lean-v3-monstrosity-adapt-name-the-source, lean-proofs-spec-catalog, idris-retirement]
---
# `discourse-core`: DRT reference state, word resolution, caller scope, and card entry points

Standard constraints apply. Core DRT ticket of `semantics-rust-rewrite`: create
`crates/deckmaste_discourse` depending on `deckmaste_semantics_v3`, and implement the DRT reference
state (`DRS`, `Referent`, `ReferentId`, `GramRole`, `OwnScopeFrame`), read-order stepping, caller
parameter binding, core noun-phrase and pronoun/demonstrative/reflexive/contrast-anchor resolution
(`docs/decisions/semantics-v3.md` §2 and §4), and the top-level `Card` / `CardFace` / `Ability` /
`Cost` entry points so corpus cards run from the first `discourse-*` ticket.

## Scope & Design Decisions

1. **Crate setup, reference state, and trace export (`Words.lean`, `Trace.lean`)**:
   - Create `crates/deckmaste_discourse` (package `deckmaste_discourse`) importing `Refusal` (and
     its temporary `Refusal::Unsupported` variant) and the context-free AST/frame checks from
     `deckmaste_semantics_v3` (`semantics-card-facts`).
   - Implement `DRS`, `Referent`, `ReferentId`, `GramRole` (`subj`, `obj`, `other`), `Determiner`,
     `Plurality`, `Payload`, `OwnScopeFrame`, and `ActorShape`.
   - Return `Refusal::Unsupported` for AST arms belonging to downstream `discourse-*` tickets
     (`discourse-outcomes`, `discourse-loops`, `discourse-conditions`, `discourse-events-deontics`,
     `discourse-cards-rules`; removed in `discourse-corpus-agreement`).
   - Emit per-card binding traces keyed by expanded core AST path (using the Lean constructor names
     in `deckmaste_semantics_v3::ast::meta`) and role tag (`Intro(k)`, `RelativeHead`, `Letter(l)`,
     etc.) matching `lean-discourse-export`'s `bindings.jsonl` schema.

2. **Read-order stepping (`ExpandedCall` and primitives)**:
   - Every `ExpandedCall` steps its arguments in the declaration's `read_order`.
   - For a primitive constructor inside an `ExpandedCall`, fields that are directly a
     `Param(ParamBinding)` of the enclosing call (`id.call == enclosing_call.call`, not compound
     expressions like `not($condition)`) keep the field positions they occupy in the primitive's
     own `read_order` (`Checked::primitive(ConstructorId)`) and are permuted among those positions
     according to their relative order in the enclosing call's `read_order`, while all other fields
     step in their position in the primitive's `read_order`. `Action::DoIf.postposed` is never
     stepped and never read, so `doOnlyIf` steps `action` before `condition` from `doOnlyIf`'s
     `read_order`.
   - Direct primitive constructor nodes outside a parameterizing call step their fields in the
     primitive declaration's `read_order` (`Checked::primitive(ConstructorId)`).

3. **Caller scope and parameter argument resolution (`ExpandedCall` + `ParamBinding`)**:
   - **Lean defects being fixed**: In Lean (`lean/Semantics/Check/Phrase.lean`), a parameter used
     twice in a Definition's body compiled to `firstMention` and `laterMention`, resolving the
     caller's argument only at `firstMention` in that mention's slot and reusing the binding at
     `laterMention` without checking it against `laterMention`'s slot:
     1. Which use came first decided what the argument meant. Written
        `doIf(is(subject, actorControls), move(subject, ...))`, `sacrifice`'s first mention was the
        `is` condition, which narrows a bare pronoun only for a predicate that demands a card, so
        `it` resolved as a bare pronoun of any kind while the move's subject slot would have
        narrowed it to a permanent (`.atSlot .permanent`). Reordering a Definition's body could
        change what a card's `"it"` referred to.
     2. A later mention was not checked against its slot, so a referent that was not a permanent
        was moved off the battlefield without a refusal.
   - **Requirement**: A parameter's argument is resolved against the constraints of all the
     parameter's uses in the body (at least its action slots), not against whichever use is first,
     and every use is checked against its own slot.
   - `Definition` (`action`, `ability`) pushes an `OwnScopeFrame` onto `DRS` for its
     `ExpandedCall.body`; `Template` stays transparent (no `OwnScopeFrame`).
   - **Declaration-level slot-constraint check**: When checking a declaration, collect the combined
     slot constraints (such as `ActorShape` and `.atSlot` kind requirements) for each `NounPhrase`
     parameter across all its uses in `body`, following the parameter when passed through to nested
     calls. If a declaration's own body uses one parameter in mutually incompatible slots, refuse
     the declaration once.
   - **Call-site `NounPhrase::Param` resolution**: On the first encounter of
     `NounPhrase::Param(ParamBinding { id, value })` for a `BindingId`, resolve `value` (entering
     the caller's `OwnScopeFrame` when `id.call` owns an `OwnScopeFrame` or
     `id.param == ParamIdx::ACTOR`) under the declaration's combined slot constraints for that
     parameter, and bind `id -> Vec<ReferentId>` in `DRS`. On every encounter of `id` (first and
     subsequent), reuse the bound `Vec<ReferentId>` and check those referents against the current
     slot's local requirements.
   - **Other `CallerScopeNode` parameters**: Stepping any non-`NounPhrase`
     `T::Param(ParamBinding { id, value })` enters the caller's `OwnScopeFrame` when `id.call`
     owns an `OwnScopeFrame` and evaluates `value` at its textual use (`Template` bodies stay
     transparent until the post-`lean-retirement` opacity question is decided).

4. **Core word and noun-phrase resolution**:
   - Indefinite introductions (`a`, `allOf`, `bare`, controller/owner possessors, self references).
   - Bare pronoun resolution (`it`, `them`, `they`) via `D'` (`GramRole` hierarchy + clause
     recency) and `J1`/`J2` sort/plurality matching.
   - Demonstrative and definite readbacks (`that <head>`, `those <head>`, `the <head>`, `chosen`)
     via `AC>R1` and kind/type/subtype/status compatibility.
   - Reflexives (`itself`, `themselves`, `yourself`) via `PrincipleA` clause-mate subject binding.
   - Contrast Anchor (`another`, `other` with no explicit `otherThan`) binding to the unique
     in-scope referent matching the head noun's sort and type.
   - `Action::OwnScope` and `Ability::Keyword` scope barriers (`[CR#702.1]`).

5. **Top-level entry points, CLI runner, and sequence order**:
   - Entry points `Card::check`, `CardFace::check`, `Ability::check`, `Action::check`, and basic
     `Cost::check`, combining the context-free refusals from `deckmaste_semantics_v3` with the `DRS`
     refusals from `deckmaste_discourse` and wired into `cargo xtask discourse check`, so the corpus
     runner and fixture runner execute from this ticket.
   - **Line context stepped in order**: Step multi-line ability text and `Action::Sequentially`
     strictly in order during both `check` and line-context threading (fixing Lean's out-of-order
     `lineTextContextWith` bug on *Flooded Strand* and *Afflicted Deserter // Werewolf Ransacker*).

## Tests & Verification

- **Corpus & fixture gate**: Every card in `plugins_v3/canon/cards/` and every equation in
  `fixtures.jsonl` whose combined `deckmaste_semantics_v3` + `deckmaste_discourse` check produces no
  `Refusal::Unsupported` agrees with `lean-discourse-export`'s `bindings.jsonl` and
  `fixtures.jsonl`, except at the documented fixes: `flagDefect` multi-use parameter entries, the
  `context`-only out-of-order readings on *Flooded Strand* and *Afflicted Deserter // Werewolf
  Ransacker*, and `Predicate::Or([])` returning `AtLeastTwo` instead of `NonEmpty`
  (`semantics-card-facts`).
- **Parameter-binding pins**:
  - `sacrifice(it)` with no permanent in scope (or after `draw(1)`) is refused
    `anaphor (.atSlot .permanent) .one 0` whether `sacrifice`'s body names its subject once or as
    `doIf(is(subject, ...), move(subject, ...))` in either order.
  - `CallerScope.expressionAmountObservesNewOutcomes` (`rereadDamage(thatMuch)` with one prior
    damage outcome in scope) refuses `[quantOutcomeInScope 2]`;
    `CallerScope.nestedTemplatesRereadTheCurrentOutcome` refuses
    `[quantOutcomeInScope 2, quantOutcomeInScope 3, quantOutcomeInScope 4]`; and
    `DefinitionsAndTemplates.templateKeepsSingleTarget`
    (`dealDamageOwnPower(target(isCreature), target(isCreature))`) introduces `2` targets.
- **Spec catalog**: Port `docs/semantics-spec-catalog.md` Part 1 §1 (Pronoun resolution), §2
  (Demonstratives and definite readbacks), §3 (Contrast Anchor), §4 (Caller scope), §5 (Binding
  identity), and Part 2 Suites A and R as native Rust tests.
