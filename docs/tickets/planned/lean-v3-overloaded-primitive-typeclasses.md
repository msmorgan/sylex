---
needs: [plugins-v3-builtin-cleanup]
---
# Resolve return-type-overloaded primitives through typeclasses

Standard constraints apply.

Collapse the return-type-overloaded authoring declarations (`nthOccurrence`, `ofColor`,
`and`, `or`, `not`, `andOr`, `did`, `was`, and the remaining shared-name families) through
Lean typeclasses, following `AnyValue` and its bare `any` spelling. Add corresponding Rust
bundle-loader support so both implementations resolve the same declaration from its expected
return type, including optional arguments. Preserve each family's existing signatures,
defaults and expansions; return-type overloading must not depend on load order.

The cleanup's `Option` dispatch and return-sort aliases keep existing declarations usable;
they do not complete this typeclass migration. Inventory the shared-name families and cover
cross-sort calls, defaults and coercions with focused Lean and Rust regressions. Compare
builtin and canon expansions, accounting explicitly for any retired declaration keys.
