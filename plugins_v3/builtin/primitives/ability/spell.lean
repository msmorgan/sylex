/-- An instant or sorcery's spell ability: `body`, as it resolves [CR#113.3a]. -/
-- [CR#113.3a]
primitive spell (body : Action) (timing : Option Timing := none) : Ability := {
  body :=
    Semantics.Primitives.Ability.spell timing body
}
