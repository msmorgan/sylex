/-- "mana of the last noted type" -/
primitive ofTheLastNotedType (subject : NounPhrase) : ProducedMana := {
  -- TODO(spelling): mana of the last noted type; wording varies with subject, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.ProducedMana.lastNoted subject
}
