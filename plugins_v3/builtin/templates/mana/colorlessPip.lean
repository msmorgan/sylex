/-- "{C}" -/
template colorlessPip : ManaSymbol := {
  spelling := "{C}",
  body :=
    Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.specific Semantics.ColorOrColorless.colorless)
}
