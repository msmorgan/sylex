---
needs: [discourse-outcomes]
---
# `discourse-conditions`: conditionals, lookbacks, relative/participial clauses, and linked context

Standard constraints apply. Conditionals, lookbacks, and cross-ability context family of
`semantics-rust-rewrite`: implement `Action::DoIf` (`doIf` and `doOnlyIf` timing), `Condition`
stepping, `DidClause` and `"this way"` lookback grounding, `which` relative clauses, Participial
Descriptions, choice context, and linked-ability `textContext` `[CR#607.2a]` in
`crates/deckmaste_discourse`.

## Scope & Design Decisions

1. **Conditionals (`doIf` and `doOnlyIf` order vs. timing `[CR#608.2c]`, `Phrase.lean`)**:
   - In `Action::DoIf { condition, action, otherwise, .. }`, when `action`'s outcomes become
     visible belongs to `DoIf`'s body semantics (`StepPhase.preEvent` vs. `.postEvent` in
     `lean/Semantics/Check/Phrase.lean`), while argument order comes from the enclosing call's
     `read_order` (`doIf` vs. `doOnlyIf`):
     - Pre-step `action` in `.preEvent` to extract announced targets (`det == .target`) and, when
       `action` precedes `condition` in the enclosing call's `read_order` (`doOnlyIf`), record
       referents whose clause or role changed in `action` into `pendingRoles` so `condition` ranks
       them as mentioned in `action` without seeing `.postEvent` moved-object successors or created
       objects (`ActionFamilies.postposedCounterConditionReadsTheUnmovedSpell`).
     - Step `condition` in that pre-event context, then step `action` in the active phase
       (`.postEvent`), and step `otherwise` (if present) seeing referents and announced amounts
       from `condition` (`lean-v3-doif-otherwise-reads-condition-mentions`).
   - In `Action::WithContinuation` (`may`, `doIfDone`), step `body` in `.postEvent` first so
     `if_did` reads the outcomes, moved successors, and created objects produced by `body`, and
     `if_not` reads only the pre-effect announcements.

2. **Lookbacks, `"this way"`, relative clauses, and Participial Descriptions**:
   - Step `Condition::Exists`, `Condition::Did`, `Predicate::Did`, and `Amount::Tally`
     (`DidClause`), verifying `"this way"` grounding against a prior action in the current ability
     or linked scope.
   - Step `NounPhrase::Which` relative clauses by binding the `RelativeHead` referent while
     checking the relative predicate.
   - Resolve Participial Descriptions (`actedOn`, `exiledWith`, `chosenFor`, etc.) against stamped
     referents in the current `DRS` or linked ability context.

3. **Choice context and linked-ability `textContext`
   (`[CR#607.2a]`, `Choices.lean`, `AbilityRules.lean`)**:
   - Compute `choiceContext` and `linkedTextContext` across a card face's abilities so linked
     abilities (`exiledWith thisArtifact`, `"the chosen color"`,
     `"cards exiled with this creature"`) resolve against the writer ability's published referents
     and choices while refusing unlinked or wrong-zone readbacks.

## Tests & Verification

- **Corpus & fixture gate**: Every corpus card and fixture with no remaining `Refusal::Unsupported`
  agrees with `bindings.jsonl` and `fixtures.jsonl` (plus the documented fixes).
- **Conditional timing pins**: `ActionFamilies.postposedCounterConditionReadsTheUnmovedSpell`, plus
  a test where `doOnlyIf`'s postposed condition re-mentions a referent announced by the action.
- **Spec catalog**: Port `docs/semantics-spec-catalog.md` Part 1 §7 (Historical move / `"this way"`
  lookback), §8 (Linked readback), and Part 2 Suites C (Choice, modes, chosen qualities), G
  (Lookback, `"this way"`, participial descriptions), and S (Linked choices, sources, departed
  spells, retention) as native Rust tests.
