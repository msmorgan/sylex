---
needs: []
---
# Typed dispatch for the verbs the engine interprets

## Why

The engine recognizes the verbs it gives semantics to by comparing interned names to string
literals: `verb.as_str() == "Destroy"`, `name.as_str() == "Discard"`, and the same for `Mill`,
`Draw`, `Fight` and `Scry`, in production paths (`cast.rs`, `activate.rs`, `decide/mod.rs`,
`payment/fulfill.rs`, `eval.rs`, `sba.rs`, `replace_registry.rs`, `step/act.rs`, `step/mod.rs`)
and in many tests. A typo at one site silently disables that behavior, and nothing lists which
verbs the engine interprets.

Some names are open-ended authored names that the engine passes through; those stay open.

## Done when

- The closed set of verbs the engine assigns semantics is one typed vocabulary (an enum or table)
  with a single mapping to its interned spellings, round-trip tested.
- Production dispatch on those verbs goes through the typed variants, with no string literal
  comparisons left for them; tests match on the typed variants too.
- Open-ended identifiers keep their current behavior, and a test pins that an unknown verb is not
  mistaken for an interpreted one.
- Each migrated dispatch family has a focused test.

## Pointers

- `rg -n 'as_str\(\) == "' crates/engine/src`
- `crates/core/src/event.rs` (`Act`), `crates/core/src/action.rs` (`Action::Composite`)
