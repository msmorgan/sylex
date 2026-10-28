/-- "mana of any color", "any one color", "any combination of colors" -/
primitive ofAnyColor (freedom : ColorFreedom) : ProducedMana := {
  -- TODO(spelling): The freedom selects one shared color or a combination of colors, changing the
  -- phrase.
  body :=
    Semantics.Primitives.ProducedMana.anyColor freedom
}
