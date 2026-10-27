primitive tokenSpecDoubleFaced (front : Semantics.CharacteristicBundle) (back : Semantics.CharacteristicBundle) : Semantics.TokenSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.TokenSpec.doubleFaced front back
}
