/-- "mana of <color>" -/
primitive ofColor (color : ColorTerm) : ManaTypeTerm := {
  spelling := "mana of <color>",
  body :=
    Semantics.Primitives.ManaTypeTerm.ofColor color
}
