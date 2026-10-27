/-- "mana of any color", "any one color", "any combination of colors" -/
primitive ofAnyColor (freedom : ColorFreedom) : ProducedMana := {
  -- TODO(spelling): mana of any color; wording varies with freedom, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.ProducedMana.anyColor freedom
}
