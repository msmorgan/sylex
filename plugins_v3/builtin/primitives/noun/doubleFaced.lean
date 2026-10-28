primitive doubleFaced (front : CharacteristicBundle) (back : CharacteristicBundle) : TokenSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.TokenSpec.doubleFaced front back
}
