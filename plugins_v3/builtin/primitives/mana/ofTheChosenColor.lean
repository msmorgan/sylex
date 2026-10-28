/-- "mana of the chosen color" [or <alternative>] -/
primitive ofTheChosenColor (alternative : Option ProducedRun := none) : ProducedMana := {
  -- TODO(spelling): Include or omit the clauses supplied by alternative.
  body :=
    Semantics.Primitives.ProducedMana.ofChosenColor alternative
}
