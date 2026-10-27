/-- "mana of the chosen color" [or <alternative>] -/
primitive ofTheChosenColor (alternative : Option ProducedRun := none) : ProducedMana := {
  -- TODO(spelling): mana of the chosen color; wording varies with alternative, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.ProducedMana.ofChosenColor alternative
}
