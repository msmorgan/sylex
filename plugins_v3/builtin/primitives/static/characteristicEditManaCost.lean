primitive characteristicEditManaCost (cost : Option Semantics.ManaCost) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.manaCost cost
}
