/-- "except it has <ability>" -/
primitive alsoHas (ability : Ability) : CopyExcept := {
  spelling := "except it has <ability>",
  body :=
    Semantics.Primitives.CopyExcept.ability ability
}
