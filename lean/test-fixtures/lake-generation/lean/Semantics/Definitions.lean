/-!
Stub of `Semantics.Definitions` for the Lake generation regression (`cargo xtask lean check`,
`crates/xtask/src/lean.rs`). It runs the real `lean/lakefile.lean` against this tree: `genCanon`
writes data modules in this `canon_card`, and check modules in `check_canon_card`
(`Stub.CanonCheck` below stands in for `Semantics.CanonCheck`).
-/

namespace Semantics.Templates
end Semantics.Templates

namespace Semantics.Definitions
end Semantics.Definitions

syntax (name := stubCard) "card" " := " term : command

/-- A card file `<name>.lean` holding `card := <n>` defines `«<name>.lean» : Nat := <n>`. -/
macro "canon_card " file:str ppLine body:command ppLine "end_canon_card" : command => do
  let `(stubCard| card := $value) := body | Lean.Macro.throwUnsupported
  `(def $(Lean.mkIdent (.mkSimple file.getString)) : Nat := $value)
