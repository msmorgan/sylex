---
needs: []
---
# Route every `also` shape of an enters replacement to an owner

## Why

`apply_as_enters` (`crates/engine/src/replace.rs`) folds a permanent's own
`Replacement::Also { would: <it enters>, also }` [CR#614.1c] into its `EnterStatus`. It handles
`Act(Tap)` of itself, a host `Choose` followed by `Act(Attach)` (enters attached), `Act(PutCounters)`
on itself, `If` (the taken branch) and `Sequentially`/`Simultaneously` of those. Every other
`Instruction` falls into one catch-all `todo!`, which can't name an honest owner because the
remaining shapes belong to different work, and the repo forbids catch-all arms.

## Done when

- Vintage-legal cards are surveyed for "As [this] enters …", "[This] enters with …" and
  "[This] enters as …" texts, and each `Instruction` shape they need in `also` is known.
- The catch-all arm is replaced by explicit arms: each shape is folded, or is a `todo!` naming
  its owning ticket (entry-time choices such as "choose a color" are `core-as-enters-choices`;
  face-down entry is `engine-face-down`; a shape with no owner gets a new ticket). A variant no
  card puts there is an arm that fails naming the defect.

## Not in scope

Implementing the folds that the survey routes elsewhere.

## Pointers

- `crates/engine/src/replace.rs` (`as_enters_status`, `apply_as_enters`)
- `crates/core/src/effect.rs` (`Instruction`)
