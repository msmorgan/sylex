/-- "mana of <color>" -/
primitive ManaTypeTerm.ofColor (color : ColorTerm) : ManaTypeTerm := {
  spelling := "mana of <color>",
  body :=
    Semantics.Primitives.ManaTypeTerm.ofColor color
}
