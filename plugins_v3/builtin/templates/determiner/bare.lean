/-- the bare plural: "creatures", "players can't gain life". Its rules meaning and its plural
referent are those of "all creatures" (`allOf`), whose tree it shares. -/
template bare (p : Predicate) : NounPhrase := {
  -- TODO(spelling): creatures; wording varies with p, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.NounPhrase.described Semantics.Templates.DetPhrase.detPhraseAll p
}
