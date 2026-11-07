---
needs: []
---
# Make broken core unrepresentable or loud in the engine

## Why

[Core](../../decisions/core.md) law 5 and [Engine](../../decisions/engine.md) law 6: a
malformed shape that core's types can rule out is a compile error, the rest is
refused at load, a run-time impossible state is a lowering or engine bug that
fails loudly, and only what the rules make impossible is ignored [CR#101.3].
Many engine sites still collapse broken core (a missing register, a
wrong-kind value, an unknown verb) into the same empty read a departed object
gives, often with only an `eprintln!`. The engine never calls
validate-at-load, so malformed core reaches play.

## Done when

- Each site under Pointers is classified as a data-shape defect or a
  rules-impossible game state, and behaves accordingly:
  - A data-shape defect (a wrong-kind register value, an unknown verb name)
    first becomes a type or compile error where core's types can rule it
    out. What the types cannot reach is refused at load or fails loudly with
    an error naming the defect.
  - A game-state read the rules make impossible (a departed object, an
    unpayable cost, a noncreature permanent's power [CR#208.3]) is a rules
    outcome: that part does nothing [CR#101.3]. The "can this be a type?"
    question never applies to these.
- The engine's entry points (`GameConfig` / `GameState::new` and card
  loading) run `sylex_core::validate` and its siblings over every card, keyword
  abilities included, and refuse a malformed card before play.
- `unbound_reference_degrades_to_null_not_panic`,
  `a_value_read_over_an_unwritten_register_degrades_to_null_not_a_panic`,
  `unresolvable_ref_in_filter_fizzles_instead_of_asserting` and
  `unavailable_declared_number_parameter_fizzles_to_zero` are rewritten or
  retired, each named with its reason in the landing.

## Not in scope

- Copying an activated ability whose source has left:
  `engine-copy-ability-source-gone`.
- Legal mechanics the engine has not built yet; their `todo!` panics are
  already loud.

## Pointers

- `crates/engine/src/activation.rs`: `activation_product`,
  `activation_objects`, `activation_number`, `activation_symbol` (missing
  activation, out-of-range register and wrong kind all read as empty; an
  `Objects` value read as one object takes the first).
- `crates/engine/src/resolve/query.rs`: `eval_reference_product` (`Reg` miss
  becomes null), `unbound_ref`, `unbound_group`, and `Reference::Single` on an
  empty or ambiguous selection.
- `crates/engine/src/resolve/count.rs`: `Count::Reg` `unwrap_or(0)`; the
  missing-note comment near the `TimesPaid` arm; the `StatOf(Power)` and
  `StatOf(Toughness)` `.expect`s, where a noncreature permanent has no power
  or toughness [CR#208.3] but a creature with no printed toughness is an
  authoring mistake.
- `crates/engine/src/resolve/action.rs`: the unknown-verb-name arm of the
  composite-verb dispatch; the zoneless-member skips in `MoveGroup` and `Move`.
- `crates/engine/src/legal.rs`: the `attachment_legal` re-entrancy guard for a
  bootstrap-impossible aura.
- `crates/engine/src/target.rs`: `resolve_frameless_reference` returning
  `None` for shapes it cannot evaluate.
- `crates/engine/src/resolve/player_action.rs`: the reveal whose player does
  not resolve.
- `crates/engine/src/resolve/effect.rs`: the `Instruction::Continuously` arm,
  which silently drops a `ForThisEvent` duration with no host instruction;
  the unknown-verb arm of `batch_act_head`.
- `crates/engine/src/state.rs`: `GameConfig`, `GameState::new`.
- `crates/core/src/region.rs`: `validate`, `validate_static`, `validate_sba`
  and their siblings.
